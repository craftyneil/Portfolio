use std::ops::Deref;

/// C'est le niveau du moteur linear actuator pour ouvrir ou fermer le toit de la serre
#[derive(Debug, Default)]
pub(crate) struct CeilingMotor(pub(crate) f32);

impl Deref for CeilingMotor {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
