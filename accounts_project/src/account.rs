use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
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


#[derive(Debug, Clone)]
pub struct Userbase {
    users: HashMap<String, User>,
}

impl Userbase {
    pub fn new() -> Self{
        Self{
            users: HashMap::new(),
        }
    }


    pub fn add_user(&mut self, user: User) {
        
        self.users.insert(user.name.clone(), user);
    }


    pub fn get_user(&self, name: &str) -> Option<&User> {
        self.users.get(name)
    }
}