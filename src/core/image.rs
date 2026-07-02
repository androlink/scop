pub mod load;

#[derive(Default)]
pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}
