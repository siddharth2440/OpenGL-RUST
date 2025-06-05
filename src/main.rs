use std::{ffi::CString, fs::File, io::Read, process, ptr::null};
use glfw::{Action, Context, Key, fail_on_errors};
use image::GenericImageView;

use crate::shaders::shader::Shader;

mod shaders {
    pub mod shader;
}

fn main() {
    let mut glfw = glfw::init(fail_on_errors!()).expect("Failed to initialize GLFW");
    let (mut window, events) = glfw
        .create_window(800, 600, "Textured Shape", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| window.get_proc_address(s));

    let vertex_shader_path = "src/glsl/texture_v.vert";
    let fragment_shader_path = "src/glsl/texture_f.frag";

    let mut vertex_code = String::new();
    File::open(vertex_shader_path)
        .expect("Failed to read vertex shader")
        .read_to_string(&mut vertex_code)
        .unwrap();

    let mut fragment_code = String::new();
    File::open(fragment_shader_path)
        .expect("Failed to read fragment shader")
        .read_to_string(&mut fragment_code)
        .unwrap();

    let shader = Shader::new(&vertex_code, &fragment_code);

    // Position, Color, TexCoords
    let vertices: [f32; 32] = [
        // positions      // colors       // texture coords
         0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
         0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
        -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left
    ];

    let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];

    let (mut vao, mut vbo, mut ebo) = (0, 0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as isize,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // positions
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, null());
        gl::EnableVertexAttribArray(0);
        // colors
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>()) as *const _);
        gl::EnableVertexAttribArray(1);
        // texture coords
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, (6 * std::mem::size_of::<f32>()) as *const _);
        gl::EnableVertexAttribArray(2);
    }

    let img = image::open("G:\\OpenGL-YT\\openglyt\\src\\assets\\texture.jpg").expect("Failed to load texture");
    let img = img.flipv().into_rgba8();
    let (width, height) = img.dimensions();
    let data = img.as_raw();

    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const _,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true);
            }
        }

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader.id);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());
        }

        window.swap_buffers();
    }
}
