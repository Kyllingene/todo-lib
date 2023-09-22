use std::fmt::Display;

use crate::{helper::IsDue, Todo, colors::StyleScheme};

/// A list of todos, under a title.
///
/// Example:
/// ```
/// use todo_lib::prelude::*;
///
/// let mut todos = TodoColumn::new("Todo");
/// todos.add(Todo::new("Buy mangos", TodoDate::Never, TodoPriority::None));
/// todos.add(Todo::new("Sort stamps", TodoDate::Never, TodoPriority::None));
///
/// todos.get("Buy mangos").expect("Failed to get todo").complete();
/// todos.pop("Sort stamps").expect("Failed to remove todo");
/// ```
#[derive(Debug, Clone)]
pub struct TodoColumn {
    pub todos: Vec<Todo>,
    pub title: String,
}

impl TodoColumn {
    /// Returns an empty TodoColumn.
    pub fn new<S: ToString>(title: S) -> Self {
        TodoColumn {
            todos: Vec::new(),
            title: title.to_string(),
        }
    }

    /// Adds a todo to the column.
    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    /// Searches for the todo by title. If found, returns it, and removes it from the column.
    pub fn pop<S: ToString>(&mut self, title: S) -> Option<Todo> {
        for (i, todo) in self.todos.iter().enumerate() {
            if todo.description.to_string(StyleScheme::default()) == title.to_string() {
                return Some(self.todos.remove(i));
            }
        }

        None
    }

    /// Searches for the todo by title. If found, returns a mutable reference to it.
    pub fn get<S: ToString>(&mut self, title: S) -> Option<&mut Todo> {
        self.todos
            .iter_mut()
            .find(|todo| todo.description.to_string(StyleScheme::default()) == title.to_string())
    }

    /// Returns the first todo found with a given metadata key.
    ///
    /// If no such todo is found, returns None.
    pub fn has_meta<S: ToString>(&mut self, key: S) -> Option<&mut Todo> {
        self.todos
            .iter_mut()
            .find(|todo| todo.get_meta(key.to_string()).is_some())
    }

    /// Returns the first todo found with a given metadata key:val pair.
    ///
    /// If no such todo is found, returns None.
    pub fn get_meta<S: ToString>(&mut self, key: S, val: S) -> Option<&mut Todo> {
        self.todos.iter_mut().find(|todo| {
            todo.get_meta(key.to_string())
                .map_or(false, |v| v == &val.to_string())
        })
    }
}

impl IsDue for TodoColumn {
    /// Returns true if any the of the contained todos are due.
    fn due(&self) -> bool {
        self.todos.iter().any(|t| t.due())
    }
}

impl Display for TodoColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "| {} |", self.title)?;
        for todo in self.todos.iter() {
            writeln!(f, "| {todo}")?;
        }

        Ok(())
    }
}

/// A table of todos.
///
/// Example:
/// ```rust
/// use todo_lib::prelude::*;
///
/// let mut todos = TodoTable::new(Some("Todos"));
/// todos.add_col("Work");
/// todos.add_col("Home");
///
/// todos.add_todo(Todo::new("Review documents", TodoDate::Never, TodoPriority::None), "Work");
/// todos.add_todo(Todo::new("Clean desk", TodoDate::Never, TodoPriority::None), "Home");
///
/// let todo1 = todos.get_todo("Clean desk", "Home");
/// assert!(todo1.is_some(), "Failed to retrieve todo 1");
///
/// todo1.unwrap().complete();
///
/// todos.move_todo("Clean desk".into(), "Home", "Work");
///
/// let todo2 = todos.get_todo("Review documents", "Work");
/// assert!(todo2.is_some(), "Failed to retrieve todo 2");
///
/// todo2.unwrap().complete();
/// ```
#[derive(Debug, Clone)]
pub struct TodoTable {
    title: String,
    columns: Vec<TodoColumn>,
}

impl IsDue for TodoTable {
    /// Returns true if any the of the contained columns contain a todo which is due.
    fn due(&self) -> bool {
        self.columns.iter().any(|tt| tt.due())
    }
}

impl Display for TodoTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "=== {} ===", self.title)?;
        for col in self.columns.iter() {
            writeln!(f, "{col}\n")?;
        }

        Ok(())
    }
}

impl TodoTable {
    /// Returns an empty TodoTable.
    pub fn new<S: ToString>(title: Option<S>) -> Self {
        TodoTable {
            title: title.map_or("Todos".into(), |s| s.to_string()),
            columns: Vec::new(),
        }
    }

    /// Adds a column to the table.
    pub fn add_col<S: ToString>(&mut self, title: S) {
        self.columns.push(TodoColumn::new(title));
    }

    /// Searches for the todo by title in a column.
    /// If found, returns a mutable reference to it.
    pub fn get_todo<S: ToString>(&mut self, title: S, col_title: S) -> Option<&mut Todo> {
        let col_title = col_title.to_string();
        self.columns
            .iter_mut()
            .find(|col| col.title == col_title)?
            .get(title)
    }

    /// Adds a todo to a column.
    pub fn add_todo<S: ToString>(&mut self, todo: Todo, col_title: S) {
        for col in self.columns.iter_mut() {
            if col.title == col_title.to_string() {
                col.add(todo);
                break;
            }
        }
    }

    /// Moves a todo from one column to another.
    /// If either column or the todo doesn't exist, returns false.
    pub fn move_todo<S: ToString>(&mut self, title: S, from: S, to: S) -> bool {
        let mut todo = None;
        let mut to_col = None;
        let iter = self.columns.iter_mut();
        for col in iter {
            if col.title == from.to_string() {
                todo = col.get(title);
                break;
            } else if col.title == to.to_string() {
                to_col = Some(col);
            }
        }

        if todo.is_none() || to_col.is_none() {
            return false;
        }

        to_col.unwrap().add(todo.unwrap().clone());
        true
    }

    /// Searches for a column by name. If found, returns a mutable reference.
    pub fn col<S: ToString>(&mut self, title: S) -> Option<&mut TodoColumn> {
        self.columns
            .iter_mut()
            .find(|col| col.title == title.to_string())
    }

    /// Returns the first todo found in a column with a given metadata key.
    ///
    /// If no such todo is found, returns None.
    pub fn has_meta<S: ToString>(&mut self, title: S, key: S) -> Option<&mut Todo> {
        self.columns
            .iter_mut()
            .find(|col| col.title == title.to_string())?
            .has_meta(key.to_string())
    }

    /// Returns the first todo found in a column with a given metadata key:val pair.
    ///
    /// If no such todo is found, returns None.
    pub fn get_meta<S: ToString>(&mut self, title: S, key: S, val: S) -> Option<&mut Todo> {
        self.columns
            .iter_mut()
            .find(|col| col.title == title.to_string())?
            .get_meta(key.to_string(), val.to_string())
    }
}
