use std::fs::read_to_string;

pub struct GLShader(pub gl::types::GLuint);

impl Drop for GLShader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.0) };
    }
}

impl GLShader {
    pub fn new(shader_type: gl::types::GLenum) -> Result<Self, String> {
        match unsafe { gl::CreateShader(shader_type) } {
            0 => Err("cannot create shader".to_string()),
            n => Ok(Self(n)),
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
    pub fn source_file(self, path: &str) -> Result<Self, String> {
        match read_to_string(path) {
            Err(e) => Err(e.to_string()),
            Ok(file_content) => Ok(self.source(file_content.as_bytes())),
        }
    }

    pub fn compile(self) -> Self {
        unsafe { gl::CompileShader(self.0) };
        self
    }

    pub fn status(self) -> Result<Self, String> {
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
                    std::ptr::null_mut(),
                    buf.as_mut_ptr() as *mut gl::types::GLchar,
                )
            };
            return Err(String::from_utf8_lossy(buf.as_slice()).to_string());
        }
        Ok(self)
    }
}
