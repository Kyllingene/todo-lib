# todo

Relatively simple todo management. Supports parsing to/from [todo.txt](http://todotxt.org).

Example:
```rust
use todo_lib::{Todo, TodoDate, TodoTable, TodoPriority};

let mut todos = TodoTable::new(Some("Todos"));
todos.add_col("Work");
todos.add_col("Home");

todos.add_todo(Todo::new("Review documents", TodoDate::Never, TodoPriority::None), "Work");
todos.add_todo(Todo::new("Clean desk", TodoDate::Never, TodoPriority::None), "Home");

let todo1 = todos.get_todo("Clean desk", "Home");
assert!(todo1.is_some(), "Failed to retrieve todo 1");

todo1.unwrap().complete();

todos.move_todo("Clean desk".into(), "Home", "Work");

let todo2 = todos.get_todo("Review documents", "Work");
assert!(todo2.is_some(), "Failed to retrieve todo 2");

todo2.unwrap().complete();
```

Parsing from todo.txt format:
```rust
use todo_lib::{Todo, IsDue};
use std::str::FromStr;

let todo_text = "2023-01-07 Create a +todo @library due:2053-01-01";
let mut todo = Todo::from_str(todo_text).unwrap();

assert_eq!(todo.to_string(), todo_text.to_string());
assert!(!todo.due());

todo.completed = true;

assert_eq!(todo.to_string(), "x 2023-01-07 Create a +todo @library due:2053-01-01");
assert!(todo.has_project_tag("todo"));
assert!(todo.has_context_tag("library"));
```