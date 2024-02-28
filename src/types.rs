use rand::distributions::Uniform;

#[derive(Copy, Clone)]
pub struct PasswordGeneratorOptions {
    pub include_lowercase: bool,
    pub include_numbers: bool,
    pub include_special_characters: bool,
    pub include_uppercase: bool,
    pub length: u8,
}

pub struct RandomPassword {
    pub characters: Vec<char>,
    pub password: String,
    pub range: Uniform<usize>,
}