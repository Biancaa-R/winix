use std::pin::Pin;
use std::{future::Future, io};

pub trait AsyncCommand {
    type Input;
    type Output;

    fn execute(
        &self,
        input: Self::Input,
    ) -> Pin<Box<dyn Future<Output = io::Result<Self::Output>> + Send + '_>>;
}

pub struct CatGrepPipeline;

impl AsyncCommand for CatGrepPipeline {
    type Input = ();
    type Output = String;

    fn execute(&self, _input: ()) -> Pin<Box<dyn Future<Output = io::Result<String>> + Send + '_>> {
        Box::pin(async move { Ok("mock output".to_string()) })
    }
}

pub async fn execute_pipeline<C: AsyncCommand<Input = ()>>(command: C) -> io::Result<C::Output> {
    command.execute(()).await
}
