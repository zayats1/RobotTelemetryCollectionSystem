use leptos::logging::log;
use leptos::serde_json;
use robot_data::{RobotInfo, RobotInfoType};
use robot_data::robot_info::BasicInfo;


    // Todo: get rid of  hardcoded urls
pub async fn fetch_robots(
) -> Result<Vec<BasicInfo>, String> {
    // Todo: Url
    let info = reqwest::get(
            "http://127.0.0.1:3000/robots/"
        )
        .await
        .map_err(|err| format!("failed to get info about robots: {}",err))?;
        log!("{:?}", info);
        info
            .json::<Result<Vec<BasicInfo>, String>>()
            .await
            .map_err(|e| format!("failed to parse telemetry: {e}"))?
}
    pub async fn fetch_info(
        id: &str,
        info_type: RobotInfoType,
    ) -> Result<Vec<RobotInfo>, String> {
        // Todo: Url
        let info_type: &str = info_type.into();
        let info =
            reqwest::get(format!(
                "http://127.0.0.1:3000/info_from/?id={}&info_type={}",
                id, info_type
            ))
            .await
                .map_err(|err| format!("failed to get telemetry: {}",err))?;
        log!("{:?}", info);
        info
            .json::<Result<Vec<RobotInfo>, String>>()
            .await
            .map_err(|e| format!("failed to parse telemetry: {e}"))?
    }

