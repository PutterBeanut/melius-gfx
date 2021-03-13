use std::ffi::c_void;
use std::ptr;
use cgmath::Matrix4;

use crate::material::Material;
use crate::transform::Transform;

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
pub struct ObjectManager {
    has_init: bool,

    buffers: Vec<Buffers>,
    materials: Vec<Material>,

    objects: Vec<Matrix4<f32>>,
    cameras: Vec<(Matrix4<f32>, Matrix4<f32>)>,
    screen_aspect_ratio: f32,
}

impl ObjectManager {
    pub fn new() -> Self {
        ObjectManager {
            has_init: false,
            buffers: vec![],
            materials: vec![],
            objects: vec![],
            cameras: vec![],
            screen_aspect_ratio: 1.0
        }
    }

    // Loads the GL functions, therefore requiring a context to load their proc address
    pub fn init<F>(&mut self, mut address: F)
    where F: FnMut(&'static str) -> *const c_void {
        if !self.has_init {
            gl::load_with(|symbol| { address(symbol) });
            unsafe {
                gl::Enable(gl::MULTISAMPLE);
                gl::Enable(gl::DEPTH_TEST);
            }

            self.has_init = true;
        }
    }

    // Creates an object with the given vertex count, positions, colors, indices, and material.
    // The Object Manager will draw the object when given the chance using the `render` function.
    pub fn create_object(&mut self,
                         vertex_count: u32,
                         positions: Vec<(f32, f32, f32)>,
                         colors: Vec<(f32, f32, f32, f32)>,
                         tex_coords: Vec<(f32, f32)>,
                         indices: Vec<u32>,
                         material: Material,
                         transform: Transform) {

        let pos_stride: f32 = positions.len() as f32 / vertex_count as f32;
        if pos_stride != (pos_stride as usize) as f32 {
            panic!("The `vertex_count` is incorrect");
        }

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
                gl::DYNAMIC_DRAW
            );

            let stride: i32 = (FOUR_BYTES * (buffer_data.len() / vertex_count as usize)) as i32;

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                0usize as *const c_void
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                4,
                gl::FLOAT,
                gl::FALSE,
                stride,
                12usize as *const c_void
            );

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                28usize as *const c_void
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
        self.objects.push(transform.get_matrix());
    }

    // Draws all created objects using the camera specified
    pub fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let mut i = 0usize;
            for object in self.buffers.iter() {

                gl::BindVertexArray(object.vao);
                self.materials[i].use_material(self.screen_aspect_ratio);

                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, object.ibo);
                gl::DrawElements(
                    gl::TRIANGLES,
                    object.index_size,
                    gl::UNSIGNED_INT,
                    ptr::null()
                );

                i += 1;
            }
        }
    }

    pub fn resize_viewport(&mut self, x: i32, y: i32) {
        unsafe {
            gl::Viewport(0, 0, x, y);
        }

        self.screen_aspect_ratio = x as f32 / y as f32;
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