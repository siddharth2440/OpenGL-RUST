// Implementing the Shaders
use std::{ffi::CString, ptr::null_mut};

pub struct Shader {
    pub id: gl::types::GLuint
}

impl Shader {
    pub fn new( vertex_shader_src:&str, fragment_shader_src:&str ) -> Self {
        let vertex_shader = unsafe { 
            gl::CreateShader(gl::VERTEX_SHADER)
        };
        // println!("Vertex Shader -> {:?}",vertex_shader);        

        let fragment_shader = unsafe { 
            gl::CreateShader(gl::FRAGMENT_SHADER)
        };
        // println!("Fragment Shader -> {:?}",fragment_shader);

        Self::shader_source(vertex_shader, vertex_shader_src);
        Self::shader_source(fragment_shader, fragment_shader_src);

        // If no Error then
        Self::compile_shader(vertex_shader);
        println!("Vertex shader compile success!");
        Self::compile_shader(fragment_shader);
        println!("Fragment shader compile success!");

        // Chk for Error
        Self::chk_for_any_err(vertex_shader);
        println!("Vertex shader compile status - No Error!");
        Self::chk_for_any_err(fragment_shader);
        println!("Fragment shader compile status - No Error!");

        let shader_program = unsafe {
            let shader_program_id = gl::CreateProgram();
            gl::AttachShader( shader_program_id, vertex_shader );
            gl::AttachShader( shader_program_id, fragment_shader );

            gl::LinkProgram(shader_program_id);
            // gl::UseProgram(shader_program_id);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            shader_program_id
        };

        // println!("Shader program -> {:?}",shader_program);

        Shader {
            id: shader_program
        }

    }

    fn shader_source( shader:u32,shader_src:&str ) {

        let c_str = CString::new(shader_src).expect("CString Error!");
        unsafe {
            gl::ShaderSource(
                shader, 
                1, 
                &c_str.as_bytes().as_ptr().cast(), 
                &(shader_src.len().try_into().unwrap())
            );
        }
    }

    fn compile_shader( shader: u32 ){
        unsafe {
            gl::CompileShader(shader); 
        }
    }

    fn chk_for_any_err( shader:u32 ){
        let mut success = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            println!("Success -> {:?}", success);
        }
        let mut log = [ 0; 512  ];
        if success == 0 {
            unsafe { gl::GetShaderInfoLog(shader, 512, null_mut(), log.as_mut_ptr() )};
            println!("Log :-> {:?}", log);
            println!("Shader Error! {:?}", log );
        }
    }
}