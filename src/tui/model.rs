use std::fmt;

use enum_iterator::Sequence;

#[derive(Debug, Clone, PartialEq, Sequence)]
pub enum Sidebar {
    Config,
    Role,
    User,
    Query,
    Webhook,
    Migration,
    Api,
    Quit,
}

impl fmt::Display for Sidebar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sidebar::Config => write!(f, "CONFIG"),
            Sidebar::Role => write!(f, "ROLE"),
            Sidebar::User => write!(f, "USER"),
            Sidebar::Query => write!(f, "QUERY"),
            Sidebar::Webhook => write!(f, "WEBHOOK"),
            Sidebar::Migration => write!(f, "MIGRATION"),
            Sidebar::Api => write!(f, "API"),
            Sidebar::Quit => write!(f, "QUIT"),
        }
    }
}
