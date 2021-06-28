use std::fmt;

#[derive(Clone, PartialEq)]
pub enum BackgroundColor {
    Background,
    SecondaryBackground,
    TertiaryBackground,
    GroupedBackground,
    SecondaryGroupedBackground,
    TertiaryGroupedBackground,
}

impl Default for BackgroundColor {
    fn default() -> Self {
        Self::Background
    }
}

impl fmt::Display for BackgroundColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Background => write!(f, "background"),
            Self::SecondaryBackground => write!(f, "secondary-background"),
            Self::TertiaryBackground => write!(f, "tertiary-background"),
            Self::GroupedBackground => write!(f, "grouped-background"),
            Self::SecondaryGroupedBackground => {
                write!(f, "secondary-grouped-background")
            }
            Self::TertiaryGroupedBackground => write!(f, "tertiary-grouped-background"),
        }
    }
}
