use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Position,
    pub normal: Normal,
    pub color: Color,
    pub texture: Texture,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Normal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Texture {
    pub x: f32,
    pub y: f32,
}

impl Normal {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Normal { x, y, z }
    }
}

impl Texture {
    pub fn new(x: f32, y: f32) -> Self {
        Texture { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Indice(
    pub gl::types::GLuint,
    pub gl::types::GLuint,
    pub gl::types::GLuint,
);

impl Position {
    pub fn new_xyz(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0f32 }
    }

    pub fn new_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl From<&[f32; 3]> for Position {
    fn from(value: &[f32; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
            w: 1.,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.x, self.y, self.z, self.w)
    }
}

impl fmt::Display for Normal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl fmt::Display for Indice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

impl fmt::Display for Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
