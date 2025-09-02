use std::{env, fs};
use iced::Task;
use crate::app::App;

mod app;

fn main() -> iced::Result {
    let path = env::args().skip(1).last().unwrap_or_else(|| {
        panic!("Usage: bamegoy <rom-file>");
    });

    let cartridge = fs::read(&path).unwrap_or_else(|err| {
        panic!("Failed to open ROM '{}': {}", path, err);
    });

    iced::application("Bamegoy", app::update, app::view).run_with(|| {
        (App::new(cartridge), Task::none())
    })
}