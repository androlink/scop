use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    time::Instant,
};

use crate::vertex::{SIndice, SNormal, STexture, SVertex};

#[derive(Debug)]
pub struct SObject {
    verticles: Vec<SVertex>,
    uvs: Vec<STexture>,
    normals: Vec<SNormal>,
    vertex_indices: Vec<SIndice>,
    normal_indices: Vec<SIndice>,
    texture_indices: Vec<SIndice>,
}

impl SObject {
    pub fn new(
        verticles: Vec<SVertex>,
        uvs: Vec<STexture>,
        normals: Vec<SNormal>,
        v_faces: Vec<SIndice>,
        t_faces: Vec<SIndice>,
        n_faces: Vec<SIndice>,
    ) -> SObject {
        SObject {
            verticles,
            uvs,
            normals,
            vertex_indices: v_faces,
            texture_indices: t_faces,
            normal_indices: n_faces,
        }
    }
}

impl SObject {
    pub fn get_vertex_indices(&self) -> &Vec<SIndice> {
        &self.vertex_indices
    }

    pub fn get_normal_indices(&self) -> &Vec<SIndice> {
        &self.normal_indices
    }

    pub fn get_texture_indices(&self) -> &Vec<SIndice> {
        &self.texture_indices
    }

    pub fn get_verticles(&self) -> &Vec<SVertex> {
        &self.verticles
    }

    pub fn get_normals(&self) -> &Vec<SNormal> {
        &self.normals
    }
}

pub struct OBJLoader {
    path: String,
}

#[derive(Debug)]
pub enum OBJError {
    Io(std::io::Error, String),
    Vertex(String),
    Face(String),
    Parse(String),
    NotEnoughtArg(String),
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
    fn parse_vertex(args: &Vec<&str>) -> Result<SVertex, OBJError> {
        if (args.len() < 3 || args.len() > 4) {
            return Err(OBJError::NotEnoughtArg("parse_vertex".to_string()));
        }
        let x: f32 = args[0]
            .parse::<f32>()
            .map_err(|e| OBJError::Parse(format!("parse_vertex: {e}").to_string()))?;
        let y: f32 = args[1]
            .parse::<f32>()
            .map_err(|e| OBJError::Parse(format!("parse_vertex: {e}").to_string()))?;
        let z: f32 = args[2]
            .parse::<f32>()
            .map_err(|e| OBJError::Parse(format!("parse_vertex: {e}").to_string()))?;
        let w: f32 = if (args.len() == 4) {
            args[4]
                .parse::<f32>()
                .map_err(|e| OBJError::Parse(format!("parse_vertex: {e}").to_string()))?
        } else {
            1.
        };

        Ok(SVertex::new_xyzw(x, y, z, w))
    }

    fn parse_vertex_normal(args: &Vec<&str>) -> Result<SNormal, OBJError> {
        if args.len() != 3 {
            return Err(OBJError::NotEnoughtArg("parse_vertex_normal".to_string()));
        }
        let x: f32 = args[0]
            .parse::<f32>()
            .map_err(|e| OBJError::Parse(format!("parse_vertex_normal: {e}").to_string()))?;
        let y: f32 = args[1]
            .parse::<f32>()
            .map_err(|e| OBJError::Parse(format!("parse_vertex_normal: {e}").to_string()))?;
        let z: f32 = args[2]
            .parse::<f32>()
            .map_err(|e| OBJError::Parse(format!("parse_vertex_normal: {e}").to_string()))?;

        Ok(SNormal::new(x, y, z))
    }

    fn parse_vertex_texture(args: &Vec<&str>) -> Result<STexture, OBJError> {
        if args.len() < 2 {
            return Err(OBJError::NotEnoughtArg("parse_vertex_texture".to_string()));
        }
        let x: f32 = args[0]
            .parse::<f32>()
            .map_err(|e| OBJError::Parse(format!("parse_face: {e}").to_string()))?;
        let y: f32 = args[1]
            .parse::<f32>()
            .map_err(|e| OBJError::Parse(format!("parse_face: {e}").to_string()))?;

        Ok(STexture::new(x, y))
    }

    fn parse_face(
        args: &Vec<&str>,
    ) -> Result<(Vec<SIndice>, Vec<SIndice>, Vec<SIndice>), OBJError> {
        if args.len() < 3 {
            return Err(OBJError::NotEnoughtArg("parse_face".to_string()));
        }
        let mut v_indices = Vec::new();
        let mut t_indices = Vec::new();
        let mut n_indices = Vec::new();
        for v in args.iter() {
            let v = v;
            let verticles: Vec<&str> = v.split('/').collect();
            let v_index = if verticles[0].len() > 0 {
                verticles[0]
                    .parse::<u32>()
                    .map_err(|e| OBJError::Parse(format!("parse_face: {e}").to_string()))?
                    - 1
            } else {
                0
            };
            let t_index = if verticles.len() > 1 && verticles[1].len() > 0 {
                verticles[1]
                    .parse::<u32>()
                    .map_err(|e| OBJError::Parse(format!("parse_face: {e}").to_string()))?
                    - 1
            } else {
                0
            };
            let n_index = if verticles.len() > 2 && verticles[2].len() > 0 {
                verticles[2]
                    .parse::<u32>()
                    .map_err(|e| OBJError::Parse(format!("parse_face: {e}").to_string()))?
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

    pub fn load(&self, obj_file: &str) -> Result<SObject, OBJError> {
        let start = Instant::now();
        let mut verticles: Vec<SVertex> = Vec::new();
        let mut uvs: Vec<STexture> = Vec::new();
        let mut normals: Vec<SNormal> = Vec::new();
        let mut v_faces: Vec<SIndice> = Vec::new();
        let mut t_faces: Vec<SIndice> = Vec::new();
        let mut n_faces: Vec<SIndice> = Vec::new();
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
                            .map_err(|e| OBJError::Vertex(format!("load: {e:?}").to_string()))?;
                        verticles.push(vertex);
                    }
                    "vn" => {
                        let normal = Self::parse_vertex_normal(&args_it.collect())
                            .map_err(|e| OBJError::Vertex(format!("load: {e:?}").to_string()))?;
                        normals.push(normal);
                    }
                    "vt" => {
                        let texture = Self::parse_vertex_texture(&args_it.collect())
                            .map_err(|e| OBJError::Vertex(format!("load: {e:?}").to_string()))?;
                        uvs.push(texture);
                    }
                    "f" => {
                        let mut face = Self::parse_face(&args_it.collect())
                            .map_err(|e| OBJError::Vertex(format!("load: {e:?}").to_string()))?;
                        v_faces.append(&mut face.0);
                        t_faces.append(&mut face.1);
                        n_faces.append(&mut face.2);
                    }

                    _ => {}
                }
            }
        }
        let stop = start.elapsed().as_millis();
        println!("obj load in {} seconde", stop as f32 / 1000.);
        Ok(SObject::new(
            verticles, uvs, normals, v_faces, t_faces, n_faces,
        ))
    }
}
