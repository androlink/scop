pub mod f32_location;
pub mod i32_location;
pub mod load;
pub mod matrix_location;
pub mod u32_location;

use crate::rendering::bind::Bindable;
use crate::rendering::bind::bind;

trait GetUniform {
    fn get_location(&self, name: &str) -> Option<gl::types::GLint>;
}

trait SetUniform<T>: GetUniform {
    type Output;
    fn set_location(&self, name: &str, var: T) -> Self::Output;
}

pub fn loadShader() -> Option<Shader> {
    todo!();
}

pub fn shader(shader: &Shader, kind: u32) {
    bind(shader);
}

#[derive(Debug, Default)]
pub struct Shader(pub gl::types::GLuint);

impl GetUniform for Shader {
    fn get_location(&self, name: &str) -> Option<gl::types::GLint> {
        let loc = unsafe { gl::GetUniformLocation(self.0, name.as_ptr() as _) };
        if loc < 0 {
            eprintln!("location name not found: {name}");
            None
        } else {
            Some(loc)
        }
    }
}

impl Bindable for Shader {
    fn bind(&self) {
        unsafe { gl::UseProgram(self.0) };
    }

    fn unbind(&self) {
        unsafe { gl::UseProgram(0) };
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.0) };
    }
}
