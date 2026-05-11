use std::{
    fs::File,
    io::{BufReader, Read},
    time::Instant,
};

pub struct BMPBuffer {}

pub struct BMPLoader {
    path: String,
}

impl BMPLoader {
    pub fn new() -> Self {
        Self {
            path: "".to_string(),
        }
    }
    pub fn path(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self
    }
    pub fn load(self, file_name: &str) -> Result<BMPBuffer, String> {
        let start = Instant::now();
        let file_path = self.path.to_string() + "/" + file_name;
        let file = File::open(file_path.clone()).map_err(|o| o.to_string())?;
        let mut reader = BufReader::new(file);
        let mut bmp_header = [0; 54];

        reader
            .read_exact(&mut bmp_header)
            .map_err(|o| o.to_string())?;

        let stop = start.elapsed().as_millis();
        println!("{} load in {} seconde", file_name, stop as f32 / 1000.);
        Ok(BMPBuffer {})
    }
}
