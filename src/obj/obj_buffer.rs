use super::*;

#[derive(Default, Debug, Clone)]
pub struct OBJBuffer {
    objects: Vec<OBJDescriptor>,
    verticles: Vec<SVertex>,
    textures: Vec<STexture>,
    normals: Vec<SNormal>,
    vertex_indices: Vec<SIndice>,
    normal_indices: Vec<SIndice>,
    texture_indices: Vec<SIndice>,
}

impl OBJBuffer {
    pub fn append(&mut self, other: &Self) -> &Self {
        // level 0
        let verticles_offset = self.verticles.len() as u32;
        let textures_offset = self.textures.len() as u32;
        let normals_offset = self.normals.len() as u32;

        self.verticles.extend(other.verticles());
        self.normals.extend(other.normals());
        self.textures.extend(other.textures());
        // level 1
        let indice_offset = self.vertex_indices.len() as i32;

        for v in other.vertex_indices() {
            self.vertex_indices.push(v.apply_offset(verticles_offset));
        }
        for n in other.normal_indices() {
            self.normal_indices.push(n.apply_offset(normals_offset));
        }
        for t in other.texture_indices() {
            self.texture_indices.push(t.apply_offset(textures_offset));
        }
        // level 2
        for o in other.objects() {
            self.objects.push(OBJDescriptor {
                name: o.name.clone(),
                start: o.start + indice_offset,
                size: o.size,
            });
        }

        self
    }
}

impl OBJBuffer {
    pub fn verticles(&self) -> &Vec<SVertex> {
        &self.verticles
    }
    pub fn normals(&self) -> &Vec<SNormal> {
        &self.normals
    }
    pub fn textures(&self) -> &Vec<STexture> {
        &self.textures
    }
    pub fn vertex_indices(&self) -> &Vec<SIndice> {
        &self.vertex_indices
    }
    pub fn normal_indices(&self) -> &Vec<SIndice> {
        &self.normal_indices
    }
    pub fn texture_indices(&self) -> &Vec<SIndice> {
        &self.texture_indices
    }
    pub fn objects(&self) -> &Vec<OBJDescriptor> {
        &self.objects
    }
}

impl OBJBuffer {
    pub fn verticles_mut(&mut self) -> &mut Vec<SVertex> {
        &mut self.verticles
    }
    pub fn normals_mut(&mut self) -> &mut Vec<SNormal> {
        &mut self.normals
    }
    pub fn textures_mut(&mut self) -> &mut Vec<STexture> {
        &mut self.textures
    }
    pub fn vertex_indices_mut(&mut self) -> &mut Vec<SIndice> {
        &mut self.vertex_indices
    }
    pub fn normal_indices_mut(&mut self) -> &mut Vec<SIndice> {
        &mut self.normal_indices
    }
    pub fn texture_indices_mut(&mut self) -> &mut Vec<SIndice> {
        &mut self.texture_indices
    }
    pub fn objects_mut(&mut self) -> &mut Vec<OBJDescriptor> {
        &mut self.objects
    }
}
