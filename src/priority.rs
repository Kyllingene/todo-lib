use std::fmt::Display;

use crate::error::InvalidPriorityError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum TodoPriority {
    #[default]
    None,
    Z,
    Y,
    X,
    W,
    V,
    U,
    T,
    S,
    R,
    Q,
    P,
    O,
    N,
    M,
    L,
    K,
    J,
    I,
    H,
    G,
    F,
    E,
    D,
    C,
    B,
    A,
}

impl TodoPriority {
    /// Increment the priority by one.
    pub fn inc(&mut self) -> &mut Self {
        match self {
            Self::None => {},
            Self::Z => *self = Self::Y,
            Self::Y => *self = Self::X,
            Self::X => *self = Self::W,
            Self::W => *self = Self::V,
            Self::V => *self = Self::U,
            Self::U => *self = Self::T,
            Self::T => *self = Self::S,
            Self::S => *self = Self::R,
            Self::R => *self = Self::Q,
            Self::Q => *self = Self::P,
            Self::P => *self = Self::O,
            Self::O => *self = Self::N,
            Self::N => *self = Self::M,
            Self::M => *self = Self::L,
            Self::L => *self = Self::K,
            Self::K => *self = Self::J,
            Self::J => *self = Self::I,
            Self::I => *self = Self::H,
            Self::H => *self = Self::G,
            Self::G => *self = Self::F,
            Self::F => *self = Self::E,
            Self::E => *self = Self::D,
            Self::D => *self = Self::C,
            Self::C => *self = Self::B,
            Self::B => *self = Self::A,
            Self::A => {}
        }

        self
    }

    /// Decrement the priority by one.
    pub fn dec(&mut self) -> &mut Self {
        match self {
            Self::None => {}
            Self::Z => *self = Self::None,
            Self::Y => *self = Self::Z,
            Self::X => *self = Self::Y,
            Self::W => *self = Self::X,
            Self::V => *self = Self::W,
            Self::U => *self = Self::V,
            Self::T => *self = Self::U,
            Self::S => *self = Self::T,
            Self::R => *self = Self::S,
            Self::Q => *self = Self::R,
            Self::P => *self = Self::Q,
            Self::O => *self = Self::P,
            Self::N => *self = Self::O,
            Self::M => *self = Self::N,
            Self::L => *self = Self::M,
            Self::K => *self = Self::L,
            Self::J => *self = Self::K,
            Self::I => *self = Self::J,
            Self::H => *self = Self::I,
            Self::G => *self = Self::H,
            Self::F => *self = Self::G,
            Self::E => *self = Self::F,
            Self::D => *self = Self::E,
            Self::C => *self = Self::D,
            Self::B => *self = Self::C,
            Self::A => *self = Self::B,
        }

        self
    }
}

impl Display for TodoPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => Ok(()),
            Self::A => write!(f, "(A)"),
            Self::B => write!(f, "(B)"),
            Self::C => write!(f, "(C)"),
            Self::D => write!(f, "(D)"),
            Self::E => write!(f, "(E)"),
            Self::F => write!(f, "(F)"),
            Self::G => write!(f, "(G)"),
            Self::H => write!(f, "(H)"),
            Self::I => write!(f, "(I)"),
            Self::J => write!(f, "(J)"),
            Self::K => write!(f, "(K)"),
            Self::L => write!(f, "(L)"),
            Self::M => write!(f, "(M)"),
            Self::N => write!(f, "(N)"),
            Self::O => write!(f, "(O)"),
            Self::P => write!(f, "(P)"),
            Self::Q => write!(f, "(Q)"),
            Self::R => write!(f, "(R)"),
            Self::S => write!(f, "(S)"),
            Self::T => write!(f, "(T)"),
            Self::U => write!(f, "(U)"),
            Self::V => write!(f, "(V)"),
            Self::W => write!(f, "(W)"),
            Self::X => write!(f, "(X)"),
            Self::Y => write!(f, "(Y)"),
            Self::Z => write!(f, "(Z)"),
        }
    }
}

impl TryFrom<&str> for TodoPriority {
    type Error = InvalidPriorityError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with('(') || !value.ends_with(')') {
            return Err(InvalidPriorityError::MissingParens);
        }

        match value {
            "(A)" => Ok(Self::A),
            "(B)" => Ok(Self::B),
            "(C)" => Ok(Self::C),
            "(D)" => Ok(Self::D),
            "(E)" => Ok(Self::E),
            "(F)" => Ok(Self::F),
            "(G)" => Ok(Self::G),
            "(H)" => Ok(Self::H),
            "(I)" => Ok(Self::I),
            "(J)" => Ok(Self::J),
            "(K)" => Ok(Self::K),
            "(L)" => Ok(Self::L),
            "(M)" => Ok(Self::M),
            "(N)" => Ok(Self::N),
            "(O)" => Ok(Self::O),
            "(P)" => Ok(Self::P),
            "(Q)" => Ok(Self::Q),
            "(R)" => Ok(Self::R),
            "(S)" => Ok(Self::S),
            "(T)" => Ok(Self::T),
            "(U)" => Ok(Self::U),
            "(V)" => Ok(Self::V),
            "(W)" => Ok(Self::W),
            "(X)" => Ok(Self::X),
            "(Y)" => Ok(Self::Y),
            "(Z)" => Ok(Self::Z),
            _ => Err(InvalidPriorityError::InvalidPriority),
        }
    }
}

impl TodoPriority {
    /// Convenience method for `!= TodoPriority::None`.
    pub fn is_some(&self) -> bool {
        !matches!(self, Self::None)
    }

    /// Convenience method for `== TodoPriority::None`.
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}
