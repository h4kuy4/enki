use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> Response<T>
where
    T: Serialize,
{
    pub fn new<I>(code: i32, msg: &str, data: I) -> Self
    where
        I: Into<Option<T>>,
    {
        Self {
            code,
            msg: String::from(msg),
            data: data.into(),
        }
    }
    pub fn ok<I>(data: I) -> Self
    where
        I: Into<Option<T>>,
    {
        Self::new(200, "OK", data)
    }
    pub fn err(code: i32, msg: &str) -> Self {
        Self::new(code, msg, None)
    }
}
