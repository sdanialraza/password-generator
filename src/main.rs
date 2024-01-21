#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod constants;
mod random_password;

use constants::MAX_PASSWORD_LENGTH;
use random_password::{RandomPassword, RandomPasswordOptions};

use copypasta::{ClipboardContext, ClipboardProvider};
use eframe::{
    egui::{CentralPanel, Context, RichText, Slider, ViewportBuilder},
    App, Error, Frame, NativeOptions,
};

fn main() -> Result<(), Error> {
    let options = NativeOptions {
        centered: true,
        follow_system_theme: true,
        viewport: ViewportBuilder::default().with_inner_size([300.0, 200.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Password Generator",
        options,
        Box::new(|_app_creator| Box::<PasswordGenerator>::default()),
    )
}

struct PasswordGenerator {
    options: RandomPasswordOptions,
    password: String,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self {
            options: RandomPasswordOptions::default(),
            password: RandomPassword::new(RandomPasswordOptions::default()).password,
        }
    }
}

impl App for PasswordGenerator {
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
            }

            if ui.button(RichText::new("Copy")).clicked() {
                let mut context = ClipboardContext::new().unwrap();

                let _copied = context.set_contents(self.password.to_string());
            }
        });
    }
}
