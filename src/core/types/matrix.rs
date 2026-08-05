use std::ops::Sub;

use crate::core::types::vector::{Vec3, Vector};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix<const N: usize, const M: usize>(pub [Vector<N>; M]);

pub type Mat2 = Matrix<2, 2>;
pub type Mat3 = Matrix<3, 3>;
pub type Mat4 = Matrix<4, 4>;

impl<const N: usize, const M: usize> From<[[f32; N]; M]> for Matrix<N, M> {
    fn from(value: [[f32; N]; M]) -> Self {
        Self(value.map(|v| v.into()))
    }
}

impl<const N: usize, const M: usize> Default for Matrix<N, M> {
    fn default() -> Self {
        [[0.; N]; M].into()
    }
}

impl<const N: usize, const M: usize> From<Matrix<N, M>> for [[f32; N]; M] {
    fn from(val: Matrix<N, M>) -> Self {
        val.0.map(|v| v.into())
    }
}

impl Mat4 {
    pub fn ident() -> Self {
        [
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]
        .into()
    }
    pub fn orthogonal() -> Self {
        [
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]
        .into()
    }

    // pub fn look_at(pos: &Vec3, target: &Vec3, up: &Vec3) -> Self {
    //     let f = target.sub(pos).norm();
    //     let s = f.mul_cross(up).norm();
    //     let t = s.mul_cross(&f);
    //     Matrix4::translate(-pos.x, -pos.y, -pos.z)
    //         * Self {
    //             data: [
    //                 [s.x, t.x, -f.x, 0.],
    //                 [s.y, t.y, -f.y, 0.],
    //                 [s.z, t.z, -f.z, 0.],
    //                 [0., 0., 0., 1.],
    //             ],
    //         }
    // }

    pub fn look_at(pos: &Vec3, target: &Vec3, up: &Vec3) -> Self {
        let forward = target.sub(pos).normalize();
        let side = forward.mul_cross(up).normalize();
        let t = side.mul_cross(&forward);
        [
            [side.0[0], t.0[0], -forward.0[0], 0.],
            [side.0[1], t.0[1], -forward.0[1], 0.],
            [side.0[2], t.0[2], -forward.0[2], 0.],
            [-side.dot(pos), -t.dot(pos), forward.dot(pos), 1.],
        ]
        .into()
    }

    // no perspective ?
    pub fn perspective(fov: f32, ratio: f32, near: f32, far: f32) -> Self {
        [
            [f32::atan(fov / 2.) * ratio, 0., 0., 0.],
            [0., f32::atan(fov / 2.), 0., 0.],
            [0., 0., -(far + near) / (far - near), -1.],
            [0., 0., -(2. * near * far) / (far - near), 0.],
        ]
        .into()

        // Self {
        //     data: [
        //         [f32::atan(fov / 2.) * ratio, 0., 0., 0.],
        //         [0., f32::atan(fov / 2.), 0., 0.],
        //         [0., 0., (far) / (far - near), 1.],
        //         [0., 0., -(near * far) / (far - near), 0.],
        //     ],
        // }
    }

    pub fn rotate_x(teta: f32) -> Self {
        [
            [1., 0., 0., 0.],
            [0., f32::cos(teta), -f32::sin(teta), 0.],
            [0., f32::sin(teta), f32::cos(teta), 0.],
            [0., 0., 0., 1.],
        ]
        .into()
    }
    pub fn rotate_y(teta: f32) -> Self {
        [
            [f32::cos(teta), 0., f32::sin(teta), 0.],
            [0., 1., 0., 0.],
            [-f32::sin(teta), 0., f32::cos(teta), 0.],
            [0., 0., 0., 1.],
        ]
        .into()
    }
    pub fn rotate_z(teta: f32) -> Self {
        [
            [f32::cos(teta), -f32::sin(teta), 0., 0.],
            [f32::sin(teta), f32::cos(teta), 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]
        .into()
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        [
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [x, y, z, 1.],
        ]
        .into()
    }
    pub fn scale_xyz(x: f32, y: f32, z: f32) -> Self {
        [
            [x, 0., 0., 0.],
            [0., y, 0., 0.],
            [0., 0., z, 0.],
            [0., 0., 0., 1.],
        ]
        .into()
    }

    pub fn scale(s: f32) -> Self {
        Self::scale_xyz(s, s, s)
    }

    pub fn transpose(&self) -> Self {
        let mut m = Self::default();
        for row in 0..4 {
            for col in 0..4 {
                m.0[row].0[col] = self.0[col].0[row];
            }
        }
        m
    }

    pub fn mat_mul(&self, other: &Self) -> Self {
        let mut m = Self::default();
        for row in 0..4 {
            for col in 0..4 {
                for index in 0..4 {
                    m.0[row].0[col] += self.0[row].0[index] * other.0[index].0[col];
                }
            }
        }
        m
    }
}
