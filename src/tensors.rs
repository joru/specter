#![allow(dead_code, non_snake_case)]
use std::ops::{Add, Mul};
#[derive(PartialEq, Debug, PartialOrd, Clone, Copy)]
pub struct Vec3 {
    data: [f32; 3],
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { data: [x, y, z] }
    }
    pub fn arr(&self) -> [f32; 3] {
        self.data
    }
    pub fn X(&self) -> f32 {
        self.data[0]
    }
    pub fn Y(&self) -> f32 {
        self.data[1]
    }
    pub fn Z(&self) -> f32 {
        self.data[2]
    }
    pub fn dot(&self, rhs: Self) -> f32 {
        self.X() * rhs.X() + self.Y() * rhs.Y() + self.Z() * rhs.Z()
    }
    pub fn map<F>(&self, fun: F) -> Self
    where
        F: Fn(f32) -> f32,
    {
        Vec3::new(fun(self.X()), fun(self.Y()), fun(self.Z()))
    }
}
impl From<[f32; 3]> for Vec3 {
    fn from(arr: [f32; 3]) -> Self {
        Self { data: arr }
    }
}
impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.X() + rhs.X(), self.Y() + rhs.Y(), self.Z() + rhs.Z())
    }
}
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.X() * rhs, self.Y() * rhs, self.Z() * rhs)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Mat {
    rows: [Vec3; 3],
}

impl Mat {
    pub fn from_columns(arr: [Vec3; 3]) -> Self {
        Self {
            rows: [
                Vec3::new(arr[0].X(), arr[1].X(), arr[2].X()),
                Vec3::new(arr[0].Y(), arr[1].Y(), arr[2].Y()),
                Vec3::new(arr[0].Z(), arr[1].Z(), arr[2].Z()),
            ],
        }
    }
    pub const fn new(rows: [Vec3; 3]) -> Self {
        Self { rows }
    }
    pub fn rows(&self) -> [Vec3; 3] {
        self.rows
    }
    pub fn at(&self, x: usize, y: usize) -> f32 {
        self.rows[y].arr()[x]
    }
    pub fn transpose(&self) -> Self {
        Self::from_columns(self.rows)
    }
}
impl From<[Vec3; 3]> for Mat {
    fn from(rows: [Vec3; 3]) -> Self {
        Self { rows }
    }
}
impl Mul<Vec3> for Mat {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let r = self.rows;
        Vec3::new(r[0].dot(rhs), r[1].dot(rhs), r[2].dot(rhs))
    }
}
impl Mul for Mat {
    type Output = Mat;

    fn mul(self, rhs: Self) -> Self::Output {
        let rc = rhs.transpose();
        Mat {
            rows: [
                Vec3::new(
                    self.rows[0].dot(rc.rows[0]),
                    self.rows[0].dot(rc.rows[1]),
                    self.rows[0].dot(rc.rows[2]),
                ),
                Vec3::new(
                    self.rows[1].dot(rc.rows[0]),
                    self.rows[1].dot(rc.rows[1]),
                    self.rows[1].dot(rc.rows[2]),
                ),
                Vec3::new(
                    self.rows[2].dot(rc.rows[0]),
                    self.rows[2].dot(rc.rows[1]),
                    self.rows[2].dot(rc.rows[2]),
                ),
            ],
        }
    }
}
impl Mul<f32> for Mat {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            rows: [self.rows[0] * rhs, self.rows[1] * rhs, self.rows[2] * rhs],
        }
    }
}
