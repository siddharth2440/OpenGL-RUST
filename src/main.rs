use std::ptr::{null, null_mut};

use gl::{ARRAY_BUFFER, VERTEX_SHADER};
use glfw::{fail_on_errors, ffi::{glfwDestroyWindow, glfwInit, glfwMakeContextCurrent, glfwTerminate}, Action, Context, Key};

fn main() {
    // Vertex Input 
    let vertices: [ [f32; 3] ; 4]  = [ 
        [ 0.5, 0.5, 0.0], [0.5, -0.5, 0.0], [ -0.5, 0.5, 0.0] , [-0.5, 0.5, 0.0]
    ];

    let indices:[u8;6] = [ 
        0, 1, 3,
        1, 2, 3
    ];

    // Vertex shader
    let gl_vertex_shader:&str = r#"
        #version 330 core
        layout ( location = 0 ) in vec3 aPos;

        void main(){
            gl_Position = vec4( aPos.x , aPos.y, aPos.z, 1.0 );
        }
    "#;

    // fragment shader
    let gl_fragment_shader:&str = r#"
        #version 330 core
        out vec4 FragColor;

        void main(){
            FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
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

        // EBO = Element Buffer Object
        let mut ebo:u32 = 0;
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData( gl::ELEMENT_ARRAY_BUFFER , size_of_val(&indices) as isize, indices.as_ptr().cast(), gl::STATIC_DRAW);

        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        gl::BindVertexArray(vao);

        let mut vbo: u32 = 0;
        gl::GenVertexArrays(1, &mut vbo);
        assert_ne!(vbo, 0);
        gl::BindBuffer(ARRAY_BUFFER, vbo);  // Binding it to the (GL_ARRAY_BUFFER -> buffer type of a VBO)
        gl::BufferData(ARRAY_BUFFER, size_of_val(&vertices) as isize, vertices.as_ptr().cast(), gl::STATIC_DRAW);   // Bufferdata :- is targeted to copy UserDefindedData -> Currently Bounded Buffer

        // So, I didn't feel comfortable with this line
        gl::VertexAttribPointer(0, 3,   gl::FLOAT, gl::FALSE, ( std::mem::size_of::<[f32;3]>() ) as gl::types::GLint , 0 as *const _ );
        gl::EnableVertexAttribArray(0);

        // Vertex Buffer
        let vertex_shader = gl::CreateShader(VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);


        gl::ShaderSource(vertex_shader, 1, &(gl_vertex_shader.as_bytes().as_ptr().cast()) , &(gl_vertex_shader.len().try_into().unwrap()) );
        gl::CompileShader(vertex_shader);

        // chk for compilation is successfull or not
        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut v:Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(vertex_shader, 1024,&mut log_len, v.as_mut_ptr().cast());
            println!("Error in Vertex Shader Compilation {:?}",String::from_utf8_lossy(&v));
        }

        // fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);

        gl::ShaderSource(fragment_shader, 1, &(gl_fragment_shader.as_ptr().cast()), &(gl_fragment_shader.len().try_into().unwrap()) );
        gl::CompileShader(fragment_shader);


        // create a shader program
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // check for error in shader-program
        let mut program_success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut program_success);
        if program_success == 0 {
            let mut v1 :Vec<u8> = Vec::with_capacity(1024);
            let mut log = 0_i32;
            gl::GetProgramInfoLog(shader_program, 1024, &mut log, v1.as_mut_ptr().cast() );
            println!("Vertex Program Error {:?}",String::from_utf8_lossy(&v1));
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
        
        gl::UseProgram(shader_program);
        // gl::DrawArrays(gl::TRIANGLES, 0, 3);
        // OpenGL Error
        let err = gl::GetError();
        if err != gl::NO_ERROR {
            eprintln!("OpenGL Error occured!! {:?} ",err);
        }
    }

    // loop until the user closes the window 
    while !window.should_close() {

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
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
        
        // unsafe  {
        //     gl::DrawArrays(gl::TRIANGLES, 0, 3);
        // }
    }
}
