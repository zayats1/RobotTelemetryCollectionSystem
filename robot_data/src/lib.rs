use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use crate::robot_info::{BasicInfo, BatteryInfo, MovementInfo, RobotLocation};

mod geodata;
pub mod robot_info;


#[derive( Debug, Clone, PartialEq)]
pub enum RobotInfo {
    BasicInfo(BasicInfo),
    Location(RobotLocation),
    Battery(BatteryInfo),
    Movement(MovementInfo),
}

impl TryFrom<String> for RobotInfo {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}


#[derive(Debug,Default,PartialEq)]
pub struct RobotInfoParsingError;

impl std::fmt::Display for RobotInfoParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }

}

impl std::error::Error for RobotInfoParsingError {}

#[derive(Default,Debug,Clone,Deserialize,Serialize,PartialEq)]
pub enum IncomingInfoType {
    #[default]
    Unknown,
    BatteryInfo,
    BasicInfo,
    Geodata,
    Movement,
 
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);

        assert_eq!(result, 4);
    }
}
