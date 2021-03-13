use std::ffi::CString;
use std::{ptr, str};

pub enum ShaderType {
    Vertex,
    Fragment,
}

fn compile_shader(shader_type: ShaderType, source: String) -> u32 {
    let shader = unsafe {
        let shader_type = match shader_type {
            ShaderType::Vertex => {
                gl::VERTEX_SHADER
            },
            ShaderType::Fragment => {
                gl::FRAGMENT_SHADER
            }
        };

        let shader = gl::CreateShader(shader_type);

        let source = CString::new(source.as_str().as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &source.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut success: i32 = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success != (gl::TRUE as i32) {
            let mut len: i32 = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            let mut info_log = Vec::with_capacity(len as usize);
            info_log.set_len((len as usize) - 1);

            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut i8
            );

            panic!(
                "{}",
                str::from_utf8(&info_log)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }

        shader
    };

    shader
}

pub fn compile_program(vertex_shader: String, fragment_shader: String) -> u32 {
    let program = unsafe {
        let program = gl::CreateProgram();

        gl::AttachShader(
            program,
            compile_shader(
                ShaderType::Vertex,
                vertex_shader
            )
        );
        gl::AttachShader(
            program,
            compile_shader(
                ShaderType::Fragment,
                fragment_shader
            )
        );
        gl::LinkProgram(program);

        let mut success: i32 = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        if success != (gl::TRUE as i32) {
            let mut len: i32 = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

            let mut info_log = Vec::with_capacity(len as usize);
            info_log.set_len((len as usize) - 1);

            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut i8
            );

            panic!(
                "{}",
                str::from_utf8(&info_log)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }

        program
    };

    program
}