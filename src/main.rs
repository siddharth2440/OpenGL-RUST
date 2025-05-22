use std::ptr::{null, null_mut};

use gl::{ARRAY_BUFFER, VERTEX_SHADER};
use glfw::{fail_on_errors, ffi::{glfwDestroyWindow, glfwInit, glfwMakeContextCurrent, glfwTerminate}, Action, Context, Key};

fn main() {
    // Vertex Input 
    let vertices: [ [f64; 3] ; 3]  = [ [ -0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0] ];

    // Vertex shader
    let gl_vertex_shader:&str = r#"
        #version 330 core
        layout ( location = 0 ) in vec3 aPos;

        void main(){
            gl_Position = vec4( aPos.x , aPos.y, aPos.z, 1.0 );
        }
    "#;

    // GLFW initialization
    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();
    let ( mut window , events ) = match  glfw.create_window(900, 500, "OpenGL-RUST", glfw::WindowMode::Windowed) {
        Some(win) => win,
        None => {
            eprintln!("Create window error!");
            std::process::exit(1);
        }
    };

    // making the windows context current
    window.make_current();
    window.set_key_polling(true);

    let ( screen_width, screen_height ) = window.get_framebuffer_size();
    // load the gl
    gl::load_with(| ptr | window.get_proc_address(ptr) as *const _ );

    unsafe {
        gl::Viewport(0, 0, screen_width, screen_height);
        gl::ClearColor(0.2, 0.1, 0.4, 0.1);


        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        gl::BindVertexArray(vao);

        let mut vbo = 0;
        gl::GenVertexArrays(1, &mut vbo);
        assert_ne!(vbo, 0);
        gl::BindBuffer(ARRAY_BUFFER, vbo);  // Binding it to the (GL_ARRAY_BUFFER -> buffer type of a VBO)
        gl::BufferData(ARRAY_BUFFER, size_of_val(&vertices) as isize, vertices.as_ptr().cast(), gl::STATIC_DRAW);   // Bufferdata :- is targeted to copy UserDefindedData -> Currently Bounded Buffer        

        // Vertex Buffer
        let vertex_shader = gl::CreateShader(VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);


        gl::ShaderSource(vertex_shader, 1, &gl_vertex_shader.as_bytes().as_ptr().cast() , &gl_vertex_shader.len().try_into().unwrap() );
        gl::CompileShader(vertex_shader);

        // chk for compilation is successfull or not
        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        let _info_log: gl::types::GLchar;

        if success == 0 {
            let mut v:Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(vertex_shader, 1024,&mut log_len, v.as_mut_ptr().cast());
            println!("Error in Vertex Shader Compilation {:?}",String::from_utf8_lossy(&v));
        }

        let err = gl::GetError();
        if err != gl::NO_ERROR {
            eprintln!("OpenGL Error occured!! {:?} ",err);
        }
    }

    // loop until the user closes the window 
    while !window.should_close() {

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();   // swapping the buffers ( front-buffer and back-buffer )

        glfw.poll_events();

        for ( _, event ) in glfw::flush_messages(&events){
            println!("{:?}",event);

            match event {
                glfw::WindowEvent::Key(Key::Escape, _ , Action::Press , _ ) => {
                    window.set_should_close(true);
                },
                _ => {}
            }
        }
    }

    // unsafe  {
    //     glfwDestroyWindow( window.window_ptr() );  // fn call to free the resources
    //     glfwMakeContextCurrent(null_mut());
    //     glfwTerminate();  // fn call clean the GLFW-related memory
    // }
}
