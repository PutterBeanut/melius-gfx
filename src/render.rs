use std::ffi::{c_void, CStr};
use std::ptr;
use memoffset::offset_of;

use crate::material::{Material, AttributeType, set_attribute};

static mut DEBUG_FILTERS: [DebugFilter; 4] = [DebugFilter::None, DebugFilter::None, DebugFilter::None, DebugFilter::None];

// The renderer takes in an array of DebugFilters to filter out one or more
// specific types of messages from the OpenGL callbacks
#[derive(Copy, Clone)]
pub enum DebugFilter {
    None,
    Info,
    Low,
    Medium,
    High
}

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
    pub vao: u32,
    pub vbo: u32,
    pub ibo: u32,
    pub index_size: i32,
}

// Public types used for the `Vertex` struct.
//
// Should be exposed to the user.
pub type Position = (f32, f32, f32);
pub type Color = (f32, f32, f32, f32);
pub type TexCoords = (f32, f32);
pub type Normals = (f32, f32, f32);
pub type TextureID = f32;

// Used for the `Vertex` struct to get the amount of floats in the entire struct
static VERTEX_DATA_SIZE: isize = 13;

// Public struct exposed to the user that allows for the creation of objects.
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Position,
    pub color: Color,
    pub tex_coords: TexCoords,
    pub normals: Normals,
    pub texture_id: TextureID,
}

// Data-Oriented struct that controls what happens with each created object and renders them.
//
// Should be exposed to the user.
pub struct Renderer {
    buffers: Vec<Buffers>,
    materials: Vec<Material>,
    attribute_queue: Vec<Vec<(String, AttributeType)>>,
}

#[allow(unused_assignments)]
impl Renderer {
    // Loads the GL functions, therefore requiring a context to load their proc address
    pub fn new<F>(mut address: F, multisample: bool, depth_test: bool, cull_face: FaceCulling) -> Self
        where F: FnMut(&'static str) -> *const c_void {
        gl::load_with(|symbol| address(symbol));
        unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(message_callback), 0 as *const c_void);

            if multisample { gl::Enable(gl::MULTISAMPLE) }
            if depth_test { gl::Enable(gl::DEPTH_TEST) }
            match cull_face {
                FaceCulling::Front => {
                    gl::Enable(gl::CULL_FACE);
                    gl::CullFace(gl::FRONT);
                },
                FaceCulling::Back => {
                    gl::Enable(gl::CULL_FACE);
                    gl::CullFace(gl::BACK);
                },
                FaceCulling::FrontAndBack => {
                    gl::Enable(gl::CULL_FACE);
                    gl::CullFace(gl::FRONT_AND_BACK);
                }
                _ => {}
            }
        }


        Renderer {
            buffers: vec![],
            materials: vec![],
            attribute_queue: vec![],
        }
    }

    pub fn set_debug_filters(&self, filters: Vec<DebugFilter>) {
        if filters.len() > 4 { panic!("Cannot have a debug filter count greater than 4!") }

        for i in 0..filters.len() {
            unsafe { DEBUG_FILTERS[i] = filters[i].clone(); }
        }
    }

    // Creates an object with the given vertex count, positions, colors, indices, and material.
    // The Object Manager will draw the object when given the chance using the `render` function.
    pub fn create_object(&mut self,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        material: Material) -> u32
    {

        let mut buffer_data: Vec<f32> = vec![];
        for vertex in vertices.iter() {
            buffer_data.extend(vec![
                vertex.position.0,
                vertex.position.1,
                vertex.position.2,
                vertex.color.0,
                vertex.color.1,
                vertex.color.2,
                vertex.color.3,
                vertex.tex_coords.0,
                vertex.tex_coords.1,
                vertex.normals.0,
                vertex.normals.1,
                vertex.normals.2,
                vertex.texture_id
            ]);
        }

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

            let stride: i32 = (FOUR_BYTES * (buffer_data.len() / vertices.len() as usize)) as i32;

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                offset_of!(Vertex, position) as *const c_void,
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                4,
                gl::FLOAT,
                gl::FALSE,
                stride,
                offset_of!(Vertex, color) as *const c_void,
            );

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                offset_of!(Vertex, tex_coords) as *const c_void,
            );

            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(
                3,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                offset_of!(Vertex, normals) as *const c_void,
            );

            gl::EnableVertexAttribArray(4);
            gl::VertexAttribPointer(
                4,
                1,
                gl::FLOAT,
                gl::FALSE,
                stride,
                offset_of!(Vertex, texture_id) as *const c_void,
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

    // Changes the vertex/index data of a given object.
    pub fn change_object(&mut self, object: u32, vertices: Option<Vec<Vertex>>, indices: Option<Vec<u32>>) {
        unsafe {
            match vertices {
                Some(vertices) => {
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.buffers[object as usize].vbo);
                    gl::BufferSubData(gl::ARRAY_BUFFER, 0, vertices.len() as isize * FOUR_BYTES as isize * VERTEX_DATA_SIZE,
                                      vertices.as_ptr() as * const c_void);
                }
                _ => {}
            }

            match indices {
                Some(indices) => {
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.buffers[object as usize].ibo);
                    gl::BufferSubData(gl::ELEMENT_ARRAY_BUFFER, 0, indices.len() as isize * FOUR_BYTES as isize,
                                      indices.as_ptr() as * const c_void);
                }
                _ => {}
            }
        }
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

extern "system" fn message_callback(_: u32, _:  u32, _: u32, severity: u32, _: i32, message: *const i8, _: *mut c_void) {
    let mut severity_str = String::new();
    let mut is_filtered = false;

    if severity == gl::DEBUG_SEVERITY_NOTIFICATION {
        unsafe {
            for filter in DEBUG_FILTERS.iter() {
                match filter {
                    DebugFilter::Info => { is_filtered = true; }
                    _ => {}
                }
            }
        }

        severity_str = "Notification".to_string()
    }
    if severity == gl::DEBUG_SEVERITY_LOW {
        unsafe {
            for filter in DEBUG_FILTERS.iter() {
                match filter {
                    DebugFilter::Low => { is_filtered = true; }
                    _ => {}
                }
            }
        }

        severity_str = "Low".to_string()
    }
    if severity == gl::DEBUG_SEVERITY_MEDIUM {
        unsafe {
            for filter in DEBUG_FILTERS.iter() {
                match filter {
                    DebugFilter::Medium => { is_filtered = true; }
                    _ => {}
                }
            }
        }

        severity_str = "Medium".to_string()
    }
    if severity == gl::DEBUG_SEVERITY_HIGH {
        unsafe {
            for filter in DEBUG_FILTERS.iter() {
                match filter {
                    DebugFilter::High => { is_filtered = true; }
                    _ => {}
                }
            }
        }

        severity_str = "High".to_string()
    }

    if is_filtered { return; }

    unsafe { println!("GL CALLBACK (severity: {}): {}", severity_str, CStr::from_ptr(message).to_str().unwrap()); }
}