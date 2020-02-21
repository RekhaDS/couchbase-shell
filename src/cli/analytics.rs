use super::util::convert_json_value_to_nu_value;
use couchbase::{AnalyticsOptions, Cluster};
use futures::executor::block_on;
use futures::stream::StreamExt;
use log::debug;
use nu::{CommandArgs, CommandRegistry, OutputStream};
use nu_errors::ShellError;
use nu_protocol::{Signature, SyntaxShape};
use nu_source::Tag;
use std::sync::Arc;

pub struct Analytics {
    cluster: Arc<Cluster>,
}

impl Analytics {
    pub fn new(cluster: Arc<Cluster>) -> Self {
        Self { cluster }
    }
}

impl nu::WholeStreamCommand for Analytics {
    fn name(&self) -> &str {
        "analytics"
    }

    fn signature(&self) -> Signature {
        Signature::build("analytics").required(
            "statement",
            SyntaxShape::String,
            "the analytics statement",
        )
    }

    fn usage(&self) -> &str {
        "Performs an analytics query"
    }

    fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        let args = args.evaluate_once(registry)?;
        let statement = args.nth(0).expect("need statement").as_string()?;

        debug!("Running analytics query {}", &statement);
        let mut result = block_on(
            self.cluster
                .analytics_query(statement, AnalyticsOptions::default()),
        )
        .unwrap();
        let stream = result
            .rows::<serde_json::Value>()
            .map(|v| convert_json_value_to_nu_value(&v.unwrap(), Tag::default()));
        Ok(OutputStream::from_input(stream))
    }
}
