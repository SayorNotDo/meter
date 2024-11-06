use futures::FutureExt;
use server::{configure, constant::CONFIG, errors::AppResult, server::AppServer, utils::task};
use tracing::info;

#[tokio::main]
async fn main() -> AppResult<()> {
    /* Tracing */
    let _file_appender_guard = configure::tracing::init()?;
    info!("The initialization of Tracing was successful.");

    /* Config */
    let config = CONFIG.clone();
    info!("Initializing server with configuration: {:?}", config);

    /* Run server with graceful shutdown */
    let server = AppServer::new(config).await?;

    info!("Starting the server...");
    task::join_all(vec![(true, server.run().boxed())]).await?;

    Ok(())
}
