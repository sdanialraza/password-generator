use rand::{distributions::Uniform, thread_rng, Rng};

use crate::constants::SPECIAL_CHARACTERS;

#[derive(Copy, Clone)]
pub struct RandomPasswordOptions {
    pub length: u8,
    pub include_lowercase: bool,
    pub include_uppercase: bool,
    pub include_numbers: bool,
    pub include_special_characters: bool,
}

impl Default for RandomPasswordOptions {
    fn default() -> Self {
        Self {
            length: 10,
            include_lowercase: true,
            include_uppercase: true,
            include_numbers: true,
            include_special_characters: true,
        }
    }
}

pub struct RandomPassword {
    pub characters: Vec<char>,
    pub password: String,
    pub range: Uniform<usize>,
}

impl RandomPassword {
    pub fn new(options: RandomPasswordOptions) -> Self {
        let mut password = String::new();
        let mut characters: Vec<char> = Vec::with_capacity(86);

        if options.include_lowercase {
            for i in b'a'..b'z' + 1 {
                characters.push(i as char);
            }
        }

        if options.include_uppercase {
            for i in b'A'..b'Z' + 1 {
                characters.push(i as char);
            }
        }

        if options.include_numbers {
            for i in b'0'..b'9' + 1 {
                characters.push(i as char)
            }
        }

        if options.include_special_characters {
            characters.append(&mut SPECIAL_CHARACTERS.to_vec())
        }

        let characters_len = characters.len();

        for _ in 1..=options.length {
            password.push(characters[thread_rng().gen_range(1..characters_len)])
        }

        RandomPassword {
            characters,
            password,
            range: Uniform::new(0, characters_len),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{RandomPassword, RandomPasswordOptions};

    #[test]
    fn it_generates_lowercase_only_password() {
        let options = RandomPasswordOptions {
            include_lowercase: true,
            include_uppercase: false,
            include_numbers: false,
            include_special_characters: false,
            length: 10,
        };

        let random_password = RandomPassword::new(options);

        for character in random_password.password.chars() {
            assert!(character.is_ascii_lowercase());
        }
    }

    #[test]
    fn it_generates_uppercase_only_password() {
        let options = RandomPasswordOptions {
            include_lowercase: false,
            include_uppercase: true,
            include_numbers: false,
            include_special_characters: false,
            length: 10,
        };

        let random_password = RandomPassword::new(options);

        for character in random_password.password.chars() {
            assert!(character.is_ascii_uppercase());
        }
    }

    #[test]
    fn it_generates_numbers_only_password() {
        let options = RandomPasswordOptions {
            include_lowercase: false,
            include_uppercase: false,
            include_numbers: true,
            include_special_characters: false,
            length: 10,
        };

        let random_password = RandomPassword::new(options);

        for character in random_password.password.chars() {
            assert!(character.is_ascii_digit());
        }
    }

    #[test]
    fn it_generates_special_characters_only_password() {
        let options = RandomPasswordOptions {
            include_lowercase: false,
            include_uppercase: false,
            include_numbers: false,
            include_special_characters: true,
            length: 10,
        };

        let random_password = RandomPassword::new(options);

        for character in random_password.password.chars() {
            assert!(character.is_ascii_punctuation());
        }
    }

    #[test]
    fn it_generates_lower_upper_numbers_special_characters_password() {
        let options = RandomPasswordOptions::default();

        let random_password = RandomPassword::new(options);

        for character in random_password.password.chars() {
            assert!(character.is_ascii_alphanumeric() || character.is_ascii_punctuation());
        }

        assert_eq!(random_password.password.len(), 10);
    }

    #[test]
    fn it_generates_random_password_with_length() {
        let options = RandomPasswordOptions {
            length: 30,
            ..RandomPasswordOptions::default()
        };

        let random_password = RandomPassword::new(options);

        for character in random_password.password.chars() {
            assert!(
                character.is_ascii_alphanumeric() || character.is_ascii_digit() || character.is_ascii_punctuation()
            );
        }

        assert_eq!(random_password.password.len(), 30);
    }
}
