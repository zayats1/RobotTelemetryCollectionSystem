use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type GPS = String; // todo add actual GPS

#[derive(Default,Debug,Clone,Deserialize,Serialize,PartialEq)]
pub struct Geodata{
    pub coordinates: GPS,
    timestamp: DateTime<Utc>
}