use rand::distributions::Uniform;

#[derive(Clone, Debug)]
pub struct PasswordGenerator {
    pub options: PasswordGeneratorOptions,
    pub password: String,
    pub recent_passwords: Vec<String>,
}

#[derive(Clone, Copy, Debug)]
pub struct PasswordGeneratorOptions {
    pub include_lowercase: bool,
    pub include_numbers: bool,
    pub include_special_characters: bool,
    pub include_uppercase: bool,
    pub length: u8,
}

#[derive(Clone, Debug)]
pub struct RandomPassword {
    pub characters: Vec<char>,
    pub password: String,
    pub range: Uniform<usize>,
}
