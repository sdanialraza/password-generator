use rand::{distributions::Uniform, thread_rng, Rng};

use crate::constants::{DEFAULT_PASSWORD_LENGTH, SPECIAL_CHARACTERS};

#[derive(Clone, Copy, Debug)]
pub struct RandomPasswordOptions {
    pub include_lowercase: bool,
    pub include_numbers: bool,
    pub include_special_characters: bool,
    pub include_uppercase: bool,
    pub length: u8,
}

impl Default for RandomPasswordOptions {
    fn default() -> Self {
        Self {
            include_lowercase: true,
            include_numbers: true,
            include_special_characters: true,
            include_uppercase: true,
            length: DEFAULT_PASSWORD_LENGTH,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RandomPassword {
    pub characters: Vec<char>,
    pub password: String,
    pub range: Uniform<usize>,
}

impl Default for RandomPassword {
    fn default() -> Self {
        Self::new(RandomPasswordOptions::default())
    }
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
    use crate::random_password::{RandomPassword, RandomPasswordOptions};

    #[test]
    fn it_generates_lowercase_only_password() {
        let options = RandomPasswordOptions {
            include_lowercase: true,
            include_uppercase: false,
            include_numbers: false,
            include_special_characters: false,
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
        };

        let random_password = RandomPassword::new(options);

        for character in random_password.password.chars() {
            assert!(character.is_ascii_punctuation());
        }
    }

    #[test]
    fn it_generates_lower_upper_numbers_special_characters_password() {
        let random_password = RandomPassword::default();

        for character in random_password.password.chars() {
            assert!(character.is_ascii_alphanumeric() || character.is_ascii_punctuation());
        }

        assert_eq!(random_password.password.len(), 16);
    }

    #[test]
    fn it_generates_random_password_with_length() {
        let options = RandomPasswordOptions {
            length: 30,
            ..Default::default()
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
