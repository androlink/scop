use crate::core::{rendering::texture::Texture, traits::bind::Bindable};

impl Texture {
    pub fn new() -> Option<Self> {
        let mut texture_id = 0;
        unsafe { gl::GenTextures(1, &mut texture_id) };
        match texture_id {
            0 => {
                eprintln!("cannot generate texture");
                None
            }
            n => Some(Self(n)),
        }
    }

    pub fn generate(&self, width: u32, height: u32, data: &[u8]) {
        self.bind();
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _) };
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _) };
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _) };
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _) };
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as _,
                width as _,
                height as _,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as _,
            )
        };
        unsafe { gl::GenerateMipmap(gl::TEXTURE_2D) };
    }
}
