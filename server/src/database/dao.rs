use libsql::{de, params, Connection, Error, Row};
use robot_data::robot_info::{BasicInfo, Geodata, MovementInfo, Vec3};

#[allow(async_fn_in_trait)]
pub trait DAO {
    async fn insert_to_db(&self, conn: &Connection) -> libsql::Result<u64>;
    async fn get_by_id(id: String, conn: &Connection) -> Result<Vec<Self>, libsql::Error>
    where
        Self: Sized;
}

impl DAO for BasicInfo {
    async fn insert_to_db(&self, conn: &Connection) -> libsql::Result<u64> {
        conn.execute(
            "INSERT INTO  BasicInfo(id,robot_type) VALUES (?, ?)",
            params![self.id.clone(), self.robot_type.to_string()],
        )
        .await
    }

    async fn get_by_id(id: String, conn: &Connection) -> Result<Vec<Self>, libsql::Error>
    where
        Self: Sized,
    {
        let stmt = conn
            .prepare("SELECT id, robot_type FROM  BasicInfo WHERE id = (?)")
            .await?;
        let mut rows = stmt.query([id]).await?;
        let mut data: Vec<BasicInfo> = vec![];

        while let Some(row) = rows.next().await? {
            data.push(
                de::from_row::<BasicInfo>(&row)
                    .map_err(|e| libsql::Error::InvalidParserState(e.to_string()))?,
            );
        }
        Ok(data)
    }
}

impl DAO for MovementInfo {
    async fn insert_to_db(&self, conn: &Connection) -> libsql::Result<u64> {
        conn.execute(
            "INSERT INTO \
             MovementInfo \
            VALUES (?,?,?,?, ?,?,?,?)",
            params![
                self.id.clone(),
                self.speed.x,
                self.speed.y,
                self.speed.z,
                self.acc.x,
                self.acc.y,
                self.acc.z,
                self.timestamp.to_string()
            ],
        )
        .await
    }

    async fn get_by_id(id: String, conn: &Connection) -> Result<Vec<Self>, libsql::Error>
    where
        Self: Sized,
    {
        let stmt = conn
            .prepare("SELECT id, robot_type FROM  Movement WHERE id = (?)")
            .await?;
        let mut rows = stmt.query([id]).await?;
        let mut data: Vec<MovementInfo> = vec![];

        while let Some(row) = rows.next().await? {
            let info = MovementInfo {
                id: row.get(0)?,
                speed: Vec3 {
                    x: sql_val_to_f32(&row, 1)?,
                    y: sql_val_to_f32(&row, 2)?,
                    z: sql_val_to_f32(&row, 3)?,
                },
                acc: Vec3 {
                    x: sql_val_to_f32(&row, 4)?,
                    y: sql_val_to_f32(&row, 5)?,
                    z: sql_val_to_f32(&row, 6)?,
                },
                timestamp: Default::default(),
            };

            data.push(info);
        }
        Ok(data)
    }
}

fn sql_val_to_f32(row: &Row, idx: i32) -> Result<f32, Error> {
    row.get_str(idx)?
        .parse::<f32>()
        .map_err(|e| libsql::Error::InvalidParserState(e.to_string()))
}



impl DAO for Geodata{
    async fn insert_to_db(&self, conn: &Connection) -> libsql::Result<u64> {
        conn.execute(
            "INSERT INTO \
             Geodata \
            VALUES (?,?,?,?, ?,?,?,?)",
            params![

            ],
        )
            .await
    }

    async fn get_by_id(id: String, conn: &Connection) -> Result<Vec<Self>, Error>
    where
        Self: Sized
    {
        todo!()
    }
}

