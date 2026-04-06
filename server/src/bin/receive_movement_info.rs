use std::sync::Arc;
use axum::{
    routing::get,
    Router,
};
use axum::routing::post;


use robot_data::robot_info::MovementInfo;
use server::database::dao::DAO;
use tracing::{info};
use tracing_subscriber;
use server::AppState;
use server::sender::get_telemetry_for_id;
use axum::extract::State;
use axum::Json;


use tracing::{debug};


#[tokio::main]
async fn main()  -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let db = libsql::Builder::new_local("RobotTelemetry.db")
        .build()
        .await.expect("Failed to initialize database");
    let app = Router::new().route("/", get(|| async { "telemetry collector" }))
        .route("/battery_info",post(receive_telemetry))
        .route("/info_from/", get(get_telemetry_for_id))
        .with_state(AppState{db:Arc::new(db)});

    


    info!("The server is starting");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3008").await?;

    axum::serve(listener, app).await
}



async fn receive_telemetry(State(state):State<AppState>, data: Json<MovementInfo>) -> Json<String> {
    let _ = format!("Received data: {:?}", data.clone());
    let res = async {
        let conn = state.db.connect()?;
                debug!("Received MovementInfo: {:?}", data);
                data.insert_to_db(&conn).await
    };
    if let Err(e) = res.await {
        Json("Error:".to_owned() + e.to_string().as_str())
    } else {
        Json("Received and saved".to_string())
    }
}
