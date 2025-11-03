use std::fmt;

pub struct User {
    pub name: String,
    pub password: String,
    pub locked: bool,
}

impl User {
    pub fn new(name: &str, password: &str) -> Self{
        Self{
            name: name.to_string(),
            password: password.to_string(),
            locked: false,
        }
    }
}
