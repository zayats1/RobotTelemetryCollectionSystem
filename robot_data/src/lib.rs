use serde::{Deserialize, Serialize};

use crate::robot_info::{BasicInfo, BatteryInfo, MovementInfo, RobotLocation};

mod geodata;
pub mod robot_info;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RobotInfo {
    BasicInfo(BasicInfo),
    Location(RobotLocation),
    Battery(BatteryInfo),
    Movement(MovementInfo),
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::robot_info::RobotType;

    #[test]
    fn it_works() {
        let telemetry =  r#"
            {
                  "type": "BasicInfo",
                  "id": 6969424242,
                  "robot_type": "Mobile"
            }
        "#;

        let res = serde_json::from_str::<RobotInfo>(telemetry).ok();
        assert_eq!(
            Some(RobotInfo::BasicInfo(BasicInfo {
                id: 6969424242,

                robot_type: RobotType::Mobile
            })),
            res
        );
    }
}
