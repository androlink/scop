use crate::core::model::obj_model::types::vector::{Vec3, Vec4};

pub type VertexCoord = Vec4;
pub type VertexTexture = Vec3;
pub type VertexNormal = Vec3;
pub type VertexColor = Vec4;

#[repr(C)]
pub struct Vertex {
    pub position: VertexCoord,
    pub texture: VertexTexture,
    pub normal: VertexNormal,
    pub color: VertexColor,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<i32>,
}
