use std::{ffi::CString, fs::File, io::Read, os::windows, process, ptr::null, time::{SystemTime, UNIX_EPOCH}};
use glfw::{fail_on_errors, ffi::glfwGetTime, Action, Context, Key, WindowEvent};
use glm::{ext::{rotate, translate}, mat4, Mat4};
use image::GenericImageView;

use crate::{ 
    lib::load_image::load_image_into_cpu, 
    shaders::shader::Shader, 
    textures::texture::Texture, 
    utils::coordinates::{TexturePath, Vertices}
};

mod shaders {
    pub mod shader;
}

mod textures {
    pub mod texture;
}

mod lib {
    pub mod load_image;
}

mod utils {
    pub mod coordinates;
    pub mod payload;
}

mod camera {
    pub mod camera;
}

const WINDOW_WIDTH:u32 = 800;
const WINDOW_HEIGHT:u32 = 800;
const WINDOW_TITLE: &str = "Can I get 3D";
const VERTEX_SHADER_PATH: &str = "G:\\OpenGL-YT\\openglyt\\src\\glsl\\texture_v.vert";
const FRAGMENT_SHADER_PATH: &str = "G:\\OpenGL-YT\\openglyt\\src\\glsl\\texture_f.frag";
const TEXTURE_PATH: &str = "G:\\OpenGL-YT\\openglyt\\src\\assets\\wall.jpg";

type ShapeVerticesAndIndices = ( [f32; 32], [ u32; 6 ] );

fn get_shape_vertices() -> ShapeVerticesAndIndices {

    let vertices = [
            //  ***  -----------  Vertices - Colors - Textures  ----------- ***  
            	-0.5, -0.5, 0.0,     1.0, 0.0, 0.0,	     0.0, 0.0, // Lower let corner
                -0.5,  0.5, 0.0,     0.0, 1.0, 0.0,	     0.0, 1.0, // Upper let corner
                 0.5,  0.5, 0.0,     0.0, 0.0, 1.0, 	 1.0, 1.0, // Upper right corner
                 0.5, -0.5, 0.0,     1.0, 1.0, 1.0,	     1.0, 0.0  // Lower right corner
    ];

    let indices = [
        0, 2, 1, // Upper triangle
	    0, 3, 2 // Lower triangle
    ];
    
    ( vertices, indices )
}


fn main() {

    type BuffersType = ( u32, u32, u32 );

    // initialize our glfw
    let mut initialize_glfw = match glfw::init(fail_on_errors!()) {
        Ok(glfw) => glfw,
        Err(e) => {
            eprintln!("GLFW - Iniialization Error -> {:?}", e);
            process::exit(1)
        }
    };

    let ( mut window, events ) =  match initialize_glfw.create_window( WINDOW_WIDTH , WINDOW_HEIGHT, WINDOW_TITLE, glfw::WindowMode::Windowed ) {
        Some( window ) => window,
        None => {
            eprintln!("Creating Window Error!");
            process::exit(1)
        }
    };

    window.make_current();
    window.set_key_polling(true);

    gl::load_with( | ptr | window.get_proc_address(ptr)  );
    
    let ( vertices, indices ) = get_shape_vertices();
    
    let mut vertex_shader_impl_buffer = String::new();
    File::open(VERTEX_SHADER_PATH).expect("Vertex shader File Reading Error!").read_to_string(&mut vertex_shader_impl_buffer).unwrap();
    let mut fragment_shader_impl_buffer = String::new();
    File::open(FRAGMENT_SHADER_PATH).expect("Fragment shader File Reading Error!").read_to_string(&mut fragment_shader_impl_buffer).unwrap();
    
    // Shaders
    let shader = Shader::new(&vertex_shader_impl_buffer, &fragment_shader_impl_buffer);
    let texture = Texture::new(TEXTURE_PATH);
    println!("Texture Id -> {:?}", texture.id);
    
    let ( mut vao, mut vbo, mut ebo ) :BuffersType = ( 0, 0, 0 );
    unsafe {
        gl::Viewport(0,0,WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
        
        // Generate -->> Arrays && Buffers
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);
        
        // Bind them
        gl::BindVertexArray(vao);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(&vertices) as isize , vertices.as_ptr() as * const _, gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, std::mem::size_of_val(&indices) as isize , indices.as_ptr() as * const _, gl::STATIC_DRAW);
        
        
        gl::VertexAttribPointer( 0, 3 , gl::FLOAT, gl::FALSE, 8 * size_of::<f32>() as i32, std::ptr::null() );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer( 1, 3 , gl::FLOAT, gl::FALSE, 8 * size_of::<f32>() as i32, (3 * size_of::<f32>()) as *const _ );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer( 2, 2 , gl::FLOAT, gl::FALSE, 8 * size_of::<f32>() as i32, (6 * size_of::<f32>()) as *const _ );
        gl::EnableVertexAttribArray(2);

        let opengl_err = gl::GetError();
        if opengl_err != gl::NO_ERROR {
            eprintln!("Error in OpenGL -> {:?}", opengl_err);
        }

    }

    println!("Shader Program ID -> {:?}", shader.id );

    // Uniforms 
    let scale_uniform = CString::new("scale").expect("Invalid Uniform Provided");
    let vertex_shader_scale_loc = unsafe {
        gl::GetUniformLocation(shader.id, scale_uniform.as_ptr() as *const _)
    };

    while !window.should_close() {
        initialize_glfw.poll_events();
        window.swap_buffers();

        for ( _, event ) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(  Key::Escape, _, Action::Press, _ ) => {
                    window.set_should_close(true);
                },
                _ => {
                    eprintln!("There is some error occured in closing our Window");
                }
            }
        }

        unsafe {
            gl::ClearColor(0.21, 0.13, 0.02, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let curtime = glfwGetTime();
            let ran_value: f32 = ((curtime.cos() / 2.0) + 0.5) as f32;
            gl::UseProgram(shader.id);
            gl::Uniform1f(vertex_shader_scale_loc, ran_value );

            gl::BindVertexArray(vao);
            
            gl::ActiveTexture(texture.id);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            
            gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, null());
        }
    }
}