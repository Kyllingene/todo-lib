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
    fn due(&self) -> bool {
        match self {
            Self::Never => false,
            Self::Daily(t) => {
                let now = LocalDateTime::now();
                let lt_now = LocalTime::hms_ms(now.hour(), now.minute(), 0, 0).unwrap();

                *t >= lt_now
            }
            Self::Day(t) => *t >= LocalDate::today(),
            Self::Instant(t) => *t >= LocalDateTime::now(),
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
    pub fn is_some(&self) -> bool {
        match self {
            Self::Never => false,
            _ => true,
        }
    }

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
    pub fn new(title: String, deadline: TodoDate, description: Option<String>) -> Self {
        Todo {
            deadline,
            created: Some(LocalDateTime::now()),

            completed: false,

            title,
            description,
        }
    }

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
    pub fn new(title: String) -> Self {
        TodoColumn {
            todos: Vec::new(),
            title,
        }
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn pop(&mut self, title: String) -> Option<Todo> {
        for (i, todo) in self.todos.iter().enumerate() {
            if todo.title == title {
                return Some(self.todos.remove(i));
            }
        }

        None
    }

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
    pub fn new(title: Option<String>) -> Self {
        TodoTable {
            title: title.unwrap_or("Todos".into()),
            columns: Vec::new(),
        }
    }

    pub fn add_col(&mut self, title: String) {
        self.columns.push(TodoColumn::new(title));
    }

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

    pub fn add_todo(&mut self, todo: Todo, col_title: String) {
        for col in self.columns.iter_mut() {
            if col.title == col_title {
                col.add(todo);
                break;
            }
        }
    }

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

    pub fn col(&self, title: String) -> Option<&TodoColumn> {
        for col in self.columns.iter() {
            if col.title == title {
                return Some(col);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use datetime::Instant;

    use crate::*;

    #[test]
    fn create_add_check_move() {
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
    fn is_due() {
        let mut todo = Todo::new("".into(), TodoDate::Instant(LocalDateTime::from_instant(Instant::at_epoch())), None);
        assert!(todo.due(), "Should be due, isn't; due: {}", todo.deadline);

        todo.complete();
        assert!(!todo.due(), "Shouldn't be due, is; due: {}", todo.deadline);
    }
}
