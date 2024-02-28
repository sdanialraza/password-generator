use copypasta::{ClipboardContext, ClipboardProvider};
use eframe::{
    egui::{CentralPanel, Context, RichText, Slider},
    App, Frame,
};
use serde_json::{from_str, json};
use std::time::Duration;

use crate::{app::PasswordGenerator, constants::MAX_PASSWORD_LENGTH, types::RandomPassword};

impl App for PasswordGenerator {
    fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(5)
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let mut passwords_to_add: Vec<String> = Vec::with_capacity(10);

        passwords_to_add.push(self.password.clone());

        let mut recent_passwords = match storage.get_string("recent_passwords") {
            Some(passwords) => {
                let passwords: Vec<_> = from_str(passwords.as_str()).unwrap_or_default();

                passwords_to_add.extend(passwords);

                passwords_to_add
            }
            None => passwords_to_add,
        };

        recent_passwords.sort();

        recent_passwords.dedup();

        recent_passwords.truncate(10);

        storage.set_string("recent_passwords", json!(recent_passwords).to_string());
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
}
