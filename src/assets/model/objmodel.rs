use crate::graphics::vertex::*;

#[derive(Default, Debug, Clone)]
pub struct OBJModel {
    pub verticles: Vec<SPosition>,
    pub textures: Vec<STexture>,
    pub normals: Vec<SNormal>,
    pub vertex_indices: Vec<SIndice>,
    pub normal_indices: Vec<SIndice>,
    pub texture_indices: Vec<SIndice>,
}
