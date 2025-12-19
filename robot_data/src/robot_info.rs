use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
pub use crate::geodata::Geodata;
#[derive(Default,Debug,Clone,Deserialize,Serialize,PartialEq)]
pub enum RobotType {
    RoboHand,
    #[default]
    Mobile,
}

#[derive(Default,Debug,Clone,Deserialize,Serialize,PartialEq)]
pub struct BatteryInfo {
    pub id: u64,
    pub capacity: f32,
    pub health: f32,
}

#[derive(Default,Debug,Clone,Deserialize,Serialize,PartialEq)]
pub struct BasicInfo {
    pub id: u64,
    pub battery_info: Option<BatteryInfo>,
    pub robot_type: RobotType,
}
#[derive(Default,Debug,Clone,Deserialize,Serialize,PartialEq)]
pub struct MovementInfo {
    pub id: u64,
    pub speed: f32,
    pub acceleration: f32,
    pub orientation: f32,
    pub timestamp: DateTime<Utc>,
}


#[derive(Default,Debug,Clone,Deserialize,Serialize,PartialEq)]
pub struct RobotLocation {
    pub id: u64,
    geodata:Geodata
}


#[derive(Default,Debug,Clone,Deserialize,Serialize,PartialEq,PartialOrd)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
