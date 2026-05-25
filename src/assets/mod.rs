mod model;
mod texture;

use crate::{assets::texture::Texture, graphics::mesh::Mesh, platform::program::ShaderProgram};

use std::collections::HashMap;

struct Assets {
    shaders: HashMap<String, ShaderProgram>,
    meshs: HashMap<String, Mesh>,
    textures: HashMap<String, Texture>,
}

impl Assets {
    pub fn load_shaders(&mut self, shaders_name: &[&str]) -> Result<(), String> {
        let path = "assets/".to_string();

        for shader_name in shaders_name {
            let shader_file = path.clone() + shader_name;
            self.shaders.insert(
                shader_name.to_string(),
                ShaderProgram::init(
                    &(shader_file.clone() + ".vert"),
                    &(shader_file.clone() + ".frag"),
                )?,
            );
        }
        Ok(())
    }

    pub fn shader(&self, name: &str) -> Option<&ShaderProgram> {
        self.shaders.get(name)
    }

    pub fn load_models(&mut self, models_name: &[&str]) -> Result<(), String> {
        Ok(())
    }
}
