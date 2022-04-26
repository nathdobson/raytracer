use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::path::Path;
use image::{ImageBuffer, ImageOutputFormat, Rgb};
use image::codecs::hdr::HdrEncoder;
use crate::geo::color::{Color, smpte2048_encode};
use crate::math::vec::Vec3;

#[derive(Default, Clone)]
struct Pixel {
    color: Color,
    count: usize,
}

#[derive(Clone)]
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
    pub fn smpte2048_encode(&self) -> Self {
        Self {
            pixels: self.pixels.iter().map(
                |(k, v)|
                    (*k, Pixel {
                        count: v.count,
                        color: (v.color / 80.0).map(smpte2048_encode),
                    })).collect(),
            size: self.size,
        }
    }
    pub fn write_to(&self, w: impl Write) -> io::Result<()> {
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
        Ok(())
    }
    pub fn to_hdr(&self) -> Vec<u8> {
        let mut bytes = vec![];
        self.write_to(&mut bytes).unwrap();
        bytes
    }
    // pub fn write(&self, file: &str) {
    //     let path = Path::new(file);
    //     let file = File::create(path).unwrap();
    //     let ref mut w = BufWriter::new(file);
    //     self.write_to(w).unwrap();
    // }
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