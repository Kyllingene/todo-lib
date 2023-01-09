//! Relatively simple todo management.
//!
//! Example:
//! ```
//! use todo_lib::{Todo, TodoDate, TodoTable, TodoPriority};
//!
//! let mut todos = TodoTable::new(Some("Todos"));
//! todos.add_col("Work");
//! todos.add_col("Home");
//!
//! todos.add_todo(Todo::new("Review documents", TodoDate::Never, None, TodoPriority::None), "Work");
//! todos.add_todo(Todo::new("Clean desk", TodoDate::Never, None, TodoPriority::None), "Home");
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

use std::error::Error;
use std::{collections::HashSet, fmt::Display};

use datetime::{convenience::Today, DatePiece, LocalDate, LocalDateTime, LocalTime, TimePiece};

pub trait IsDue {
    fn due(&self) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum TodoPriority {
    #[default]
    None,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Display for TodoPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => Ok(()),
            Self::A => write!(f, "(A) "),
            Self::B => write!(f, "(B) "),
            Self::C => write!(f, "(C) "),
            Self::D => write!(f, "(D) "),
            Self::E => write!(f, "(E) "),
            Self::F => write!(f, "(F) "),
            Self::G => write!(f, "(G) "),
            Self::H => write!(f, "(H) "),
            Self::I => write!(f, "(I) "),
            Self::J => write!(f, "(J) "),
            Self::K => write!(f, "(K) "),
            Self::L => write!(f, "(L) "),
            Self::M => write!(f, "(M) "),
            Self::N => write!(f, "(N) "),
            Self::O => write!(f, "(O) "),
            Self::P => write!(f, "(P) "),
            Self::Q => write!(f, "(Q) "),
            Self::R => write!(f, "(R) "),
            Self::S => write!(f, "(S) "),
            Self::T => write!(f, "(T) "),
            Self::U => write!(f, "(U) "),
            Self::V => write!(f, "(V) "),
            Self::W => write!(f, "(W) "),
            Self::X => write!(f, "(X) "),
            Self::Y => write!(f, "(Y) "),
            Self::Z => write!(f, "(Z) "),
        }
    }
}

impl TodoPriority {
    /// Convenience method for `!= TodoPriority::None`.
    pub fn is_some(&self) -> bool {
        match self {
            Self::None => false,
            _ => true,
        }
    }

    /// Convenience method for `== TodoPriority::None`.
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
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
            Self::Never => write!(f, ""),
            Self::Always => write!(f, "due:always"),
            Self::Daily(t) => write!(f, "due:daily-{}:{}:{}", t.hour(), t.minute(), t.second()),
            Self::Day(t) => write!(
                f,
                "due:{}-{}-{}",
                t.month().months_from_january() + 1,
                t.day(),
                t.year()
            ),
            Self::Instant(t) => write!(
                f,
                "due: {}-{}-{}_{}:{}:{}",
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

#[derive(Clone, Debug)]
pub struct TagContainsWhitespaceError();

impl Display for TagContainsWhitespaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tag contains whitespace")
    }
}

impl Error for TagContainsWhitespaceError {}

/// A todo tag.
///
/// NOTE: ONLY use `TodoTag::project` and `TodoTag::context` to create a tag.
/// This ensures that the tags contain no whitespace.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TodoTag {
    Project(String),
    Context(String),
}

impl Display for TodoTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Project(t) => write!(f, "+{}", t),
            Self::Context(t) => write!(f, "@{}", t),
        }
    }
}

impl TodoTag {
    pub fn project<S: ToString>(tag: S) -> Result<Self, TagContainsWhitespaceError> {
        if tag.to_string().chars().any(|ch| ch.is_whitespace()) {
            Err(TagContainsWhitespaceError {})
        } else {
            Ok(Self::Project(tag.to_string()))
        }
    }

    pub fn context<S: ToString>(tag: S) -> Result<Self, TagContainsWhitespaceError> {
        if tag.to_string().chars().any(|ch| ch.is_whitespace()) {
            Err(TagContainsWhitespaceError {})
        } else {
            Ok(Self::Context(tag.to_string()))
        }
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
 * use todo_lib::{Todo, TodoDate, IsDue, TodoPriority};
 *
 * let mut todo = Todo::new(
 *     "Check the mail",
 *     TodoDate::Always,
 *     None,
 *     TodoPriority::None,
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
    pub priority: TodoPriority,

    pub deadline: TodoDate,
    pub created: Option<LocalDate>,
    pub completion_date: Option<LocalDate>,
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
        let tick = if self.completed { "x " } else { "" };
        let priority = if self.priority.is_some() {
            self.priority.to_string()
        } else {
            "".into()
        };
        let completion = if self.completion_date.is_some() && self.created.is_some() {
            let date = self.completion_date.unwrap();
            format!(
                "{}-{}-{} ",
                date.year(),
                date.month().months_from_january() + 1,
                date.day(),
            )
        } else {
            "".into()
        };
        let creation = if self.created.is_some() {
            let date = self.created.unwrap();
            format!(
                "{}-{}-{} ",
                date.year(),
                date.month().months_from_january() + 1,
                date.day(),
            )
        } else {
            "".into()
        };
        let description = if self.description.is_some() {
            format!("{} | {} ", self.title, self.description.clone().unwrap())
        } else {
            format!("{} ", self.title)
        };
        let deadline = self.deadline.clone();

        write!(
            f,
            "{tick}{priority}{completion}{creation}{description}{deadline}",
        )
    }
}

impl Todo {
    /// Returns a new Todo.
    pub fn new<S: ToString>(
        title: S,
        deadline: TodoDate,
        description: Option<S>,
        priority: TodoPriority,
    ) -> Self {
        Todo {
            deadline,
            created: Some(LocalDate::today()),

            completed: false,
            priority,

            title: title.to_string(),
            description: description.map(|s| s.to_string()),
            completion_date: None,
        }
    }

    /// Marks the todo as complete.
    ///
    /// Sets completion date to current day.
    pub fn complete(&mut self) {
        self.completed = true;
        self.completion_date = Some(LocalDate::today());
    }

    /// Checks if the todo has a certain tag, started with `start`
    fn has_tag_raw(&self, start: char, t: String) -> bool {
        let mut tag = String::new();
        let mut in_tag = false;
        let mut last_char = ' ';

        let data = if self.description.is_some() {
            format!("{} {}", self.title, self.description.clone().unwrap())
        } else {
            self.title.clone()
        };

        for ch in data.chars() {
            if ch == start && last_char == ' ' {
                in_tag = true;
            } else if in_tag && !ch.is_whitespace() {
                tag.push(ch);
            } else if in_tag && ch.is_whitespace() {
                if tag == t {
                    return true;
                }

                in_tag = false;
                tag.clear();
            }

            last_char = ch;
        }

        false
    }

    /// Checks if the todo has a certain project tag.
    pub fn has_project_tag(&self, tag: String) -> bool {
        self.has_tag_raw('+', tag)
    }

    /// Checks if the todo has a certain context tag.
    pub fn has_context_tag(&self, tag: String) -> bool {
        self.has_tag_raw('@', tag)
    }

    /// Checks if the todo has a certain tag.
    pub fn has_tag(&self, tag: TodoTag) -> bool {
        match tag {
            TodoTag::Project(t) => self.has_project_tag(t),
            TodoTag::Context(t) => self.has_context_tag(t),
        }
    }

    /// Returns all the tags in the todo.
    pub fn tags(&self) -> HashSet<TodoTag> {
        let mut set = HashSet::new();

        let mut tag = String::new();
        let mut in_project_tag = false;
        let mut in_context_tag = false;
        let mut last_char = ' ';

        let data = if self.description.is_some() {
            format!("{} {}", self.title, self.description.clone().unwrap())
        } else {
            self.title.clone()
        };

        for ch in data.chars() {
            if ch == '+' && last_char == ' ' {
                in_project_tag = true;
                in_context_tag = false;
            } else if ch == '@' && last_char == ' ' {
                in_project_tag = false;
                in_context_tag = true;
            } else if (in_project_tag || in_context_tag) && !ch.is_whitespace() {
                tag.push(ch);
            } else if (in_project_tag || in_context_tag) && ch.is_whitespace() {
                if in_project_tag {
                    set.insert(TodoTag::project(tag.clone()).unwrap());
                } else {
                    set.insert(TodoTag::context(tag.clone()).unwrap());
                }

                in_project_tag = false;
                in_context_tag = false;
                tag.clear();
            }

            last_char = ch;
        }

        set
    }
}

/**
 * A list of todos, under a title.
 *
 * Example:
 * ```
 * use todo_lib::{Todo, TodoColumn, TodoDate, TodoPriority};
 *
 * let mut todos = TodoColumn::new("Todo");
 * todos.add(Todo::new("Buy mangos", TodoDate::Never, None, TodoPriority::None));
 * todos.add(Todo::new("Sort stamps", TodoDate::Never, None, TodoPriority::None));
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
 * use todo_lib::{Todo, TodoDate, TodoTable, TodoPriority};
 *
 * let mut todos = TodoTable::new(Some("Todos"));
 * todos.add_col("Work");
 * todos.add_col("Home");
 *
 * todos.add_todo(Todo::new("Review documents", TodoDate::Never, None, TodoPriority::None), "Work");
 * todos.add_todo(Todo::new("Clean desk", TodoDate::Never, None, TodoPriority::None), "Home");
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

        todos.add_todo(
            Todo::new("1", TodoDate::Never, None, TodoPriority::None),
            "A",
        );
        todos.add_todo(
            Todo::new("2", TodoDate::Never, None, TodoPriority::None),
            "B",
        );

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
        let mut todo = Todo::new(
            "",
            TodoDate::Instant(LocalDateTime::at(0)),
            None,
            TodoPriority::None,
        );
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

        let todo = Todo::new(
            "",
            TodoDate::Day(LocalDate::today()),
            None,
            TodoPriority::None,
        );
        assert!(
            todo.due(),
            "Should be due, isn't; deadline{}",
            todo.deadline
        );

        let todo = Todo::new(
            "",
            TodoDate::Daily(LocalTime::midnight()),
            None,
            TodoPriority::None,
        );
        assert!(
            todo.due(),
            "Should be due, isn't; deadline{}",
            todo.deadline
        );

        let todo = Todo::new(
            "",
            TodoDate::Daily(LocalTime::hms(23, 59, 59).expect("Failed to create LocalTime")),
            None,
            TodoPriority::None,
        );
        assert!(
            !todo.due(),
            "Shouldn't be due, is; deadline{}",
            todo.deadline
        );

        let todo = Todo::new("", TodoDate::Always, None, TodoPriority::None);
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

    #[test]
    fn todo_display() {
        let mut todo = Todo::new(
            "Todo #1",
            TodoDate::Always,
            Some("This is a todo"),
            TodoPriority::B,
        );

        let today = LocalDate::today();
        assert_eq!(
            todo.to_string(),
            format!(
                "(B) {}-{}-{} Todo #1 | This is a todo due:always",
                today.year(),
                today.month().months_from_january() + 1,
                today.day()
            )
        );

        todo.complete();
        assert_eq!(
            todo.to_string(),
            format!(
                "x (B) {0}-{1}-{2} {0}-{1}-{2} Todo #1 | This is a todo due:always",
                today.year(),
                today.month().months_from_january() + 1,
                today.day()
            )
        );
    }

    #[test]
    fn tag_parsing() {
        assert!(
            TodoTag::project("a ").is_err(),
            "TodoTag::project accepted space"
        );
        assert!(
            TodoTag::project("b\n").is_err(),
            "TodoTag::project accepted newline"
        );
        assert!(
            TodoTag::project("c\t").is_err(),
            "TodoTag::project accepted tab"
        );

        assert!(
            TodoTag::context("a ").is_err(),
            "TodoTag::context accepted space"
        );
        assert!(
            TodoTag::context("b\n").is_err(),
            "TodoTag::context accepted newline"
        );
        assert!(
            TodoTag::context("c\t").is_err(),
            "TodoTag::context accepted tab"
        );

        let a = Todo::new(
            "+1 @2 3+4",
            TodoDate::Never,
            Some("@a +b-c @def@ghi jk@lm"),
            TodoPriority::None,
        );
        let tags = a.tags();

        assert!(
            tags.contains(&TodoTag::project("1").unwrap()),
            "Couldn't find +1"
        );
        assert!(
            tags.contains(&TodoTag::context("2").unwrap()),
            "Couldn't find @2"
        );
        assert!(
            !tags.contains(&TodoTag::project("4").unwrap()),
            "Shouldn't find +4"
        );
        assert!(
            tags.contains(&TodoTag::context("a").unwrap()),
            "Couldn't find @a"
        );
        assert!(
            tags.contains(&TodoTag::project("b-c").unwrap()),
            "Couldn't find +b-c"
        );
        assert!(
            tags.contains(&TodoTag::context("def@ghi").unwrap()),
            "Couldn't find @def@ghi"
        );
        assert!(
            !tags.contains(&TodoTag::context("lm").unwrap()),
            "Shouldn't find @lm"
        );
    }
}
