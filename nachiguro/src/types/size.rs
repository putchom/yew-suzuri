use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Size {
    S,
    M,
    L,
}

impl Default for Size {
    fn default() -> Self {
        Self::M
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::S => write!(f, "s"),
            Self::M => write!(f, "m"),
            Self::L => write!(f, "l"),
        }
    }
}
