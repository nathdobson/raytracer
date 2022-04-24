use crate::Color;

#[derive(Copy, Clone, Default, Debug)]
pub struct Material {
    pub diffuse: Color,
    pub dielectric: Option<(f64, f64)>,
}

impl Material {
    pub fn nan() -> Self {
        Material { diffuse: Color::nan(), dielectric: None }
    }
}