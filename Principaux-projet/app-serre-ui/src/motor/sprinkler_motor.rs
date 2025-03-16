use std::ops::Deref;

#[derive(Debug, Default)]
pub(crate) struct SprinklerMotor(pub(crate) u32);

impl Deref for SprinklerMotor {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
