use axum::extract::{ Query, State};
use axum::Json;

use serde::Deserialize;
use robot_data::robot_info::Geodata;
use robot_data::robot_info::{BasicInfo, BatteryInfo, MovementInfo};
use robot_data::{RobotInfo, RobotInfoType};
use crate::AppState;
use crate::database::fetch::FetchRobotInfo;

#[derive(Deserialize)]
pub struct TelemetryQuery {
    id: String,
    info_type: String,
}

pub async fn send_telemetry(
    State(state):State<AppState>,
    Query(params): Query<TelemetryQuery>,
) -> Json<Result<Vec<RobotInfo>, String>> {

   let res =  async {
       let conn = state.db
           .connect()?;
       let id = params.id;
       let info_type = RobotInfoType::try_from(params.info_type.as_str()).map_err(|e| libsql::Error::InvalidParserState(e.to_string()))?;
       let packed =match info_type{
           RobotInfoType::BasicInfo => BasicInfo::fetch(id, &conn).await,
           RobotInfoType::Geodata => Geodata::fetch(id, &conn).await,
           RobotInfoType::Battery => BatteryInfo::fetch(id, &conn).await,
           RobotInfoType::Movement => MovementInfo::fetch(id, &conn).await,
       };
      packed
   };
    match res.await {
        Ok(robot_info) => {Json(Ok(robot_info)) }
        Err(e) => {Json(Err(e.to_string()))}
    }
}


