use robot_data::robot_info::BasicInfo;

#[derive(Clone,  PartialEq,Default)]
pub(crate) struct AppState {
   pub  selected_info :Option<BasicInfo>,
    pub selected_id :Option<String>,
}

impl AppState {
    pub fn new (selected_info:Option<BasicInfo>,selected_id:Option<String>)-> Self {
       Self{
           selected_info,
           selected_id
       }
    }
}
