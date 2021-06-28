use std::fmt;

#[derive(Clone, PartialEq)]
pub enum ListDividerElement {
    Hr,
    Li,
}

impl Default for ListDividerElement {
    fn default() -> Self {
        Self::Hr
    }
}

impl fmt::Display for ListDividerElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Hr => write!(f, "hr"),
            Self::Li => write!(f, "li"),
        }
    }
}
