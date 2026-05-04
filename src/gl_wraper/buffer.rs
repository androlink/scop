pub use gl::ARRAY_BUFFER as Array;
pub use gl::ELEMENT_ARRAY_BUFFER as Element_Array;

pub use gl::types::GLenum as BufferType;

pub struct Buffer<const BT: gl::types::GLenum>(pub gl::types::GLuint);
impl<const BT: gl::types::GLenum> Buffer<BT> {
    pub fn new() -> Option<Self> {
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        if vbo != 0 { Some(Self(vbo)) } else { None }
    }

    pub fn bind(&self) -> &Self {
        unsafe { gl::BindBuffer(BT, self.0) };
        self
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(BT, 0) }
    }
    pub fn data<N>(&self, data: &[N], usage: gl::types::GLenum) -> &Self
    where
        N: Sized,
    {
        self.bind();
        unsafe {
            gl::BufferData(
                BT as gl::types::GLenum,
                (data.len() * size_of::<N>()).try_into().unwrap(),
                data.as_ptr().cast(),
                usage,
            );
        };
        self
    }

    pub fn draw(&self, mode: gl::types::GLenum, count: gl::types::GLsizei) {
        self.bind();
        unsafe { gl::DrawElements(mode, 3 * count, gl::UNSIGNED_INT, 0 as *const _) }
    }
}

impl<const BT: gl::types::GLenum> Drop for Buffer<BT> {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.0) };
    }
}

pub fn buffer_data<N>(ty: BufferType, data: &[N], usage: gl::types::GLenum)
where
    N: Sized,
{
    unsafe {
        gl::BufferData(
            ty as gl::types::GLenum,
            (data.len() * size_of::<N>()).try_into().unwrap(),
            data.as_ptr().cast(),
            usage,
        );
    }
}
