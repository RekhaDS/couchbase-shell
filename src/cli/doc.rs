use async_stream::stream;
use async_trait::async_trait;
use nu_cli::{CommandArgs, CommandRegistry, OutputStream};
use nu_errors::ShellError;
use nu_protocol::{Signature, UntaggedValue};
use nu_source::Tag;

pub struct Doc;

#[async_trait]
impl nu_cli::WholeStreamCommand for Doc {
    fn name(&self) -> &str {
        "doc"
    }

    fn signature(&self) -> Signature {
        Signature::build("doc")
    }

    fn usage(&self) -> &str {
        "Perform document operations against a bucket or collection"
    }

    async fn run(
        &self,
        _args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        let registry = registry.clone();
        let stream = stream! {
            yield UntaggedValue::string(nu_cli::get_help(&Doc, &registry))
            .into_value(Tag::unknown())
        };
        Ok(OutputStream::from_input(stream))
    }
}
