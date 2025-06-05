use std::{ffi::CString, fs::File, io::{self, Read}, process, ptr::null};
use glfw::{fail_on_errors, ffi::glfwGetTime, Action, Context, Key};
use image::GenericImageView;

use crate::shaders::shader::Shader;

mod shaders {
    pub mod shader;
}

mod textures {
    pub mod texture;
}

fn my_code() {
    println!("Drawing Shapes: llgm & Rectangle");

    let mut fs_file_content_buffer = String::new();
    let _open_fragment_shader_file = match File::open("G:\\OpenGL-YT\\openglyt\\src\\glsl\\texture_f.frag"){
        Ok(mut file) => file.read_to_string(&mut fs_file_content_buffer).unwrap(),
        Err(err) => {
            eprintln!("Error!, FIle reading Error! -> {:?}",err);
            process::exit(1);
        }
    };

    // let fragment_shader_file_reader = ::BufReader::new(open_fragment_shader_file);
    let mut vs_buffer = String::new();
    let _open_vertex_shader_file = match File::open("G:\\OpenGL-YT\\openglyt\\src\\glsl\\texture_v.vert"){
        Ok( mut vert_file ) => vert_file.read_to_string(&mut vs_buffer).unwrap(),
        Err(err) => {
            eprintln!("Error!, FIle reading Error! -> {:?}",err);
            process::exit(1);
        }
    };


    let fragment_shader_rectangle = r#"
        #version 330 core
        out vec4 FragColor;
        uniform vec4 rectangleColorViaUniform;
        void main() {
            FragColor = rectangleColorViaUniform;
        }
    "#;

    // Vertices for Texture -> I will figure out in future
    // let llgm_vertices:[ f32;32 ] = [
    //     // Positions        // Colors      // Texture coordinates
    //     0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
    //     0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
    //     -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
    //     -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left
    // ];

    let llgm_vertices: [[f32; 3]; 4] = [
        [-0.6, -0.4, 0.0],  // Bottom left
        [ 0.0, -0.4, 0.0],  // Bottom right
        [ 0.3,  0.4, 0.0],  // Top right
        [-0.3,  0.4, 0.0]   // Top left
    ];
    
    let llgm_indices: [u32; 6] = [
        0, 3, 2,  // Upper Triangle
        2, 1, 0   // Lower Triangle
    ];

    let rectangle_vertices: [[f32; 3]; 4] = [
        [0.5, -0.5, 0.0],   // Bottom left
        [1.5, -0.5, 0.0],   // Bottom right
        [1.5,  0.5, 0.0],   // Top right
        [0.5,  0.5, 0.0]    // Top left
    ];

    let rectangle_indices: [u32; 6] = [
        0, 1, 2,  // First Triangle
        0, 3, 2   // Second Triangle
    ];

    const WIDTH: u32 = 900;
    const HEIGHT: u32 = 600;
    const TITLE: &str = "Shapes: llgm & Rectangle";

    let mut glfw = match glfw::init(fail_on_errors!()) {
        Ok(glfw) => glfw,
        Err(e) => {
            eprintln!("GLFW initialization failed: {:?}", e);
            process::exit(1);
        }
    };

    let (mut window, events) = match glfw.create_window(WIDTH, HEIGHT, TITLE, glfw::WindowMode::Windowed) {
        Some(win_options) => win_options,
        None => {
            eprintln!("Error creating window!");
            process::exit(1);
        }
    };

    let (w_width, w_height) = window.get_framebuffer_size();
    gl::load_with(|ptr| window.get_proc_address(ptr));

    let (mut vao_llgm, mut vbo_llgm, mut ebo_llgm) = (0, 0, 0);
    let (mut vao_rectangle, mut vbo_rectangle, mut ebo_rectangle) = (0, 0, 0);

    unsafe {
        gl::GenVertexArrays(1, &mut vao_llgm);
        gl::BindVertexArray(vao_llgm);
        gl::GenBuffers(1, &mut vbo_llgm);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_llgm);
        gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(&llgm_vertices) as isize, llgm_vertices.as_ptr().cast(), gl::STATIC_DRAW);
        gl::GenBuffers(1, &mut ebo_llgm);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_llgm);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, std::mem::size_of_val(&llgm_indices) as isize, llgm_indices.as_ptr().cast(), gl::STATIC_DRAW);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, std::mem::size_of::<[f32; 3]>() as i32, null());
        gl::EnableVertexAttribArray(0);

        gl::GenVertexArrays(1, &mut vao_rectangle);
        gl::BindVertexArray(vao_rectangle);
        gl::GenBuffers(1, &mut vbo_rectangle);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_rectangle);
        gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(&rectangle_vertices) as isize, rectangle_vertices.as_ptr().cast(), gl::STATIC_DRAW);
        gl::GenBuffers(1, &mut ebo_rectangle);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_rectangle);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, std::mem::size_of_val(&rectangle_indices) as isize, rectangle_indices.as_ptr().cast(), gl::STATIC_DRAW);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, std::mem::size_of::<[f32; 3]>() as i32, null());
        gl::EnableVertexAttribArray(0);


        let stride = 8 * std::mem::size_of::<f32>() as i32;

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * std::mem::size_of::<f32>()) as *const _);
        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * std::mem::size_of::<f32>()) as *const _);
        gl::EnableVertexAttribArray(2);

    }

    // Configuring the Texture details
        let img = image::open("G:\\OpenGL-YT\\openglyt\\src\\assets\\texture.jpg").expect("Failed to load texture");
        let img = img.flipv().into_rgba8();
        let (width, height) = img.dimensions();
        let data = img.as_raw();

        let mut texture = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            // Set texture parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // Upload texture data
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

    window.make_current();
    window.set_key_polling(true);

    while !window.should_close() {
        window.swap_buffers();
        glfw.poll_events();

        unsafe {
            gl::Viewport(0, 0, w_width, w_height);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            // let llgm_shader_pgm = _create_shader_program(&vs_buffer, &fs_file_content_buffer);
            // gl::UseProgram(llgm_shader_pgm);

            let llgm_shader_pgm = Shader::new(&vs_buffer, &fs_file_content_buffer);
            gl::UseProgram(llgm_shader_pgm.id);
            
            // Bind Texture 
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture );

            // Uniform Location
            let get_time_val = glfwGetTime();
            let cont_color_change  = get_time_val.sin() as f32;
            let llgm_color_location = gl::GetUniformLocation(llgm_shader_pgm.id, CString::new("llgm_uniform_color").unwrap().as_ptr());
            gl::Uniform4f(llgm_color_location, 0.7, cont_color_change, 0.5, 1.0);
            gl::BindVertexArray(vao_llgm);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());
            gl::DeleteProgram(llgm_shader_pgm.id);

            // let rectangle_shader_pgm = _create_shader_program(&vs_buffer, fragment_shader_rectangle);
            // gl::UseProgram(rectangle_shader_pgm);

            let rectangle_shader_pgm = Shader::new(&vs_buffer, fragment_shader_rectangle);
            gl::UseProgram(rectangle_shader_pgm.id);

            // Find the uniform location
            let time_value = glfwGetTime();
            let color_value = ( time_value.sin() / 2.0 ) + 0.5 ;
            let rectangle_color_location = gl::GetUniformLocation(rectangle_shader_pgm.id , CString::new("rectangleColorViaUniform").unwrap().as_ptr());
            // Set the uniform color (adjust as needed)
            gl::Uniform4f(rectangle_color_location, color_value as f32, 0.2, 0.2 as f32, 1.0);
            
            // Bind and draw the rectangle
            gl::BindVertexArray(vao_rectangle);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());
            gl::DeleteProgram(rectangle_shader_pgm.id);
        }

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }
    }
}

// Function to create a shader program
fn _create_shader_program(vertex_src: &str, fragment_src: &str) -> u32 {
    let vertex_shader = unsafe { 
        gl::CreateShader(gl::VERTEX_SHADER) 
    };
    _compile_shader(vertex_shader, vertex_src);
    
    let fragment_shader = unsafe { 
        gl::CreateShader(gl::FRAGMENT_SHADER) 
    };
    _compile_shader(fragment_shader, fragment_src);

    let program = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    program
}

// Function to compile a shader
fn _compile_shader(shader: u32, source: &str) {
    let c_str = std::ffi::CString::new(source).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut info_log = vec![0; 512];
            gl::GetShaderInfoLog(shader, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut _);
            eprintln!("Shader compilation failed: {}", String::from_utf8_lossy(&info_log));
        }
    }
}