use std::fmt::Display;

use datetime::{convenience::Today, DatePiece, LocalDate, LocalDateTime, LocalTime, TimePiece};

trait IsDue {
    fn due(&self) -> bool;
}

#[derive(Debug, Clone, Default)]
pub enum TodoDate {
    #[default]
    Never,

    Daily(LocalTime),
    Day(LocalDate),
    Instant(LocalDateTime),
}

impl IsDue for TodoDate {
    /// Returns true if it is currently on or past the due date
    fn due(&self) -> bool {
        match self {
            Self::Never => false,
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
            Self::Never => write!(f, ""),
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
    /// Convenience method for `!= TodoDate::Never`
    pub fn is_some(&self) -> bool {
        match self {
            Self::Never => false,
            _ => true,
        }
    }

    /// Convenience method for `== TodoDate::Never`
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Todo {
    pub title: String,
    pub description: Option<String>,

    pub completed: bool,

    pub deadline: TodoDate,
    pub created: Option<LocalDateTime>,
}

impl IsDue for Todo {
    /// Returns true if it is currently on or past the due date,
    /// unless the todo is already complete
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
    /// Returns a new Todo
    pub fn new(title: String, deadline: TodoDate, description: Option<String>) -> Self {
        Todo {
            deadline,
            created: Some(LocalDateTime::now()),

            completed: false,

            title,
            description,
        }
    }

    /// Marks the todo as complete
    pub fn complete(&mut self) {
        self.completed = true;
    }
}

#[derive(Debug, Clone)]
pub struct TodoColumn {
    pub todos: Vec<Todo>,
    pub title: String,
}

impl TodoColumn {
    /// Returns an empty TodoColumn
    pub fn new(title: String) -> Self {
        TodoColumn {
            todos: Vec::new(),
            title,
        }
    }

    /// Adds a todo to the column
    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    /// Searches for the todo by title. If found, returns it, and removes it from the column
    pub fn pop(&mut self, title: String) -> Option<Todo> {
        for (i, todo) in self.todos.iter().enumerate() {
            if todo.title == title {
                return Some(self.todos.remove(i));
            }
        }

        None
    }

    /// Searches for the todo by title. If found, returns a mutable reference to it
    pub fn get(&mut self, title: String) -> Option<&mut Todo> {
        for todo in self.todos.iter_mut() {
            if todo.title == title {
                return Some(todo);
            }
        }

        None
    }
}

impl IsDue for TodoColumn {
    /// Returns true if any the of the contained todos are due
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

#[derive(Debug, Clone)]
pub struct TodoTable {
    title: String,
    columns: Vec<TodoColumn>,
}

impl IsDue for TodoTable {
    /// Returns true if any the of the contained columns contain a todo which is due
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
    /// Returns an empty TodoTable
    pub fn new(title: Option<String>) -> Self {
        TodoTable {
            title: title.unwrap_or("Todos".into()),
            columns: Vec::new(),
        }
    }

    /// Adds a column to the table
    pub fn add_col(&mut self, title: String) {
        self.columns.push(TodoColumn::new(title));
    }

    /// Searches for the todo by title in a column.
    /// If found, returns a mutable reference to it
    pub fn get_todo(&mut self, title: String, col_title: String) -> Option<&mut Todo> {
        for col in self.columns.iter_mut() {
            if col.title == col_title {
                if let Some(todo) = col.get(title.clone()) {
                    return Some(todo);
                }
            }
        }

        None
    }

    /// Adds a todo to a column
    pub fn add_todo(&mut self, todo: Todo, col_title: String) {
        for col in self.columns.iter_mut() {
            if col.title == col_title {
                col.add(todo);
                break;
            }
        }
    }

    /// Moves a todo from one column to another.
    /// If either column or the todo doesn't exist, returns false
    pub fn move_todo(&mut self, title: String, from: String, to: String) -> bool {
        let mut todo = None;
        let mut to_col = None;
        let iter = self.columns.iter_mut();
        for col in iter {
            if col.title == from {
                todo = col.get(title);
                break;
            } else if col.title == to {
                to_col = Some(col);
            }
        }

        if todo.is_none() || to_col.is_none() {
            return false;
        }

        to_col.unwrap().add(todo.unwrap().clone());
        true
    }

    /// Searches for a column by name. If found, returns a mutable reference
    pub fn col(&mut self, title: String) -> Option<&mut TodoColumn> {
        for col in self.columns.iter_mut() {
            if col.title == title {
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
    /// Tests creation, addition, completion, and moving of todos and tabless
    fn create_add_complete_move() {
        let mut todos = TodoTable::new(None);
        todos.add_col("A".into());
        todos.add_col("B".into());

        todos.add_todo(Todo::new("1".into(), TodoDate::Never, None), "A".into());
        todos.add_todo(Todo::new("2".into(), TodoDate::Never, None), "B".into());

        let todo1 = todos.get_todo("1".into(), "A".into());
        assert!(todo1.is_some(), "Failed to retrieve todo 1");

        todo1.unwrap().complete();

        assert!(todos.move_todo("2".into(), "B".into(), "A".into()));

        let todo2 = todos.get_todo("2".into(), "A".into());
        assert!(todo2.is_some(), "Failed to retrieve todo 2");

        todo2.unwrap().complete();
    }

    #[test]
    /// Tests due dates
    fn is_due() {
        let mut todo = Todo::new("".into(), TodoDate::Instant(LocalDateTime::at(0)), None);
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

        let todo = Todo::new("".into(), TodoDate::Day(LocalDate::today()), None);
        assert!(
            todo.due(),
            "Should be due, isn't; deadline{}",
            todo.deadline
        );

        let todo = Todo::new("".into(), TodoDate::Daily(LocalTime::midnight()), None);
        assert!(
            todo.due(),
            "Should be due, isn't; deadline{}",
            todo.deadline
        );

        let todo = Todo::new(
            "".into(),
            TodoDate::Daily(LocalTime::hms(23, 59, 59).expect("Failed to create LocalTime")),
            None,
        );
        assert!(
            !todo.due(),
            "Shouldn't be due, is; deadline{}",
            todo.deadline
        );

        let todo = Todo::new("".into(), TodoDate::Never, None);
        assert!(
            !todo.due(),
            "Shouldn't be due, is; deadline{}",
            todo.deadline
        );
    }
}
