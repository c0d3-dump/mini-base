#[derive(Debug, Clone)]
pub enum ColType {
    Null,
    Integer(i64),
    String(String),
}

#[derive(Debug, Clone)]
pub struct ColInfo {
    pub cid: i64,
    pub name: String,
    pub ctype: String,
    pub notnull: bool,
    pub dflt_value: String,
    pub pk: bool,
}
