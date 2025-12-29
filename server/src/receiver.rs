use std::sync::Arc;
use axum::Json;
use robot_data::RobotInfo;

#[derive(Debug,Default,Clone)]
pub struct AppState {
    received: Vec<Json<RobotInfo>>
}


pub async fn receive_telemetry(data: Json<RobotInfo>) -> Json<RobotInfo>   {
    let _ = format!("Received data: {:?}",data.clone() );
    data
}
