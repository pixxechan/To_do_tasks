# To-Do Tasks (Rust CLI)

A simple command-line to-do list application written in Rust.  
Manage your daily tasks efficiently from your terminal!

## Features

- **Add Tasks:** Quickly add new tasks to your to-do list.
- **List Tasks:** See all your tasks with their completion status.
- **Mark as Done:** Mark tasks as completed.
- **Simple Interface:** Easy-to-use text-based menu.
- **Lightweight:** No dependencies beyond Rust standard library.

## Usage

1. **Run the Application**

   Compile and run the program:

   ```bash
   cargo run
   ```

2. **Menu Options**

   - `1. Add task` - Enter a new task description to add it to your list.
   - `2. List tasks` - View all your current tasks and their status.
   - `3. Mark task as done` - Mark a task as completed by entering its number.
   - `4. Exit` - Quit the application.

## Example

```
--- To-Do List ---
1. Add task
2. List tasks
3. Mark task as done
4. Exit
Choose an option: 1
Enter task description: Buy groceries
Task added!
```

## Notes

- **Persistence:** This version does not save your tasks after exiting. All data is kept in memory.
- **Improvements Welcome!** File persistence, task deletion, and more features can be added.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## Contributing

Pull requests and suggestions are welcome!  
Feel free to fork the repository and submit your improvements.
