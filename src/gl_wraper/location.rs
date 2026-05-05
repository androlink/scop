use crate::mat4::Matrix4;

pub struct AttributeLocation(pub gl::types::GLuint);
pub struct MatrixLocation(pub gl::types::GLint);

impl AttributeLocation {
    pub fn enable(&self) -> &Self {
        unsafe { gl::EnableVertexAttribArray(self.0) };
        self
    }

    pub fn disable(&self) -> &Self {
        unsafe { gl::DisableVertexAttribArray(self.0) };
        self
    }

    pub fn assign(&self, size: gl::types::GLint, type_: gl::types::GLenum) -> &Self {
        unsafe {
            gl::VertexAttribPointer(
                self.0,
                size,
                type_,
                gl::FALSE,
                0 as gl::types::GLint,
                std::ptr::null(),
            )
        };
        self
    }
}

impl MatrixLocation {
    pub fn set(&self, mat4: &Matrix4) -> &Self {
        unsafe { gl::UniformMatrix4fv(self.0, 1, gl::FALSE, mat4.data.as_ptr() as *const _) };
        self
    }
}
