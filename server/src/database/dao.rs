use chrono::{DateTime, NaiveDateTime, Utc};
use libsql::{de, params, Connection, Row};
use robot_data::robot_info::{BasicInfo, BatteryInfo, Geodata, MovementInfo, Vec3};

// A set of methods for accessing data from db into RoboInfo structs
#[allow(async_fn_in_trait)]
pub trait DAO: Sized {
    async fn insert_to_db(&self, conn: &Connection) -> libsql::Result<u64>;
    async fn get_by_id(id: String, conn: &Connection) -> Result<Vec<Self>, libsql::Error>;

    async fn remove_from_db(&self, id: String, conn: &Connection) -> libsql::Result<usize>;
}

#[allow(async_fn_in_trait)]
pub trait GetAll: Sized {
    async fn get_all(conn: &Connection) -> Result<Vec<Self>, libsql::Error>;

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

        while let Some(row) = rows.next().await? {  // todo: compact the code or rewrite it
            data.push(
                de::from_row::<BasicInfo>(&row)
                    .map_err(|e| libsql::Error::InvalidParserState(e.to_string()))?,
            );
        }
        Ok(data)
    }

    async fn remove_from_db(&self, id: String, conn: &Connection) -> libsql::Result<usize> {
        let stmt = conn
            .prepare("DELETE FROM BasicInfo WHERE id = (1?)")
            .await?;
        stmt.execute([id]).await
    }
}
    impl GetAll for BasicInfo {
        async fn get_all(conn: &Connection) -> Result<Vec<Self>, libsql::Error> {
            let stmt = conn
                .prepare("SELECT * FROM  BasicInfo")
                .await?;
            let mut rows = stmt.query([""]).await?;
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
            .prepare("SELECT id, speed_x,speed_y,speed_z,acc_x,acc_y,acc_z,timestamp FROM  MovementInfo WHERE id = (?)")
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
                timestamp: sql_val_to_time(&row, 7)?,
            };

            data.push(info);
        }
        Ok(data)
    }

    async fn remove_from_db(&self, id: String, conn: &Connection) -> libsql::Result<usize> {
        let stmt = conn
            .prepare("DELETE FROM MovementInfo WHERE id = (1?)")
            .await?;
        stmt.execute([id]).await
    }
}

// Todo: tests
fn sql_val_to_f32(row: &Row, idx: i32) -> Result<f32, libsql::Error> {
    row.get_str(idx)?
        .parse::<f32>()
        .map_err(|e| libsql::Error::InvalidParserState(e.to_string()))
}

impl DAO for Geodata {
    async fn insert_to_db(&self, conn: &Connection) -> libsql::Result<u64> {
        conn.execute(
            "INSERT INTO \
             Geodata \
            VALUES (?,?,?)",
            params![
                self.id.clone(),
                self.coordinates.clone(),
                self.timestamp.to_string(),
            ],
        )
        .await
    }

    async fn get_by_id(id: String, conn: &Connection) -> Result<Vec<Self>, libsql::Error>
    where
        Self: Sized,
    {
        let stmt = conn
            .prepare("SELECT id, coordinates,timestamp FROM Geodata WHERE id = (?)")
            .await?;
        let mut rows = stmt.query([id]).await?;
        let mut data: Vec<Geodata> = vec![];

        while let Some(row) = rows.next().await? {
            let info = Geodata {
                id: row.get(0)?,
                coordinates: row.get(1)?,
                timestamp: sql_val_to_time(&row, 3)?,
            };

            data.push(info);
        }
        Ok(data)
    }

    async fn remove_from_db(&self, id: String, conn: &Connection) -> libsql::Result<usize> {
        let stmt = conn.prepare("DELETE FROM Geodata WHERE id = (1?)").await?;
        stmt.execute([id]).await
    }
}

// Todo: tests
fn sql_val_to_time(row: &Row, idx: i32) -> Result<DateTime<Utc>, libsql::Error> {
    let s = row.get_str(idx)?;
    let now_utc = Utc::now();
    let naive = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.3f %Z")
        .map_err(|e| libsql::Error::InvalidParserState(e.to_string()))?;
    let dt: DateTime<Utc> = DateTime::<Utc>::from_naive_utc_and_offset(naive, *now_utc.offset());

    Ok(dt)
}

impl DAO for BatteryInfo {
    async fn insert_to_db(&self, conn: &Connection) -> libsql::Result<u64> {
        conn.execute(
            "INSERT INTO \
             BatteryInfo \
            VALUES (?,?,?,?,?)",
            params![
                self.id.clone(),
                self.capacity,
                self.total_capacity,
                self.health,
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
            .prepare("SELECT id, capacity,total_capacity, health,timestamp FROM  BatteryInfo WHERE id = (?)")
            .await?;
        let mut rows = stmt.query([id]).await?;
        let mut data: Vec<BatteryInfo> = vec![];

        while let Some(row) = rows.next().await? {
            let info = BatteryInfo {
                id: row.get(0)?,
                capacity: sql_val_to_f32(&row, 1)?,
                total_capacity: sql_val_to_f32(&row, 2)?,
                health: sql_val_to_f32(&row, 3)?,
                timestamp: sql_val_to_time(&row, 4)?,
            };
            data.push(info);
        }
        Ok(data)
    }

    async fn remove_from_db(&self, id: String, conn: &Connection) -> libsql::Result<usize> {
        let stmt = conn
            .prepare("DELETE FROM BatteryInfo WHERE id = (1?)")
            .await?;
        stmt.execute([id]).await
    }
}
