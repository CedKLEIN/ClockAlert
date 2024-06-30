use rusqlite::{Connection, Error as RusqliteError};
use std::sync::Mutex;

// Serialize mandatory, see https://tauri.app/v1/guides/features/command/#complete-example 
#[derive(serde::Serialize)]
pub struct Alarm {
    pub id: i32,
    pub time: String
}

pub struct Database {
    conn_mutex: Mutex<Connection>,
}

impl Database {
    const INSERT_ALARM_QUERY: &'static str = "INSERT INTO alarms (time) VALUES (?)";
    const REMOVE_ALARM_QUERY: &'static str = "DELETE FROM alarms WHERE id = ?";
    const SELECT_ALARMS_QUERY: &'static str = "SELECT id, time FROM alarms ORDER BY time";
    const CREATE_TABLE_QUERY: &'static str = "CREATE TABLE IF NOT EXISTS alarms (
                      id    INTEGER PRIMARY KEY,
                      time  TEXT NOT NULL UNIQUE
                      )";

    pub fn new(db_file: &str) -> Result<Self, RusqliteError> {
        let conn = Connection::open(db_file)?;
        Ok(Self {
            conn_mutex: Mutex::new(conn),
        })
    }

    pub fn use_connection(&self) -> Result<(), RusqliteError> {
        self.conn_mutex.lock().unwrap().execute(
            Self::CREATE_TABLE_QUERY,
            [],
        )?;
        Ok(())
    }

    pub fn add_alarm(&self, time: String) -> Result<(), RusqliteError> {
        self.conn_mutex.lock().unwrap().execute(
            Self::INSERT_ALARM_QUERY,
            &[&time],
        )?;
        Ok(())
    }

    pub fn remove_alarm(&self, id: i32) -> Result<(), RusqliteError> {
        self.conn_mutex.lock().unwrap().execute(
            Self::REMOVE_ALARM_QUERY,
            &[&id],
        )?;
        Ok(())
    }

    pub fn list_alarms(&self) -> Result<Vec<Alarm>, RusqliteError> {
        let binding = self.conn_mutex.lock().unwrap();
        let mut statement = binding.prepare(Self::SELECT_ALARMS_QUERY)?;
        let alarms_from_db = statement.query_map([], |row| {
            Ok(Alarm {
                id: row.get(0)?,
                time: row.get(1)?
            })
        })?;

        let mut alarms = Vec::new();
        for time_result in alarms_from_db {
            let time: Alarm = time_result?;
            alarms.push(time);
        }
        Ok(alarms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_database_in_memory() -> Database {
        let conn = Connection::open_in_memory().unwrap();
        Database {
            conn_mutex: Mutex::new(conn),
        }
    }

    #[test]
    fn list_alarms_when_empty() {
        let db = setup_database_in_memory();
        assert!(db.use_connection().is_ok());
        
        let alarms = db.list_alarms().unwrap();

        assert_eq!(alarms.len(), 0);
    }

    #[test]
    fn add_and_list_alarm() {
        let db = setup_database_in_memory();
        assert!(db.use_connection().is_ok());
        
        db.add_alarm("14:34:23".to_string()).unwrap();

        let alarms = db.list_alarms().unwrap();

        assert_eq!(alarms.len(), 1);
        assert_eq!(alarms[0].time, "14:34:23");
    }

    #[test]
    fn add_mutiple_alarms_and_check_ordering() {
        let db = setup_database_in_memory();
        assert!(db.use_connection().is_ok());
        
        db.add_alarm("14:34:23".to_string()).unwrap();
        db.add_alarm("18:37:27".to_string()).unwrap();
        db.add_alarm("18:39:27".to_string()).unwrap();

        let alarms = db.list_alarms().unwrap();

        assert_eq!(alarms.len(), 3);
        assert_eq!(alarms[0].time, "14:34:23");
        assert_eq!(alarms[1].time, "18:37:27");
        assert_eq!(alarms[2].time, "18:39:27");
    }

    #[test]
    fn add_mutiple_same_alarms() {
        let db = setup_database_in_memory();
        assert!(db.use_connection().is_ok());
        
        db.add_alarm("14:34:23".to_string()).unwrap();

        if let Err(err) = db.add_alarm("14:34:23".to_string()) {
            assert_eq!(err.to_string(), "UNIQUE constraint failed: alarms.time");
        } else {
            panic!("Expected a UNIQUE constraint error, but the operation succeeded.");
        }

        let alarms = db.list_alarms().unwrap();

        assert_eq!(alarms.len(), 1);
        assert_eq!(alarms[0].time, "14:34:23");
    }

    #[test]
    fn remove_alarm() {
        let db = setup_database_in_memory();
        assert!(db.use_connection().is_ok());
        
        db.add_alarm("12:00".to_string()).unwrap();

        let alarms = db.list_alarms().unwrap();
        let alarm_id = alarms[0].id;

        db.remove_alarm(alarm_id).unwrap();

        let alarms_after_removal = db.list_alarms().unwrap();
        assert_eq!(alarms_after_removal.len(), 0);
    }
}
