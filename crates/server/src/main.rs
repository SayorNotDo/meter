use server::configure::env::get_env_source;
use server::constant::ENV_PREFIX;
use server::errors::AppResult;
use server::server::AppServer;
use tracing::info;

use server::configure;

#[tokio::main]
async fn main() -> AppResult<()> {
    /* Tracing */
    let _file_appender_guard = configure::tracing::init()?;
    info!("The initialization of Tracing was successful.");

    /* Config */
    let config = configure::Config::read(get_env_source(ENV_PREFIX))?;

    /* Run server with graceful shutodwn */
    let server = AppServer::new(config).await?;
    server.run().await?;

    Ok(())
}
