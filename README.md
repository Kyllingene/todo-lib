# todo

Relatively simple todo management.

Example:
```rust
use todo_lib::{Todo, TodoDate, TodoTable};

let mut todos = TodoTable::new(Some("Todos"));
todos.add_col("Work");
todos.add_col("Home");

todos.add_todo(Todo::new("Review documents", TodoDate::Never, None), "Work");
todos.add_todo(Todo::new("Clean desk", TodoDate::Never, None), "Home");

let todo1 = todos.get_todo("Clean desk", "Home");
assert!(todo1.is_some(), "Failed to retrieve todo 1");

todo1.unwrap().complete();

todos.move_todo("Clean desk".into(), "Home", "Work");

let todo2 = todos.get_todo("Review documents", "Work");
assert!(todo2.is_some(), "Failed to retrieve todo 2");

todo2.unwrap().complete();
```