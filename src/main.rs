use meliusgfx::material::Material;
use meliusgfx::transform::Transform;
use meliusgfx::texture::{Texture, WrappingType, FilteringType};
use glfw::Context;

static VERTEX_SHADER: &str = "
#version 330 core
layout (location = 0) in vec3 vertexPosition;
layout (location = 1) in vec4 vertexColor;
layout (location = 2) in vec2 texCoords;

out vec4 fragmentColor;
out vec2 fragmentTexCoords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    fragmentColor = vertexColor;
    fragmentTexCoords = texCoords;
    gl_Position = projection * view * model * vec4(vertexPosition, 1);
}
";
static FRAGMENT_SHADER: &str = "
#version 330 core
in vec4 fragmentColor;
in vec2 fragmentTexCoords;

uniform sampler2D texture0;

out vec4 FragColor;

void main() {
    FragColor = texture(texture0, fragmentTexCoords) * fragmentColor;
}
";

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::Samples(Some(4)));

    let (mut window, _) = glfw.create_window(800, 600, "Hello Window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    let mut object_manager = meliusgfx::object::ObjectManager::new();
    object_manager.init(|symbol| { window.get_proc_address(symbol) });


    object_manager.create_object(
        4,
        vec![
            (-0.5, -0.5,  0.0),
            (-0.5,  0.5,  0.0),
            ( 0.5, -0.5,  0.0),
            ( 0.5,  0.5,  0.0),
        ],
        vec![
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
        ],
        vec![
            (0.0, 0.0),
            (0.0, 1.0),
            (1.0, 1.0),
            (1.0, 0.0),
        ],
        vec![
            0, 1, 3, 3, 2, 0
        ],
        Material::from_shaders(
            VERTEX_SHADER.clone().to_string(),
            FRAGMENT_SHADER.clone().to_string(),
            vec![
                Texture::new(
                    WrappingType::Repeat,
                    FilteringType::Linear,
                    FilteringType::Linear,
                    0,
                    Texture::get_from_location("wall.jpg"),
                )
            ]
        ),
        Transform::identity()
    );



    while !window.should_close() {
        object_manager.render();
        object_manager.resize_viewport(window.get_size().0, window.get_size().1);

        window.swap_buffers();
        glfw.poll_events();
    }

    object_manager.terminate();
}