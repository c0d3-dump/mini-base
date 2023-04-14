#[derive(Debug, Clone)]
pub enum ColType {
    Null,
    Integer(i64),
    String(String),
}

// pub trait BaseDatabase {
//     fn new() -> Self;
// }
