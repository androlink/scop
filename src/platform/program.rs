use std::ffi::{CStr, CString};

use crate::{
    platform::location::{AttributeLocation, F32Location, MatrixLocation},
    shader::GLShader,
};

pub struct ShaderProgram(pub gl::types::GLuint);

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.0) };
    }
}

#[derive(Debug)]
pub enum ProgramError {
    LOCATION(CString),
}

impl ShaderProgram {
    pub fn init(vertex: &str, fragment: &str) -> Result<Self, String> {
        let frag_shader = GLShader::new(gl::FRAGMENT_SHADER)?
            .source_file(fragment)?
            .compile()
            .status()?;
        let vert_shader = GLShader::new(gl::VERTEX_SHADER)?
            .source_file(vertex)?
            .compile()
            .status()?;

        let program = ShaderProgram::new()?
            .attach_shader(&frag_shader)
            .attach_shader(&vert_shader)
            .link()
            .status()?
            .detach_shader(&frag_shader)
            .detach_shader(&vert_shader);
        Ok(program)
    }

    pub fn new() -> Result<Self, String> {
        match unsafe { gl::CreateProgram() } {
            0 => Err("cannot create program".to_string()),
            n => Ok(Self(n)),
        }
    }

    pub fn r#use(&self) {
        unsafe { gl::UseProgram(self.0) };
    }

    pub fn attach_shader(self, shader: &GLShader) -> Self {
        unsafe { gl::AttachShader(self.0, shader.0) };
        self
    }

    pub fn detach_shader(self, shader: &GLShader) -> Self {
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

    pub fn get_attribute_location(&self, name: &CStr) -> Result<AttributeLocation, ProgramError> {
        let loc = unsafe { gl::GetAttribLocation(self.0, name.as_ptr()) };
        if loc < 0 {
            Err(ProgramError::LOCATION(CString::from(name)))
        } else {
            Ok(AttributeLocation(loc as gl::types::GLuint))
        }
    }

    pub fn get_matrix_location(&self, name: &CStr) -> Result<MatrixLocation, ProgramError> {
        let loc = unsafe { gl::GetUniformLocation(self.0, name.as_ptr()) };
        if loc < 0 {
            Err(ProgramError::LOCATION(CString::from(name)))
        } else {
            Ok(MatrixLocation(loc))
        }
    }

    pub fn get_float_location(&self, name: &CStr) -> Result<F32Location, ProgramError> {
        let loc = unsafe { gl::GetUniformLocation(self.0, name.as_ptr()) };
        if loc < 0 {
            Err(ProgramError::LOCATION(CString::from(name)))
        } else {
            Ok(F32Location(loc))
        }
    }
}
