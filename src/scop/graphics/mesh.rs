use std::mem::offset_of;

use crate::assets::mesh::Vertex;
use crate::platform::buffer::*;
use crate::platform::vertex_array::*;

pub struct Mesh {
    pub vao: VertexArray,
    pub vbo: VBO,
    pub ebo: EBO,
    pub index_count: i32,
}

impl Mesh {
    pub fn new(mesh: &crate::assets::mesh::Mesh) -> Result<Self, String> {
        let vao = VertexArray::new()?;
        let vbo = VBO::new()?;
        let ebo = EBO::new()?;

        vao.bind();
        vbo.data(&mesh.verticles, gl::STATIC_DRAW);
        ebo.data(&mesh.indices, gl::STATIC_DRAW);

        unsafe { gl::EnableVertexAttribArray(0) };
        unsafe {
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size_of::<Vertex>() as _, 0 as _)
        };
        unsafe { gl::EnableVertexAttribArray(1) };
        unsafe {
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>() as _,
                offset_of!(Vertex, normal) as _,
            )
        };
        unsafe { gl::EnableVertexAttribArray(2) };
        unsafe {
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>() as _,
                offset_of!(Vertex, texture) as _,
            )
        };

        unsafe { gl::EnableVertexAttribArray(3) };
        unsafe {
            gl::VertexAttribPointer(
                3,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>() as _,
                offset_of!(Vertex, color) as _,
            )
        };

        vao.unbind();

        Ok(Self {
            vao,
            vbo,
            ebo,
            index_count: mesh.indices.len() as _,
        })
    }

    pub fn draw(&self) {
        self.vao.bind();
        self.ebo.draw(gl::TRIANGLES, self.index_count);
        self.vao.unbind();
    }
}
