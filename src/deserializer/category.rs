use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Category {
    NameOnly { name: String },
    WithID { id: i32, name: String },
}
