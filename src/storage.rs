use crate::models::Storage;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Mendapatkan path file tasks.json secara global di folder Home pengguna (~/.todo-cli/tasks.json)
fn get_storage_path() -> PathBuf {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let todo_dir = home_dir.join(".todo-cli");
    
    // Pastikan direktori ~/.todo-cli sudah ada
    if !todo_dir.exists() {
        let _ = fs::create_dir_all(&todo_dir);
    }
    
    todo_dir.join("tasks.json")
}

pub fn load_data() -> Storage {
    let path = get_storage_path();
    if !path.exists() {
        return Storage { tasks: Vec::new() };
    }

    let data = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(_) => return Storage { tasks: Vec::new() },
    };

    serde_json::from_str(&data).unwrap_or_else(|_| Storage { tasks: Vec::new() })
}

pub fn save_data(storage: &Storage) -> io::Result<()> {
    let path = get_storage_path();
    let data = serde_json::to_string_pretty(storage)?;
    fs::write(path, data)
}
