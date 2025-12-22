mod api;
mod app_state;
mod jwt;
mod models;

use axum::Router;
use service::CommonService;
use sys_core::config::Config;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;
use tracing::{error, info};

use crate::app_state::AppState;

fn root() -> Router<AppState> {
    let static_files = ServeDir::new("./web").fallback(ServeDir::new("./web").append_index_html_on_directories(true).not_found_service(ServeFile::new("./web/index.html")));
    Router::new().nest("/api", api::router()).fallback_service(static_files)
}

pub async fn serve(config: Config, com: CommonService) {
    use tower_http::cors::{Any, CorsLayer};

    let cors_layer = CorsLayer::new()
        // Allow requests from any origin
        .allow_origin(Any)
        // Allow the standard methods (GET, POST, PUT, DELETE, etc.)
        .allow_methods(Any)
        // Allow any header
        .allow_headers(Any);

    let app = AppState { com };
    let app = root().with_state(app).layer(cors_layer);

    let server = config.server;
    info!("Server will serve at {}:{}.", server.host, server.port);
    let listener = tokio::net::TcpListener::bind((server.host, server.port)).await.unwrap();
    match axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await {
        Ok(()) => info!("Server exited."),
        Err(err) => error!("Server close unexpected: {err}"),
    }
}

use tokio::select;
use tokio::signal;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate()).expect("failed to install SIGTERM handler").recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
