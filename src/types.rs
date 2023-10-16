use std::fmt::{Display, Formatter};

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Angle {
    pitch: f32,
    yaw: f32,
    roll: f32
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
