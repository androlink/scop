mod parse;
mod vertex;

pub use vertex::*;

#[derive(Debug, Default)]
pub struct ShapeOBJ {
    pub verticles: Vec<Vertex>,
    pub indices: Vec<u32>,
}

#[derive(Debug, Default)]
pub struct Model {
    pub name: String,
    pub mesh: ShapeOBJ,
}

impl Model {
    pub fn new(mesh: ShapeOBJ, name: String) -> Model {
        Model { mesh, name }
    }
}
