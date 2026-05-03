use std::ffi::{CStr, CString};

use crate::shader::Shader;

pub struct Program(pub gl::types::GLuint);

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.0) };
    }
}

impl Program {
    pub fn new() -> Option<Self> {
        match unsafe { gl::CreateProgram() } {
            0 => None,
            n => Some(Self(n)),
        }
    }

    pub fn r#use(&self) {
        unsafe { gl::UseProgram(self.0) };
    }

    pub fn attach_shader(self, shader: &Shader) -> Self {
        unsafe { gl::AttachShader(self.0, shader.0) };
        self
    }

    pub fn detach_shader(self, shader: &Shader) -> Self {
        unsafe { gl::DetachShader(self.0, shader.0) };
        self
    }

    pub fn link(self) -> Self {
        unsafe { gl::LinkProgram(self.0) };
        self
    }

    pub fn status(self) -> Result<Self, String> {
        let mut success: gl::types::GLint = 1;
        unsafe { gl::GetProgramiv(self.0, gl::LINK_STATUS, &mut success) };

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe { gl::GetProgramiv(self.0, gl::INFO_LOG_LENGTH, &mut len) };
            let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    self.0,
                    len,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr() as *mut gl::types::GLchar,
                )
            };
            return Err(String::from_utf8_lossy(buf.as_slice()).to_string());
        }
        Ok(self)
    }

    pub fn get_attribute_location(&self, name: &CStr) -> Option<gl::types::GLuint> {
        let loc = unsafe { gl::GetAttribLocation(self.0, name.as_ptr()) };
        if loc < 0 {
            None
        } else {
            Some(loc as gl::types::GLuint)
        }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteProgram(self.0);
        };
    }
}
