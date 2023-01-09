use std::f32::consts::PI;

#[allow(dead_code)]
pub fn deg_to_rad(degree: f32) -> f32 {
    degree * (PI / 180.0)
}

#[allow(dead_code)]
pub fn rad_to_deg(rad: f32) -> f32 {
    rad * (180.0 / PI)
}
