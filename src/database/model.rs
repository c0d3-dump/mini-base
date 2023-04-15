#[derive(Debug, Clone)]
pub enum ColType {
    Integer(Option<i64>),
    String(Option<String>),
}

#[derive(Debug, Clone)]
pub struct ColInfo {
    pub cid: i64,
    pub name: String,
    pub ctype: String,
    pub notnull: bool,
    pub dflt_value: Option<String>,
    pub pk: bool,
}

#[derive(Clone, Debug, Default)]
pub enum ExecType {
    #[default]
    QUERY,
    EXECUTION,
}
