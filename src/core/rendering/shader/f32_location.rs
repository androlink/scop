use super::{GetUniform, SetUniform, Shader};

impl SetUniform<f32> for Shader {
    type Output = ();

    fn set_location(&self, name: &str, var: f32) -> Self::Output {
        if let Some(loc) = self.get_location(name) {
            unsafe { gl::Uniform1f(loc, var) };
        };
    }
}

impl SetUniform<(f32, f32)> for Shader {
    type Output = ();

    fn set_location(&self, name: &str, var: (f32, f32)) -> Self::Output {
        if let Some(loc) = self.get_location(name) {
            unsafe { gl::Uniform2f(loc, var.0, var.1) };
        };
    }
}

impl SetUniform<(f32, f32, f32)> for Shader {
    type Output = ();

    fn set_location(&self, name: &str, var: (f32, f32, f32)) -> Self::Output {
        if let Some(loc) = self.get_location(name) {
            unsafe { gl::Uniform3f(loc, var.0, var.1, var.2) };
        };
    }
}
