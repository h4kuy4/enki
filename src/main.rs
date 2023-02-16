#[macro_use]
extern crate log;

use dotenv::dotenv;
use enki::{database, router, Config, Result};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    info!("Loading configs.");
    let cfg = Config::from_env()?;

    info!("Connecting database.");
    let db = database::init(cfg.database_url).await?;

    info!("Enki Start.");

    let app = router::init(db);

    info!("Listening on {}", &cfg.listen_addr);
    axum::Server::bind(&cfg.listen_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
