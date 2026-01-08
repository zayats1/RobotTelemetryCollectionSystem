use reqwest::Client;
use robot_data::{RobotInfo, RobotInfoType};
use std::sync::Arc;
use robot_data::robot_info::BasicInfo;

struct AppModel {
    client: Arc<reqwest::Client>,
}

impl AppModel {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Client::new()),
        }
    }

pub async fn fetch_robots(
    &self,
) -> Result<Vec<BasicInfo>, String> {
    // Todo: Url
    let info = self
        .client
        .get(
            "127.0.0.1:3000/robots/"
        )
        .send()
        .await
        .map_err(|err| err.to_string())?;
    info.json::<Vec<BasicInfo>>()
        .await
        .map_err(|err| err.to_string())
}
    pub async fn fetch_info(
        &self,
        id: String,
        info_type: RobotInfoType,
    ) -> Result<Vec<RobotInfo>, String> {
        // Todo: Url
        let info_type: &str = info_type.into();
        let info = self
            .client
            .get(format!(
                "127.0.0.1:3000/info_from/?id={}&info_type={}",
                id, info_type
            ))
            .send()
            .await
            .map_err(|err| err.to_string())?;
        info.json::<Vec<RobotInfo>>()
            .await
            .map_err(|err| err.to_string())
    }
}
