use std::{process, ptr::null};

use gl::VERTEX_ARRAY;
use glfw::{fail_on_errors, Action, Context, Key};

mod other_shapes {
    pub mod bothsidebyside;
    pub mod parallelogram;
    pub mod rhombus;
    pub mod traingle_rect;
}

fn create_vertex_shader() -> String {
    let v_shader = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;

        void main(){
            gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
        }
    "#;

    return String::from(v_shader);
}


fn create_fragment_shader() -> String {
    let f_shader = r#"
        #version 330 core
        out vec4 FragColor;

        void main(){
            FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
        }
    "#;

    String::from(f_shader)
}


fn main(){

    // Parallelogram Vertices
    let llgm_vertices: [[f32;3];4] = [
        [ -0.6, -0.4, 0.0 ],  // Bottom left
        [ 0.2, -0.4, 0.0 ],  // Bottom Right
        [ 0.6, 0.4, 0.0 ],  // Top-Right
        [ -0.2, 0.4, 0.0 ]  // Top-Left
    ];

    // Parallelogram indices
    let llgm_indices: [ f32 ; 6] = [
        0.0 , 3.0 , 2.0,
        2.0 , 1.0 , 0.0
    ];

    // Vertices for Rhombus
    let rhombus_vertices: [ [f32; 3]; 4 ] = [
        [-0.4 , 0.0 , 0.0], // left
        [0.0, -0.4, 0.0], // bottom
        [0.4, 0.0, 0.0],  // right
        [0.0, 0.4, 0.0] //  up
    ];

    // Indices to draw Rhombus
    let indices: [ u32; 6 ] = [
        0, 3, 2,
        2, 1, 0
    ];
    
    // Intialize our GLFW
    let mut glfw_initialized = match glfw::init(fail_on_errors!()){
        Ok(glfw) => glfw,
        Err(e) => {
            eprintln!("GLFW initialization error -> {:?}",e);
            process::exit(1);
        }
    };

    // Creating a window of size (500 X 600) px's
    let ( mut window, events ) = match glfw_initialized.create_window(1024, 800, "Rhombus", glfw::WindowMode::Windowed){
        Some( window ) => window,
        None => {
            eprintln!("Unable to initialize window");
            process::exit(1);
        }
    };

    // Giving the context of OpenGL to our Window
    window.make_current();
    window.set_key_polling(true);

    let ( screen_width , screen_height ) = window.get_framebuffer_size();
    gl::load_with(| ptr | window.get_proc_address(ptr) as *const _ );

    // Drawing process starts
    unsafe {
        gl::Viewport(0, 0, screen_width, screen_height);
        gl::ClearColor(0.0, 0.1, 0.1, 1.0);

        // VAO settings
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // VBO settings
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // gl::BufferData( 
        //     gl::ARRAY_BUFFER, 
        //     std::mem::size_of_val(&rhombus_vertices) as isize  ,
        //     rhombus_vertices.as_ptr().cast(), 
        //     gl::STATIC_DRAW );
        gl::BufferData( 
            gl::ARRAY_BUFFER, 
            std::mem::size_of_val(&llgm_vertices) as isize  ,
            llgm_vertices.as_ptr().cast(), 
            gl::STATIC_DRAW );
        
        // EBO settings
        let mut ebo = 0;
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        // gl::BufferData(
        //     gl::ELEMENT_ARRAY_BUFFER, 
        //     std::mem::size_of_val(&indices) as isize , 
        //     indices.as_ptr().cast(), 
        //     gl::STATIC_DRAW
        // );
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER, 
            std::mem::size_of_val(&llgm_indices) as isize , 
            indices.as_ptr().cast(), 
            gl::STATIC_DRAW
        );

        // Set our VertexAttribPointer
        gl::VertexAttribPointer(0, 3 , gl::FLOAT, gl::FALSE, std::mem::size_of::< [ f32; 3 ] >().try_into().unwrap() , null() );
        gl::EnableVertexAttribArray(0);

        // Doing the shaders part
        let vertex_shader = create_vertex_shader();
        let fragment_shader = create_fragment_shader();

        let v_shader = gl::CreateShader(gl::VERTEX_SHADER);
        assert_ne!( v_shader, 0 );

        gl::ShaderSource( 
                v_shader,
                1, 
                &vertex_shader.as_bytes().as_ptr().cast(),
                &(vertex_shader.len().try_into().unwrap())
        );
        gl::CompileShader(v_shader);

        let f_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        assert_ne!( f_shader, 0 );
        
        gl::ShaderSource( 
            f_shader , 
            1, 
            &fragment_shader.as_bytes().as_ptr().cast(), 
            &(fragment_shader.len().try_into().unwrap())
        );
        gl::CompileShader(f_shader);

        // gl program
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, v_shader);
        gl::AttachShader(shader_program, f_shader);
        gl::LinkProgram(shader_program);

        // Delete the Shaders
        gl::DeleteShader(v_shader);
        gl::DeleteShader(f_shader);

        gl::UseProgram(shader_program);

        // chk for opengl errors
        let err = gl::GetError();
        eprintln!("{:?}",err);
        if err != gl::NO_ERROR {
            eprintln!("OpenGL Error occured!! {:?} ",err);
        }
    }

    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT , null());
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT , null());
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            let opengl_err = gl::GetError();
            if opengl_err != gl::NO_ERROR {
                eprintln!("There is an Error in OpenGL Configs, Chk again");
            }
        }

        window.swap_buffers();
        glfw_initialized.poll_events();

        for ( _ , events ) in glfw::flush_messages(&events) {

            match events {
                glfw::WindowEvent::Key(Key::Escape, _ , Action::Press, _ ) => {
                    window.set_should_close(true);
                },
                _ => {
                    eprintln!("Error! , Window Close Error.. ");
                }
            }
        }
    }

}