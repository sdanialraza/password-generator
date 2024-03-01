use std::ops::RangeInclusive;

pub const APP_ID: &str = "password-generator";
pub const APP_NAME: &str = "Password Generator";

pub const DEFAULT_PASSWORD_LENGTH: u8 = 16;

pub const PASSWORD_LENGTH_RANGE: RangeInclusive<u8> = 4..=128;

pub const SPECIAL_CHARACTERS: [char; 14] = ['!', '@', '#', '$', '%', '&', '^', '&', '(', ')', '[', ']', '-', '+'];

pub const WINDOW_DIMENSIONS: (f32, f32) = (300.0, 425.0);
