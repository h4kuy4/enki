use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Jwt {
    pub token: String,
}

impl From<String> for Jwt {
    fn from(token: String) -> Self {
        Self { token }
    }
}
