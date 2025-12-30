
use axum::Json;
use duckdb::Connection;
use robot_data::RobotInfo;
use server::database::dao::DAO;
#[derive(Debug,Default,Clone)]
pub struct AppState {
    received: Vec<Json<RobotInfo>>
}


pub async fn receive_telemetry(data: Json<RobotInfo>) -> Json<String>   {
    let _ = format!("Received data: {:?}",data.clone() );
    let conn = Connection::open("server/RobotTelemetry.duckdb").unwrap();
    let res = match  data.0{

        RobotInfo::BasicInfo(info) => {
               info.insert_to_db(&conn)
        }
        RobotInfo::Location(_) => { todo!()}
        RobotInfo::Battery(_) => { todo!()}
        RobotInfo::Movement(_) => {todo!()}
    };
    if let Err(e) = res {
        Json("Error:".to_owned() + e.to_string().as_str())
    } else {
        Json("Received and saved".to_string())
    }
}
