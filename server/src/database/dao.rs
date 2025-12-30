use duckdb::{params, Connection, Error};
use robot_data::robot_info::BasicInfo;
use robot_data::RobotInfo;

pub trait DAO {
    fn insert_to_db(&self, conn: &Connection) -> duckdb::Result<usize>;
    fn get_by_id(id: u64, conn: &Connection) -> Result<Vec<Self>, duckdb::Error>
    where
        Self: Sized;
}





impl DAO for BasicInfo {
    fn insert_to_db(&self, conn: &Connection) -> duckdb::Result<usize> {
        conn.execute(
            "INSERT INTO  main.BasicInfo(id,robot_type) VALUES (?, ?)",
            params![self.id, self.robot_type.to_string()],
        )
    }

    fn get_by_id(id: u64, conn: &Connection) -> Result<Vec<Self>, duckdb::Error>
    where
        Self: Sized,
    {
        let mut stmt = conn.prepare("SELECT id, robot_type FROM  main.BasicInfo WHERE id = (?)")?;
        stmt.query_map(params![id], |row| {
            Ok(BasicInfo {
                id,
                robot_type: row
                    .get::<usize, String>(1)?
                    .as_str()
                    .try_into()
                    .map_err(|e: &str| duckdb::Error::InvalidColumnName(e.to_string()))?,
            })
        })?
        .collect()
    }
}
