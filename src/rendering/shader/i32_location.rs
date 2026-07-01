use super::{GetUniform, SetUniform, Shader};

impl SetUniform<i32> for Shader {
    type Output = ();

    fn set_location(&self, name: &str, var: i32) -> Self::Output {
        if let Some(loc) = self.get_location(name) {
            unsafe { gl::Uniform1i(loc, var) };
        };
    }
}

impl SetUniform<(i32, i32)> for Shader {
    type Output = ();

    fn set_location(&self, name: &str, var: (i32, i32)) -> Self::Output {
        if let Some(loc) = self.get_location(name) {
            unsafe { gl::Uniform2i(loc, var.0, var.1) };
        };
    }
}

impl SetUniform<(i32, i32, i32)> for Shader {
    type Output = ();

    fn set_location(&self, name: &str, var: (i32, i32, i32)) -> Self::Output {
        if let Some(loc) = self.get_location(name) {
            unsafe { gl::Uniform3i(loc, var.0, var.1, var.2) };
        };
    }
}
