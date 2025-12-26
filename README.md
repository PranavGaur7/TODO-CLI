# TODO-CLI

A lightweight Command Line Interface (CLI) to-do list manager written in Rust. This tool allows you to manage tasks efficiently from your terminal, with data persistence using a local JSON file.

## 🚀 Features

* **Add Tasks**: Create new tasks easily.
* **Mark as Done**: Update task status to completed.
* **Delete Tasks**: Remove tasks from the list.
* **List Tasks**: View all tasks with visual status indicators (✅ / ❌).
* **Persistence**: Automatically saves and loads tasks from `todo.json`.

## 🛠️ Prerequisites

* [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your machine.

## 📦 Installation

1. Clone the repository:
```bash
git clone https://github.com/PranavGaur7/TODO-CLI.git
cd TODO-CLI

```


2. Build the project:
```bash
cargo build --release

```



## 💻 Usage

The application requires a **command** and a **name** argument to run.

**Note:** Based on the current argument parsing logic, you must provide a third argument (task name) even for commands that might not logically require it (like `list`).

### 1. Add a Task

Adds a new task to your list.

```bash
cargo run -- add "Buy Groceries"

```

### 2. List Tasks

Displays all current tasks. You must provide a placeholder argument (e.g., `_` or `all`) for the command to run successfully.

```bash
cargo run -- list _

```

*Output Example:*

```text
Buy Groceries ❌
Walk the dog ✅

```

### 3. Mark Task as Done

Marks a specific task as completed.

```bash
cargo run -- done "Buy Groceries"

```

### 4. Delete a Task

Removes a task from the list permanently.

```bash
cargo run -- delete "Buy Groceries"

```

## 📂 Project Structure

The project relies on the following structure:

* `src/main.rs`: Contains the application logic, argument parsing, and file I/O.
* `todo.json`: Stores the tasks in JSON format. This file is automatically created if it does not exist.
* `Cargo.toml`: Manages dependencies and project metadata.

## 🦀 Dependencies

This project uses the following Rust crates:

* **[serde](https://crates.io/crates/serde)**: Framework for serializing and deserializing Rust data structures.
* **[serde_json](https://crates.io/crates/serde_json)**: For parsing and formatting JSON data.
* **[uuid](https://crates.io/crates/uuid)**: For generating unique IDs (included in dependencies).

---

**Would you like me to refactor the `Args::build` function in `main.rs` so that the `list` command doesn't require a dummy argument?**
