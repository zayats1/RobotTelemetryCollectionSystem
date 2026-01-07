use axum::extract::{Path, State};
use crate::database::dao::DAO;
use axum::Json;
use libsql::Builder;
use robot_data::robot_info::Geodata;
use robot_data::robot_info::{BasicInfo, BatteryInfo, MovementInfo};
use robot_data::{RobotInfo, RobotInfoType};
use crate::AppState;

pub async fn send_telemetry(
    State(state):State<AppState>,
    Path((id, info_type)): Path<(String, String)>,
) -> Json<Result<Vec<RobotInfo>, String>> {
    let res = async {
        let conn = state.db
            .connect()
            .map_err(|e| format!("Failed to build database connection: {:?}", e))?;
        match RobotInfoType::try_from(info_type.as_str()) {
            Ok(RobotInfoType::BasicInfo) => Ok(BasicInfo::get_by_id(id, &conn)
                .await
                .map_err(|e| format!("Failed to get basic info: {:?}", e))?
                .iter()
                .map(|i| RobotInfo::BasicInfo(i.clone()))
                .collect::<Vec<RobotInfo>>()),
            Ok(RobotInfoType::Geodata) => Ok(Geodata::get_by_id(id, &conn)
                .await
                .map_err(|e| format!("Failed to get geodata info: {:?}", e))?
                .iter()
                .map(|i| RobotInfo::Geodata(i.clone()))
                .collect::<Vec<RobotInfo>>()),
            Ok(RobotInfoType::Battery) => Ok(BatteryInfo::get_by_id(id, &conn)
                .await
                .map_err(|e| format!("Failed to get battery info: {:?}", e))?
                .iter()
                .map(|i| RobotInfo::Battery(i.clone()))
                .collect::<Vec<RobotInfo>>()),
            Ok(RobotInfoType::Movement) => Ok(MovementInfo::get_by_id(id, &conn)
                .await
                .map_err(|e| format!("Failed to get movement info: {:?}", e))?
                .iter()
                .map(|i| RobotInfo::Movement(i.clone()))
                .collect::<Vec<RobotInfo>>()),
            Err(e) => Err(e.to_string()),
        }
    }
    .await;
    Json(res)
}

