use axum::Json;
use libsql::Builder;

use robot_data::RobotInfo;
use server::database::dao::DAO;
#[derive(Debug, Default, Clone)]
pub struct AppState {
    received: Vec<Json<RobotInfo>>,
}

pub async fn receive_telemetry(data: Json<RobotInfo>) -> Json<String> {
    let _ = format!("Received data: {:?}", data.clone());
    let res = async {
        let db = Builder::new_local("server/RobotTelemetry.db").build().await?;
        let conn = db.connect()?;
        match data.0 {
            RobotInfo::BasicInfo(info) => info.insert_to_db(&conn).await,
            RobotInfo::Location(_) => {
                todo!()
            }
            RobotInfo::Battery(_) => {
                todo!()
            }
            RobotInfo::Movement(_) => {
                todo!()
            }
        }
    };
    if let Err(e) = res.await {
        Json("Error:".to_owned() + e.to_string().as_str())
    } else {
        Json("Received and saved".to_string())
    }
}
