use meliusgfx::material::{Material, AttributeType};
use meliusgfx::texture::{Texture, WrappingType, FilteringType};
use meliusgfx::render::{Renderer, FaceCulling, DebugFilter, Vertex};
use glfw::Context;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 3));
    glfw.window_hint(glfw::WindowHint::Samples(Some(4)));

    let (mut window, _) = glfw.create_window(800, 600, "Test Window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    let mut renderer = Renderer::new(|x| { window.get_proc_address(x) }, true, true, FaceCulling::Front, vec![DebugFilter::Info]);

    let my_triangle = renderer.create_object(
        Some(vec![
            Vertex {
                position: (-0.5, -0.5, 0.0),
                color: (1.0, 1.0, 1.0, 1.0),
                tex_coords: ( 0.0,  0.0),
                normals: (0.0, 0.0, 0.0),
                texture_id: 0.0
            },
            Vertex {
                position: ( 0.0,  0.5, 0.0),
                color: (1.0, 1.0, 1.0, 1.0),
                tex_coords: ( 0.5,  1.0),
                normals: (0.0, 0.0, 0.0),
                texture_id: 0.0
            },
            Vertex {
                position: ( 0.5, -0.5, 0.0),
                color: (1.0, 1.0, 1.0, 1.0),
                tex_coords: ( 1.0,  0.0),
                normals: (0.0, 0.0, 0.0),
                texture_id: 0.0
            },
        ]),
        Some(vec![0, 1, 2]),
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
            vec![
                ("time", AttributeType::Float1(0.1))
            ],
        ),
    );

    while !window.should_close() {
        renderer.render((0.0, 0.0, 0.0, 0.0));
        window.swap_buffers();
        glfw.poll_events();
    }

}
