use crate::{graphics::Graphics, platform::Platform};

pub struct App {
    pub platform: Platform,
    pub graphics: Graphics,
}

impl App {
    pub fn new() -> Result<Self, String> {
        let platform = Platform::new()?;
        let graphics = Graphics::new()?;
        Ok(Self { platform, graphics })
    }
}
