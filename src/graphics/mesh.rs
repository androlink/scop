use crate::platform::array::*;
use crate::platform::buffer::*;

pub struct Mesh {
    vao: VertexArray,
    vbo: VBO,
    ebo: EBO,
    index_count: i32,
}
