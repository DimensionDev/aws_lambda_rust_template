mod controller;
mod error;

use lambda_http::{service_fn, Error as LambdaError};
use crate::controller::entrypoint;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    lambda_http::run(service_fn(entrypoint)).await?;
    Ok(())
}
