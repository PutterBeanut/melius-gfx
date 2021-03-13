use std::ffi::CStr;

use crate::texture::Texture;
use crate::shader;

use cgmath::{Deg, Matrix4, perspective, vec3};
use cgmath::prelude::*;

pub enum AttributeType {
    Float1(*const f32),
    Float2(*const f32)
}

fn c_str(value: &str) -> *const i8 {
    let format = format!("{0}\0", value);
    let as_str = format.as_str();
    let as_bytes = as_str.as_bytes();
    let cstr = unsafe { CStr::from_bytes_with_nul_unchecked(as_bytes) }.as_ptr();

    cstr
}

pub struct Material {
    program: u32,
    pub textures: Vec<Texture>,
    attributes: Vec<>
}

impl Material {
    pub fn from_shaders(vertex_shader: String, fragment_shader: String, textures: Vec<Texture>) -> Self {
        Material {
            program: shader::compile_program(vertex_shader, fragment_shader),
            textures,
            attributes: vec![]
        }
    }

    pub fn use_material(&self, screen_aspect_ratio: f32) {
        unsafe {
            gl::UseProgram(self.program);

            let model: Matrix4<f32> = Matrix4::from_angle_x(Deg(-45.0));
            let view : Matrix4<f32> = Matrix4::from_translation(vec3(0.0, 0.0, -1.0));
            let projection: Matrix4<f32> = perspective(Deg(90.0), screen_aspect_ratio, 0.1, 100.0);
            
            let model_loc = gl::GetUniformLocation(self.program, c_str("model"));
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
            let view_loc = gl::GetUniformLocation(self.program, c_str("view"));
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);
            let projection_loc = gl::GetUniformLocation(self.program, c_str("projection"));
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection.as_ptr());

            let mut i = 0usize;
            for texture in self.textures.iter() {
                texture.use_texture(i);
                i += 1;
            }
        }
    }

    pub fn delete(&mut self) {
        unsafe {
            gl::DeleteShader(self.program);
        }
    }
}

#[allow(dead_code)]
impl Material {
    fn with_attribute(&mut self, n: &str, t: AttributeType) {
        unsafe {
            
        }
    }
}