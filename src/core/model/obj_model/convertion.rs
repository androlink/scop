use std::any::Any;

use crate::core::model::{
    obj_model::{
        load::OBJLoadError,
        types::{Face, OBJMesh, OBJModel},
    },
    types::{Mesh, Model, Vertex},
};

impl TryInto<Model> for OBJModel {
    fn try_into(self) -> Result<Model, Self::Error> {
        let mut meshes = vec![];
        for mesh in self.meshes.as_slice() {
            let vertex_face = mesh
                .face
                .iter()
                .map(|v| {
                    if let [Some(a), Some(b), Some(c)] = v.verticies.map(|v| {
                        Some(Vertex {
                            position: *self.verticles.get(v.position as usize)?,
                            texture: *self.textures.get(v.texture as usize)?,
                            normal: *self.normals.get(v.normal as usize)?,
                            color: *self.colors.get(v.color as usize)?,
                        })
                    }) {
                        Ok([a, b, c])
                    } else {
                        Err(OBJLoadError::InvalidFace)
                    }
                })
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.to_string())?;
            meshes.push(Mesh {
                vertices: vertex_face.into_iter().flatten().collect(),
                indices: (0..(mesh.face.len() as i32 * 3)).collect(),
            });
        }
        Ok(Model { meshes })
    }

    type Error = String;
}
