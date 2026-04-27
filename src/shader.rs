use std::{fs::File, io::Read};

pub fn load_shader(shader_type: gl::types::GLenum, shader_str: &[u8]) -> Result<u32, ()> {
    let shader = match unsafe { gl::CreateShader(shader_type) } {
        0 => return Err(()),
        n => n,
    };
    unsafe {
        gl::ShaderSource(
            shader,
            1,
            &(shader_str.as_ptr().cast()),
            &(shader_str.len().try_into().unwrap()),
        );
    }
    unsafe { gl::CompileShader(shader) };
    unsafe {
        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        }
    }
    Ok(shader)
}

pub fn load_shader_file(shader_type: gl::types::GLenum, shader_path: &str) -> Result<u32, ()> {
    let mut file = File::open(shader_path).expect("no file ?");
    let mut file_content = vec![];
    file.read_to_end(&mut file_content).expect("no content ?");
    load_shader(shader_type, &file_content)
}
