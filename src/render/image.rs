use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use image::{ImageBuffer, ImageOutputFormat, Rgb};
use image::codecs::hdr::HdrEncoder;
use crate::{Vec3};
use crate::geo::color::Color;

#[derive(Default)]
struct Pixel {
    color: Color,
    count: usize,
}

pub struct ImageBuilder {
    pixels: HashMap<(usize, usize), Pixel>,
    size: (usize, usize),
}

impl ImageBuilder {
    pub fn new(size: (usize, usize)) -> Self {
        ImageBuilder { pixels: HashMap::new(), size }
    }
    pub fn insert(&mut self, x: usize, y: usize, c: Color) {
        self.pixels.entry((x, y)).or_default().push(c);
    }
    pub fn size(&self) -> (usize, usize) {
        self.size
    }
    pub fn write(&self, file: &str) {
        let path = Path::new(file);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut image = ImageBuffer::<Rgb<f32>, _>::new(self.size.0 as u32, self.size.1 as u32);
        for (x, y, p) in image.enumerate_pixels_mut() {
            let mut c = if let Some(pixel) = self.pixels.get(&(x as usize, y as usize)) {
                pixel.average()
            } else {
                Vec3::broadcast(f64::NAN)
            };
            if c.into_iter().any(|x| x.is_nan()) {
                c = if (x + y) % 2 == 0 {
                    Color::broadcast(0.0)
                } else {
                    Color::broadcast(1.0)
                };
            }
            *p = Rgb([c.x() as f32, c.y() as f32, c.z() as f32])
        }
        let data: Vec<_> = image.enumerate_pixels().map(|(_, _, v)| *v).collect();

        let encoder = HdrEncoder::new(w);
        encoder.encode(&data, image.width() as usize, image.height() as usize).unwrap();
    }
}

impl Pixel {
    pub fn push(&mut self, color: Color) {
        self.count += 1;
        self.color += color;
    }
    fn average(&self) -> Color {
        self.color / (self.count as f64)
    }
}