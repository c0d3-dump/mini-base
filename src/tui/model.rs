use std::fmt;

use enum_iterator::Sequence;

#[derive(Debug, Clone, PartialEq, Sequence)]
pub enum Sidebar {
    AUTH,
    ROLE,
    QUERY,
    EDITOR,
    SERVER,
    QUIT,
}

impl fmt::Display for Sidebar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sidebar::AUTH => write!(f, "AUTH"),
            Sidebar::ROLE => write!(f, "ROLE"),
            Sidebar::QUERY => write!(f, "QUERY"),
            Sidebar::EDITOR => write!(f, "EDITOR"),
            Sidebar::SERVER => write!(f, "SERVER"),
            Sidebar::QUIT => write!(f, "QUIT"),
        }
    }
}
