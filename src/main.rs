use std::{process, ptr::null};

use glfw::{fail_on_errors, Action, Context, Key};

fn main() {
    println!("lets draw Rhombus");

    let c_vertex_shader:&str = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;

        void main(){
            gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
        }
    "#; 

    let c_fragment_shader:&str = r#"
        #version 330 core
        out vec4 FragColor;

        void main(){
            FragColor = vec4(0.4f, 0.3f, 0.1f, 1.0f);
        }
    "#;

    let c_fragment_shader1:&str = r#"
        #version 330 core
        out vec4 FragColor;

        void main(){
            FragColor = vec4(0.4f, 0.3f, 0.1f, 1.0f);
        }
    "#;
    //   -------------------------------------------------   Rhombus ---------------------------------------------------
    let rhombus_vertices:[ [f32; 3]; 4 ] = [
        [-0.5,  0.0, 0.0],  // Left point (index 0)
        [ 0.0,  0.5, 0.0],  // Top point (index 1)
        [ 0.5,  0.0, 0.0],  // Right point (index 2)
        [ 0.0, -0.5, 0.0]
    ];
    // Rhombus Vertices
    let rhombus_indices:[u32; 6] = [
        0, 1, 2, 
        0, 2, 3
    ];

    //   -------------------------------------------------   Rhombus ---------------------------------------------------

    //   ------------------------------------------------- Parallelogram -----------------------------------------------
    let parallelogram_vertices:[ [f32;3];4 ] = [
        [ -0.5, -0.5,  0.0 ],
        [ -0.5, -0.5,  0.0 ],
        [ -0.5, -0.5,  0.0 ],
        [ -0.5, -0.5,  0.0 ]
    ];

    let parallelogram_indices: [u32; 6] = [
        0, 1, 2,
        0, 2, 3
    ];


    //   ------------------------------------------------- Parallelogram -----------------------------------------------

    const HEIGHT:u32 = 600;
    const WIDTH:u32 = 900;
    const TITLE:&str = "lets draw a Rhombus";

    // Initialize GLFW
    let mut glfw_initialized = match  glfw::init(fail_on_errors!()) {
        Ok(glfw) => glfw,
        Err(e) => {
            eprintln!("Error in GLFW initialization {:?}",e);
            process::exit(1);
        }
    };

    // Using GLFW let's create a window
    let (mut window , events ) = match glfw_initialized.create_window(WIDTH, HEIGHT, TITLE, glfw::WindowMode::Windowed){
        Some( win_options ) => win_options,
        None => {
            eprintln!("Error!, Window Creation");
            process::exit(1)
        }
    };

    let ( screen_width,screen_height ) = window.get_framebuffer_size();
    
    // load OpenGL
    gl::load_with( | ptr | window.get_proc_address(ptr) as *const _ );
    // Doing the GL stuff and then render our shape
    let mut vao = 0; // intialize VERTEX ARRAY OBJECT
    let mut vbo = 0; // initialize Vertex Buffer Object
    let mut ebo = 0; // initialize Element Buffer Object

    let mut vao1 = 0;
    let mut vbo1 = 0;
    let mut ebo1 = 0;

    unsafe {
        // Vertex Arrays
        gl::GenVertexArrays( 1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenVertexArrays( 1, &mut vao1);
        gl::BindVertexArray(vao1);

        // Vertex Buffer
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            std::mem::size_of_val(&rhombus_vertices) as isize, 
            rhombus_vertices.as_ptr().cast(), // raw pointer
            gl::STATIC_DRAW
        );

        gl::GenBuffers(1, &mut vbo1);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo1);
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            std::mem::size_of_val(&parallelogram_vertices) as isize, 
            parallelogram_vertices.as_ptr().cast(), // raw pointer
            gl::STATIC_DRAW
        );

        // Element Buffer
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(&rhombus_indices) as isize, 
            rhombus_indices.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        gl::GenBuffers(1, &mut ebo1);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo1);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(&parallelogram_indices) as isize, 
            parallelogram_indices.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        // Set Vertex Atribute Array
        gl::VertexAttribPointer(
            0,
            3,  // means number of coordinaes [ (x,y,z) -> 3] [ (x,y) -> 2]
            gl::FLOAT, 
            gl::FALSE, 
            std::mem::size_of::<[f32;3]>() as i32, 
            null()
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            0, 
            3,  // means number of coordinaes [ (x,y,z) -> 3] [ (x,y) -> 2]
            gl::FLOAT, 
            gl::FALSE, 
            std::mem::size_of::<[f32;3]>() as i32, 
            null()
        );
        gl::EnableVertexAttribArray(0);

        // Chk for OpenGL Error
        let err = gl::GetError();
        if err != gl::NO_ERROR {
            eprintln!("OpenGL Error!");
        }
    }

    window.make_current();
    window.set_key_polling(true);

    // Window Session
    while !window.should_close(){
        
        // Double Buffer
        window.swap_buffers();
        glfw_initialized.poll_events();

        unsafe {
            gl::Viewport(0, 0, screen_width, screen_height);
            gl::Clear(gl::COLOR_BUFFER_BIT);        
            gl::ClearColor(0.1, 0.1, 0.2, 1.0);

            // Do the Shaders Part
            
            // 1. 
            let my_vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                my_vertex_shader,
                1,
                &c_vertex_shader.to_string().as_bytes().as_ptr().cast(), 
                std::ptr::null()
            );
            gl::CompileShader(my_vertex_shader);

            let my_fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                my_fragment_shader, 
                1, 
                &c_fragment_shader.to_string().as_bytes().as_ptr().cast() , 
                std::ptr::null());
            gl::CompileShader(my_fragment_shader);

            // lets create a program
            let shader_program1 = gl::CreateProgram();
            gl::AttachShader(shader_program1, my_vertex_shader);
            gl::AttachShader(shader_program1, my_fragment_shader);
            gl::LinkProgram(shader_program1);

            gl::UseProgram(shader_program1);
            // gl::BindVertexArray(vao);

            // Delete Shaders 
            gl::DeleteShader(my_vertex_shader);
            gl::DeleteShader(my_fragment_shader);

            // Draw the Elements
            gl::DrawElements(
                gl::TRIANGLES, 
                6,  // number of indices
                gl::UNSIGNED_INT, 
                std::ptr::null());
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

            // 2. 
            let my_vertex_shader1 = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                my_vertex_shader1,
                1,
                &c_vertex_shader.to_string().as_bytes().as_ptr().cast(), 
                std::ptr::null()
            );
            gl::CompileShader(my_vertex_shader1);

            let my_fragment_shader1 = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                my_fragment_shader, 
                1, 
                &c_fragment_shader1.to_string().as_bytes().as_ptr().cast() , 
                std::ptr::null());
            gl::CompileShader(my_fragment_shader1);

            // lets create a program
            let shader_program2 = gl::CreateProgram();
            gl::AttachShader(shader_program2, my_vertex_shader1);
            gl::AttachShader(shader_program2, my_fragment_shader1);
            gl::LinkProgram(shader_program2);

            gl::UseProgram(shader_program2);
            // gl::BindVertexArray(vao1);

            // Delete Shaders 
            gl::DeleteShader(my_vertex_shader1);
            gl::DeleteShader(my_fragment_shader1);

            // Draw the Elements
            gl::DrawElements(
                gl::TRIANGLES, 
                6,  // number of indices
                gl::UNSIGNED_INT, 
                std::ptr::null());
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        }

        for ( _ , event ) in glfw::flush_messages(&events){
            match event {
                glfw::WindowEvent::Key(  Key::Escape, _, Action::Press, _ ) => {
                    window.set_should_close(true);
                },
                _ => {
                    println!("Wrong Key Press!, Press (Esc) Key to Close the window");
                }
            }
        }
    }
}