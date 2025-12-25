use axum::Json;
use robot_data::{RobotInfo, RobotInfoParsingError};

async fn receive_telemetry((info_type,robot_data):(Json<String>,Json<String>)) -> Result<RobotInfo,RobotInfoParsingError>   {
    let _ = format!("Received type: {}, data: {}", info_type.to_string(),robot_data.to_string());
    let parsed = RobotInfo::parse(info_type.as_str(), robot_data.as_str())?;
    Ok(parsed)
}