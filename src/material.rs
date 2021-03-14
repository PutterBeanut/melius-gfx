use std::ffi::CStr;

use crate::texture::Texture;
use crate::shader;

#[derive(Copy, Clone)]
pub enum AttributeType {
    Float1(f32),
    Float2(f32, f32),
    Float3(f32, f32, f32),
    Float4(f32, f32, f32, f32),
    Int1(i32),
    Int2(i32, i32),
    Int3(i32, i32, i32),
    Int4(i32, i32, i32, i32),
    UInt1(u32),
    UInt2(u32, u32),
    UInt3(u32, u32, u32),
    UInt4(u32, u32, u32, u32),
    VecFloat1(i32, *const f32),
    VecFloat2(i32, *const f32),
    VecFloat3(i32, *const f32),
    VecFloat4(i32, *const f32),
    VecInt1(i32, *const i32),
    VecInt2(i32, *const i32),
    VecInt3(i32, *const i32),
    VecInt4(i32, *const i32),
    VecUInt1(i32, *const u32),
    VecUInt2(i32, *const u32),
    VecUInt3(i32, *const u32),
    VecUInt4(i32, *const u32),
    Matrix2(*const f32),
    Matrix3(*const f32),
    Matrix4(*const f32),
    Matrix2x3(*const f32),
    Matrix3x2(*const f32),
    Matrix2x4(*const f32),
    Matrix4x2(*const f32),
    Matrix3x4(*const f32),
    Matrix4x3(*const f32),
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
    attributes: Vec<(String, AttributeType)>
}

impl Material {
    pub fn from_shaders(vertex_shader: String, fragment_shader: String, textures: Vec<Texture>, attributes: Vec<(&str, AttributeType)>) -> Self {
        Material {
            program: shader::compile_program(vertex_shader, fragment_shader),
            attributes: attributes.into_iter().map(|(n, t)| (n.to_string(), t)).collect::<Vec<_>>(),
            textures,
        }
    }

    pub fn use_material(&mut self) {
        unsafe {
            gl::UseProgram(self.program);

            let mut i = 0usize;
            for texture in self.textures.iter() {
                texture.use_texture(i);
                i += 1;
            }

            for (n, t) in self.attributes.iter() {
                set_attribute(self.program, (*n).clone(), (*t).clone());
            }
        }
    }

    pub fn get_program_id(&self) -> u32 {
        self.program.clone()
    }

    pub fn delete(&mut self) {
        unsafe {
            gl::DeleteShader(self.program);
        }
    }
}

pub fn set_attribute(p: u32, n: String, t: AttributeType) {
    unsafe {
        let attrib_loc = gl::GetUniformLocation(p, c_str(n.as_str()));
        match t {
            AttributeType::Float1(a) => { gl::Uniform1f(attrib_loc, a) }
            AttributeType::Float2(a, b) => { gl::Uniform2f(attrib_loc, a, b) }
            AttributeType::Float3(a, b, c) => { gl::Uniform3f(attrib_loc, a, b, c) }
            AttributeType::Float4(a, b, c, d) => { gl::Uniform4f(attrib_loc, a, b, c, d) }
            AttributeType::Int1(a) => { gl::Uniform1i(attrib_loc, a) }
            AttributeType::Int2(a, b) => { gl::Uniform2i(attrib_loc, a, b) }
            AttributeType::Int3(a, b, c) => { gl::Uniform3i(attrib_loc, a, b, c) }
            AttributeType::Int4(a, b, c, d) => { gl::Uniform4i(attrib_loc, a, b, c, d) }
            AttributeType::UInt1(a) => { gl::Uniform1ui(attrib_loc, a) }
            AttributeType::UInt2(a, b) => { gl::Uniform2ui(attrib_loc, a, b) }
            AttributeType::UInt3(a, b, c) => { gl::Uniform3ui(attrib_loc, a, b, c) }
            AttributeType::UInt4(a, b, c, d) => { gl::Uniform4ui(attrib_loc, a, b, c, d) }
            AttributeType::VecFloat1(a, b) => { gl::Uniform1fv(attrib_loc, a, b) }
            AttributeType::VecFloat2(a, b) => { gl::Uniform2fv(attrib_loc, a, b) }
            AttributeType::VecFloat3(a, b) => { gl::Uniform3fv(attrib_loc, a, b) }
            AttributeType::VecFloat4(a, b) => { gl::Uniform4fv(attrib_loc, a, b) }
            AttributeType::VecInt1(a, b) => { gl::Uniform1iv(attrib_loc, a, b) }
            AttributeType::VecInt2(a, b) => { gl::Uniform2iv(attrib_loc, a, b) }
            AttributeType::VecInt3(a, b) => { gl::Uniform3iv(attrib_loc, a, b) }
            AttributeType::VecInt4(a, b) => { gl::Uniform4iv(attrib_loc, a, b) }
            AttributeType::VecUInt1(a, b) => { gl::Uniform1uiv(attrib_loc, a, b) }
            AttributeType::VecUInt2(a, b) => { gl::Uniform2uiv(attrib_loc, a, b) }
            AttributeType::VecUInt3(a, b) => { gl::Uniform3uiv(attrib_loc, a, b) }
            AttributeType::VecUInt4(a, b) => { gl::Uniform4uiv(attrib_loc, a, b) }
            AttributeType::Matrix2(a) => { gl::UniformMatrix2fv(attrib_loc, 1, gl::FALSE, a) }
            AttributeType::Matrix3(a) => { gl::UniformMatrix3fv(attrib_loc, 1, gl::FALSE, a) }
            AttributeType::Matrix4(a) => { gl::UniformMatrix4fv(attrib_loc, 1, gl::FALSE, a) }
            AttributeType::Matrix2x3(a) => { gl::UniformMatrix2x3fv(attrib_loc, 1, gl::FALSE, a) }
            AttributeType::Matrix3x2(a) => { gl::UniformMatrix3x2fv(attrib_loc, 1, gl::FALSE, a) }
            AttributeType::Matrix2x4(a) => { gl::UniformMatrix2x4fv(attrib_loc, 1, gl::FALSE, a) }
            AttributeType::Matrix4x2(a) => { gl::UniformMatrix4x2fv(attrib_loc, 1, gl::FALSE, a) }
            AttributeType::Matrix3x4(a) => { gl::UniformMatrix3x4fv(attrib_loc, 1, gl::FALSE, a) }
            AttributeType::Matrix4x3(a) => { gl::UniformMatrix4x3fv(attrib_loc, 1, gl::FALSE, a) }
        }
    }
}