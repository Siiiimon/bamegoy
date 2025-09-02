mod app;

fn main() -> iced::Result {
    iced::run("Bamegoy", app::update, app::view)
}