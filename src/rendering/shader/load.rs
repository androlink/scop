use std::{fs::read_to_string, path::Path};

use crate::rendering::shader::Shader;

impl Shader {
    pub fn new(fragment: &str, vertex: &str) -> Option<Self> {
        let frag_shader = ShaderChunk::new(gl::FRAGMENT_SHADER)?
            .source_file(fragment)?
            .compile()
            .compile_status()?;
        let vert_shader = ShaderChunk::new(gl::VERTEX_SHADER)?
            .source_file(vertex)?
            .compile()
            .compile_status()?;
        match unsafe { gl::CreateProgram() } {
            0 => {
                eprintln!("cannot create program");
                None
            }
            n => Some(
                Self(n)
                    .attach_shader(&frag_shader)
                    .attach_shader(&vert_shader)
                    .link()
                    .status()?
                    .detach_shader(&frag_shader)
                    .detach_shader(&vert_shader),
            ),
        }
    }

    fn attach_shader(self, shader: &ShaderChunk) -> Self {
        unsafe { gl::AttachShader(self.0, shader.0) };
        self
    }

    fn detach_shader(self, shader: &ShaderChunk) -> Self {
        unsafe { gl::DetachShader(self.0, shader.0) };
        self
    }

    fn link(self) -> Self {
        unsafe { gl::LinkProgram(self.0) };
        self
    }

    fn status(self) -> Option<Self> {
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
            eprintln!(
                "{}",
                String::from_utf8(buf).unwrap_or("unkown error".to_string())
            );
            return None;
        }
        Some(self)
    }
}

pub struct ShaderChunk(pub gl::types::GLuint);

impl Drop for ShaderChunk {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.0) };
    }
}

impl ShaderChunk {
    pub fn new(shader_type: gl::types::GLenum) -> Option<Self> {
        match unsafe { gl::CreateShader(shader_type) } {
            0 => {
                eprintln!("cannot create shader");
                None
            }
            n => Some(Self(n)),
        }
    }
    pub fn source(self, source: &[u8]) -> Self {
        unsafe {
            gl::ShaderSource(
                self.0,
                1,
                &(source.as_ptr().cast()),
                &(source.len().try_into().unwrap()),
            );
        }
        self
    }

    pub fn source_file(self, path: &str) -> Option<Self> {
        let content = read_to_string(path)
            .inspect_err(|e| eprintln!("{e}"))
            .ok()?;
        Some(self.source(content.as_bytes()))
    }

    pub fn compile(self) -> Self {
        unsafe { gl::CompileShader(self.0) };
        self
    }

    pub fn compile_status(self) -> Option<Self> {
        let mut success: gl::types::GLint = 1;
        unsafe { gl::GetShaderiv(self.0, gl::COMPILE_STATUS, &mut success) };

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe { gl::GetShaderiv(self.0, gl::INFO_LOG_LENGTH, &mut len) };
            let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
            unsafe {
                gl::GetShaderInfoLog(
                    self.0,
                    len,
                    &mut len,
                    buf.as_mut_ptr() as *mut gl::types::GLchar,
                )
            };
            unsafe { buf.set_len(len as usize) };
            eprintln!(
                "{}",
                String::from_utf8(buf).unwrap_or("unkown error".to_string())
            );
            return None;
        }
        Some(self)
    }
}
