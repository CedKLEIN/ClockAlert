// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
use database::{Alarm, Database};
use std::{thread, sync::Arc, sync::Mutex, time::Duration};
use chrono::Local;
use tauri::{Manager, State};

#[tauri::command]
fn add_alarm(db: State<'_, Arc<Mutex<Database>>>, time: String) {
    if let Err(err) = db.lock().unwrap().add_alarm(time) {
        println!("Error: {}", err);
    }
}

#[tauri::command]
fn remove_alarm(db: State<'_, Arc<Mutex<Database>>>, id: i32) {
    if let Err(err) = db.lock().unwrap().remove_alarm(id) {
        println!("Error: {}", err);
    }
}

#[tauri::command]
fn list_alarms(db: State<'_, Arc<Mutex<Database>>>) -> Result<Vec<Alarm>, String> {
    match db.lock().unwrap().list_alarms() {
        Ok(response) => Ok(response),
        Err(err) => {
            println!("Error: {}", err);
            Ok(vec![])
        }
    }
}

fn main() {
    let db_file = "ClockAlertDB.sqlite";

    let db = match Database::new(db_file) {
        Ok(db) => db,
        Err(err) => {
            eprintln!("Error initializing database: {}", err);
            return;
        }
    };

    if let Err(e) = db.use_connection() {
        eprintln!("Error using dataabse connection: {}", e);
    }

    let db = Arc::new(Mutex::new(db));

    tauri::Builder::default()
    .manage(db.clone())
    .invoke_handler(tauri::generate_handler![
        add_alarm,
        remove_alarm,
        list_alarms
    ])
    .setup(move |app| {
        let app_handle = app.handle();
        let db_clone = db.clone();

        thread::spawn(move || loop {
            let current_time = Local::now().format("%H:%M:%S").to_string();

            if let Ok(alarms) = db_clone.lock().unwrap().list_alarms() {
                for alarm in alarms {
                    if alarm.time == current_time {
                        app_handle.emit_all("alarm_triggered", alarm.id).unwrap();
                    }
                }
            }
            thread::sleep(Duration::from_secs(1));
        });

        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}
