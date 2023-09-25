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
