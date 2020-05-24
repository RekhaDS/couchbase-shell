//! The `kv-upsert` command performs a KV upsert operation.

use super::util::convert_nu_value_to_json_value;

use crate::state::State;
use couchbase::UpsertOptions;

use futures::executor::block_on;
use futures::{FutureExt, StreamExt};
use nu_cli::{CommandArgs, CommandRegistry, OutputStream};
use nu_errors::ShellError;
use nu_protocol::{MaybeOwned, Signature, SyntaxShape, TaggedDictBuilder, UntaggedValue};
use nu_source::Tag;
use std::sync::Arc;

pub struct KvUpsert {
    state: Arc<State>,
}

impl KvUpsert {
    pub fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}

impl nu_cli::WholeStreamCommand for KvUpsert {
    fn name(&self) -> &str {
        "kv upsert"
    }

    fn signature(&self) -> Signature {
        Signature::build("kv upsert")
            .optional("id", SyntaxShape::String, "the document id")
            .optional("content", SyntaxShape::String, "the document content")
            .named(
                "id-column",
                SyntaxShape::String,
                "the name of the id column if used with an input stream",
                None,
            )
            .named(
                "bucket",
                SyntaxShape::String,
                "the name of the bucket",
                None,
            )
            .named(
                "content-column",
                SyntaxShape::String,
                "the name of the content column if used with an input stream",
                None,
            )
    }

    fn usage(&self) -> &str {
        "Upsert a document through Key/Value"
    }

    fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        block_on(run_upsert(self.state.clone(), args, registry))
    }
}

async fn run_upsert(
    state: Arc<State>,
    args: CommandArgs,
    registry: &CommandRegistry,
) -> Result<OutputStream, ShellError> {
    let args = args.evaluate_once(registry).await?;

    let id_column = args
        .get("id-column")
        .map(|id| id.as_string().unwrap())
        .unwrap_or_else(|| String::from("id"));

    let content_column = args
        .get("content-column")
        .map(|content| content.as_string().unwrap())
        .unwrap_or_else(|| String::from("content"));

    let bucket_name = match args
        .get("bucket")
        .map(|id| id.as_string().unwrap())
        .or_else(|| state.active_cluster().active_bucket())
    {
        Some(v) => v,
        None => {
            return Err(ShellError::untagged_runtime_error(format!(
                "Could not auto-select a bucket - please use --bucket instead"
            )))
        }
    };

    let bucket = state.active_cluster().bucket(&bucket_name);
    let collection = Arc::new(bucket.default_collection());

    let input_args = if args.nth(0).is_some() && args.nth(1).is_some() {
        let id = args.nth(0).unwrap().as_string()?;
        let content = serde_json::from_str(&args.nth(1).unwrap().as_string()?).unwrap();
        vec![(id, content)]
    } else {
        vec![]
    };

    let filtered = args.input.filter_map(move |i| {
        let id_column = id_column.clone();
        let content_column = content_column.clone();
        async move {
            if let UntaggedValue::Row(dict) = i.value {
                let mut id = None;
                let mut content = None;
                if let MaybeOwned::Borrowed(d) = dict.get_data(id_column.as_ref()) {
                    id = Some(d.as_string().unwrap());
                }
                if let MaybeOwned::Borrowed(d) = dict.get_data(content_column.as_ref()) {
                    content = Some(convert_nu_value_to_json_value(d).unwrap());
                }
                if id.is_some() && content.is_some() {
                    return Some((id.unwrap(), content.unwrap()));
                }
            }
            None
        }
    });

    let mapped = filtered
        .chain(futures::stream::iter(input_args))
        .then(move |(id, content)| {
            let collection = collection.clone();
            async move {
                collection
                    .upsert(id, content, UpsertOptions::default())
                    .await
            }
        })
        .fold((0, 0), |(mut success, mut failed), res| async move {
            match res {
                Ok(_) => success += 1,
                Err(_) => failed += 1,
            };
            (success, failed)
        })
        .map(|(success, failed)| {
            let tag = Tag::default();
            let mut collected = TaggedDictBuilder::new(&tag);
            collected.insert_untagged("processed", UntaggedValue::int(success + failed));
            collected.insert_untagged("success", UntaggedValue::int(success));
            collected.insert_untagged("failed", UntaggedValue::int(failed));

            collected.into_value()
        })
        .into_stream();

    Ok(OutputStream::from_input(mapped))
}
