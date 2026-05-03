use std::ops::Mul;

#[derive(Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(value: (f32, f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from(value: (f64, f64, f64)) -> Self {
        Self {
            x: value.0 as f32,
            y: value.1 as f32,
            z: value.2 as f32,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Matrix4 {
    pub data: [[f32; 4]; 4],
}

impl Matrix4 {
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

    pub fn look_at(eye: &Vec3, center: &Vec3, up: &Vec3) -> Self {
        let f = center.sub(eye).norm();
        let s = f.mul_cross(up).norm();
        let t = s.mul_cross(&f);
        Self::translate(eye.x, eye.y, eye.z)
            * Self {
                data: [
                    [s.x, t.x, -f.x, 0.],
                    [s.y, t.y, -f.y, 0.],
                    [s.z, t.z, -f.z, 0.],
                    [0., 0., 0., 1.],
                ],
            }
    }

    // no perspective ?
    pub fn perspective(fov: f32, ratio: f32, near: f32, far: f32) -> Self {
        Self {
            data: [
                [f32::atan(fov / 2.) * ratio, 0., 0., 0.],
                [0., f32::atan(fov / 2.), 0., 0.],
                [0., 0., (far) / (far - near), 1.],
                [0., 0., -(near * far) / (far - near), 0.],
            ],
        }
    }

    pub fn rotate_x(teta: f32) -> Self {
        Self {
            data: [
                [1., 0., 0., 0.],
                [0., f32::cos(teta), -f32::sin(teta), 0.],
                [0., f32::sin(teta), f32::cos(teta), 0.],
                [0., 0., 0., 1.],
            ],
        }
    }
    pub fn rotate_y(teta: f32) -> Self {
        Self {
            data: [
                [f32::cos(teta), 0., f32::sin(teta), 0.],
                [0., 1., 0., 0.],
                [-f32::sin(teta), 0., f32::cos(teta), 0.],
                [0., 0., 0., 1.],
            ],
        }
    }
    pub fn rotate_z(teta: f32) -> Self {
        Self {
            data: [
                [f32::cos(teta), -f32::sin(teta), 0., 0.],
                [f32::sin(teta), f32::cos(teta), 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [x, y, z, 1.],
            ],
        }
    }
    pub fn scale_xyz(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [
                [x, 0., 0., 0.],
                [0., y, 0., 0.],
                [0., 0., z, 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn scale(s: f32) -> Self {
        Self::scale_xyz(s, s, s)
    }

    pub fn transpose(&self) -> Self {
        let mut m = Self::default();
        for row in 0..4 {
            for col in 0..4 {
                m.data[row][col] = self.data[col][row];
            }
        }
        m
    }

    pub fn mat_mul(&self, other: &Self) -> Self {
        let mut m = Self::default();
        for row in 0..4 {
            for col in 0..4 {
                for index in 0..4 {
                    m.data[row][col] += self.data[row][index] * other.data[index][col];
                }
            }
        }
        m
    }
}

impl Mul for Matrix4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.mat_mul(&rhs)
    }
}

impl Vec3 {
    pub fn mul_cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn len(&self) -> f32 {
        (self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.)).sqrt()
    }

    pub fn norm(&self) -> Self {
        let inv_norm = 1. / self.len();
        self.scale(inv_norm)
    }

    pub fn scale(&self, n: f32) -> Self {
        Self {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
        }
    }
}
