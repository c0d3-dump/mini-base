use std::fmt;

use enum_iterator::Sequence;

#[derive(Debug, Clone, PartialEq, Sequence)]
pub enum Sidebar {
    ROLE,
    USERS,
    QUERY,
    SERVER,
    QUIT,
}

impl fmt::Display for Sidebar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sidebar::ROLE => write!(f, "ROLE"),
            Sidebar::USERS => write!(f, "USERS"),
            Sidebar::QUERY => write!(f, "QUERY"),
            Sidebar::SERVER => write!(f, "SERVER"),
            Sidebar::QUIT => write!(f, "QUIT"),
        }
    }
}
