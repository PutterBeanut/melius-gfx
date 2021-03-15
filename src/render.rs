use std::ffi::c_void;
use std::ptr;

use crate::material::{Material, AttributeType, set_attribute};

// In the init process, one of the parameters is a `FaceCulling` enum
// to give the user more control
pub enum FaceCulling {
    None,
    Front,
    Back,
    FrontAndBack,
}

// Used for readability instead of using just `4`
//
// Should NOT be exposed to the user.
static FOUR_BYTES: usize = 4;

// Data-Oriented struct containing all of the OpenGL buffers to be used per object.
// VAO (Vertex Array Object): Contains the VertexAttribPointer data when being drawn.
// VBO (Vertex Buffer Object): Contains the position, color, etc. data.
// IBO (Index Buffer Object): Contains the indices (the order in which the vertices are rendered).
// Index Size: The length of the IBO array data.
//
// Should NOT be exposed to the user.
struct Buffers {
    vao: u32,
    vbo: u32,
    ibo: u32,
    index_size: i32,
}

// Data-Oriented struct that controls what happens with each created object and renders them.
//
// Should be exposed to the user.
pub struct Renderer {
    has_init: bool,
    buffers: Vec<Buffers>,
    materials: Vec<Material>,
    attribute_queue: Vec<Vec<(String, AttributeType)>>,
}

#[allow(unused_assignments)]
impl Renderer {
    pub fn new() -> Self {
        Renderer {
            has_init: false,
            buffers: vec![],
            materials: vec![],
            attribute_queue: vec![],
        }
    }

    // Loads the GL functions, therefore requiring a context to load their proc address
    pub fn init<F>(&mut self, mut address: F, multisample: bool, depth_test: bool, stencil_test: bool, cull_face: FaceCulling)
        where F: FnMut(&'static str) -> *const c_void {
        if !self.has_init {
            gl::load_with(|symbol| { address(symbol) });
            unsafe {
                if multisample { gl::Enable(gl::MULTISAMPLE) }
                if depth_test { gl::Enable(gl::DEPTH_TEST) }
                match cull_face {
                    Front => {
                        gl::Enable(gl::CULL_FACE);
                        gl::CullFace(gl::FRONT);
                    },
                    Back => {
                        gl::Enable(gl::CULL_FACE);
                        gl::CullFace(gl::BACK);
                    },
                    FrontAndBack => {
                        gl::Enable(gl::CULL_FACE);
                        gl::CullFace(gl::FRONT_AND_BACK);
                    }
                    _ => {}
                }
            }

            self.has_init = true;
        }
    }

    // Creates an object with the given vertex count, positions, colors, indices, and material.
    // The Object Manager will draw the object when given the chance using the `render` function.
    pub fn create_object(&mut self,
        vertex_count: u32,
        positions: Vec<(f32, f32, f32)>,
        mut colors: Vec<(f32, f32, f32, f32)>,
        mut tex_coords: Vec<(f32, f32)>,
        mut normals: Vec<(f32, f32, f32)>,
        indices: Vec<u32>,
        material: Material) -> u32
    {

        if !self.has_init { panic!("The renderer has not been initialized yet! Try calling `Renderer::init(&mut self, ...)`") }

        let stride: f32 = positions.len() as f32 / vertex_count as f32;
        if stride != (stride as usize) as f32 {
            panic!("The `vertex_count` is incorrect");
        }

        if colors.len() == 0 { colors = vec![(1.0, 1.0, 1.0, 1.0); vertex_count as usize]; }
        if tex_coords.len() == 0 { tex_coords = vec![(1.0, 1.0); vertex_count as usize]; }
        if normals.len() == 0 { normals = vec![(1.0, 1.0, 1.0); vertex_count as usize]; }

        let buffer_data: Vec<f32> = {
            let mut buffer_data: Vec<f32> = vec![];

            for i in 0..positions.len() {
                buffer_data.extend(vec![
                    positions[i].0,
                    positions[i].1,
                    positions[i].2,
                    colors[i].0,
                    colors[i].1,
                    colors[i].2,
                    colors[i].3,
                    tex_coords[i].0,
                    tex_coords[i].1,
                    normals[i].0,
                    normals[i].1,
                    normals[i].2,
                ]);
            }

            buffer_data
        };

        let (vao, vbo, ibo) = unsafe {
            let mut vao = 0u32;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            let mut vbo = 0u32;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (FOUR_BYTES * buffer_data.len()) as isize,
                buffer_data.as_ptr() as *const c_void,
                gl::DYNAMIC_DRAW,
            );

            let stride: i32 = (FOUR_BYTES * (buffer_data.len() / vertex_count as usize)) as i32;

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                0 as *const c_void,
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                4,
                gl::FLOAT,
                gl::FALSE,
                stride,
                12 as *const c_void,
            );

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                28 as *const c_void,
            );

            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(
                3,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                36 as *const c_void,
            );

            let mut ibo = 0u32;
            gl::GenBuffers(1, &mut ibo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (FOUR_BYTES * indices.len()) as isize,
                indices.as_ptr() as *const c_void,
                gl::DYNAMIC_DRAW,
            );

            (vao, vbo, ibo)
        };

        self.buffers.push(Buffers {
            vao,
            vbo,
            ibo,
            index_size: indices.len() as i32,
        });
        self.materials.push(material);
        self.attribute_queue.push(vec![]);

        self.buffers.len() as u32 - 1u32
    }

    pub fn set_material_attribute(&mut self, object: u32, n: &str, t: AttributeType) {
        self.attribute_queue[object as usize].push((n.to_string(), t));
    }

    // Draws all created objects
    pub fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let mut i = 0usize;
            for object in self.buffers.iter() {
                gl::BindVertexArray(object.vao);
                self.materials[i].use_material();

                for (attribute_name, attribute_type) in self.attribute_queue[i].iter() {
                    set_attribute(self.materials[i].get_program_id(), (*attribute_name).clone(), (*attribute_type).clone());
                }
                self.attribute_queue[i] = Vec::new();

                if object.ibo == 0 {
                    gl::BindBuffer(gl::ARRAY_BUFFER, object.vbo);
                    gl::DrawArrays(
                        gl::TRIANGLES,
                        0,
                        object.index_size,
                    );
                } else {
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, object.ibo);
                    gl::DrawElements(
                        gl::TRIANGLES,
                        object.index_size,
                        gl::UNSIGNED_INT,
                        ptr::null(),
                    );
                }

                i += 1;
            }
        }
    }

    pub fn resize_viewport(&mut self, x: i32, y: i32) {
        unsafe {
            gl::Viewport(0, 0, x, y);
        }
    }

    // This will destroy all buffers and free the occupied memory.
    pub fn terminate(&mut self) {
        let mut i = 0;
        for buffers in self.buffers.iter() {
            unsafe {
                gl::DeleteVertexArrays(1, &buffers.vao);
                gl::DeleteBuffers(1, &buffers.vbo);
                gl::DeleteBuffers(1, &buffers.ibo);

                self.materials[i].delete();
            }

            i += 1;
        }
    }
}