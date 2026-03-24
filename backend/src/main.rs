mod config;
mod cron;
mod handlers;
mod routes;
mod state;
mod tls;
mod weather;
mod webdriver;

use crate::handlers::ws::ws_broadcast;
use crate::routes::get_routes;
use config::Config;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tls::init();
    tracing_subscriber::fmt()
        // .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("backend=debug".parse()?)
                .add_directive("sqlx=debug".parse()?)
                .add_directive("tokio_cron_scheduler=debug".parse()?)
                .add_directive("tower_http=debug".parse()?),
        )
        .init();

    let config = Config::init();
    let pool = PgPoolOptions::new().connect(&config.database_url).await?;

    let app_state = AppState::new(pool, config);
    let app = get_routes(app_state.clone());

    cron::init(app_state).await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
