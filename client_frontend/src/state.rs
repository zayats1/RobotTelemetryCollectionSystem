use std::sync::Arc;
use robot_data::robot_info::BasicInfo;
use robot_data::RobotInfoType;

#[derive(Clone,  PartialEq,Default)]
    pub(crate) struct AppState {
    pub  selected_info : Option<BasicInfo>,
    pub selected_info_type :Option<RobotInfoType>,
    pub selected_id :Option<String>,
}

impl AppState {
    pub fn new (selected_info:Option<BasicInfo>,selected_id:Option<String>,selected_info_type:Option<RobotInfoType>)-> Self {
       Self{
           selected_info,
           selected_info_type,
           selected_id
       }
    }
}
