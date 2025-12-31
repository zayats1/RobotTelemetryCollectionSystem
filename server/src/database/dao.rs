
use libsql::{de,params, Connection};
use robot_data::robot_info::BasicInfo;
use robot_data::RobotInfo;

pub    trait DAO {
   async fn insert_to_db(&self, conn: &Connection) ->  libsql::Result<u64>;
   async  fn get_by_id(id: u64, conn: &Connection) -> Result<Vec<Self>, libsql::Error>
    where
        Self: Sized;
}





impl DAO for BasicInfo {
   async  fn insert_to_db(&self, conn: &Connection) -> libsql::Result<u64> {
        conn.execute(
            "INSERT INTO  BasicInfo(id,robot_type) VALUES (?, ?)",
            params![self.id, self.robot_type.to_string()],
        ).await
    }

    async fn get_by_id(id: u64, conn: &Connection) -> Result<Vec<Self>, libsql::Error>
    where
        Self: Sized,
    {
        let stmt = conn.prepare("SELECT id, robot_type FROM  BasicInfo WHERE id = (?)").await?;
        let mut rows = stmt
            .query([id])
            .await?;
         let mut data:Vec<BasicInfo> = vec![];

         while let Some(row )  = rows.next().await?{
              data.push(de::from_row::<BasicInfo>(&row).map_err(|e| libsql::Error::InvalidParserState(e.to_string()))?);
         }
         Ok(data)

    }
}
