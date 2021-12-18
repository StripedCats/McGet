use {
    cli::prelude::*
};

#[tokio::main]
async fn main() -> RResult<()> {
    CliApp::run().await
}
