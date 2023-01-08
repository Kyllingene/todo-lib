//! Relatively simple todo management.
//!
//! Example:
//! ```
//! use todo_lib::{Todo, TodoDate, TodoTable};
//!
//! let mut todos = TodoTable::new(Some("Todos"));
//! todos.add_col("Work");
//! todos.add_col("Home");
//!
//! todos.add_todo(Todo::new("Review documents", TodoDate::Never, None), "Work");
//! todos.add_todo(Todo::new("Clean desk", TodoDate::Never, None), "Home");
//!
//! let todo1 = todos.get_todo("Clean desk", "Home");
//! assert!(todo1.is_some(), "Failed to retrieve todo 1");
//!
//! todo1.unwrap().complete();
//!
//! todos.move_todo("Clean desk".into(), "Home", "Work");
//!
//! let todo2 = todos.get_todo("Review documents", "Work");
//! assert!(todo2.is_some(), "Failed to retrieve todo 2");
//!
//! todo2.unwrap().complete();
//! ```

use std::fmt::Display;

use datetime::{convenience::Today, DatePiece, LocalDate, LocalDateTime, LocalTime, TimePiece};

pub trait IsDue {
    fn due(&self) -> bool;
}

/**
 * A due date for a Todo. Encapsulates data structures from crate `datetime`.
 *
 * Example:
 * ```
 * use todo_lib::{TodoDate, IsDue};
 * use datetime::{LocalDate, convenience::Today};
 *
 * let today_deadline = TodoDate::Day(LocalDate::today());
 * assert!(today_deadline.due());
 *
 * let indefinite_deadline = TodoDate::Never;
 * assert!(!indefinite_deadline.due());
 * ```
 */
#[derive(Debug, Clone, Default)]
pub enum TodoDate {
    #[default]
    Never,
    Always,

    Daily(LocalTime),
    Day(LocalDate),
    Instant(LocalDateTime),
}

impl IsDue for TodoDate {
    /// Returns true if it is currently on or past the due date.
    fn due(&self) -> bool {
        match self {
            Self::Never => false,
            Self::Always => true,
            Self::Daily(t) => {
                let now = LocalDateTime::now();
                let lt_now = LocalTime::hms_ms(now.hour(), now.minute(), 0, 0).unwrap();

                *t <= lt_now
            }
            Self::Day(t) => *t <= LocalDate::today(),
            Self::Instant(t) => *t <= LocalDateTime::now(),
        }
    }
}

impl Display for TodoDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Never => write!(f, ": due never"),
            Self::Always => write!(f, ": due always"),
            Self::Daily(t) => write!(f, ": due {}:{}:{} daily", t.hour(), t.minute(), t.second()),
            Self::Day(t) => write!(
                f,
                ": due {}/{}/{}",
                t.month().months_from_january() + 1,
                t.day(),
                t.year()
            ),
            Self::Instant(t) => write!(
                f,
                ": due {}/{}/{} {}:{}:{}",
                t.month().months_from_january() + 1,
                t.day(),
                t.year(),
                t.hour(),
                t.minute(),
                t.second()
            ),
        }
    }
}

impl TodoDate {
    /// Convenience method for `!= TodoDate::Never`.
    pub fn is_some(&self) -> bool {
        match self {
            Self::Never => false,
            _ => true,
        }
    }

    /// Convenience method for `== TodoDate::Never`.
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}

/**
 * A todo.
 *
 * When creating with `Todo::new`, sets `created` to the current time.
 * When creating with `Todo::default`, sets `created` to None.
 *
 * Example:
 * ```
 * use todo_lib::{Todo, TodoDate, IsDue};
 *
 * let mut todo = Todo::new(
 *     "Check the mail",
 *     TodoDate::Always,
 *     None
 * );
 *
 * assert!(!todo.completed && todo.due());
 *
 * todo.complete();
 *
 * assert!(todo.completed && !todo.due());
 * ```
 */
#[derive(Debug, Clone, Default)]
pub struct Todo {
    pub title: String,
    pub description: Option<String>,

    pub completed: bool,

    pub deadline: TodoDate,
    pub created: Option<LocalDateTime>,
}

impl IsDue for Todo {
    /// Returns true if it is currently on or past the due date,.
    /// unless the todo is already complete.
    fn due(&self) -> bool {
        !self.completed && self.deadline.due()
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tick = if self.completed { "X" } else { " " };

        write!(f, "[{}] {}{}", tick, self.title, self.deadline)
    }
}

impl Todo {
    /// Returns a new Todo.
    pub fn new<S: ToString>(title: S, deadline: TodoDate, description: Option<S>) -> Self {
        Todo {
            deadline,
            created: Some(LocalDateTime::now()),

            completed: false,

            title: title.to_string(),
            description: description.map(|s| s.to_string()),
        }
    }

    /// Marks the todo as complete.
    pub fn complete(&mut self) {
        self.completed = true;
    }
}

/**
 * A list of todos, under a title.
 *
 * Example:
 * ```
 * use todo_lib::{Todo, TodoColumn, TodoDate};
 *
 * let mut todos = TodoColumn::new("Todo");
 * todos.add(Todo::new("Buy mangos", TodoDate::Never, None));
 * todos.add(Todo::new("Sort stamps", TodoDate::Never, None));
 *
 * todos.get("Buy mangos").expect("Failed to get todo").complete();
 * todos.pop("Sort stamps").expect("Failed to remove todo");
 * ```
 */
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
            if todo.title == title.to_string() {
                return Some(self.todos.remove(i));
            }
        }

        None
    }

    /// Searches for the todo by title. If found, returns a mutable reference to it.
    pub fn get<S: ToString>(&mut self, title: S) -> Option<&mut Todo> {
        for todo in self.todos.iter_mut() {
            if todo.title == title.to_string() {
                return Some(todo);
            }
        }

        None
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
        write!(f, "| {} |\n", self.title)?;
        for todo in self.todos.iter() {
            write!(f, "| {}\n", todo)?;
        }

        Ok(())
    }
}

/**
 * A table of todos.
 *
 * Example:
 * ```
 * use todo_lib::{Todo, TodoDate, TodoTable};
 *
 * let mut todos = TodoTable::new(Some("Todos"));
 * todos.add_col("Work");
 * todos.add_col("Home");
 *
 * todos.add_todo(Todo::new("Review documents", TodoDate::Never, None), "Work");
 * todos.add_todo(Todo::new("Clean desk", TodoDate::Never, None), "Home");
 *
 * let todo1 = todos.get_todo("Clean desk", "Home");
 * assert!(todo1.is_some(), "Failed to retrieve todo 1");
 *
 * todo1.unwrap().complete();
 *
 * todos.move_todo("Clean desk".into(), "Home", "Work");
 *
 * let todo2 = todos.get_todo("Review documents", "Work");
 * assert!(todo2.is_some(), "Failed to retrieve todo 2");
 *
 * todo2.unwrap().complete();
 * ```
 */
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
            write!(f, "{}\n\n", col)?;
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
        for col in self.columns.iter_mut() {
            if col.title == col_title.to_string() {
                if let Some(todo) = col.get(title.to_string()) {
                    return Some(todo);
                }
            }
        }

        None
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
        for col in self.columns.iter_mut() {
            if col.title == title.to_string() {
                return Some(col);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    /// Tests creation, addition, completion, and moving of todos and tables.
    fn create_add_complete_move() {
        let mut todos = TodoTable::new::<String>(None);
        todos.add_col("A");
        todos.add_col("B");

        todos.add_todo(Todo::new("1", TodoDate::Never, None), "A");
        todos.add_todo(Todo::new("2", TodoDate::Never, None), "B");

        let todo1 = todos.get_todo("1", "A");
        assert!(todo1.is_some(), "Failed to retrieve todo 1");

        todo1.unwrap().complete();

        assert!(todos.move_todo("2", "B", "A"));

        let todo2 = todos.get_todo("2", "A");
        assert!(todo2.is_some(), "Failed to retrieve todo 2");

        todo2.unwrap().complete();
    }

    #[test]
    /// Tests due dates.
    fn is_due() {
        let mut todo = Todo::new("", TodoDate::Instant(LocalDateTime::at(0)), None);
        assert!(
            todo.due(),
            "Should be due, isn't; deadline{}",
            todo.deadline
        );

        todo.complete();
        assert!(
            !todo.due(),
            "Shouldn't be due, is; deadline{}",
            todo.deadline
        );

        let todo = Todo::new("", TodoDate::Day(LocalDate::today()), None);
        assert!(
            todo.due(),
            "Should be due, isn't; deadline{}",
            todo.deadline
        );

        let todo = Todo::new("", TodoDate::Daily(LocalTime::midnight()), None);
        assert!(
            todo.due(),
            "Should be due, isn't; deadline{}",
            todo.deadline
        );

        let todo = Todo::new(
            "",
            TodoDate::Daily(LocalTime::hms(23, 59, 59).expect("Failed to create LocalTime")),
            None,
        );
        assert!(
            !todo.due(),
            "Shouldn't be due, is; deadline{}",
            todo.deadline
        );

        let todo = Todo::new("", TodoDate::Always, None);
        assert!(
            todo.due(),
            "Should be due, isn't; deadline{}",
            todo.deadline
        );

        let todo = Todo::default();
        assert!(
            !todo.due(),
            "Shouldn't be due, is; deadline{}",
            todo.deadline
        );
    }
}
