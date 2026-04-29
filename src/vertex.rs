pub struct SPosition(pub f32, pub f32, pub f32);
pub struct SColor(pub f32, pub f32, pub f32);

pub struct SVertex(pub SPosition, pub SColor);

pub struct SIndice(
    pub gl::types::GLuint,
    pub gl::types::GLuint,
    pub gl::types::GLuint,
);

impl SVertex {}

impl SPosition {}

impl SColor {}
