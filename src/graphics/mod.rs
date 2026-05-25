pub mod mesh;

mod manager;

pub struct Graphics {}

impl Graphics {
    pub fn new() -> Result<Graphics, String> {
        Ok(Self {})
    }
}
