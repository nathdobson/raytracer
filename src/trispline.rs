use glam::Mat4;
use ordered_float::NotNan;
use crate::math::{Mat3, VecExt};
use crate::Vec3;
use rand::thread_rng;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct HermiteSpline {
    p0: Vec3,
    p1: Vec3,
    d0: Vec3,
    d1: Vec3,
}

#[derive(Debug, Clone)]
pub struct Spline {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    d: Vec3,
}

#[derive(Debug, Clone)]
pub struct NormalHermiteTrispline {
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    p01: Vec3,
    p12: Vec3,
    p20: Vec3,
    n0: Vec3,
    n1: Vec3,
    n2: Vec3,
    control: Vec3,
}

#[derive(Debug, Clone)]
pub struct HermiteTrispline {
    s01: HermiteSpline,
    s02: HermiteSpline,
    s21: HermiteSpline,
    control: Vec3,
}

#[derive(Debug, Clone)]
pub struct SplineTrispline {
    s01: Spline,
    s02: Spline,
    s21: Spline,
    control: Vec3,
}

#[derive(Debug, Clone)]
struct Trispline {
    m00: Vec3,
    m01: Vec3,
    m02: Vec3,
    m03: Vec3,
    m10: Vec3,
    m11: Vec3,
    m12: Vec3,
    m20: Vec3,
    m21: Vec3,
    m30: Vec3,
}

impl From<NormalHermiteTrispline> for HermiteTrispline {
    fn from(NormalHermiteTrispline { p0, p1, p2, p01, p12, p20, n0, n1, n2, control }: NormalHermiteTrispline) -> Self {
        let d01 = (p2 - p01).cross(n0);
        let d10 = (p2 - p01).cross(n1);
        let d02 = -(p1 - p20).cross(n0);
        let d20 = -(p1 - p20).cross(n2);
        let d12 = -(p0 - p12).cross(n1);
        let d21 = -(p0 - p12).cross(n2);
        let mut result = HermiteTrispline {
            s01: HermiteSpline { p0: p0, p1: p1, d0: d01, d1: d10 },
            s02: HermiteSpline { p0: p0, p1: p2, d0: d02, d1: d20 },
            s21: HermiteSpline { p0: p2, p1: p1, d0: d21, d1: d12 },
            control,
        };
        result.normalize_speed();
        result
    }
}

impl From<HermiteTrispline> for SplineTrispline {
    fn from(HermiteTrispline { s01, s02, s21, control }: HermiteTrispline) -> Self {
        SplineTrispline {
            s01: s01.into(),
            s02: s02.into(),
            s21: s21.into(),
            control,
        }
    }
}

impl From<SplineTrispline> for Trispline {
    fn from(SplineTrispline {
                s01: Spline { a: af, b: bf, c: cf, d: df },
                s02: Spline { a: ag, b: bg, c: cg, d: dg },
                s21: Spline { a: ah, b: bh, c: ch, d: dh },
                control,
            }: SplineTrispline) -> Self {
        let s1 = cf - cg - 2.0 * bg - 3.0 * ag - ch;
        let s2 = bf + bg + 3.0 * ag - bh;
        let s3 = af - ag - ah;
        let m11 = control;
        let m12 = -m11 - s1;
        let m21 = s3 + m12;
        Trispline {
            m00: df,
            m01: cg,
            m02: bg,
            m03: ag,
            m10: cf,
            m11,
            m12,
            m20: bf,
            m21,
            m30: af,
        }
    }
}

impl From<HermiteSpline> for Spline {
    fn from(HermiteSpline { p0, p1, d0, d1 }: HermiteSpline) -> Self {
        let a = 2.0 * p0 - 2.0 * p1 + 1.0 * d0 + 1.0 * d1;
        let b = -3.0 * p0 + 3.0 * p1 - 2.0 * d0 - 1.0 * d1;
        let c = d0;
        let d = p0;
        Spline { a, b, c, d }
    }
}

impl HermiteTrispline {
    pub fn normalize_speed(&mut self) {
        self.s01.normalize_speed();
        self.s02.normalize_speed();
        self.s21.normalize_speed();
    }
}

impl HermiteSpline {
    pub fn normalize_speed(&mut self) {
        let distance = self.p0.distance(self.p1);
        self.d0 = self.d0.normalize() * distance;
        self.d1 = self.d1.normalize() * distance;
    }
}

impl Spline {
    pub fn position(&self, time: f64) -> Vec3 {
        self.a * (time * time * time) + self.b * (time * time) + self.c * time + self.d
    }
    pub fn derivative(&self, time: f64) -> Vec3 {
        self.a * (3.0 * time * time) + self.b * (2.0 * time) + self.c
    }
}

impl Trispline {
    pub fn position(&self, t: f64, u: f64) -> Vec3 {
        self.m30 * (t * t * t) + self.m20 * (t * t) + self.m10 * t + self.m00
            + self.m21 * (t * t * u) + self.m11 * (t * u) + self.m01 * u
            + self.m12 * (t * u * u) + self.m02 * (u * u)
            + self.m03 * (u * u * u)
    }
    pub fn ddt(&self, t: f64, u: f64) -> Vec3 {
        self.m30 * (3.0 * t * t) + self.m20 * (2.0 * t) + self.m10
            + self.m21 * (2.0 * t * u) + self.m11 * u
            + self.m12 * (u * u)
    }
    pub fn ddu(&self, t: f64, u: f64) -> Vec3 {
        self.m21 * (t * t) + self.m11 * (t) + self.m01
            + self.m12 * (t * 2.0 * u) + self.m02 * (2.0 * u)
            + self.m03 * (3.0 * u * u)
    }
    pub fn normal(&self, t: f64, u: f64) -> Vec3 {
        self.ddt(t, u).cross(self.ddu(t, u)).normalize()
    }
}

#[test]
fn test() {
    let mut rng = thread_rng();

    let (d, c, tri) = (0..1000000).map(|i| {
        let control = Vec3::new(rng.gen_range(-0.9..-0.8), rng.gen_range(2.28..2.3), rng.gen_range(2.28..2.3));
        let mut nht = NormalHermiteTrispline {
            p0: Vec3::new(1.0, 0.0, 0.0),
            p1: Vec3::new(0.0, 1.0, 0.0),
            p2: Vec3::new(0.0, 0.0, 1.0),
            p01: Vec3::new(0.0, 0.0, -1.0),
            p12: Vec3::new(-1.0, 0.0, 0.0),
            p20: Vec3::new(0.0, -1.0, 0.0),
            n0: Vec3::new(1.0, 0.0, 0.0),
            n1: Vec3::new(0.0, 1.0, 0.0),
            n2: Vec3::new(0.0, 0.0, 1.0),
            control,
        };
        let mut ht: HermiteTrispline = nht.into();
        let mut st: SplineTrispline = ht.into();
        let mut tri: Trispline = st.into();
        // println!("{:#?}", tri);
        let na = tri.normal(0.0, 0.5);
        let nb = tri.normal(0.5, 0.0);
        let nc = tri.normal(0.5, 0.5);
        let da = na.distance(Vec3::new(1.0, 0.0, 1.0).normalize());
        let db = nb.distance(Vec3::new(1.0, 1.0, 0.0).normalize());
        let dc = nc.distance(Vec3::new(0.0, 1.0, 1.0).normalize());
        (da + db + dc, control, tri)
    }).min_by_key(|(d, c, _)| NotNan::new(*d).unwrap()).unwrap();
    for k in 0..=10 {
        let k = k as f64 / 10.0;
        // println!("{:?}", tri.normal(0.0, k).y);
    }
    for i in 0..=50 {
        for k in 0..=50 {
            if i + k <= 50 {
                let t = i as f64 / 50.0;
                let u = k as f64 / 50.0;
                let p = tri.position(t, u);
                println!("{} {} {}", p.x, p.y, p.z);
            }
        }
    }
    // for k in 0..=10 {
    //     let p = t.position(k as f64 / 10.0, 0.0);
    //     println!("{} {} {}", p.x, p.y, p.z);
    // }
    // for k in 0..=10 {
    //     let p = t.position(0.0, k as f64 / 10.0);
    //     println!("{} {} {}", p.x, p.y, p.z);
    // }
    // for k in 0..=10 {
    //     let p = t.position(k as f64 / 10.0, 1.0 - (k as f64 / 10.0));
    //     println!("{} {} {}", p.x, p.y, p.z);
    // }
    // let mut hermite = HermiteSpline {
    //     p0: Vec3::new(1.0, 0.0, 0.0),
    //     p1: Vec3::new(0.0, 1.0, 0.0),
    //     d0: Vec3::new(1.0, 1.0, 0.0).normalize(),
    //     d1: Vec3::new(1.0, 1.0, 0.0).normalize(),
    // };
    // hermite.normalize_speed();
    // let s: Spline = hermite.into();
    //
    // for t in 0..=10 {
    //     let p = s.position(t as f64 / 10.0);
    //     println!("{} {} {}", p.x, p.y, p.z);
    // }
    // println!("{:?}", s);
    // println!("{:?}", s.position(-1.0));
    // println!("{:?}", s.position(0.0));
    // println!("{:?}", s.position(1.0));
    // println!("{:?}", s.derivative(-1.0));
    // println!("{:?}", s.derivative(1.0));
}