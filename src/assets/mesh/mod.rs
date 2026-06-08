use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
    vec,
};

mod vertex;
use sdl2::libc::sched_param;
pub use vertex::*;

#[derive(Debug, Default)]
pub struct Mesh {
    pub verticles: Vec<Vertex>,
    pub indices: Vec<u32>,
}

#[derive(Debug, Default)]
pub struct Model {
    pub name: String,
    pub mesh: Mesh,
}

impl Model {
    /// Create a new model, associating a name with a [`Mesh`].
    pub fn new(mesh: Mesh, name: String) -> Model {
        Model { mesh, name }
    }
}

#[derive(Default)]
struct OBJLoadContext {
    models: Vec<Model>,
    pos: Vec<f32>,
    texcoord: Vec<f32>,
    colors: Vec<f32>,
    normal: Vec<f32>,
    faces: Vec<Face>,
    name: String,
}

impl OBJLoadContext {
    fn apply_model(&mut self) -> Result<(), LoadError> {
        let mesh = create_mesh(
            &self.faces,
            &self.pos,
            &self.normal,
            &self.texcoord,
            &self.colors,
        )?;
        let name = self.name.clone();
        self.models.push(Model::new(mesh, name));
        self.faces.clear();
        Ok(())
    }
}

fn create_mesh(
    faces: &[Face],
    positions: &[f32],
    normals: &[f32],
    textures: &[f32],
    colors: &[f32],
) -> Result<Mesh, LoadError> {
    let triangles =
        faces.iter().try_fold(
            Vec::<FaceIndex>::new(),
            |mut acc, f| match triangulate_face(f) {
                Ok(mut trs) => {
                    acc.append(&mut trs);
                    Ok(acc)
                }
                Err(e) => Err(e),
            },
        )?;
    let mut indices = vec![];
    let vertex =
        triangles
            .iter()
            .enumerate()
            .try_fold(
                Vec::<Vertex>::new(),
                |mut acc, (i, f)| match index_to_vertex(f, positions, normals, textures, colors) {
                    Ok(vertex) => {
                        indices.push(i as u32);
                        acc.push(vertex);
                        Ok(acc)
                    }
                    Err(e) => Err(e),
                },
            )?;
    Ok(Mesh {
        verticles: vertex,
        indices,
    })
}

fn index_to_vertex(
    index: &FaceIndex,
    positions: &[f32],
    normals: &[f32],
    textures: &[f32],
    colors: &[f32],
) -> Result<Vertex, LoadError> {
    let vi = (index.v_i as usize) * 3;
    let Some(position) = positions.get((vi)..(vi + 3)) else {
        return Err(LoadError::FaceVertexOutOfBounds);
    };
    let position = Position {
        x: position[0],
        y: position[1],
        z: position[2],
        w: 1.,
    };

    let vni = (index.vn_i as usize) * 3;
    let Some(normal) = normals.get((vni)..(vni + 3)) else {
        return Err(LoadError::FaceNormalOutOfBounds);
    };
    let normal = Normal {
        x: normal[0],
        y: normal[1],
        z: normal[2],
    };
    let vti = (index.vt_i as usize) * 2;
    let Some(texture) = textures.get((vti)..(vti + 2)) else {
        return Err(LoadError::FaceTexCoordOutOfBounds);
    };
    let texture = Texture {
        x: texture[0],
        y: texture[1],
    };

    let vci = (index.vc_i as usize) * 3;
    let Some(color) = colors.get((vci)..(vci + 3)) else {
        return Err(LoadError::FaceTexCoordOutOfBounds);
    };
    let color = Color {
        r: color[0],
        g: color[1],
        b: color[1],
        a: 1.,
    };

    Ok(Vertex {
        position,
        color,
        normal,
        texture,
    })
}

fn triangulate_face(face: &Face) -> Result<Vec<FaceIndex>, LoadError> {
    let mut triangle: Vec<FaceIndex> = vec![];
    let origin = face.indices.first().ok_or(LoadError::GenericFailure)?;
    for (&b, &c) in face.indices.iter().skip(1).zip(face.indices.iter().skip(2)) {
        triangle.push(*origin);
        triangle.push(b);
        triangle.push(c);
    }
    Ok(triangle)
}

#[derive(Debug, Default, Clone, Copy)]
struct FaceIndex {
    pub v_i: u32,
    pub vt_i: u32,
    pub vn_i: u32,
    pub vc_i: u32,
}

#[derive(Debug, Default)]
struct Face {
    pub indices: Vec<FaceIndex>,
}

pub fn load_file(file: &str) -> Result<Vec<Model>, LoadError> {
    let mut context = OBJLoadContext {
        name: file.to_string(),
        ..Default::default()
    };
    context.pos.append(&mut vec![0., 0., 0.]);
    context.normal.append(&mut vec![0., 0., 0.]);
    context.texcoord.append(&mut vec![0., 0.]);
    context.colors.append(&mut vec![0., 0., 0.]);
    let file = File::open(file).map_err(|_| LoadError::OpenFileFailed)?;
    let read_buffer = BufReader::new(file);
    for line in read_buffer.lines() {
        let line = line.map_err(|_| LoadError::ReadError)?;
        parse_line(&mut context, line)?;
    }
    context.apply_model()?;

    Ok(context.models)
}

fn parse_line(ctx: &mut OBJLoadContext, line: String) -> Result<(), LoadError> {
    let (cmd, args) = {
        let mut words = line.split_ascii_whitespace();
        (words.next(), words.collect::<Vec<&str>>())
    };

    match cmd {
        Some("v") if !parse_value::<3>(&mut ctx.pos, &args) => {
            return Err(LoadError::PositionParseError);
        }

        Some("vn") if !parse_value::<3>(&mut ctx.normal, &args) => {
            return Err(LoadError::PositionParseError);
        }

        Some("vt") if !parse_value::<2>(&mut ctx.texcoord, &args) => {
            return Err(LoadError::PositionParseError);
        }

        Some("f") => {
            if let Some(mut f) = parse_face(
                &args,
                [
                    ctx.pos.len() as isize,
                    ctx.normal.len() as isize,
                    ctx.texcoord.len() as isize,
                ],
            ) {
                f.indices
                    .iter_mut()
                    .for_each(|f| f.vc_i = ctx.colors.len() as u32 / 3);
                ctx.faces.push(f);
                let gray = rand::random::<f32>() % 1.;
                ctx.colors.push(gray);
                ctx.colors.push(gray);
                ctx.colors.push(gray);
            } else {
                return Err(LoadError::FaceParseError);
            }
        }

        // Some("o") => {
        //     ctx.apply_model()?;
        //     let Some(name) = args.first() else {
        //         return Err(LoadError::InvalidObjectName);
        //     };
        //     ctx.name = name.to_string();
        // }
        Some("#") => {} // comment
        _ => {}         // anything,
    }
    Ok(())
}

fn parse_value<const N: usize>(dest: &mut Vec<f32>, args: &[&str]) -> bool {
    if args.len() != N {
        return false;
    }
    for i in args.iter().take(N) {
        match i.parse::<f32>() {
            Ok(i) => dest.push(i),
            Err(_) => return false,
        }
    }
    true
}

fn parse_face(args: &[&str], max_index: [isize; 3]) -> Option<Face> {
    if args.len() < 3 {
        return None;
    }
    let mut face: Face = Default::default();
    for arg in args {
        let Some(vertex) = parse_vertex_indice(arg, max_index) else {
            return None;
        };
        face.indices.push(vertex);
    }
    Some(face)
}

fn parse_vertex_indice(arg: &str, max_index: [isize; 3]) -> Option<FaceIndex> {
    let mut vertex_indice: [isize; 3] = [0; _];

    for (i, opt) in arg.split("/").enumerate() {
        if opt.is_empty() {
            continue;
        };
        let Ok(value) = opt.parse::<isize>() else {
            return None; // not a number
        };
        match i {
            0..3 => {
                vertex_indice[i] = if value < 0 {
                    max_index[i] + value
                } else {
                    value
                };
            }
            _ => return None, // more than 3 opt
        };
    }

    Some(FaceIndex {
        v_i: vertex_indice[0] as u32,
        vt_i: vertex_indice[1] as u32,
        vn_i: vertex_indice[2] as u32,
        vc_i: 0,
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadError {
    OpenFileFailed,
    ReadError,
    UnrecognizedCharacter,
    PositionParseError,
    NormalParseError,
    TexcoordParseError,
    FaceParseError,
    InvalidObjectName,
    InvalidPolygon,
    FaceVertexOutOfBounds,
    FaceTexCoordOutOfBounds,
    FaceNormalOutOfBounds,
    FaceColorOutOfBounds,
    GenericFailure,
}

impl Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match *self {
            LoadError::OpenFileFailed => "fail to open file",
            LoadError::ReadError => "read error",
            LoadError::UnrecognizedCharacter => "unrecognized char",
            LoadError::PositionParseError => "position parse error",
            LoadError::NormalParseError => "normal parse error",
            LoadError::TexcoordParseError => "texture parse error",
            LoadError::FaceParseError => "face parse error",
            LoadError::InvalidObjectName => "invalid object name",
            LoadError::InvalidPolygon => "invalid polygone",
            LoadError::FaceVertexOutOfBounds => "vertex index out of bound",
            LoadError::FaceTexCoordOutOfBounds => "texture index out of bound",
            LoadError::FaceNormalOutOfBounds => "normal index out of bound",
            LoadError::FaceColorOutOfBounds => "color index out of bound",
            LoadError::GenericFailure => "some error but i dont know",
        };
        f.write_str(data)
    }
}
