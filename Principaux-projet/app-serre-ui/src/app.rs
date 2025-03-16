use crate::{
    ceiling_motor::CeilingMotor, humidity::Humidity, sprinkler_motor::SprinklerMotor,
    temperature::Temperature,
};

#[derive(Debug, Default)]
struct App {
    temperature: Temperature,
    humidity: Humidity,
    sprinkler_motor_steps: SprinklerMotor,
    ceiling_motor_extended_percentage: CeilingMotor,
}

impl App {
    fn new() -> Self {
        let mut app = App::default();

        app
    }
}

#[derive(Debug, Default)]
enum AppState {
    #[default]
    Automatic,
    Manual,
}
