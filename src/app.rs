use copypasta::{ClipboardContext, ClipboardProvider};
use eframe::{
    egui::{CentralPanel, Context, RichText, Slider},
    App, Frame, Storage,
};
use serde_json::{from_str, json};
use std::time::Duration;

use crate::{
    constants::PASSWORD_LENGTH_RANGE,
    random_password::{RandomPassword, RandomPasswordOptions},
};

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

impl App for PasswordGenerator {
    fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(1)
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        if !self.recent_passwords.contains(&self.password) {
            self.recent_passwords.push(self.password.clone());
        }

        let recent_passwords = match storage.get_string("recent_passwords") {
            Some(passwords) => {
                let mut passwords: Vec<_> = from_str(passwords.as_str()).unwrap_or_default();
                passwords.push(self.password.clone());
                passwords.truncate(10);
                passwords
            }
            None => vec![self.password.clone()],
        };

        storage.set_string("recent_passwords", json!(recent_passwords).to_string());
    }

    fn update(&mut self, context: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(context, |ui| {
            ui.label(RichText::new(self.password.to_string()).size(18.0));

            ui.add(Slider::new(&mut self.options.length, PASSWORD_LENGTH_RANGE).text("Length"));

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

            ui.horizontal(|ui| {
                if ui.button(RichText::new("Generate").size(14.0)).clicked() {
                    if !checkbox_options.iter().any(|&x| x) {
                        return self.password = String::from("At least one of checkboxes should be checked");
                    }

                    if self.recent_passwords.len() >= 10 {
                        self.recent_passwords.remove(0);
                    }

                    self.password = RandomPassword::default().password;
                    self.recent_passwords.push(self.password.clone());
                }

                if ui.button(RichText::new("Copy").size(14.0)).clicked() {
                    let mut context = ClipboardContext::new().unwrap();

                    let _copied = context.set_contents(self.password.to_string());
                }
            });

            if !self.recent_passwords.is_empty() {
                ui.separator();

                ui.heading("Recent Passwords");

                ui.vertical(|ui| {
                    for password in self.recent_passwords.iter().rev() {
                        ui.label(RichText::new(password.to_string()).size(14.0));
                    }
                });
            }
        });
    }
}
