use std::{fs, time::Instant};

use super::texture::Texture;

#[derive(Default)]
pub struct BMPLoader {
    path: String,
}

impl BMPLoader {
    pub fn new() -> Self {
        BMPLoader {
            path: "".to_string(),
        }
    }

    pub fn path(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self
    }

    pub fn load(&mut self, obj_file: &str) -> Result<Texture, String> {
        let mut buffer = Texture::default();
        let start = Instant::now();
        let file_path = self.path.to_string() + "/" + obj_file;
        let file_content = fs::read_to_string(file_path).map_err(|o| o.to_string())?;

        let stop = start.elapsed().as_millis();
        println!("{} load in {} seconde", obj_file, stop as f32 / 1000.);
        Ok(buffer)
    }
}
