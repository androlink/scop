use crate::mat4::Matrix4;

pub struct AttributeLocation(pub gl::types::GLuint);
pub struct MatrixLocation(pub gl::types::GLint);
pub struct F32Location(pub gl::types::GLint);

impl AttributeLocation {
    pub fn enable(&self) -> &Self {
        unsafe { gl::EnableVertexAttribArray(self.0) };
        self
    }

    pub fn disable(&self) -> &Self {
        unsafe { gl::DisableVertexAttribArray(self.0) };
        self
    }

    pub fn set(&self, size: gl::types::GLint, type_: gl::types::GLenum) -> &Self {
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

impl F32Location {
    pub fn set1(&self, f: f32) -> &Self {
        unsafe { gl::Uniform1f(self.0, f) };
        self
    }
    pub fn set2(&self, f1: f32, f2: f32) -> &Self {
        unsafe { gl::Uniform2f(self.0, f1, f2) };
        self
    }
    pub fn set3(&self, f1: f32, f2: f32, f3: f32) -> &Self {
        unsafe { gl::Uniform3f(self.0, f1, f2, f3) };
        self
    }
    pub fn set4(&self, f1: f32, f2: f32, f3: f32, f4: f32) -> &Self {
        unsafe { gl::Uniform4f(self.0, f1, f2, f3, f4) };
        self
    }
}
