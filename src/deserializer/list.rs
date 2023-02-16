use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct List {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
