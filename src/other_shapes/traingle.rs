use std::{process, ptr::null};

use glfw::{fail_on_errors, Action, Context, Key};

fn hello_triangle(){
    println!("learning OpenGL!");

    // Triangle Shaders ( Vertex,Fragment )
    let c_vertex_shader = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;

        void main(){
            gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
        }
    "#;

    let c_fragment_shader = r#"
        #version 330 core
        out vec4 FragColor;

        void main(){
            FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
        }
    "#;

    const WINDOW_HEIGHT:u32 = 600;
    const WINDOW_WIDTH:u32 = 900;
    const WINDOW_TITLE:&str = "OpenGL";
    // Intialize the GLFW
    let mut intialize_glfw = match glfw::init(fail_on_errors!()){
        Ok(glfw) => glfw,
        Err(e) => {
            println!("Glfw Intialization Error! -> {:?}",e);
            process::exit(1);
        }
    };

    // create a window
    let (mut window, events) = match intialize_glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE, glfw::WindowMode::Windowed){
        Some(win) => win,
        None => {
            eprintln!("Error!, Create window Error");
            process::exit(1);
        }
    };

    let ( win_width, win_height ) = window.get_framebuffer_size();
    // loading the OpenGL
    gl::load_with( | ptr | window.get_proc_address(ptr) );

    // Triangle Vertices and Indices
    let triangle_vertices : [[f32;3];3] = [
        [-0.5, -0.5, 0.0],
        [0.5, -0.5, 0.0],
        [0.0,  0.5, 0.0]
    ];

    // will defined when we use index buffer
    // let triangle_indices : [ f32;3 ] = [];

    // Create A VAO -> Vertex Array Objects
    let mut vao:u32 = 0;
    let mut vbo:u32 = 0;
    unsafe {
        gl::Viewport(0, 0, win_width, win_height);

        // VAO Settings
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        
        // VBO Settings
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            std::mem::size_of_val(&triangle_vertices) as isize, 
            triangle_vertices.as_ptr() as *const _,
            gl::STATIC_DRAW
        );

        gl::VertexAttribPointer(
            0, 
            3, 
            gl::FLOAT, 
            gl::FALSE, 
            std::mem::size_of::<[f32;3]>() as i32,
            null()
        );
        gl::EnableVertexAttribArray(0);

        let err = gl::GetError();
        if err != gl::NO_ERROR {
            eprintln!("OpenGL Error!");
        }
    }


    // Make the current context Window
    window.make_current();
    window.set_key_polling(true);

    // CREATING WINDOW DURATION
    while !window.should_close(){

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);

            // Triangle Shaders
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                &c_vertex_shader.to_string().as_bytes().as_ptr().cast(), 
                &c_vertex_shader.to_string().len().try_into().unwrap()
            );
            gl::CompileShader(vertex_shader);

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader, 
                1, 
                &c_fragment_shader.as_bytes().as_ptr().cast(), 
                &c_fragment_shader.len().try_into().unwrap()
            );
            gl::CompileShader(fragment_shader);

            // Create a shader program
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            gl::UseProgram(shader_program);

            // Delete Shaders
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        
        // swap buffers ( front Buffers )
        window.swap_buffers();
        // Poll the Events
        intialize_glfw.poll_events();
        for ( _ , event ) in glfw::flush_messages(&events){
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _ ) => {
                    window.set_should_close(true);
                },
                _ => {
                    println!("Close Window Error!");
                }
            }
        }
    }
}