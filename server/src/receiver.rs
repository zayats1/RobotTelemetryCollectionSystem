use axum::Json;
use robot_data::RobotInfo;

pub async fn receive_telemetry(data: Json<RobotInfo>) -> Json<RobotInfo>   {
    let _ = format!("Received data: {:?}",data.clone() );
    data
}
