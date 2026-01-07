mod receiver;

use std::sync::Arc;
use axum::{
    routing::get,
    Router,
};
use axum::routing::post;
use libsql::{Database};
use crate::receiver::receive_telemetry;
use tracing::{info};
use tracing_subscriber;


#[derive(Debug,Clone)]
struct AppState{
    db: Arc<Database>,
}
#[tokio::main]
async fn main()  -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let db = libsql::Builder::new_local("server/RobotTelemetry.db")
        .build()
        .await.expect("Failed to initialize database");
    let app = Router::new().route("/", get(|| async { "telemetry collector" }))
        .route("/telemetry",post(receive_telemetry)).with_state(AppState{db:Arc::new(db)});




    // run our app with hyper, listening globally on port 3000


    info!("The server is starting");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app).await
}