use std::fmt::Display;

use chrono::{Datelike, Local, NaiveDateTime};

use crate::helper::IsDue;

/// A due date for a Todo. Encapsulates data structures from crate `datetime`.
///
/// Example:
/// ```
/// use todo_lib::prelude::*;
///
/// let today_deadline = TodoDate::Day(Local::now().naive_local());
/// assert!(today_deadline.due());
///
/// let indefinite_deadline = TodoDate::Never;
/// assert!(!indefinite_deadline.due());
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum TodoDate {
    /// Never due.
    #[default]
    Never,
    /// Always due.
    Always,
    /// Due on a specific day.
    Day(NaiveDateTime),
}

impl IsDue for TodoDate {
    /// Returns true if it is currently on or past the due date.
    fn due(&self) -> bool {
        match self {
            Self::Never => false,
            Self::Always => true,
            Self::Day(t) => *t <= Local::now().naive_local(),
        }
    }
}

impl Display for TodoDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Never => write!(f, ""),
            Self::Always => write!(f, "due:0000-00-00"),
            Self::Day(t) => write!(f, "due:{}-{:02}-{:02}", t.year(), t.month(), t.day(),),
        }
    }
}

impl TodoDate {
    /// Convenience method for `!= TodoDate::Never`.
    #[inline]
    pub fn is_some(&self) -> bool {
        !matches!(self, Self::Never)
    }

    /// Convenience method for `== TodoDate::Never`.
    #[inline]
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}
