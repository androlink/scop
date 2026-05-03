use core::fmt;

#[derive(Debug)]
pub struct SVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug)]
pub struct SNormal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct STexture {
    pub x: f32,
    pub y: f32,
}

impl SNormal {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        SNormal { x, y, z }
    }
}

impl STexture {
    pub fn new(x: f32, y: f32) -> Self {
        STexture { x, y }
    }
}

#[derive(Debug)]
pub struct SColor(pub f32, pub f32, pub f32);

#[derive(Debug)]
pub struct SIndice(
    pub gl::types::GLuint,
    pub gl::types::GLuint,
    pub gl::types::GLuint,
);

impl SVertex {
    pub fn new_xyz(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0f32 }
    }

    pub fn new_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl fmt::Display for SVertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v {} {} {} {}", self.x, self.y, self.z, self.w)
    }
}

impl fmt::Display for SNormal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vn {} {} {}", self.x, self.y, self.z)
    }
}

impl fmt::Display for STexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vt {} {}", self.x, self.y)
    }
}

impl SColor {}
