# melius-gfx
A Safe OpenGL wrapper library for Melius.

## Example
#### NOTE: THIS EXAMPLE ASSUMES KNOWLEDGE OF CONTEXT LIBRARIES (glfw-rs, glutin, etc.) AND OPENGL

### Getting started:
To install melius-gfx, just chuck this line of code (is toml *technically* code?) into your Cargo.toml file: ```toml
meliusgfx = "1.1.2"```

melius-gfx makes it relatively easy to call MOST of the OpenGL functions without needing a single `unsafe {}` block. To get started,
we must create a `Renderer`. The renderer contains all the "objects" (which are just all the needed buffers) and functions to manipulate
these "objects" in later code.

Now, the way to create a new renderer is simply defined as:
```rust
pub fn new<F>(mut address: F, multisample: bool, depth_test: bool, cull_face: FaceCulling, debug_filters: Vec<DebugFilter>) -> Self where F: FnMut(&'static str) -> *const c_void { ... }
```
Okay... so maybe it's *not* so simple...

But luckily I invented this function, therefore I can (hopefully) explain it to you!

The address is a rust closure to get your context window's proc address. With glfw-rs, you would pass in this argument like so:
```rust
Renderer::new(|x| context_window.get_proc_address(x), ...);
```
Now *THAT'S* easy, right?

The `multisample` and `depth_test` are both booleans that simply enable depth testing and multisampling. Like it seriously can't get more complicated than that...

The `FaceCulling` asks OpenGL whether to, you GUESSED it, Cull faces! The options are just like OpenGL (including none, which would be just *not* enabling face culling): `None, Front, Back, FrontAndBack`

Oooooh! Now for the fun part! dEeEbUUgGg fiIIllTttEErrss....

A debug filter is an enum with the following severity levels: `Info, Low, Medium, High`, and all it does is filter out any OpenGL debug callbacks (an empty vec would be allowing the console to receive all debug messages).

YAY! You made it to the end... of what takes up one line of code. Here's an example of it all put together:
```rust
let mut renderer = Renderer::new(|x| { window.get_proc_address(x) }, true, true, FaceCulling::Front, vec![DebugFilter::Info]); // Enable depth testing and multisampling, set the face culling to only Front faces, and filter out any `Info` debug messages.
```

### Creating an "object"
It gets generally easy from here... I think.

Creating an "object" is defined like this:
```rust
pub fn create_object(&mut self, vertices: Option<Vec<Vertex>>, indices: Option<Vec<u32>>, material: Material) -> u32 { ... }
```
The vertices and indices are pretty self-explanatory. A `Vertex` contains `position, color, tex_coords, normals, texture_id`, and the indices, of course, it just a `u32` vector. The MATERIAL, on the other hand,
is a little more complicated. Here's an example on creating a material:
```rust
Material::from_shader_files(
    "path/to/my/vertex/shader",
    "path/to/my/fragment/shader",
    vec![                                                               // An array of textures to use
        Texture::new(                                                   // The texture's index correlates to the texture's ID in the shaders
            WrappingType::Repeat,                                       // Wrapping type
            FilteringType::Linear,                                      // Filtering type
            FilteringType::Linear,                                      // Mipmap Filtering type
            0,                                                          // Mipmap levels
            Texture::get_from_location("path/to/texture/file.png")      // Texture data
        )
    ],
    vec![                                                               // An array of attributes to set in the shaders that will definitely be constant so that you don't have to set it every frame.
        ("myConstantValue", AttributeType::Float1(my_constant_f32_value)),
    ],
);
```

Remember that the `create_object` function returns a `u32` value which is considered to be the "object's" "id," which can later be used to modify the "object."
Here's an example of an "object" being created:
```rust
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
            ("time", AttributeType::Float1(5.0))
        ],
    ),
);
```
The last thing that needs to be done is insert a couple of lines into the window's event loop. If we have an event loop like so:
```rust
while !window.should_close() {
    window.swap_buffers();
    glfw.poll_events();
}
```
Then we can simply input this code at the top of the while loop: `renderer.render((1.0, 1.0, 1.0, 1.0))`, in which the four float tuple is what the background color should be.

If you did everything right, you should see something like this (if not then please start an issue if necessary):
![triangle](https://github.com/PutterBeanut/melius-gfx/example/triangle.png)

### Modifying an "object"
We can modify an object in two ways (at least that *I* can think of): Setting the shader attributes or changing the vertex and index buffers. To set the shader attributes (that are likely to change every frame), you would call:
```rust
renderer.set_material_attribute(my_triangle, "myAttribute", AttributeType::Float1(my_attribute_value));
```
and to change the vertex/index buffers, you call:
```rust
renderer.change_object(my_triangle, Some(new_vertices), Some(new_indices));
```