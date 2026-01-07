use axum::extract::State;
use axum::Json;
use libsql::Builder;
use robot_data::RobotInfo;
use server::database::dao::DAO;
use tracing::{debug, info};
use crate::AppState;

pub async fn receive_telemetry(State(state):State<AppState>, data: Json<RobotInfo>) -> Json<String> {
    let _ = format!("Received data: {:?}", data.clone());
    let res = async {
        let conn = state.db.connect()?;
        match data.0 {
            RobotInfo::BasicInfo(info) => {
                debug!("Received BasicInfo: {:?}", info);
                info.insert_to_db(&conn).await
            }

            RobotInfo::Geodata(info) => {
                debug!("Received Geodata: {:?}", info);
                info.insert_to_db(&conn).await
            }
            RobotInfo::Battery(info) => {
                debug!("Received BatteryInfo: {:?}", info);
                info.insert_to_db(&conn).await
            }
            RobotInfo::Movement(info) => {
                debug!("Received movement: {:?}", info);
                info.insert_to_db(&conn).await
            }
        }
    };
    if let Err(e) = res.await {
        Json("Error:".to_owned() + e.to_string().as_str())
    } else {
        Json("Received and saved".to_string())
    }
}
