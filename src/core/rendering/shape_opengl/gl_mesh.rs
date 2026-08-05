use std::mem::offset_of;

use crate::core::{
    model::types::{Mesh, Vertex},
    rendering::{
        buffer::{EBO, VBO},
        shape_opengl::GLMesh,
        vertex_array::VertexArray,
    },
};

impl GLMesh {
    pub fn new(mesh: &Mesh) -> Result<Self, String> {
        let vao = VertexArray::new()?;
        let vbo = VBO::new()?;
        let ebo = EBO::new()?;

        vao.bind();
        vbo.data(&mesh.vertices, gl::STATIC_DRAW);
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
