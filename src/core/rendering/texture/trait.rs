use super::Texture;
use crate::core::r#trait::bind::Bindable;

impl Bindable for Texture {
    fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.0) };
    }
}

impl Bindable for &(Texture, u32) {
    fn bind(&self) {
        unsafe { gl::ActiveTexture(self.1) };
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.0.0) };
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.0) };
    }
}
