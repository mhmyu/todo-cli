mod models;
mod storage;
mod git_sync;

use chrono::Local;
use clap::{Parser, Subcommand};
use git_sync::auto_sync_to_github;
use models::Task;
use storage::{load_data, save_data};

#[derive(Parser)]
#[command(name = "todo-cli")]
#[command(about = "Aplikasi To-Do List Harian berbasis Terminal dengan Alokasi Waktu & Sub-Task", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Menambah tugas utama baru
    Add {
        #[arg(short = 'n', long)]
        title: String,
        #[arg(short = 't', long)]
        time: u32,
    },
    /// Menambah sub-tugas ke dalam tugas utama
    Subtask {
        #[arg(long)]
        task_id: usize,
        #[arg(short = 'n', long)]
        title: String,
        #[arg(short = 't', long)]
        time: u32,
    },
    /// Menampilkan daftar tugas untuk hari ini
    List,
    /// Menandai sub-tugas selesai untuk hari ini
    Done {
        #[arg(long)]
        task_id: usize,
        #[arg(long)]
        subtask_id: usize,
    },
    /// Menandai tugas utama/proyek selesai secara permanen (berhenti muncul setiap hari)
    Complete {
        #[arg(long)]
        task_id: usize,
    },
    /// Mengarsipkan tasks.json lama sebagai history dan memulai periode baru dengan tasks.json yang bersih
    Archive,
}

fn main() {
    let cli = Cli::parse();
    let mut storage = load_data();
    let today = Local::now().format("%Y-%m-%d").to_string();

    match cli.command {
        Commands::Add { title, time } => {
            let new_id = storage.tasks.len() + 1;
            let new_task = Task::new(new_id, title.clone(), time);
            storage.tasks.push(new_task);

            if let Err(e) = save_data(&storage) {
                println!("✗ Gagal menyimpan data: {}", e);
            } else {
                println!("✓ Berhasil menambah tugas utama '{}' dengan alokasi {} menit/hari.", title, time);
                auto_sync_to_github();
            }
        }
        Commands::Subtask { task_id, title, time } => {
            let task_result = {
                let task = storage.tasks.iter_mut().find(|t| t.id == task_id);
                match task {
                    Some(t) => {
                        let task_title = t.title.clone();
                        match t.add_subtask(title.clone(), time) {
                            Ok(_) => Ok(task_title),
                            Err(err_msg) => Err(err_msg),
                        }
                    }
                    None => Err(format!("Tugas utama dengan ID {} tidak ditemukan.", task_id)),
                }
            };

            match task_result {
                Ok(task_title) => {
                    if let Err(e) = save_data(&storage) {
                        println!("✗ Gagal menyimpan data: {}", e);
                    } else {
                        println!("✓ Berhasil menambah sub-task '{}' ({}m) ke tugas '{}'.", title, time, task_title);
                        auto_sync_to_github();
                    }
                }
                Err(err_msg) => println!("✗ {}", err_msg),
            }
        }
        Commands::List => {
            println!("========================================");
            println!(" DAFTAR TUGAS HARIAN - Tanggal: {}", today);
            println!("========================================");

            let active_tasks: Vec<&Task> = storage.tasks.iter().filter(|t| !t.is_globally_completed).collect();

            if active_tasks.is_empty() {
                println!("Belum ada tugas aktif. Gunakan perintah 'add' untuk menambah tugas.");
            } else {
                for task in active_tasks {
                    let current_sub_total = task.current_subtasks_total_time();
                    println!("\n[ID: {}] {} (Alokasi Maks: {}m | Total Sub-Task: {}m)", 
                        task.id, task.title, task.time_allocation_minutes, current_sub_total);
                    
                    if task.subtasks.is_empty() {
                        println!("  (Belum ada sub-task)");
                    } else {
                        for sub in &task.subtasks {
                            let is_done = sub.completion_history.get(&today).copied().unwrap_or(false);
                            let status_symbol = if is_done { "[✓]" } else { "[ ]" };
                            println!("  {} Sub-ID: {} - {} ({}m)", status_symbol, sub.id, sub.title, sub.time_allocation_minutes);
                        }
                    }
                }
            }
            println!("========================================");
        }
        Commands::Done { task_id, subtask_id } => {
            let update_result = {
                let task = storage.tasks.iter_mut().find(|t| t.id == task_id);
                match task {
                    Some(t) => {
                        let subtask = t.subtasks.iter_mut().find(|s| s.id == subtask_id);
                        match subtask {
                            Some(s) => {
                                let current_status = s.completion_history.get(&today).copied().unwrap_or(false);
                                s.completion_history.insert(today.clone(), !current_status);
                                Ok(s.title.clone())
                            }
                            None => Err(format!("Sub-task dengan ID {} tidak ditemukan pada tugas ini.", subtask_id)),
                        }
                    }
                    None => Err(format!("Tugas utama dengan ID {} tidak ditemukan.", task_id)),
                }
            };

            match update_result {
                Ok(sub_title) => {
                    if let Err(e) = save_data(&storage) {
                        println!("✗ Gagal menyimpan data: {}", e);
                    } else {
                        println!("✓ Status sub-task '{}' untuk tanggal {} berhasil diperbarui!", sub_title, today);
                        auto_sync_to_github();
                    }
                }
                Err(err_msg) => println!("✗ {}", err_msg),
            }
        }
        Commands::Complete { task_id } => {
            let update_result = {
                let task = storage.tasks.iter_mut().find(|t| t.id == task_id);
                match task {
                    Some(t) => {
                        t.is_globally_completed = true;
                        Ok(t.title.clone())
                    }
                    None => Err(format!("Tugas utama dengan ID {} tidak ditemukan.", task_id)),
                }
            };

            match update_result {
                Ok(task_title) => {
                    if let Err(e) = save_data(&storage) {
                        println!("✗ Gagal menyimpan data: {}", e);
                    } else {
                        println!("✓ Tugas utama '{}' (ID: {}) telah ditandai SELESAI SECARA PERMANEN dan tidak akan muncul lagi.", task_title, task_id);
                        auto_sync_to_github();
                    }
                }
                Err(err_msg) => println!("✗ {}", err_msg),
            }
        }
        Commands::Archive => {
            use std::fs;

            // Ambil path global tasks.json menggunakan logika yang sama dengan storage.rs
            let home_dir = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
            let todo_dir = home_dir.join(".todo-cli");
            let file_path = todo_dir.join("tasks.json");
            
            if !file_path.exists() {
                println!("ℹ File tasks.json tidak ditemukan di direktori global, tidak ada yang perlu diarsipkan.");
                return;
            }

            // 1. Load data saat ini
            let current_storage = load_data();

            // 2. Tentukan nama file arsip di dalam folder ~/.todo-cli/
            let archive_filename = format!("tasks_archive_{}.json", Local::now().format("%Y-%m-%d_%H-%M-%S"));
            let archive_path = todo_dir.join(archive_filename);

            // 3. Pindahkan file tasks.json lama menjadi file arsip
            match fs::rename(&file_path, &archive_path) {
                Ok(_) => {
                    println!("✓ Berhasil mengarsipkan seluruh riwayat lama ke: {:?}", archive_path);
                    
                    // 4. Saring tugas yang BELUM selesai secara global untuk dibawa ke periode baru
                    let active_tasks: Vec<Task> = current_storage
                        .tasks
                        .into_iter()
                        .filter(|t| !t.is_globally_completed)
                        .collect();

                    let count_carried_over = active_tasks.len();

                    // 5. Buat storage baru hanya dengan tugas-tugas yang masih aktif/belum selesai
                    let new_storage = models::Storage {
                        tasks: active_tasks,
                    };

                    if let Err(e) = save_data(&new_storage) {
                        println!("✗ Gagal membuat file tasks.json baru: {}", e);
                    } else {
                        println!("✓ File tasks.json baru berhasil dibuat dengan membawa {} tugas aktif yang belum selesai.", count_carried_over);
                        auto_sync_to_github();
                    }
                }
                Err(e) => println!("✗ Gagal mengarsipkan file: {}", e),
            }
        }
    }
}
