use bevy::prelude::Vec3;

pub fn iso_transform(x: f32, y: f32, z: f32, w: f32, h: f32, is_unit: bool) -> Vec3 {
    let xb = (x * w - y * w) / 2.;
    let yb = (-x * h - y * h) / 2. + z * h;
    let mut zb = x + y + z;

    if is_unit {
        zb += 0.5;
    }

    Vec3::new(xb, yb, zb)
}
