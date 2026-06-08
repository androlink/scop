pub struct Texture(gl::types::GLuint);

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.0) };
    }
}

impl Texture {
    pub fn new() -> Result<Self, String> {
        let mut texture_id = 0;
        unsafe { gl::GenTextures(1, &mut texture_id) };
        match texture_id {
            0 => Err("cannot create shader".to_string()),
            n => Ok(Self(n)),
        }
    }
    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.0) };
    }
    pub fn generate(&self, width: i32, height: i32, data: &[u8]) {
        self.bind();
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _) };
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _) };
        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as _,
            )
        };
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _) };
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as _,
                width,
                height,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as _,
            )
        };
        unsafe { gl::GenerateMipmap(gl::TEXTURE_2D) };
    }
}
