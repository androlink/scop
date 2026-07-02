use super::{GetUniform, SetUniform, Shader};

impl SetUniform<u32> for Shader {
    type Output = ();

    fn set_location(&self, name: &str, var: u32) -> Self::Output {
        if let Some(loc) = self.get_location(name) {
            unsafe { gl::Uniform1ui(loc, var) };
        };
    }
}

impl SetUniform<(u32, u32)> for Shader {
    type Output = ();

    fn set_location(&self, name: &str, var: (u32, u32)) -> Self::Output {
        if let Some(loc) = self.get_location(name) {
            unsafe { gl::Uniform2ui(loc, var.0, var.1) };
        };
    }
}

impl SetUniform<(u32, u32, u32)> for Shader {
    type Output = ();

    fn set_location(&self, name: &str, var: (u32, u32, u32)) -> Self::Output {
        if let Some(loc) = self.get_location(name) {
            unsafe { gl::Uniform3ui(loc, var.0, var.1, var.2) };
        };
    }
}
