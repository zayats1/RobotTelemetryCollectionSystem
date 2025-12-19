use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::fmt::Formatter;

use crate::robot_info::{BasicInfo, BatteryInfo, MovementInfo, RobotLocation};

mod geodata;
pub mod robot_info;

#[derive(Debug, Clone, PartialEq)]
pub enum RobotInfo {
    BasicInfo(BasicInfo),
    Location(RobotLocation),
    Battery(BatteryInfo),
    Movement(MovementInfo),
}

impl RobotInfo {
    
    fn parse(robot_type: &str,robot_data: &str) -> Result<Self, RobotInfoParsingError> {
        let robot_type = serde_json::from_str::<Value>(robot_type).map_err(|_| RobotInfoParsingError)?;
        let info_type =
            IncomingInfoType::try_from(robot_type["type"].as_str().ok_or(RobotInfoParsingError)?)?;
        match info_type {
            IncomingInfoType::Unknown => Err(RobotInfoParsingError),
            IncomingInfoType::BatteryInfo => {
                let bat = serde_json::from_str::<BatteryInfo>(
                    robot_data,
                )
                .map_err(|_| RobotInfoParsingError)?;
                Ok(RobotInfo::Battery(bat))
            }
            IncomingInfoType::BasicInfo => {
                let s = robot_data;
                let basic = serde_json::from_str::<BasicInfo>(
                    s
                )
                .map_err(|_| RobotInfoParsingError)?;
                Ok(RobotInfo::BasicInfo(basic))
            }
            IncomingInfoType::Location => {
                let loc = serde_json::from_str::<RobotLocation>(
                    robot_data,
                )
                .map_err(|_| RobotInfoParsingError)?;
                Ok(RobotInfo::Location(loc))
            }
            IncomingInfoType::Movement => {
                let mov = serde_json::from_str::<MovementInfo>(
                    robot_data,
                )
                .map_err(|_| RobotInfoParsingError)?;
                Ok(RobotInfo::Movement(mov))
            }
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct RobotInfoParsingError;

impl std::fmt::Display for RobotInfoParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl std::error::Error for RobotInfoParsingError {}

#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum IncomingInfoType {
    #[default]
    Unknown,
    BatteryInfo,
    BasicInfo,
    Location,
    Movement,
}

impl TryFrom<&str> for IncomingInfoType {
    type Error = RobotInfoParsingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "BatteryInfo" => Ok(IncomingInfoType::BatteryInfo),
            "BasicInfo" => Ok(IncomingInfoType::BasicInfo),
            "Location" => Ok(IncomingInfoType::Location),
            "Movement" => Ok(IncomingInfoType::Movement),
            _ => Err(RobotInfoParsingError),
        }
    }
}

impl fmt::Display for IncomingInfoType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::robot_info::RobotType;

    #[test]
    fn it_works() {
        let robot_type = r#"
    {
  "type": "BasicInfo"
}
"#;
let data = r#"
     {
   "id": 6969424242,
    "robot_type": "Mobile"
}
"#;
        let res = RobotInfo::parse(robot_type,data);
        assert_eq!(
            Ok(RobotInfo::BasicInfo(BasicInfo {
                id: 6969424242,

                robot_type: RobotType::Mobile
            })),
            res
        );
    }
}
