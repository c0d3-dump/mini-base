use std::fmt;

use enum_iterator::Sequence;

#[derive(Debug, Clone, PartialEq, Sequence)]
pub enum Sidebar {
    Role,
    Users,
    Query,
    Server,
    Quit,
}

impl fmt::Display for Sidebar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sidebar::Role => write!(f, "ROLE"),
            Sidebar::Users => write!(f, "USERS"),
            Sidebar::Query => write!(f, "QUERY"),
            Sidebar::Server => write!(f, "SERVER"),
            Sidebar::Quit => write!(f, "QUIT"),
        }
    }
}
