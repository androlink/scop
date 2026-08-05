use std::ffi::CString;

use crate::core::traits::bind::Bindable;

use super::Shader;

pub trait GetUniform {
    fn get_location(&self, name: &str) -> Option<gl::types::GLint>;
}

pub trait SetUniform<T>: GetUniform {
    type Output;
    fn set_location(&self, name: &str, var: T) -> Self::Output;
}

impl GetUniform for Shader {
    fn get_location(&self, name: &str) -> Option<gl::types::GLint> {
        let c_name = CString::new(name).ok()?;
        let loc = unsafe { gl::GetUniformLocation(self.0, c_name.as_ptr()) };
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
