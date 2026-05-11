use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

use super::*;

#[derive(Debug)]
pub enum OBJError {
    Io(std::io::Error, String),
    Vertex(String),
    Face(String),
    Parse(String),
    NotEnoughArg(String),
    NoObject,
}

#[derive(Debug, Clone)]
pub struct OBJDescriptor {
    pub name: String,
    pub start: gl::types::GLsizei,
    pub size: gl::types::GLsizei,
}

#[derive(Default)]
pub struct OBJLoader {
    path: String,
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

    pub fn load(&mut self, obj_file: &str) -> Result<OBJBuffer, OBJError> {
        let mut buffer = OBJBuffer::default();
        let start = Instant::now();
        let file_path = self.path.to_string() + "/" + obj_file;
        let file = File::open(file_path.clone()).map_err(|o| OBJError::Io(o, file_path.clone()))?;
        let reader = BufReader::new(file);
        buffer.objects_mut().push(OBJDescriptor {
            name: obj_file.to_string(),
            start: 0,
            size: 0,
        });
        for line in reader.lines() {
            let line = line.map_err(|o| OBJError::Io(o, file_path.clone()))?;
            let mut args_it = line.split_whitespace();
            if let Some(id) = args_it.next() {
                let args: Vec<_> = args_it.clone().collect();
                match id {
                    "o" => {
                        let v_start = buffer.vertex_indices().len() as gl::types::GLsizei;

                        if let Some(obj) = buffer.objects_mut().last_mut() {
                            obj.size = v_start - obj.start;
                        }
                        let name = args[0].to_string();
                        let obj = OBJDescriptor {
                            name,
                            start: v_start,
                            size: 0,
                        };
                        println!("o {}", obj.name);
                        buffer.objects_mut().push(obj);
                    }
                    "v" => {
                        let vertex = Self::parse_vertex(&args)
                            .map_err(|e| OBJError::Vertex(format!("load: {e:?}").to_string()))?;
                        //println!("{}", vertex);
                        buffer.verticles_mut().push(vertex);
                    }
                    "vn" => {
                        let normal = Self::parse_vertex_normal(&args)
                            .map_err(|e| OBJError::Vertex(format!("load: {e:?}").to_string()))?;
                        buffer.normals_mut().push(normal);
                    }
                    "vt" => {
                        let texture = Self::parse_vertex_texture(&args)
                            .map_err(|e| OBJError::Vertex(format!("load: {e:?}").to_string()))?;
                        buffer.textures_mut().push(texture);
                    }
                    "f" => {
                        let mut face = Self::parse_face(&args)
                            .map_err(|e| OBJError::Vertex(format!("load: {e:?}").to_string()))?;

                        //println!("f {:?}", face);
                        buffer.vertex_indices_mut().append(&mut face.0);
                        buffer.texture_indices_mut().append(&mut face.1);
                        buffer.normal_indices_mut().append(&mut face.2);
                    }

                    _ => {}
                }
            }
        }
        let end = buffer.vertex_indices().len() as gl::types::GLsizei;

        if let Some(obj) = buffer.objects_mut().last_mut() {
            obj.size = end - obj.start;
        }
        let stop = start.elapsed().as_millis();
        println!("{} load in {} seconde", obj_file, stop as f32 / 1000.);
        Ok(buffer)
    }

    fn parse_vertex(args: &[&str]) -> Result<SVertex, OBJError> {
        if args.len() < 3 || args.len() > 4 {
            return Err(OBJError::NotEnoughArg("parse_vertex".to_string()));
        }
        let x: f32 = args[0]
            .parse::<f32>()
            .map_err(|e| OBJError::Parse(format!("parse_vertex: {e}")))?;
        let y: f32 = args[1]
            .parse::<f32>()
            .map_err(|e| OBJError::Parse(format!("parse_vertex: {e}")))?;
        let z: f32 = args[2]
            .parse::<f32>()
            .map_err(|e| OBJError::Parse(format!("parse_vertex: {e}")))?;
        let w: f32 = if args.len() == 4 {
            args[4]
                .parse::<f32>()
                .map_err(|e| OBJError::Parse(format!("parse_vertex: {e}")))?
        } else {
            1.
        };

        Ok(SVertex::new_xyzw(x, y, z, w))
    }

    fn parse_vertex_normal(args: &[&str]) -> Result<SNormal, OBJError> {
        if args.len() != 3 {
            return Err(OBJError::NotEnoughArg("parse_vertex_normal".to_string()));
        }
        let x: f32 = args[0]
            .parse::<f32>()
            .map_err(|e| OBJError::Parse(format!("parse_vertex_normal: {e}")))?;
        let y: f32 = args[1]
            .parse::<f32>()
            .map_err(|e| OBJError::Parse(format!("parse_vertex_normal: {e}")))?;
        let z: f32 = args[2]
            .parse::<f32>()
            .map_err(|e| OBJError::Parse(format!("parse_vertex_normal: {e}")))?;

        Ok(SNormal::new(x, y, z))
    }

    fn parse_vertex_texture(args: &[&str]) -> Result<STexture, OBJError> {
        if args.len() < 2 {
            return Err(OBJError::NotEnoughArg("parse_vertex_texture".to_string()));
        }
        let x: f32 = args[0]
            .parse::<f32>()
            .map_err(|e| OBJError::Parse(format!("parse_face: {e}")))?;
        let y: f32 = args[1]
            .parse::<f32>()
            .map_err(|e| OBJError::Parse(format!("parse_face: {e}")))?;

        Ok(STexture::new(x, y))
    }

    fn parse_face(args: &[&str]) -> Result<(Vec<SIndice>, Vec<SIndice>, Vec<SIndice>), OBJError> {
        if args.len() < 3 {
            return Err(OBJError::NotEnoughArg("parse_face".to_string()));
        }
        let mut v_indices = Vec::new();
        let mut t_indices = Vec::new();
        let mut n_indices = Vec::new();
        for v in args.iter() {
            let verticles: Vec<&str> = v.split('/').collect();
            let v_index = if !verticles[0].is_empty() {
                verticles[0]
                    .parse::<u32>()
                    .map_err(|e| OBJError::Parse(format!("parse_face: {e}")))?
                    - 1
            } else {
                0
            };
            let t_index = if verticles.len() > 1 && !verticles[1].is_empty() {
                verticles[1]
                    .parse::<u32>()
                    .map_err(|e| OBJError::Parse(format!("parse_face: {e}")))?
                    - 1
            } else {
                0
            };
            let n_index = if verticles.len() > 2 && !verticles[2].is_empty() {
                verticles[2]
                    .parse::<u32>()
                    .map_err(|e| OBJError::Parse(format!("parse_face: {e}")))?
                    - 1
            } else {
                0
            };
            v_indices.push(v_index);
            t_indices.push(t_index);
            n_indices.push(n_index);
        }

        let mut face_v_indices = Vec::<SIndice>::new();
        let mut face_t_indices = Vec::<SIndice>::new();
        let mut face_n_indices = Vec::<SIndice>::new();

        for i in 0..(v_indices.len() - 2) {
            face_v_indices.push(SIndice(
                v_indices[0],
                v_indices[(1 + i) % v_indices.len()],
                v_indices[(2 + i) % v_indices.len()],
            ));
            face_t_indices.push(SIndice(
                t_indices[0],
                t_indices[(1 + i) % t_indices.len()],
                t_indices[(2 + i) % t_indices.len()],
            ));
            face_n_indices.push(SIndice(
                n_indices[0],
                n_indices[(1 + i) % n_indices.len()],
                n_indices[(2 + i) % n_indices.len()],
            ));
        }
        // let face_v_indice = SIndice(v_indices[0], v_indices[1], v_indices[2]);
        // let face_t_indice = SIndice(t_indices[0], t_indices[1], t_indices[2]);
        // let face_n_indice = SIndice(n_indices[0], n_indices[1], n_indices[2]);

        Ok((face_v_indices, face_t_indices, face_n_indices))
    }
}
