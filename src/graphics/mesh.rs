use std::mem::offset_of;

use crate::platform::buffer::*;
use crate::platform::vertex_array::*;

pub struct Mesh {
    vao: VertexArray,
    vbo: VBO,
    ebo: EBO,
    index_count: i32,
}

struct MeshData {
    verticles: Vec<f32>,
    indices: Vec<i32>,
}

impl Mesh {
    pub fn new(data: &MeshData) -> Result<Self, String> {
        let vao = VertexArray::new()?;
        let vbo = VBO::new()?;
        let ebo = EBO::new()?;

        vao.bind();
        vbo.data(&data.verticles, gl::STATIC_DRAW);
        ebo.data(&data.indices, gl::STATIC_DRAW);

        unsafe { gl::EnableVertexAttribArray(0) };
        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<SVertex>() as _,
                0 as _,
            )
        };
        unsafe { gl::EnableVertexAttribArray(1) };
        unsafe {
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<SVertex>() as _,
                offset_of!(SVertex, normal) as _,
            )
        };
        unsafe { gl::EnableVertexAttribArray(2) };
        unsafe {
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                size_of::<SVertex>() as _,
                offset_of!(SVertex, texture) as _,
            )
        };

        vao.unbind();

        Ok(Self {
            vao,
            vbo,
            ebo,
            index_count: data.indices.len() as _,
        })
    }
}
