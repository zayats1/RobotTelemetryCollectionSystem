use robot_data::robot_info::BasicInfo;

#[derive(Clone,  PartialEq)]
pub(crate) struct AppState {
   pub  selected_info :Option<BasicInfo>,
}

impl AppState {
    pub fn new (selected_info:Option<BasicInfo>)-> Self {
       Self{
           selected_info,
       }
    }
}
