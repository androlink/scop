use crate::core::model::types::{VertexColor, VertexCoord, VertexNormal, VertexTexture};

#[derive(Default, Debug, Clone, Copy)]
pub struct Material();

#[derive(Default, Debug, Clone)]
pub struct Face {
    pub verticies: [FaceVertex; 3],
}

#[derive(Default, Debug, Clone, Copy)]
pub struct FaceVertex {
    pub position: i32,
    pub texture: i32,
    pub normal: i32,
    pub color: i32,
}

#[derive(Default, Debug, Clone)]
pub struct OBJMesh {
    pub face: Vec<Face>,
    pub material: Option<Material>,
}

#[derive(Default, Debug, Clone)]
pub struct OBJModel {
    pub verticles: Vec<VertexCoord>,
    pub normals: Vec<VertexNormal>,
    pub textures: Vec<VertexTexture>,
    pub colors: Vec<VertexColor>,
    pub meshes: Vec<OBJMesh>,
}

impl FaceVertex {
    pub fn new(v: i32, t: i32, n: i32, c: i32) -> Self {
        FaceVertex {
            position: v,
            texture: t,
            normal: n,
            color: c,
        }
    }
}

impl Face {
    pub fn new(verticies: [FaceVertex; 3]) -> Self {
        Face { verticies }
    }
}
