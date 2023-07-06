use bevy::prelude::Vec3;

// Warning! As soon as zb is dependant on tile width / height the unit move code breaks
pub fn iso_transform(x: f32, y: f32, z: f32, w: f32, h: f32, is_unit: bool) -> Vec3 {
    let xb = (x * w - y * w) / 2.;
    let yb = (-x * h - y * h) / 2. + z * h;
    let mut zb = x + y + z;

    if is_unit {
        zb += 0.5;
    }

    Vec3::new(xb, yb, zb)
}

pub enum IsometricDirection {
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl IsometricDirection {
    pub fn from_vec(dir: (i32, i32)) -> Option<Self> {
        match dir {
            (0, -1) => Some(Self::UpRight),
            (-1, 0) => Some(Self::UpLeft),
            (1, 0) => Some(Self::DownRight),
            (0, 1) => Some(Self::DownLeft),
            _ => None,
        }
    }
}
