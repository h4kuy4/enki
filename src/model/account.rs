#[derive(Debug)]
pub struct Account {
    pub user_name: String,
    pub password: String,
}

impl Account {
    pub fn new(user_name: &str, password: &str) -> Self {
        Self {
            user_name: String::from(user_name),
            password: String::from(password),
        }
    }

    pub fn verify(&self, user_name: &str, password: &str) -> bool {
        self.user_name == user_name && self.password == password
    }
}
