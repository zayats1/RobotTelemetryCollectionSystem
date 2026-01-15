use libsql::Connection;
use robot_data::robot_info::{BasicInfo, BatteryInfo, Geodata, MovementInfo};
use robot_data::RobotInfo;
use crate::database::dao::DAO;


trait Wrap<T> : IntoIterator{
    fn wrap(&self,fun:fn(T)->RobotInfo) -> Vec<RobotInfo>;
}


impl <T>Wrap<T> for Vec<T> where T:Clone{
    fn wrap(&self, fun: fn(T) -> RobotInfo) -> Vec<RobotInfo>{
         self.iter().map(|i: &T| fun(i.clone())).collect::<Vec<RobotInfo>>()
    }
}


#[allow(async_fn_in_trait)]
pub trait FetchRobotInfo {
    async fn fetch(id:  String, conn: &Connection) -> Result<Vec<RobotInfo>, libsql::Error>;
}

macro_rules! impl_fetch {
    ($ty:ty, $variant:path) => {
        impl FetchRobotInfo for $ty {
            async fn fetch(
                id: String,
                conn: &Connection,
            ) -> Result<Vec<RobotInfo>, libsql::Error> {
                Ok(
                    <$ty>::get_by_id(id, conn)
                        .await?
                        .into_iter()
                        .map($variant)
                        .collect(),
                )
            }
        }
    };
}

impl_fetch!(BasicInfo, RobotInfo::BasicInfo);
impl_fetch!(BatteryInfo, RobotInfo::Battery);
impl_fetch!(Geodata, RobotInfo::Geodata);
impl_fetch!(MovementInfo, RobotInfo::Movement);