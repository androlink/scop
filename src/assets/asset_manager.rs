use crate::{
    assets::{scop_obj::Mesh, texture::Texture},
    platform::program::ShaderProgram,
};

use std::collections::HashMap;

#[derive(Default)]
pub struct Assets {
    assets_path: String,
    shader_path: String,
    model_path: String,
    texture_path: String,

    shaders: HashMap<String, ShaderProgram>,
    meshs: HashMap<String, Mesh>,
    textures: HashMap<String, Texture>,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn load_shaders(&mut self, shaders_name: &[&str]) -> Result<(), String> {
        for shader_name in shaders_name {
            let shader_file = shader_name.to_string();
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

    pub fn load_models(&mut self, models_name: &[&str]) -> Result<(), String> {
        for model_name in models_name {
            let m = super::scop_obj::load_file(model_name).map_err(|e| e.to_string())?;
            for m in m {
                self.meshs.insert(m.name, m.mesh);
            }
        }
        Ok(())
    }

    pub fn load_textures(&mut self, textures_name: &[&str]) -> Result<(), String> {
        Ok(())
    }

    pub fn texture(&mut self, name: &str) -> Option<&Texture> {
        self.textures.get(name)
    }

    pub fn shader(&mut self, name: &str) -> Option<&ShaderProgram> {
        self.shaders.get(name)
    }

    pub fn mesh(&mut self, name: &str) -> Option<&Mesh> {
        self.meshs.get(name)
    }
}
