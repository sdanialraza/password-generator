#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui::ViewportBuilder, run_native, Error, NativeOptions};
use serde_json::from_str;

mod app;
mod constants;
mod gui;
mod types;

use crate::{
    app::PasswordGenerator,
    constants::{APP_ID, APP_NAME, WINDOW_DIMENSIONS},
};

fn main() -> Result<(), Error> {
    let options = NativeOptions {
        centered: true,
        follow_system_theme: true,
        viewport: ViewportBuilder::default()
            .with_app_id(APP_ID)
            .with_inner_size(WINDOW_DIMENSIONS),
        ..Default::default()
    };

    run_native(
        APP_NAME,
        options,
        Box::new(|creation_context| {
            let storage = match creation_context.storage {
                Some(storage) => storage,
                None => return Box::<PasswordGenerator>::default(),
            };

            let recent_passwords = match storage.get_string("recent_passwords") {
                Some(passwords) => from_str(passwords.as_str()).unwrap_or_default(),
                None => return Box::<PasswordGenerator>::default(),
            };

            Box::new(PasswordGenerator {
                recent_passwords,
                ..Default::default()
            })
        }),
    )
}
