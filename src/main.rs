mod shader;

use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint, WindowMode};
use shader::Shader;
use std::path::Path;

macro_rules! concat_tuples {
    ( $( ( $(  $element:expr ),* ) ),* $(,)? ) => {
        [
        $(
            $(
                $element,
            )*
        )*
        ]
    };
}

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("glfw initialization failed");
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    #[cfg(target_os = "macos")]
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    let (mut window, _) = glfw
        .create_window(800, 600, "LearnOpenGL", WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    glfw.make_context_current(Some(&window));

    gl::load_with(|symbol| window.get_proc_address(symbol).cast());

    window.set_framebuffer_size_callback(framebuffer_size_callback);

    let shader_program = unsafe { Shader::new(Path::new(""), Path::new("")) };

    let crazy_triangle: [f32; 18] = concat_tuples!(
        (-0.5, -0.5, 0.0, 0.0, 1.0, 0.0),
        (0.5, -0.5, 0.0, 1.0, 0.0, 0.0),
        (0.0, 0.5, 0.0, 0.0, 0.0, 1.0)
    );

    assert_eq!(
        size_of_val(&crazy_triangle),
        crazy_triangle.len() * size_of::<f32>(),
        "forgot to specify f32 instead of f64?"
    );

    let mut vbo = 0;
    let mut vao = 0;

    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::GenVertexArrays(1, &mut vao);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (crazy_triangle.len() * size_of::<f32>()) as isize,
            crazy_triangle.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        // position
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            false as _,
            6 * size_of::<f32>() as i32,
            0 as _,
        );
        gl::EnableVertexAttribArray(0);
        // color
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            false as _,
            6 * size_of::<f32>() as i32,
            (3 * size_of::<f32>()) as _,
        );
        gl::EnableVertexAttribArray(1);
    }

    while !window.should_close() {
        process_input(&mut window);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader_program.activate();

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            // let time_value = glfw.get_time();
            // let green_value = (time_value.sin() as f32 / 2.) + 0.5;

            // gl::UseProgram(yellow_shader_program);
            // gl::Uniform4f(vertex_color_location, 0.0, green_value, 0.0, 1.0);

            // gl::BindVertexArray(vao);
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);

            // gl::DrawElements(
            //     gl::TRIANGLES,
            //     indices.len() as i32,
            //     gl::UNSIGNED_INT,
            //     0 as _,
            // );

            // gl::BindVertexArray(0);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn framebuffer_size_callback(_window: &mut glfw::Window, width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}

fn process_input(window: &mut glfw::Window) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }
}

// let square: [f32; 12] = vertices![
//     (0.5, 0.5, 0.0),
//     (0.5, -0.5, 0.0),
//     (-0.5, -0.5, 0.0),
//     (-0.5, 0.5, 0.0)
// ];
// let indices = [0, 1, 3, 1, 2, 3];

// let (mut vbo, mut vao, mut ebo) = (0, 0, 0);

// gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
// gl::BufferData(
// gl::ELEMENT_ARRAY_BUFFER,
//     (indices.len() * size_of::<f32>()) as isize,
//     indices.as_ptr().cast(),
//     gl::STATIC_DRAW,
// );
//
// gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

// const VERTEX_SHADER_SOURCE: &CStr = cr#"
//     #version 330 core
//     layout (location = 0) in vec3 aPos;
//     layout (location = 1) in vec3 aColor;
//
//     out vec3 vertexColor;
//
//     void main()
//     {
//         gl_Position = vec4(aPos, 1.0);
//         vertexColor = aColor;
//     }
// "#;
//
// const ORANGE_FRAGMENT_SHADER_SOURCE: &CStr = cr#"
//     #version 330 core
//     out vec4 FragColor;
//
//     in vec3 vertexColor;
//
//     void main()
//     {
//         FragColor = vec4(vertexColor, 1.0);
//     }
// "#;
//
// const YELLOW_FRAGMENT_SHADER_SOURCE: &CStr = cr#"
//     #version 330 core
//     out vec4 FragColor;
//
//     uniform vec4 ourColor;
//
//     void main()
//     {
//         FragColor = ourColor;
//     }
// "#;
