enum BluetoothMotorCommand {
    CeilingMotor { percentage: f32 },
    SprinklerMotor { steps: i32 },
}

impl From<BluetoothMotorCommand> for String {
    fn from(value: BluetoothMotorCommand) -> Self {
        match value {
            BluetoothMotorCommand::CeilingMotor { percentage } => {
                format!("CeilingMotor {}", percentage)
            }
            BluetoothMotorCommand::SprinklerMotor { steps } => {
                format!("SprinklerMotor {}", steps)
            }
        }
    }
}
