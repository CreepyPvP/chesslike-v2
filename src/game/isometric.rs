use bevy::prelude::Transform;

pub fn iso_transform(x: f32, y: f32, z: f32, w: f32, h: f32) -> Transform {
    let xb = (x * w - y * w) / 2.;
    let yb = (-x * h - y * h) / 2. + z * h;
    let zb = x + y + z;

    Transform::from_xyz(xb, yb, zb)
}
