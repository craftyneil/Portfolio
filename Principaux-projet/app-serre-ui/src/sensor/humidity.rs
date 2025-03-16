use std::ops::Deref;

#[derive(Debug, Default)]
pub(crate) struct Humidity(pub(crate) f32);

impl Deref for Humidity {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
