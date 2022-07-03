//! A simple 2-D point for testing

use ordered_float::NotNan;
use rand::Rng;
use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use num_traits::Bounded;

use crate::Point;

/// A simple `f32` 2-D point type for testing
#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub struct P2(pub [NotNan<f32>; 2]);
impl P2 {
    /// Creates a new point from (x,y).
    pub fn new(x: f32, y: f32) -> P2 {
        P2([NotNan::new(x).unwrap(), NotNan::new(y).unwrap()])
    }
}
impl Bounded for P2 {
    fn min_value() -> P2 {
        P2([NotNan::<f32>::min_value(), NotNan::<f32>::min_value()])
    }
    fn max_value() -> P2 {
        P2([NotNan::<f32>::max_value(), NotNan::<f32>::max_value()])
    }
}
impl Point<f32> for P2 {
    const DIM: u32 = 2;
    fn set(&mut self, index: u32, value: NotNan<f32>) {
        self.0[index as usize] = value;
    }
    fn get(&self, index: u32) -> NotNan<f32> {
        self.0[index as usize]
    }
}
impl Add for P2 {
    type Output = P2;

    fn add(self, rhs: P2) -> Self::Output {
        P2([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]])
    }
}
impl Sub for P2 {
    type Output = P2;

    fn sub(self, rhs: P2) -> Self::Output {
        P2([self.0[0] - rhs.0[0], self.0[1] - rhs.0[1]])
    }
}
impl Display for P2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.0[0], self.0[1])
    }
}

/// Creates a random point whose coordinate are in the interval [-100:100].
pub fn random_point() -> P2 {
    let mut rng = rand::thread_rng();
    P2([
        NotNan::new(rng.gen_range(-100.0..100.0)).unwrap(),
        NotNan::new(rng.gen_range(-100.0..100.0)).unwrap(),
    ])
}

/// Creates a random cloud of count points using [random_point()] for each.
pub fn random_point_cloud(count: u32) -> Vec<P2> {
    (0..count).map(|_| random_point()).collect()
}


/// A simple `f64` 3-D point type for testing
#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub struct P3(pub [NotNan<f64>; 3]);

impl P3 {
    /// Creates a new point from (x,y).
    pub fn new(x: f64, y: f64, z: f64) -> P3 {
        P3([NotNan::new(x).unwrap(), NotNan::new(y).unwrap(),  NotNan::new(z).unwrap()])
    }
}
impl Bounded for P3 {
    fn min_value() -> P3 {
        P3([NotNan::<f64>::min_value(), NotNan::<f64>::min_value(), NotNan::<f64>::min_value()])
    }
    fn max_value() -> P3 {
        P3([NotNan::<f64>::max_value(), NotNan::<f64>::max_value(),  NotNan::<f64>::max_value()])
    }
}
impl Point<f64> for P3 {
    const DIM: u32 = 3;
    fn set(&mut self, index: u32, value: NotNan<f64>) {
        self.0[index as usize] = value;
    }
    fn get(&self, index: u32) -> NotNan<f64> {
        self.0[index as usize]
    }
}
impl Add for P3 {
    type Output = P3;

    fn add(self, rhs: P3) -> Self::Output {
        P3([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1], self.0[2] + rhs.0[2]])
    }
}
impl Sub for P3 {
    type Output = P3;

    fn sub(self, rhs: P3) -> Self::Output {
        P3([self.0[0] - rhs.0[0], self.0[1] - rhs.0[1],  self.0[2] - rhs.0[2]])
    }
}
impl Display for P3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.0[0], self.0[1], self.0[2])
    }
}

/// Creates a random point whose coordinate are in the interval [-100:100].
pub fn random_point_p3() -> P3 {
    let mut rng = rand::thread_rng();
    P3([
        NotNan::new(rng.gen_range(0.0..1.0)).unwrap(),
        NotNan::new(rng.gen_range(0.0..1.0)).unwrap(),
        NotNan::new(rng.gen_range(0.0..1.0)).unwrap(),
    ])
}

/// Creates a random cloud of count points using [random_point()] for each.
pub fn random_point_cloud_3d(count: u32) -> Vec<P3> {
    (0..count).map(|_| random_point_p3()).collect()
}

