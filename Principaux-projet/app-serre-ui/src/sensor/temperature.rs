use std::ops::Deref;

#[derive(Debug, Default)]
pub(crate) struct Temperature(pub(crate) f32);

impl Deref for Temperature {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
