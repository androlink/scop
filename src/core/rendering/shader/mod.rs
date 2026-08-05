pub mod load;

pub mod r#trait;

pub mod f32_location;
pub mod i32_location;
pub mod matrix_location;
pub mod u32_location;

use self::r#trait::*;
use crate::core::traits::bind::bind;

pub fn load_shader(frag: &str, vert: &str) -> Option<Shader> {
    Shader::new(frag, vert)
}

pub enum PolygoneKind {
    Point,
    Line,
    Triangle,
}

pub fn shader(shader: &Shader, kind: Option<PolygoneKind>) {
    let kind = match kind {
        Some(PolygoneKind::Point) => gl::POINT,
        Some(PolygoneKind::Line) => gl::LINE,
        Some(PolygoneKind::Triangle) => gl::FILL,
        None => gl::FILL,
    };
    unsafe { gl::PolygonMode(gl::FRONT, kind) };

    bind(shader);
}

pub fn reset_shader() {
    unsafe { gl::UseProgram(0) };
}

#[derive(Debug, Default)]
pub struct Shader(pub gl::types::GLuint);
