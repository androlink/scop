use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use sdl2::{render::Vertex, sys::_Float32};

use crate::vertex::{SIndice, SPosition};

pub struct SObject {
    verticles: Vec<SPosition>,
    uvs: Vec<SPosition>,
    normals: Vec<SPosition>,
    faces: Vec<SIndice>,
}

impl SObject {
    pub fn new(
        verticles: Vec<SPosition>,
        uvs: Vec<SPosition>,
        normals: Vec<SPosition>,
        faces: Vec<SIndice>,
    ) -> SObject {
        SObject {
            verticles,

            uvs,
            normals,
            faces,
        }
    }
}

pub struct OBJLoader {
    path: String,
}

#[derive(Debug)]
pub enum OBJError {
    Io(std::io::Error, String),
    Vertex(String),
}

impl OBJLoader {
    pub fn new() -> Self {
        OBJLoader {
            path: "".to_string(),
        }
    }

    pub fn path(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self
    }
    fn parse_vertex(args: &Vec<&str>) -> Result<SPosition, ()> {
        let x: f32 = args[0].parse::<f32>().map_err(|_| ())?;
        let y: f32 = args[0].parse::<f32>().map_err(|_| ())?;
        let z: f32 = args[0].parse::<f32>().map_err(|_| ())?;
        let _w: f32 = args[0].parse::<f32>().map_err(|_| ())?;

        Ok(SPosition(x, y, z))
    }

    pub fn load(&self, obj_file: &str) -> Result<SObject, OBJError> {
        let mut verticles: Vec<SPosition> = Vec::new();
        let mut uvs: Vec<SPosition> = Vec::new();
        let mut normals: Vec<SPosition> = Vec::new();
        let mut faces: Vec<SIndice> = Vec::new();
        let file_path = self.path.to_string() + "/" + obj_file;
        let file = File::open(file_path.to_string())
            .map_err(|o| OBJError::Io(o, file_path.to_string()))?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.map_err(|o| OBJError::Io(o, file_path.to_string()))?;
            let mut args_it = line.split_whitespace();
            if let Some(id) = args_it.next() {
                match id {
                    "v" => {
                        let vertex = Self::parse_vertex(&args_it.collect())
                            .map_err(|_| OBJError::Vertex("lol".to_string()))?;
                        verticles.push(vertex);
                    }
                    "vf" => {}
                    "vn" => {}
                    "f" => {}

                    _ => {}
                }
            }
        }
        Ok(SObject::new(verticles, uvs, normals, faces))
    }
}
