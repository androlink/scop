use std::{
    fs::File,
    io::{BufRead, BufReader},
    mem,
};

use rand::random;
use sdl2::libc::rand;

use crate::core::{
    model::{
        obj_model::{
            OBJLoader,
            load::OBJLoadError,
            types::{Face, FaceVertex, OBJMesh, OBJModel},
        },
        types::{VertexColor, VertexCoord, VertexNormal, VertexTexture},
    },
    traits::loader::Loader,
};

#[derive(Default, Debug)]
struct ParseContext {
    object_name: String,
    groupe_name: String,
    vertex_list: Vec<VertexCoord>,
    texture_list: Vec<VertexTexture>,
    normal_list: Vec<VertexNormal>,
    color_list: Vec<VertexColor>,
    face_list: Vec<Face>,
    meshes: Vec<Vec<Face>>,
}

impl ParseContext {
    fn new() -> Self {
        Self {
            vertex_list: vec![VertexCoord::default()],
            texture_list: vec![VertexTexture::default()],
            normal_list: vec![VertexNormal::default()],
            color_list: vec![[1., 1., 1., 1.].into()],
            ..Default::default()
        }
    }
}

impl Loader<BufReader<File>> for OBJLoader {
    type Output = Result<OBJModel, OBJLoadError>;

    fn load(&self, source: BufReader<File>) -> Self::Output {
        let mut ctx = ParseContext::new();
        for line in source.lines() {
            let line = line.map_err(OBJLoadError::Io)?;
            ctx.parse_line(line)?;
        }
        ctx.apply_mesh();
        Ok(OBJModel {
            verticles: ctx.vertex_list,
            normals: ctx.normal_list,
            textures: ctx.texture_list,
            colors: ctx.color_list,
            meshes: ctx
                .meshes
                .iter()
                .map(|f| OBJMesh {
                    face: f.clone(),
                    material: None,
                })
                .collect(),
        })
    }
}

impl ParseContext {
    fn apply_mesh(&mut self) {
        if !self.face_list.is_empty() {
            self.meshes.push(mem::take(&mut self.face_list));
        }
    }

    fn parse_line(&mut self, line: String) -> Result<(), OBJLoadError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.as_slice() {
            ["v", rest @ ..] => self.parse_vertex(rest)?,
            ["vt", rest @ ..] => self.parse_vertex_texture(rest)?,
            ["vn", rest @ ..] => self.parse_vertex_normal(rest)?,
            ["vp", _rest @ ..] => {}
            ["f", rest @ ..] => self.parse_face(rest)?,
            ["l", _rest @ ..] => {}
            ["p", _rest @ ..] => {}
            ["o", _name] => {}
            ["g", _name] => {}
            ["s", _name] => {}
            ["usemtl", _name] => {}
            ["mtllib", _file] => {}
            ["#", ..] | [] => {} // comment/blank
            [] => {}
            _ => {}
        }
        Ok(())
    }

    fn parse_vertex(&mut self, args: &[&str]) -> Result<(), OBJLoadError> {
        let values = args
            .iter()
            .map(|v| v.parse::<f32>())
            .collect::<Result<Vec<f32>, _>>()
            .map_err(|_| OBJLoadError::InvalidFloat)?;
        let vertex = match values.as_slice() {
            [x, y, z] => VertexCoord::from([*x, *y, *z, 1.]),
            [x, y, z, w] => VertexCoord::from([*x, *y, *z, *w]),
            _ => {
                return Err(OBJLoadError::MissingField);
            }
        };
        self.vertex_list.push(vertex);
        Ok(())
    }

    fn parse_vertex_texture(&mut self, args: &[&str]) -> Result<(), OBJLoadError> {
        let values = args
            .iter()
            .map(|v| v.parse::<f32>())
            .collect::<Result<Vec<f32>, _>>()
            .map_err(|_| OBJLoadError::InvalidFloat)?;
        let vertex = match values.as_slice() {
            [u] => VertexTexture::from([*u, 0., 0.]),
            [u, v] => VertexTexture::from([*u, *v, 0.]),
            [u, v, w] => VertexTexture::from([*u, *v, *w]),
            _ => {
                return Err(OBJLoadError::MissingField);
            }
        };

        self.texture_list.push(vertex);
        Ok(())
    }

    fn parse_vertex_normal(&mut self, args: &[&str]) -> Result<(), OBJLoadError> {
        let values = args
            .iter()
            .map(|v| v.parse::<f32>())
            .collect::<Result<Vec<f32>, _>>()
            .map_err(|_| OBJLoadError::InvalidFloat)?;
        let vertex = match values.as_slice() {
            [x, y, z] => VertexNormal::from([*x, *y, *z]),
            _ => {
                return Err(OBJLoadError::MissingField);
            }
        };
        self.normal_list.push(vertex);
        Ok(())
    }

    fn parse_face(&mut self, args: &[&str]) -> Result<(), OBJLoadError> {
        let gray = rand::random::<f32>() % 1.;
        self.color_list.push([gray, gray, gray, 1.].into());
        let parse_face_vertex = |arg: &str| {
            let values = arg
                .split("/")
                .map(|v| {
                    if v.is_empty() {
                        Ok(0)
                    } else {
                        v.parse::<i32>()
                    }
                })
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| OBJLoadError::InvalidInteger)?;
            let values = match values.as_slice() {
                [v] => Ok([*v, 0, 0]),
                [v, t] => Ok([*v, *t, 0]),
                [v, t, n] => Ok([*v, *t, *n]),
                _ => Err(OBJLoadError::InvalidFace),
            }?;
            let [v, t, n] = values;
            let v = if v < 0 {
                v + self.vertex_list.len() as i32
            } else {
                v
            };
            let t = if t < 0 {
                t + self.texture_list.len() as i32
            } else {
                t
            };
            let n = if n < 0 {
                n + self.normal_list.len() as i32
            } else {
                n
            };
            Ok(FaceVertex::new(v, t, n, self.color_list.len() as i32 - 1))
        };

        fn triangulate_face(face: &[FaceVertex]) -> Result<Vec<Face>, OBJLoadError> {
            if face.len() < 3 {
                return Err(OBJLoadError::InvalidFace);
            }
            let mut triangle: Vec<Face> = vec![];
            let origin = face[0];
            for (&b, &c) in face.iter().skip(1).zip(face.iter().skip(2)) {
                triangle.push(Face {
                    verticies: [origin, b, c],
                });
            }
            Ok(triangle)
        }
        let face = args
            .iter()
            .map(|arg| parse_face_vertex(arg))
            .collect::<Result<Vec<_>, _>>()?;
        self.face_list.append(&mut triangulate_face(&face)?);
        Ok(())
    }
}
