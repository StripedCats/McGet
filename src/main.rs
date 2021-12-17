use mcget_core::{prelude::*, cli_helpers::RResult};

#[tokio::main]
async fn main() -> RResult<()> {
    start().await
}
