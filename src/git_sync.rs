use std::path::PathBuf;
use std::process::Command;

pub fn auto_sync_to_github() {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let todo_dir = home_dir.join(".todo-cli");

    // Pastikan folder .todo-cli memiliki repository git
    if !todo_dir.join(".git").exists() {
        return;
    }

    // 1. Git add semua perubahan (tasks.json & file arsip) di dalam ~/.todo-cli
    let status = Command::new("git")
        .args(["-C", todo_dir.to_str().unwrap(), "add", "."])
        .status();

    if status.is_err() {
        return;
    }

    // 2. Git commit (diabaikan jika tidak ada perubahan / nothing to commit)
    let _ = Command::new("git")
        .args([
            "-C",
            todo_dir.to_str().unwrap(),
            "commit",
            "-m",
            "chore: auto-sync global tasks and archives",
        ])
        .status();

    // 3. Git push ke remote origin main
    let push_status = Command::new("git")
        .args([
            "-C",
            todo_dir.to_str().unwrap(),
            "push",
            "origin",
            "main",
        ])
        .status();

    match push_status {
        Ok(s) if s.success() => {
            println!("✓ Berhasil menyinkronkan data global ke GitHub!");
        }
        _ => {
            // Gagal push (misal: belum set remote, atau tidak ada koneksi internet)
            // Dibiarkan silent agar aplikasi tidak error/crash
        }
    }
}
