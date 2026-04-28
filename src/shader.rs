use std::{fs::File, io::Read};

pub struct Shader(pub gl::types::GLuint);

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.0) };
    }
}

impl Shader {
    pub fn new(shader_type: gl::types::GLenum) -> Option<Self> {
        match unsafe { gl::CreateShader(shader_type) } {
            0 => None,
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
        println!("loading shader {path}");
        let mut file = match File::open(path) {
            Err(_) => return None,
            Ok(file) => file,
        };
        let mut file_content = vec![];
        file.read_to_end(&mut file_content).expect("no content ?");
        Some(self.source(&file_content))
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

    pub fn delete(self) {
        unsafe { gl::DeleteShader(self.0) };
    }
}
