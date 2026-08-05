use std::ops::{Add, Div, Mul, Sub};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector<const N: usize>(pub [f32; N]);

pub type Vec2 = Vector<2>;
pub type Vec3 = Vector<3>;
pub type Vec4 = Vector<4>;

impl<const N: usize> From<[f32; N]> for Vector<N> {
    fn from(value: [f32; N]) -> Self {
        Vector(value)
    }
}

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        [0.; N].into()
    }
}

impl From<Vec2> for Vec3 {
    fn from(value: Vec2) -> Self {
        Self([value.0[0], value.0[1], 0.])
    }
}

impl From<Vec3> for Vec4 {
    fn from(value: Vec3) -> Self {
        Self([value.0[0], value.0[1], value.0[2], 0.])
    }
}

impl<const N: usize> Vector<N> {
    pub fn scale(self, value: f32) -> Self {
        self.0.map(|v| v * value).into()
    }
}

impl<const N: usize> Mul<f32> for Vector<N> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        self.0.map(|v| v * rhs).into()
    }
}

impl<const N: usize> Div<f32> for Vector<N> {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self.0.map(|v| v * rhs).into()
    }
}

impl<const N: usize> Add<f32> for Vector<N> {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        self.0.map(|v| v + rhs).into()
    }
}

impl<const N: usize> Sub<f32> for Vector<N> {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        self.0.map(|v| v + rhs).into()
    }
}

impl<const N: usize> Mul<&Self> for Vector<N> {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        let mut r = [0.; N];
        for i in 0..N {
            r[i] = self.0[i] * rhs.0[i];
        }
        r.into()
    }
}

impl<const N: usize> Div<&Self> for Vector<N> {
    type Output = Self;

    fn div(self, rhs: &Self) -> Self::Output {
        let mut r = [0.; N];
        for i in 0..N {
            r[i] = self.0[i] / rhs.0[i];
        }
        r.into()
    }
}

impl<const N: usize> Add<&Self> for Vector<N> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        let mut r = [0.; N];
        for i in 0..N {
            r[i] = self.0[i] + rhs.0[i];
        }
        r.into()
    }
}

impl<const N: usize> Sub<&Self> for Vector<N> {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        let mut r = [0.; N];
        for i in 0..N {
            r[i] = self.0[i] - rhs.0[i];
        }
        r.into()
    }
}

impl<const N: usize> Vector<N> {
    pub fn norm(&self) -> f32 {
        self.0
            .iter()
            .fold(f32::default(), |acc, &a| (a * a) + acc)
            .sqrt()
    }

    pub fn normalize(&self) -> Self {
        let inv_norm = 1. / self.norm();
        self.scale(inv_norm)
    }

    pub fn dot(&self, v: &Self) -> f32 {
        self.0
            .iter()
            .zip(v.0.iter())
            .fold(0., |acc, (a, b)| acc + *a * *b)
    }
}

impl Vec3 {
    pub fn mul_cross(&self, other: &Self) -> Self {
        [
            self.0[1] * other.0[2] - self.0[2] * other.0[1],
            self.0[2] * other.0[0] - self.0[0] * other.0[2],
            self.0[0] * other.0[1] - self.0[1] * other.0[0],
        ]
        .into()
    }
}
