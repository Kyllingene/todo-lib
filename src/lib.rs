//! Relatively simple todo management. Supports parsing to/from [todo.txt](http://todotxt.org).
//!
//! Example:
//! ```rust
//! use todo_lib::prelude::*;
//!
//! let mut todos = TodoTable::new(Some("Todos"));
//! todos.add_col("Work");
//! todos.add_col("Home");
//!
//! todos.add_todo(Todo::new("Review documents", TodoDate::Never, TodoPriority::None), "Work");
//! todos.add_todo(Todo::new("Clean desk", TodoDate::Never, TodoPriority::None), "Home");
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
//!
//! Parsing from todo.txt format:
//! ```rust
//! use todo_lib::prelude::*;
//!
//! let todo_text = "2023-01-07 Create a +todo @library due:2053-01-01";
//! let mut todo = Todo::from_str(todo_text).unwrap();
//!
//! assert_eq!(todo.to_string(), todo_text.to_string());
//! assert!(!todo.due());
//!
//! todo.completed = true;
//! assert!(todo.has_project_tag("todo"));
//! assert!(todo.has_context_tag("library"));
//! ```

use std::str::FromStr;
use std::{collections::HashSet, fmt::Display};

pub use chrono;
use chrono::prelude::*;

pub mod colors;
pub mod due;
pub mod error;
pub mod helper;
pub mod prelude;
pub mod priority;
pub mod table;

#[cfg(test)]
mod test;

use colors::StyleScheme;
pub use due::TodoDate;
use error::*;
use helper::*;
pub use priority::TodoPriority;
pub use table::{TodoColumn, TodoTable};

/// A todo tag.
///
/// NOTE: ONLY use `TodoTag::project` and `TodoTag::context` to create a tag.
/// This ensures that the tags are valid.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TodoTag {
    Project(String),
    Context(String),
}

impl Display for TodoTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Project(t) => write!(f, "+{t}"),
            Self::Context(t) => write!(f, "@{t}"),
        }
    }
}

impl TodoTag {
    pub fn project<S: ToString>(tag: S) -> Option<Self> {
        if tag.to_string().chars().any(|ch| ch.is_whitespace()) {
            None
        } else {
            Some(Self::Project(tag.to_string()))
        }
    }

    pub fn context<S: ToString>(tag: S) -> Option<Self> {
        if tag.to_string().chars().any(|ch| ch.is_whitespace()) {
            None
        } else {
            Some(Self::Context(tag.to_string()))
        }
    }
}

#[derive(Clone, Debug)]
pub enum TodoSegment {
    String(String),
    Tag(TodoTag),
}

impl TodoSegment {
    pub fn to_string(&self, style: StyleScheme, reset: &str) -> String {
        match self {
            TodoSegment::String(s) => format!("{}{s}{reset}", style.description),
            TodoSegment::Tag(tag) => format!(
                "{}{tag}{reset}",
                match tag {
                    TodoTag::Context(_) => style.context,
                    TodoTag::Project(_) => style.project,
                }
            ),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TodoDescription(Vec<TodoSegment>);

impl TodoDescription {
    pub fn to_string(&self, style: StyleScheme, reset: &str) -> String {
        let mut s = String::new();

        if self.0.is_empty() {
            return s;
        }

        let end = self.0.len() - 1;
        for (i, seg) in self.0.iter().enumerate() {
            s.push_str(&seg.to_string(style, reset));

            if i != end {
                s.push(' ');
            }
        }

        s.push_str(reset);

        s
    }
}

/// A todo.
///
/// When creating with `Todo::new`, sets `created` to the current time.
/// When creating with `Todo::default`, sets `created` to None.
///
/// Example:
/// ```
/// use todo_lib::prelude::*;
///
/// let mut todo = Todo::new(
///     "Check the mail",
///     TodoDate::Always,
///     TodoPriority::None,
/// );
///
/// assert!(!todo.completed && todo.due());
///
/// todo.complete();
///
/// assert!(todo.completed && !todo.due());
/// ```

#[derive(Debug, Clone, Default)]
pub struct Todo {
    pub description: TodoDescription,

    pub completed: bool,
    pub priority: TodoPriority,
    pub metadata: Map<String, String>,

    pub deadline: TodoDate,
    pub creation: Option<NaiveDateTime>,
    pub completion_date: Option<NaiveDateTime>,
}

impl Todo {
    /// Returns a new todo.
    pub fn new<S: AsRef<str>>(title: S, deadline: TodoDate, priority: TodoPriority) -> Self {
        let mut description = Vec::new();
        for part in title.as_ref().split(' ') {
            if let Some(context) = part.strip_prefix('@') {
                description.push(TodoSegment::Tag(TodoTag::Context(context.to_string())));
            } else if let Some(project) = part.strip_prefix('+') {
                description.push(TodoSegment::Tag(TodoTag::Project(project.to_string())));
            } else {
                description.push(TodoSegment::String(part.to_string()));
            }
        }

        Todo {
            deadline,
            creation: Some(Local::now().naive_local()),

            completed: false,
            priority,
            metadata: Map::new(),

            description: TodoDescription(description),
            completion_date: None,
        }
    }

    /// Sets the creation date to current day.
    /// Helpful for parsing from a string.
    pub fn set_creation(&mut self) {
        self.creation = Some(Local::now().naive_local());
    }

    /// Marks the todo as complete.
    ///
    /// Sets completion date to current day.
    pub fn complete(&mut self) {
        self.completed = true;
        self.completion_date = Some(Local::now().naive_local());
    }

    /// Checks if the todo has a certain project tag.
    pub fn has_project_tag<S: AsRef<str>>(&self, tag: S) -> bool {
        for part in &self.description.0 {
            if let TodoSegment::Tag(TodoTag::Project(t)) = part {
                if t == tag.as_ref() {
                    return true;
                }
            }
        }

        false
    }

    /// Checks if the todo has a certain context tag.
    pub fn has_context_tag<S: AsRef<str>>(&self, tag: S) -> bool {
        for part in &self.description.0 {
            if let TodoSegment::Tag(TodoTag::Context(t)) = part {
                if t == tag.as_ref() {
                    return true;
                }
            }
        }

        false
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

        for part in &self.description.0 {
            if let TodoSegment::Tag(tag) = part {
                set.insert(tag.clone());
            }
        }

        set
    }

    /// Returns all the metadata of the todo.
    pub fn metadata(&self) -> &Map<String, String> {
        &self.metadata
    }

    /// Sets a metadata tag to the todo.
    pub fn set_meta<S: ToString>(&mut self, key: S, val: S) {
        self.metadata.insert(key.to_string(), val.to_string());
    }

    /// Returns an option containing the value corresponding to the key.
    ///
    /// Returns None if the key doesn't exist.
    pub fn get_meta<S: ToString>(&self, key: S) -> Option<&String> {
        self.metadata.get(&key.to_string())
    }

    /// Removes the given metadata from the todo, if it exists.
    pub fn delete_meta(&mut self, key: &String) {
        self.metadata.remove(key);
    }

    /// Colorizes the todo as a string.
    pub fn colored(&self, style: StyleScheme) -> String {
        let (reset, style) = style.get_colors(self.completed);

        let tick = if self.completed {
            format!("{}x ", style.faded)
        } else {
            String::new()
        };

        let priority = if self.priority.is_some() {
            format!("{}{}{}", style.priority, self.priority, reset)
        } else {
            "".into()
        };

        let completion = if self.creation.is_some() {
            if let Some(date) = self.completion_date {
                format!("{}{:04}-{:02}-{:02}{reset} ", style.completion, date.year(), date.month(), date.day())
            } else {
                "".into()
            }
        } else {
            "".into()
        };
        
        let creation = if let Some(date) = self.creation {
            format!("{}{:04}-{:02}-{:02}{reset} ", style.creation, date.year(), date.month(), date.day())
        } else {
            "".into()
        };

        let metadata = format!(
            "{}{}{reset}",
            style.metadata,
            self.metadata,
        );

        let mut deadline = format!("{}{}{reset}", style.deadline, self.deadline);
        if !(self.metadata.is_empty() || self.deadline.is_none()) {
            deadline += " ";
        }

        let mut description = self.description.to_string(style, reset);
        if !((self.metadata.is_empty() && self.deadline.is_none()) || self.description.0.is_empty()) {
            description += " ";
        }

        format!(
            "{tick}{priority}{completion}{creation}{description}{deadline}{metadata}"
        )
    }
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
            self.priority.to_string() + " "
        } else {
            "".into()
        };

        let completion = if self.creation.is_some() {
            if let Some(date) = self.completion_date {
                format!("{:04}-{:02}-{:02} ", date.year(), date.month(), date.day(),)
            } else {
                "".into()
            }
        } else {
            "".into()
        };
        
        let creation = if let Some(date) = self.creation {
            format!("{:04}-{:02}-{:02} ", date.year(), date.month(), date.day(),)
        } else {
            "".into()
        };

        let metadata = self.metadata.to_string();

        let mut deadline = self.deadline.to_string();
        if !(metadata.is_empty() || deadline.is_empty()) {
            deadline += " ";
        }

        let mut description = self.description.to_string(StyleScheme::default(), "");
        if !((metadata.is_empty() && deadline.is_empty()) || description.is_empty()) {
            description += " ";
        }

        write!(
            f,
            "{tick}{priority}{completion}{creation}{description}{deadline}{metadata}"
        )
    }
}

impl FromStr for Todo {
    type Err = TodoParseError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut todo = Todo::default();

        if let Some(remainder) = s.strip_prefix("x ") {
            s = remainder;
            todo.completed = true;
        }

        let mut parts = s.split(' ').peekable();

        let mut description = Vec::new();

        if let Ok(p) = TodoPriority::try_from(*parts.peek().ok_or(TodoParseError::BadPriority)?) {
            todo.priority = p;
            parts.next();
        }

        if let Some(part) = parts.peek() {
            if let Ok(date) = NaiveDate::parse_from_str(part, "%F") {
                todo.creation =
                    Some(date.and_hms_opt(0, 0, 0).ok_or(TodoParseError::BadDate)?);
                parts.next();
            }
        }

        if let Some(part) = parts.peek() {
            if let Ok(date) = NaiveDate::parse_from_str(part, "%F") {
                todo.completion_date = std::mem::replace(
                    &mut todo.creation,
                    Some(date.and_hms_opt(0, 0, 0).ok_or(TodoParseError::BadDate)?),
                );
                parts.next();
            }
        }

        for part in parts {
            if part.matches(':').count() == 1 {
                let meta: Vec<&str> = part.split(':').collect();
                todo.metadata
                    .insert(meta[0].to_string(), meta[1].to_string());
            } else if !part.is_empty() && !part.chars().all(|ch| ch.is_whitespace()) {
                if let Some(context) = part.strip_prefix('@') {
                    description.push(TodoSegment::Tag(TodoTag::Context(context.to_string())));
                } else if let Some(project) = part.strip_prefix('+') {
                    description.push(TodoSegment::Tag(TodoTag::Project(project.to_string())));
                } else {
                    description.push(TodoSegment::String(part.to_string()));
                }
            }
        }

        if let Some(date) = todo.metadata.get(&"due".to_string()) {
            if date == "today" {
                if let Some(created) = todo.creation.clone() {
                    todo.deadline = TodoDate::Day(created);
                } else {
                    todo.deadline = TodoDate::Day(Local::now().naive_local());
                }

                todo.metadata.remove(&"due".to_string());
            } else if let Some(offset) = date.strip_suffix('d') {
                let today = if let Some(created) = todo.creation.clone() {
                    created
                } else {
                    Local::now().naive_local()
                };

                let offset = offset.parse::<u32>().map_err(|_| TodoParseError::BadDate)?;

                let day = today.ordinal0() + offset;
                let year = today.year() + day as i32 / 366;
                todo.deadline = TodoDate::Day(
                    today
                        .with_year(year)
                        .ok_or(TodoParseError::BadDate)?
                        .with_day(day % 366)
                        .ok_or(TodoParseError::BadDate)?,
                );

                todo.metadata.remove(&"due".to_string());
            } else if let Ok(date) = NaiveDate::parse_from_str(date, "%F") {
                todo.deadline =
                    TodoDate::Day(date.and_hms_opt(0, 0, 0).ok_or(TodoParseError::BadDate)?);
                todo.metadata.remove(&"due".to_string());
            }
        }

        todo.description = TodoDescription(description);

        Ok(todo)
    }
}
