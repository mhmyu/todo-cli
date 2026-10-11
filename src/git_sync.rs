use git2::{Repository, Signature};
use std::path::Path;

pub fn auto_sync_to_github() {
    // 1. Buka repository git dari direktori saat ini
    let repo = match Repository::discover(".") {
        Ok(r) => r,
        Err(_) => {
            // Jika belum diinisialisasi git, lewati sinkronisasi otomatis
            return;
        }
    };

    // 2. Tambahkan file tasks.json ke index (git add)
    let mut index = match repo.index() {
        Ok(idx) => idx,
        Err(_) => return,
    };

    if let Err(_) = index.add_path(Path::new("tasks.json")) {
        return;
    }

    if let Err(_) = index.write() {
        return;
    }

    // 3. Buat commit otomatis
    let tree_id = match index.write_tree() {
        Ok(id) => id,
        Err(_) => return,
    };

    let tree = match repo.find_tree(tree_id) {
        Ok(t) => t,
        Err(_) => return,
    };

    let signature = match Signature::now("Todo CLI Bot", "todocli@local.dev") {
        Ok(s) => s,
        Err(_) => return,
    };

    // Ambil HEAD commit saat ini (jika ada) sebagai parent commit
    let parent_commit = match repo.head() {
        Ok(head) => match head.target() {
            Some(target) => match repo.find_commit(target) {
                Ok(c) => vec![c],
                Err(_) => vec![],
            },
            None => vec![],
        },
        Err(_) => vec![],
    };

    let parents: Vec<&git2::Commit> = parent_commit.iter().collect();

    // Lakukan commit
    let commit_result = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "auto-sync: update daily tasks",
        &tree,
        &parents,
    );

    if commit_result.is_err() {
        return;
    }

    println!("ℹ [Git Sync] Perubahan berhasil di-commit secara lokal.");

    // 4. Lakukan Push ke Remote Repository (origin main/master)
    let mut remote = match repo.find_remote("origin") {
        Ok(r) => r,
        Err(_) => {
            println!("ℹ [Git Sync] Remote 'origin' tidak ditemukan. Lewati proses push ke GitHub.");
            return;
        }
    };

    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
    });

    let mut push_options = git2::PushOptions::new();
    push_options.remote_callbacks(callbacks);

    // Ambil nama branch aktif saat ini (Menyesuaikan git2 v0.21.0: mengembalikan Result)
    let head = match repo.head() {
        Ok(h) => h,
        Err(_) => return,
    };
    
    let branch_name = match head.shorthand() {
        Ok(b) => b,
        Err(_) => "main", // Default ke "main" jika gagal dibaca
    };

    let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
    
    print!("ℹ [Git Sync] Mengirim (push) ke GitHub... ");
    match remote.push(&[refspec.as_str()], Some(&mut push_options)) {
        Ok(_) => println!("Berhasil!"),
        Err(e) => println!("Gagal push otomatis (Pastikan SSH/Token Git sudah disetup): {}", e),
    }
}
