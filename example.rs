use meliusgfx::material::Material;
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

    let mut renderer = Renderer::new(|address| { window.get_proc_address(address) }, true, true, FaceCulling::Front, vec![DebugFilter::Info]);

    let vertices = vec![
        Vertex {
            position: (-0.5,  0.5,  0.5),
            color: (1.0, 1.0, 1.0, 1.0),
            tex_coords: (-0.5,  0.5),
            normals: (0.0, 0.0, 0.0),
            texture_id: 0.0
        },
        Vertex {
            position: ( 0.5,  0.5,  0.5),
            color: (1.0, 1.0, 1.0, 1.0),
            tex_coords: ( 0.5,  0.5),
            normals: (0.0, 0.0, 0.0),
            texture_id: 0.0
        },
        Vertex {
            position: ( 0.5, -0.5,  0.5),
            color: (1.0, 1.0, 1.0, 1.0),
            tex_coords: ( 0.5, -0.5),
            normals: (0.0, 0.0, 0.0),
            texture_id: 0.0
        },
        Vertex {
            position: (-0.5, -0.5,  0.5),
            color: (1.0, 1.0, 1.0, 1.0),
            tex_coords: (-0.5, -0.5),
            normals: (0.0, 0.0, 0.0),
            texture_id: 0.0
        },
        Vertex {
            position: (-0.5, -0.5, -0.5),
            color: (1.0, 1.0, 1.0, 1.0),
            tex_coords: (-0.5, -0.5),
            normals: (0.0, 0.0, 0.0),
            texture_id: 0.0
        },
        Vertex {
            position: (-0.5,  0.5, -0.5),
            color: (1.0, 1.0, 1.0, 1.0),
            tex_coords: (-0.5,  0.5),
            normals: (0.0, 0.0, 0.0),
            texture_id: 0.0
        },
        Vertex {
            position: ( 0.5,  0.5, -0.5),
            color: (1.0, 1.0, 1.0, 1.0),
            tex_coords: ( 0.5,  0.5),
            normals: (0.0, 0.0, 0.0),
            texture_id: 0.0
        },
        Vertex {
            position: ( 0.5, -0.5, -0.5),
            color: (1.0, 1.0, 1.0, 1.0),
            tex_coords: ( 0.5, -0.5),
            normals: (0.0, 0.0, 0.0),
            texture_id: 0.0
        },
    ];

    let _ = renderer.create_object(
        Some(vertices),
        Some(vec![
            0, 1, 2, 2, 3, 0,
        ]),
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

    while !window.should_close() {
        renderer.render((0.0, 0.0, 0.0, 0.0));
        window.swap_buffers();
        glfw.poll_events();
    }

}
