use meliusgfx::material::{Material, AttributeType};
use meliusgfx::texture::{Texture, WrappingType, FilteringType};
use meliusgfx::render::{Renderer, FaceCulling};
use glutin::window::WindowBuilder;
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::event::{Event, WindowEvent};
use cgmath::{Matrix4, perspective, Deg, vec3};
use cgmath::prelude::*;
use glutin::ContextBuilder;
use glutin::dpi::PhysicalSize;

fn main() {

    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Window Test").with_inner_size(PhysicalSize { width: 800.0, height: 600.0 });

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    let mut renderer = Renderer::new();
    renderer.init(|address| { windowed_context.get_proc_address(address) }, true, true, FaceCulling::Front);

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
    el.run(move |event, _, control_flow| {
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }

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

        windowed_context.swap_buffers().unwrap();
    });

}