use crate::math::vec::Vec3;

pub type Color = Vec3<f64>;

const M1: f64 = 1305.0 / 8192.0;
const M2: f64 = 2523.0 / 32.0;
const C1: f64 = 107.0 / 128.0;
const C2: f64 = 2413.0 / 128.0;
const C3: f64 = 2392.0 / 128.0;

pub fn smpte2048_decode(x: f64) -> f64 {
    (0.0f64.maximum(x.powf(1.0 / M2) - C1) / (C2 - C3 * x.powf(1.0 / M2))).powf(1.0 / M1)
}

pub fn smpte2048_encode(x: f64) -> f64 {
    ((C1 + C2 * x.powf(M1)) / (1.0 + C3 * x.powf(M1))).powf(M2)
}

#[test]
fn test_smpte2048() {
    println!("{:?}", smpte2048_decode(0.0));
    println!("{:?}", smpte2048_encode(0.0));
    println!("{:?}", smpte2048_decode(1.0));
    println!("{:?}", smpte2048_encode(1.0));
    for x in 0..100 {
        let x1 = (x as f64) / 100.0;
        let x2 = smpte2048_decode(smpte2048_encode(x1));
        assert!(x1 - x2 < 1e-10);
    }
}