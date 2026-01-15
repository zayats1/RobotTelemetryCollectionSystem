use leptos::logging::log;
use leptos::server_fn::serde::de::DeserializeOwned;
use robot_data::RobotInfoType;
use robot_data::robot_info::{BasicInfo, BatteryInfo, MovementInfo};
use crate::env::TELEMETRY_CONTROL_BACKEND_URL;

pub type FetchRes<T> = Result<Vec<T>, String>;

    // Todo: get rid of  hardcoded urls
pub async fn fetch_robots(
) -> Result<Vec<BasicInfo>, String> {
    // Todo: Url

    let info = reqwest::get(
        {
            format!("{}/robots/", TELEMETRY_CONTROL_BACKEND_URL)
        },
    )
        .await
        .map_err(|err| format!("failed to get info about robots: {}",err))?;
        log!("{:?}", info);
        info
            .json::<Result<Vec<BasicInfo>, String>>()
            .await
            .map_err(|e| format!("failed to parse telemetry: {e}"))?
}
    async fn fetch_info<T: DeserializeOwned+ Sized>(
        id: String,
        info_type: RobotInfoType,
    ) -> FetchRes<T> {
        let url = TELEMETRY_CONTROL_BACKEND_URL;
        let info_type: &str = info_type.into();
        let info =
            reqwest::get(format!(
                "{}/info_from/?id={}&info_type={}",
                url,&id, info_type
            ))
            .await
                .map_err(|err| format!("failed to get telemetry: {}",err))?;
        log!("{:?}", info);
        info
            .json::<Result<Vec<T>, String>>()
            .await
            .map_err(|e| format!("failed to parse telemetry: {e}"))?
    }

pub async  fn fetch_battery_info(id: String) -> FetchRes<BatteryInfo> {
    fetch_info::<BatteryInfo>(id, RobotInfoType::Battery).await
}
pub async  fn fetch_movement_info(id: String) -> FetchRes<MovementInfo> {
    fetch_info::<MovementInfo>(id, RobotInfoType::Movement).await
}

// Todo
//
// pub async  fn fetch_location_info(id: String) -> FetchRes<Geodata> {
//     fetch_info::<Geodata>(id, RobotInfoType::Geodata).await
// }