use crate::random_password::{RandomPassword, RandomPasswordOptions};

#[derive(Clone, Debug)]
pub struct PasswordGenerator {
    pub options: RandomPasswordOptions,
    pub password: String,
    pub recent_passwords: Vec<String>,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self {
            options: RandomPasswordOptions::default(),
            password: RandomPassword::default().password,
            recent_passwords: Vec::with_capacity(10),
        }
    }
}
