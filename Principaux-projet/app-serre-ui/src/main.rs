mod app;
mod bluetooth;
mod motor;
mod sensor;
mod style;

fn main() -> iced::Result {
    iced::run("the app", app::Counter::update, app::Counter::view)
}
