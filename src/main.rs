#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod constants;
mod random_password;

use constants::MAX_PASSWORD_LENGTH;
use random_password::{RandomPassword, RandomPasswordOptions};

use std::time::Duration;

use copypasta::{ClipboardContext, ClipboardProvider};
use eframe::{
    egui::{CentralPanel, Context, RichText, Slider, ViewportBuilder},
    run_native, App, Error, Frame, NativeOptions,
};
use serde_json::{from_str, json};

fn main() -> Result<(), Error> {
    let options = NativeOptions {
        centered: true,
        follow_system_theme: true,
        viewport: ViewportBuilder::default()
            .with_app_id("password-generator")
            .with_inner_size([300.0, 200.0]),
        ..Default::default()
    };

    run_native(
        "Password Generator",
        options,
        Box::new(|creation_context| {
            if creation_context.storage.is_none() {
                return Box::<PasswordGenerator>::default();
            }

            let storage = creation_context.storage.unwrap();

            if storage.get_string("recent_passwords").is_none() {
                println!("recent_passwords got nothin");
                return Box::<PasswordGenerator>::default();
            }

            let recent_passwords = storage.get_string("recent_passwords").unwrap_or_default();

            let recent_passwords: Vec<String> = from_str(recent_passwords.as_str()).unwrap_or_default();

            Box::new(PasswordGenerator {
                recent_passwords,
                ..Default::default()
            })
        }),
    )
}

struct PasswordGenerator {
    options: RandomPasswordOptions,
    password: String,
    recent_passwords: Vec<String>,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self {
            options: RandomPasswordOptions::default(),
            password: RandomPassword::new(RandomPasswordOptions::default()).password,
            recent_passwords: Vec::with_capacity(10),
        }
    }
}

impl App for PasswordGenerator {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let mut passwords_to_add: Vec<String> = Vec::with_capacity(10);

        passwords_to_add.push(self.password.clone());

        let stored_recent_passwords = storage.get_string("recent_passwords").unwrap_or_default();

        let stored_recent_passwords: Vec<String> = from_str(stored_recent_passwords.as_str()).unwrap_or_default();

        if !stored_recent_passwords.is_empty() {
            passwords_to_add.extend(stored_recent_passwords);
        }

        passwords_to_add.sort();

        passwords_to_add.dedup();

        passwords_to_add.truncate(10);

        storage.set_string("recent_passwords", json!(passwords_to_add).to_string());
    }

    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label(RichText::new(self.password.to_string()).size(18.0));

            ui.add(Slider::new(&mut self.options.length, 1..=MAX_PASSWORD_LENGTH).text("Length"));

            ui.checkbox(&mut self.options.include_lowercase, "Include Lowercase");
            ui.checkbox(&mut self.options.include_uppercase, "Include Uppercase");
            ui.checkbox(&mut self.options.include_numbers, "Include Numbers");
            ui.checkbox(
                &mut self.options.include_special_characters,
                "Include Special Characters",
            );

            let checkbox_options = [
                self.options.include_lowercase,
                self.options.include_uppercase,
                self.options.include_numbers,
                self.options.include_special_characters,
            ];

            if ui.button(RichText::new("Generate")).clicked() {
                if !checkbox_options.iter().any(|&x| x) {
                    return self.password = String::from("At least one of checkboxes should be checked");
                }

                self.password = RandomPassword::new(self.options).password;

                if self.recent_passwords.len() >= 10 {
                    self.recent_passwords.pop();
                }

                self.recent_passwords.push(self.password.clone());
            }

            if ui.button(RichText::new("Copy")).clicked() {
                let mut context = ClipboardContext::new().unwrap();

                let _copied = context.set_contents(self.password.to_string());
            }

            ui.separator();

            ui.heading("Recent Passwords");

            ui.vertical(|ui| {
                for password in self.recent_passwords.iter().rev() {
                    ui.label(RichText::new(password.to_string()).size(14.0));
                }
            });
        });
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        Duration::from_secs(5)
    }
}
