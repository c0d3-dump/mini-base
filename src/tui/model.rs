use std::fmt;

use enum_iterator::Sequence;

#[derive(Debug, Clone, PartialEq, Sequence)]
pub enum Sidebar {
    Config,
    Role,
    Users,
    Query,
    Apis,
    Migration,
    Quit,
}

impl fmt::Display for Sidebar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sidebar::Config => write!(f, "CONFIG"),
            Sidebar::Role => write!(f, "ROLE"),
            Sidebar::Users => write!(f, "USERS"),
            Sidebar::Query => write!(f, "QUERY"),
            Sidebar::Apis => write!(f, "APIS"),
            Sidebar::Migration => write!(f, "MIGRATION"),
            Sidebar::Quit => write!(f, "QUIT"),
        }
    }
}
