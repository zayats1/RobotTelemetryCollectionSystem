use robot_data::{RobotInfo, RobotInfoType};
use robot_data::robot_info::BasicInfo;

#[derive(Clone, PartialEq)]
pub(crate) struct AppModel {
    data : Option<Vec<BasicInfo>>,
}

impl AppModel {
    pub fn new() -> Self {
        Self {
            data: None
        }
    }
}

    // Todo: get rid of  hardcoded urls
pub async fn fetch_robots(
) -> Result<Vec<BasicInfo>, String> {
    // Todo: Url
        let client = reqwest::Client::new();
    let info = client
        .get(
            "127.0.0.1:3000/robots/"
        )
        .send()
        .await
        .map_err(|err| err.to_string())?;
    info.json::<Result<Vec<BasicInfo>, String>>()
        .await
        .map_err(|err|format!("failed  to  fetch robots:{}" ,err))?
}
    pub async fn fetch_info(
        id: String,
        info_type: RobotInfoType,
    ) -> Result<Vec<RobotInfo>, String> {
        // Todo: Url
        let client = reqwest::Client::new();
        let info_type: &str = info_type.into();
        let info = client
            .get(format!(
                "127.0.0.1:3000/info_from/?id={}&info_type={}",
                id, info_type
            ))
            .send()
            .await
            .map_err(|err| err.to_string())?;
        info.json::<Result<Vec<RobotInfo>, String>>()
            .await
            .map_err(|err|format!("failed  to  fetch info:{}" ,err))?
    }

