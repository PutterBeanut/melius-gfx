use meliusgfx::material::{Material, AttributeType};
use meliusgfx::texture::{Texture, WrappingType, FilteringType};
use glfw::Context;
use cgmath::{Deg, Matrix, vec3, Matrix4, perspective};

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

    let mut renderer = meliusgfx::render::Renderer::new();
    renderer.init(|symbol| { window.get_proc_address(symbol) });

    let object = renderer.create_object_without_indices(
        36,
        vec![
            (-0.5, -0.5, -0.5),
            ( 0.5, -0.5, -0.5),
            ( 0.5,  0.5, -0.5),
            ( 0.5,  0.5, -0.5),
            (-0.5,  0.5, -0.5),
            (-0.5, -0.5, -0.5),
            (-0.5, -0.5,  0.5),
            ( 0.5, -0.5,  0.5),
            ( 0.5,  0.5,  0.5),
            ( 0.5,  0.5,  0.5),
            (-0.5,  0.5,  0.5),
            (-0.5, -0.5,  0.5),
            (-0.5,  0.5,  0.5),
            (-0.5,  0.5, -0.5),
            (-0.5, -0.5, -0.5),
            (-0.5, -0.5, -0.5),
            (-0.5, -0.5,  0.5),
            (-0.5,  0.5,  0.5),
            ( 0.5,  0.5,  0.5),
            ( 0.5,  0.5, -0.5),
            ( 0.5, -0.5, -0.5),
            ( 0.5, -0.5, -0.5),
            ( 0.5, -0.5,  0.5),
            ( 0.5,  0.5,  0.5),
            (-0.5, -0.5, -0.5),
            ( 0.5, -0.5, -0.5),
            ( 0.5, -0.5,  0.5),
            ( 0.5, -0.5,  0.5),
            (-0.5, -0.5,  0.5),
            (-0.5, -0.5, -0.5),
            (-0.5,  0.5, -0.5),
            ( 0.5,  0.5, -0.5),
            ( 0.5,  0.5,  0.5),
            ( 0.5,  0.5,  0.5),
            (-0.5,  0.5,  0.5),
            (-0.5,  0.5, -0.5),
        ],
        vec![
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
        ],
        vec![
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (1.0, 1.0),
            (0.0, 1.0),
            (0.0, 0.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (1.0, 1.0),
            (0.0, 1.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (0.0, 1.0),
            (0.0, 1.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (0.0, 1.0),
            (0.0, 1.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (0.0, 1.0),
            (1.0, 1.0),
            (1.0, 0.0),
            (1.0, 0.0),
            (0.0, 0.0),
            (0.0, 1.0),
            (0.0, 1.0),
            (1.0, 1.0),
            (1.0, 0.0),
            (1.0, 0.0),
            (0.0, 0.0),
            (0.0, 1.0)
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
                ),
            ],
            vec![]
        ),
    );

    while !window.should_close() {
        renderer.render();
        renderer.resize_viewport(window.get_size().0, window.get_size().1);

        let projection: Matrix4<f32> = perspective(Deg(90.0), window.get_size().0 as f32 / window.get_size().1 as f32, 0.1, 1000.0);
        let mut model: Matrix4<f32> = Matrix4::<f32>::from_angle_x(Deg(glfw.get_time() as f32 * 30.0));
        model = model * Matrix4::from_angle_z(Deg(glfw.get_time() as f32 * 30.0));

        renderer.set_material_attribute(object, "model", AttributeType::Matrix4(
            model.as_ptr()));
        renderer.set_material_attribute(object, "view", AttributeType::Matrix4(
            &Matrix4::<f32>::from_translation(vec3(0.0, 0.0, -2.0))[0][0]));
        renderer.set_material_attribute(object, "projection", AttributeType::Matrix4(
            projection.as_ptr()));

        window.swap_buffers();
        glfw.poll_events();
    }

    renderer.terminate();
}