use crate::core::rendering::{
    buffer::{EBO, VBO},
    vertex_array::VertexArray,
};

pub mod gl_mesh;

pub struct GLMesh {
    pub vao: VertexArray,
    pub vbo: VBO,
    pub ebo: EBO,
    pub index_count: i32,
}

pub struct GLModel {
    meshes: Vec<GLMesh>,
}
