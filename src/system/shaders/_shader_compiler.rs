pub fn compile_shader(src: &str, ty: gl::types::GLenum) -> gl::types::GLuint {
    unsafe {
        let shader = gl::CreateShader(ty);
        gl::ShaderSource(shader, 1, &src.as_bytes().as_ptr().cast(), &src.len().try_into().unwrap());
        gl::CompileShader(shader);

        let mut success = 0;

        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success ==  0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            
            gl::GetShaderInfoLog(shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            
            panic!("Shader Compile Error: {}", String::from_utf8_lossy(&v));
        }
        shader
    }
}
