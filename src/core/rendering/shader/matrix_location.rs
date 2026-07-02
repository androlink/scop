use super::{GetUniform, SetUniform, Shader};

impl SetUniform<&[[f32; 4]; 4]> for Shader {
    type Output = ();

    fn set_location(&self, name: &str, var: &[[f32; 4]; 4]) -> Self::Output {
        if let Some(loc) = self.get_location(name) {
            unsafe { gl::UniformMatrix4fv(loc, 1, gl::FALSE, var.as_ptr() as _) };
        };
    }
}

impl SetUniform<&[[f32; 3]; 3]> for Shader {
    type Output = ();

    fn set_location(&self, name: &str, var: &[[f32; 3]; 3]) -> Self::Output {
        if let Some(loc) = self.get_location(name) {
            unsafe { gl::UniformMatrix3fv(loc, 1, gl::FALSE, var.as_ptr() as _) };
        };
    }
}

impl SetUniform<&[[f32; 2]; 2]> for Shader {
    type Output = ();

    fn set_location(&self, name: &str, var: &[[f32; 2]; 2]) -> Self::Output {
        if let Some(loc) = self.get_location(name) {
            unsafe { gl::UniformMatrix2fv(loc, 1, gl::FALSE, var.as_ptr() as _) };
        };
    }
}
