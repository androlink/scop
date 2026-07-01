pub mod load;

#[derive(Default)]
pub struct Texture {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}
