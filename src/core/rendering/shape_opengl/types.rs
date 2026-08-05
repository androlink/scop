use crate::core::rendering::{
    buffer::{EBO, VBO},
    vertex_array::VertexArray,
};

pub struct GLMesh {
    pub vao: VertexArray,
    pub vbo: VBO,
    pub ebo: EBO,
    pub index_count: i32,
}

pub struct GLModel {
    pub meshes: Vec<GLMesh>,
}
