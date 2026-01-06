use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum RobotType {
    RoboHand,
    #[default]
    Mobile,
}

impl Display for RobotType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let robot_type = match self {
            RobotType::RoboHand => "RoboHand",
            RobotType::Mobile => "Mobile",
        };
        write!(f, "({})", robot_type)
    }
}

impl TryFrom<&str> for RobotType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "RoboHand" => Ok(RobotType::RoboHand),
            "Mobile" => Ok(RobotType::Mobile),
            &_ => Err("Unknown Robot Type"),
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct BatteryInfo {
    pub id: String,
    pub capacity: f32,
    pub total_capacity: f32,
    pub health: f32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct BasicInfo {
    pub id:  String,
    // pub battery_info: Option<BatteryInfo>,
    pub robot_type: RobotType,
}
#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct MovementInfo {
    pub id:  String,
    pub speed: Vec3,  // m/s
    pub acc: Vec3,   // m/s^2
    pub timestamp: DateTime<Utc>,
}

pub type GPS = String; // TODO use actual GPS
#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Geodata {
    pub id:  String,
    pub coordinates: GPS,
    pub timestamp: DateTime<Utc>
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
