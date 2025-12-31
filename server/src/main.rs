mod receiver;


use axum::{
    routing::get,
    Router,
};
use axum::routing::post;
use crate::receiver::receive_telemetry;
use tracing::{info, Level};
use tracing_subscriber;
#[tokio::main]
async fn main()  -> Result<(), std::io::Error> {
    // build our application with a single route
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", get(|| async { "telemetry collector" }))
        .route("/telemetry",post(|data| receive_telemetry(data)));


    // run our app with hyper, listening globally on port 3000


    info!("The server is starting");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app).await
}