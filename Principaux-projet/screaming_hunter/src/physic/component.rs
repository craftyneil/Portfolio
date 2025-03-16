use bevy::{math::Vec3, prelude::Component};

/// use for storing the next calculated position of an entity before appliying it to the [`Transform`]
#[derive(Debug, Default, Component)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn normalize_to(&mut self, length: f32) {
        self.value = self.value.normalize_or_zero() * length;
    }
}
