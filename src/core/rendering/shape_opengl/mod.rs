use crate::core::rendering::{
    buffer::{EBO, VBO},
    vertex_array::VertexArray,
};

pub mod mesh;

pub struct ShapeOpengl {
    pub vao: VertexArray,
    pub vbo: VBO,
    pub ebo: EBO,
    pub index_count: i32,
}
