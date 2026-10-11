# Todo-CLI: Daily Hierarchical Task Manager in Rust

A robust, terminal-based daily task manager built with **Rust**. Designed for time-boxing, structured task hierarchies, daily task persistence, intelligent log archiving, and automated cross-system GitHub synchronization.

## 🚀 Key Features & Core Logic

1. **Global Storage Architecture**:
   - Stores data globally at `~/.todo-cli/tasks.json` using system home directory resolution.
   - Accessible from any terminal path or directory seamlessly without fragmenting project files.

2. **Hierarchical Tasks & Time-Boxing Validation**:
   - Organizes tasks into parent projects containing multiple sub-tasks.
   - Enforces **strict daily time allocation**: The total sum of sub-task durations cannot exceed the maximum daily time allocated for the parent project.

3. **Daily Recurrence & Completed Resolution**:
   - Sub-tasks track daily completions across dates (`YYYY-MM-DD`).
   - Active tasks recur daily until explicitly completed or globally resolved using the `complete` command.

4. **Smart Historical Archiving (`archive`)**:
   - Archives current tasks and completion history into a timestamped file (`tasks_archive_*.json`).
   - Automatically carries over unfinished active tasks to a newly initialized `tasks.json`.

5. **Automated GitHub Synchronization**:
   - Seamlessly uses `git2` to perform background `staging`, `committing`, and `pushing` of data changes directly to your remote repository.

---

## 🛠️ Tech Stack & Dependencies

* **Language**: Rust (2024 Edition)
* **CLI Parser**: `clap` (with derive macros)
* **Data Serialization**: `serde` & `serde_json`
* **Date & Time**: `chrono`
* **Path Resolution**: `dirs`
* **Git Automation**: `git2`

---

## ⚙️ Installation & Setup

### Prerequisites

Ensure you have Rust and Cargo installed on your system. If not, install them via [rustup.rs](https://rustup.rs/).

### Step 1: Install Globally

Run the following command inside the project directory:

```bash
cargo install --path . --force
```

This compiles the binary in release mode and moves it into your system PATH (`~/.cargo/bin/`), making `todo-cli` accessible from anywhere in your terminal.

### Step 2: One-Time Global Git Sync Setup (Optional)

To enable automatic GitHub backup across any directory:

```bash
cd ~/.todo-cli
git init
git branch -M main
git remote add origin <YOUR_GITHUB_REPOSITORY_URL>
```

---

## 📖 Command Reference & Usage

### 1. Create a Parent Task (Project)
Create a top-level task with a daily time limit (in minutes).

```bash
todo-cli add --title "Landing Page Development" -t 60
```
*(Shorthand: `-n` for title, `-t` for time limit)*

### 2. Add a Sub-Task
Add a sub-task under a specific parent task ID (time validation is enforced automatically).

```bash
todo-cli subtask --task-id 1 -n "Design Hero Section" -t 20
```

### 3. List Today's Tasks
View all active projects, sub-tasks, and today's completion status (`[✓]` or `[ ]`).

```bash
todo-cli list
```

### 4. Toggle Sub-Task Completion
Toggle the completion status of a sub-task for today's date.

```bash
todo-cli done --task-id 1 --subtask-id 1
```

### 5. Mark Parent Task as Globally Complete
Globally complete a project so it no longer appears in your daily list.

```bash
todo-cli complete --task-id 1
```

### 6. Archive History & Reset Period
Archive all history to a timestamped file and carry unfinished tasks into a fresh `tasks.json`.

```bash
todo-cli archive
```