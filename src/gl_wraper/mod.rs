pub mod array;
pub mod buffer;
pub mod location;
pub mod program;

pub use array::VertexArray;
pub use buffer::Buffer;
pub use buffer::*;
pub use program::Program;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    /// Just show the points.
    Point = gl::POINT as isize,
    /// Just show the lines.
    Line = gl::LINE as isize,
    /// Fill in the polygons.
    Fill = gl::FILL as isize,
}

/// Sets the font and back polygon mode to the mode given.
pub fn polygon_mode(mode: PolygonMode) {
    unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, mode as gl::types::GLenum) };
}
