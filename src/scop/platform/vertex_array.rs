pub struct VertexArray(pub gl::types::GLuint);
impl VertexArray {
    pub fn new() -> Result<Self, String> {
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) };
        if vao != 0 {
            Ok(Self(vao))
        } else {
            Err("fail to generate vertex array".to_string())
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.0) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }

    pub fn clear_binding() {
        unsafe { gl::BindVertexArray(0) }
    }

    pub fn draw(&self, mode: gl::types::GLenum, count: gl::types::GLsizei) {
        self.bind();
        unsafe { gl::DrawArrays(mode, 0, count) }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.0) };
    }
}
