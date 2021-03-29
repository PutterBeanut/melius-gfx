use meliusgfx::material::{Material, AttributeType};
use meliusgfx::texture::{Texture, WrappingType, FilteringType};
use meliusgfx::render::{Renderer, FaceCulling, DebugFilter};
use cgmath::{Matrix4, perspective, Deg, vec3};
use cgmath::prelude::*;
use glfw::Context;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 3));

    let (mut window, _) = glfw.create_window(800, 600, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    let mut renderer = Renderer::new();
    renderer.init(|address| { window.get_proc_address(address) }, true, true, FaceCulling::Front);
    renderer.set_debug_filters(vec![DebugFilter::Info]);

    let object = renderer.create_object(
        8,
        vec![
            (-0.5,  0.5,  0.5),
            ( 0.5,  0.5,  0.5),
            ( 0.5, -0.5,  0.5),
            (-0.5, -0.5,  0.5),
            (-0.5, -0.5, -0.5),
            (-0.5,  0.5, -0.5),
            ( 0.5,  0.5, -0.5),
            ( 0.5, -0.5, -0.5),
        ],
        vec![],
        vec![
            (-0.5,  0.5),
            ( 0.5,  0.5),
            ( 0.5, -0.5),
            (-0.5, -0.5),
            (-0.5, -0.5),
            (-0.5,  0.5),
            ( 0.5,  0.5),
            ( 0.5, -0.5),
        ],
        vec![],
        vec![
            0, 1, 2, 2, 3, 0,
            6, 7, 2, 2, 1, 6,
            4, 3, 2, 6, 5, 4,
            4, 7, 6, 2, 7, 4,
            4, 5, 0, 0, 3, 4,
            6, 1, 0, 0, 5, 6
        ],
        Material::from_shader_files(
            "tests/vertex_shader.glsl",
            "tests/fragment_shader.glsl",
            vec![
                Texture::new(
                    WrappingType::Repeat,
                    FilteringType::Linear,
                    FilteringType::Linear,
                    0,
                    Texture::get_from_location("wall.jpg")
                )
            ],
            vec![],
        ),
    );

    let mut time = 0f32;
    while !window.should_close() {

        let projection: Matrix4<f32> = perspective(Deg(90.0),
                                                   800.0 / 600.0, 0.1, 1000.0);
        let mut model: Matrix4<f32> = Matrix4::<f32>::from_angle_x(Deg(time));
        model = model * Matrix4::from_angle_z(Deg(time));

        renderer.set_material_attribute(object, "model", AttributeType::Matrix4(
            model.as_ptr()));
        renderer.set_material_attribute(object, "view", AttributeType::Matrix4(
            Matrix4::<f32>::from_translation(vec3(0.0, 0.0, -2.0)).as_ptr()));
        renderer.set_material_attribute(object, "projection", AttributeType::Matrix4(
            projection.as_ptr()));

        renderer.render();

        time += 0.01;

        window.swap_buffers();
        glfw.poll_events();
    }

}