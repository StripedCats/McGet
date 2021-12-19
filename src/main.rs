use {
    cli::prelude::*
};

#[tokio::main(worker_threads = 4)]
async fn main() -> RResult<()> {
    CliApp::run().await
}
