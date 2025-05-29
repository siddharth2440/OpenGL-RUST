use std::{process, ptr::null};

use glfw::{fail_on_errors, Action, Context, Key};

mod other_shapes{
    pub mod traingle;
    pub mod rhombus;
    pub mod parallelogram;
    pub mod bothsidebyside;
}

fn main(){
    println!(": Drawing Rectangle");

    // shaders -> Rectangle
    // Triangle Shaders ( Vertex,Fragment )
    let rect_vertex_shader = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;

        void main(){
            gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
        }
    "#;

    let rect_fragment_shader = r#"
        #version 330 core
        out vec4 FragColor;

        void main(){
            FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
        }
    "#;

    // Rectanlge Vertices
    let rectanlge_vertices:[[f32;3];4] = [
        [ 0.5,  0.5, 0.0 ],  // Bottom Left
        [ 0.5, -0.5, 0.0 ],   // Bottom Right
        [ -0.5, 0.5, 0.0 ],   // Top Right
        [ -0.5,  -0.5, 0.0 ]    // Top Left
    ];

    let rectanlge_indices: [u32;6] = [
        0 , 1, 3,
        0, 2, 3
    ];
 
    const HEIGHT:u32 = 600;
    const WIDTH:u32 = 800;
    const WINDOW_TITLE:&str = "Hello Rectangle";
    // Initialize GLFW
    let mut glfw_init = match glfw::init(fail_on_errors!()) {
        Ok(initialized) => initialized,
        Err(err) => {
            eprintln!("Error!, GLFW Iniialization -> {:?}",err);
            process::exit(1);
        }
    };

    let (mut window,events) = match glfw_init.create_window( WIDTH, HEIGHT, WINDOW_TITLE, glfw::WindowMode::Windowed) {
        Some(win) => win,
        None => {
            eprintln!("Error!, Window Creation Error! ");
            process::exit(1);
        }
    };

    // Get window height and width
    let ( w_width , w_height ) = window.get_framebuffer_size();
    // load the OpenGL context
    gl::load_with( | ptr | window.get_proc_address(ptr) );

    let mut vao:u32 = 0;
    let mut vbo:u32 = 0;
    let mut ebo:u32 = 0;

    unsafe {
        // VAO Settings
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // VBO Settings
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            std::mem::size_of_val(&rectanlge_vertices) as isize, 
            rectanlge_vertices.as_ptr().cast(), 
            gl::STATIC_DRAW
        );

        // EBO config....
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER, 
            std::mem::size_of_val(&rectanlge_indices) as isize, 
            rectanlge_indices.as_ptr().cast(), 
            gl::STATIC_DRAW
        );

        // Configure VertexAttrib pointer
        gl::VertexAttribPointer(
            0, 
            3,
            gl::FLOAT, gl::FALSE, 
            std::mem::size_of::<[f32;3]>() as i32, 
            null()
        );
        gl::EnableVertexAttribArray(0);

        // check for Error
        let err = gl::GetError();
        if err != gl::NO_ERROR {
            eprintln!("Error!, OpenGL -> {:?}",err);
        }
    }

    // Make Current Window Context
    window.make_current();
    window.set_key_polling(true);
    while !window.should_close(){
        // Swapping Front <-> Back Buffers ie., Double Buffer
        window.swap_buffers();
        glfw_init.poll_events();   // Polling the Events

        unsafe {
            gl::Viewport(0, 0, w_width, w_height);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(0.0, 0.1, 0.2, 1.0);

            // Shaders Part
            let _rect_vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                _rect_vertex_shader, 
                1,
                &rect_vertex_shader.to_string().as_bytes().as_ptr().cast(), 
                null()
            );
            gl::CompileShader(_rect_vertex_shader);

            let _rect_fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                _rect_fragment_shader, 1, 
                &rect_fragment_shader.to_string().as_ptr().cast(), 
                null()
            );
            gl::CompileShader(_rect_fragment_shader);

            // shader program
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, _rect_vertex_shader);
            gl::AttachShader(shader_program, _rect_fragment_shader);
            gl::LinkProgram(shader_program);

            // use program
            gl::UseProgram(shader_program);
            
            // delete shaders
            gl::DeleteShader(_rect_vertex_shader);
            gl::DeleteShader(_rect_fragment_shader);

            gl::DrawElements(
                gl::TRIANGLES, 
                6, 
                gl::UNSIGNED_INT, null()
            );

            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }

        for ( _ , window_event ) in glfw::flush_messages(&events){
            match window_event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    eprintln!("Window is now closing");
                    window.set_should_close(true);
                },
                _ => {
                    eprintln!("Error!, Window Closing")
                }
            }
        }

    }
}