use std::{future::Future, path::PathBuf};

use crate::{ListenerData, OrganizerState};
use tauri::{Runtime, State, Window};

#[tauri::command]
pub fn add_listener(listener: ListenerData, state: State<OrganizerState>) {
    let mut s = state.organizer.lock().unwrap();
    println!("Adding Listener: {:#?}", listener);
    s.push(listener)
}

#[tauri::command]
pub fn update_listener(listener: ListenerData, state: State<OrganizerState>) {
    let mut s = state.organizer.lock().unwrap();
    s.replace(listener);
}

#[tauri::command]
pub fn delete_listener(listener: ListenerData, state: State<OrganizerState>) {
    println!("Deleting listener: {:?}", listener);
    let mut s = state.organizer.lock().unwrap();
    s.delete(listener)
}

#[tauri::command(async)]
pub fn organize<R: Runtime>(state: State<OrganizerState>, window: Window<R>) -> () {
    let mut s = state.organizer.lock().unwrap();
    println!("-------------------------------------------------------Organizing-------------------------------------------------------");
    // println!("{:#?}\n\n", s.get_data());
    s.organize(&window)
}

#[tauri::command]
pub fn dir_len(path: PathBuf) -> i32 {
    if path.exists() {
        let paths = std::fs::read_dir(path).unwrap();

        return paths.collect::<Vec<_>>().len() as i32;
    }
    -1
}

#[tauri::command]
pub fn debug<R: Runtime>(arg: ListenerData, window: Window<R>) -> String {
    println!("{:?}", arg);
    window.emit("jkl", "JKL").unwrap();
    format!("{:#?}", arg)
}