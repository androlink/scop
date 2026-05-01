use std::ops::Mul;

use sdl2::sys::key_t;

#[derive(Clone, Debug, Default)]
pub struct Matrix {
    pub data: [[f32; 4]; 4],
}

impl Matrix {
    pub fn ident() -> Self {
        Self {
            data: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }
    pub fn orthogonal() -> Self {
        Self {
            data: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }
    pub fn perspective(fov: f32, ratio: f32, near: f32, far: f32) -> Self {
        Self {
            data: [
                [ratio, 0., 0., 0.],
                [0., fov, 0., 0.],
                [0., 0., -(far + near) / (far - near), -1.],
                [0., 0., -2. * (near * far) / (far - near), 0.],
            ],
        }
    }

    pub fn translate(&self, x: f32, y: f32, z: f32) -> Self {
        let mut m = self.clone();
        m.data[0][3] += x;
        m.data[1][3] += y;
        m.data[2][3] += z;
        m
    }
    pub fn scale_xyz(&self, x: f32, y: f32, z: f32) -> Self {
        let mut m = self.clone();
        m.data[0][0] *= x;
        m.data[1][1] *= y;
        m.data[2][2] *= z;
        m
    }

    pub fn scale(&self, s: f32) -> Self {
        let mut m = self.clone();
        m.data[0][0] *= s;
        m.data[1][1] *= s;
        m.data[2][2] *= s;
        m
    }

    pub fn mat_mul(&self, other: &Self) -> Self {
        let mut m = Self::default();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    m.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        m
    }
}

impl Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.mat_mul(&rhs)
    }
}
