#[macro_use]
extern crate log;

use dotenv::dotenv;
use enki::{database, model::Account, router, Config, Result};
use jwt_auth::Jwt;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    info!("Enki Start.");

    info!("Loading configs.");
    let cfg = Config::from_env()?;

    info!("Connecting database.");
    let db = database::init(cfg.database_url).await?;

    info!("initing jwt authorization.");
    let jwt = Jwt::new(&cfg.jwt_secret);

    info!("initing admin account");
    let account = Account::new(&cfg.user_name, &cfg.password);

    let app = router::init(db, jwt, account);

    info!("Listening on {}", &cfg.listen_addr);
    axum::Server::bind(&cfg.listen_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
