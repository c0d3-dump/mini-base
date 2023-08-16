use std::fmt;

use enum_iterator::Sequence;

#[derive(Debug, Clone, PartialEq, Sequence)]
pub enum Sidebar {
    ROLE,
    QUERY,
    AUTH,
    EDITOR,
    SERVER,
    QUIT,
}

impl fmt::Display for Sidebar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sidebar::ROLE => write!(f, "ROLE"),
            Sidebar::QUERY => write!(f, "QUERY"),
            Sidebar::AUTH => write!(f, "AUTH"),
            Sidebar::EDITOR => write!(f, "EDITOR"),
            Sidebar::SERVER => write!(f, "SERVER"),
            Sidebar::QUIT => write!(f, "QUIT"),
        }
    }
}
