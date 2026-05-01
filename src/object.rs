use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::vertex::{SIndice, SNormal, STexture, SVertex};

#[derive(Debug)]
pub struct SObject {
    verticles: Vec<SVertex>,
    uvs: Vec<STexture>,
    normals: Vec<SNormal>,
    verticle_faces: Vec<SIndice>,
    normal_faces: Vec<SIndice>,
    texture_face: Vec<SIndice>,
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
            verticle_faces: v_faces,
            texture_face: t_faces,
            normal_faces: n_faces,
        }
    }
}

impl SObject {
    pub fn get_vertex_face(&self) -> &Vec<SIndice> {
        &self.verticle_faces
    }

    pub fn get_normal_faces(&self) -> &Vec<SIndice> {
        &self.normal_faces
    }

    pub fn get_texture_face(&self) -> &Vec<SIndice> {
        &self.texture_face
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
    fn parse_vertex(args: &Vec<&str>) -> Result<SVertex, ()> {
        if (args.len() < 3 || args.len() > 4) {
            return Err(());
        }
        let x: f32 = args[0].parse::<f32>().map_err(|_| ())?;
        let y: f32 = args[1].parse::<f32>().map_err(|_| ())?;
        let z: f32 = args[2].parse::<f32>().map_err(|_| ())?;
        let w: f32 = if (args.len() == 4) {
            args[0].parse::<f32>().map_err(|_| ())?
        } else {
            1.
        };

        Ok(SVertex::new_xyzw(x, y, z, w))
    }

    fn parse_vertex_normal(args: &Vec<&str>) -> Result<SNormal, ()> {
        if (args.len() != 3) {
            return Err(());
        }
        let x: f32 = args[0].parse::<f32>().map_err(|_| ())?;
        let y: f32 = args[1].parse::<f32>().map_err(|_| ())?;
        let z: f32 = args[2].parse::<f32>().map_err(|_| ())?;

        Ok(SNormal::new(x, y, z))
    }

    fn parse_vertex_texture(args: &Vec<&str>) -> Result<STexture, ()> {
        if (args.len() < 3 || args.len() > 4) {
            return Err(());
        }
        let x: f32 = args[0].parse::<f32>().map_err(|_| ())?;
        let y: f32 = args[1].parse::<f32>().map_err(|_| ())?;

        Ok(STexture::new(x, y))
    }

    fn parse_face(args: &Vec<&str>) -> Result<(Vec<SIndice>, Vec<SIndice>, Vec<SIndice>), ()> {
        if args.len() < 3 {
            return Err(());
        }
        let mut v_indices = Vec::new();
        let mut t_indices = Vec::new();
        let mut n_indices = Vec::new();
        for v in args.iter() {
            let v = v;
            let verticles: Vec<&str> = v.split('/').collect();
            let v_index = verticles[0].parse::<u32>().map_err(|_| ())?;
            let t_index = verticles[0].parse::<u32>().map_err(|_| ())?;
            let n_index = verticles[0].parse::<u32>().map_err(|_| ())?;
            v_indices.push(v_index);
            t_indices.push(t_index);
            n_indices.push(n_index);
        }
        let face_v_indices = SIndice(v_indices[0], v_indices[1], v_indices[2]);
        let face_t_indices = SIndice(t_indices[0], t_indices[1], t_indices[2]);
        let face_n_indices = SIndice(n_indices[0], n_indices[1], n_indices[2]);

        Ok((
            vec![face_v_indices],
            vec![face_t_indices],
            vec![face_n_indices],
        ))
    }

    pub fn load(&self, obj_file: &str) -> Result<SObject, OBJError> {
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
                            .map_err(|_| OBJError::Vertex("lol".to_string()))?;
                        verticles.push(vertex);
                    }
                    "vf" => {
                        let normal = Self::parse_vertex_normal(&args_it.collect())
                            .map_err(|_| OBJError::Vertex("lol".to_string()))?;
                        normals.push(normal);
                    }
                    "vn" => {
                        let texture = Self::parse_vertex_texture(&args_it.collect())
                            .map_err(|_| OBJError::Vertex("lol".to_string()))?;
                        uvs.push(texture);
                    }
                    "f" => {
                        let mut face = Self::parse_face(&args_it.collect())
                            .map_err(|_| OBJError::Face("lol".to_string()))?;
                        v_faces.append(&mut face.0);
                        t_faces.append(&mut face.1);
                        n_faces.append(&mut face.2);
                    }

                    _ => {}
                }
            }
        }
        Ok(SObject::new(
            verticles, uvs, normals, v_faces, t_faces, n_faces,
        ))
    }
}
