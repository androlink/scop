use std::{collections::HashMap, default, sync::Arc, time::Instant};

use crate::{
    assets::{mesh::Mesh, texture::Texture},
    manager::Storage,
    platform::program::ShaderProgram,
};

#[derive(Default)]
pub struct Assets {
    shaders: Storage<String, ShaderProgram>,
    models: Storage<String, Mesh>,
    textures: Storage<String, Texture>,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn load_shaders(&mut self, shaders_name: &[&str]) -> Result<(), String> {
        for shader_name in shaders_name.iter() {
            self.load_shader(shader_name)?;
        }
        Ok(())
    }
    pub fn load_shader(&mut self, shader_name: &str) -> Result<(), String> {
        if !self.shaders.has(shader_name) {
            let shader_file = shader_name.to_string();
            self.shaders.insert(
                shader_name.to_string(),
                ShaderProgram::init(
                    &(shader_file.clone() + ".vert"),
                    &(shader_file.clone() + ".frag"),
                )?,
            );
        };
        Ok(())
    }

    pub fn shader(&self, name: &str) -> Option<&Arc<ShaderProgram>> {
        self.shaders.get(name)
    }

    pub fn load_models(&mut self, models_name: &[&str]) -> Result<(), String> {
        for model_name in models_name {
            let now = Instant::now();
            let m = super::mesh::load_obj(model_name).map_err(|e| e.to_string())?;
            for m in m {
                self.models.insert(m.name, m.mesh);
            }
            println!("{}s", now.elapsed().as_millis() as f32 / 1000.)
        }
        Ok(())
    }

    pub fn mesh(&self, name: &str) -> Option<&Arc<Mesh>> {
        self.models.get(name)
    }

    // pub fn load_textures(&mut self, textures_name: &[&str]) -> Result<(), String> {
    //     Ok(())
    // }

    // pub fn texture(&mut self, name: &str) -> Option<&Texture> {
    //     self.textures.get(name)
    // }
}
