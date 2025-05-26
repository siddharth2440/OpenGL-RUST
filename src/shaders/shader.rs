use std::{ffi::CString, fs, ptr::null};

use gl;
use glfw::ffi::glfwGetTime;

pub struct Shader {
    pub id: gl::types::GLuint
}


impl Shader {
    // new
    pub fn new( vertex_path:&str , fragment_path:&str ) -> Self {
        let vertex_code = match fs::read_to_string(vertex_path) {
            Ok( v_code ) => v_code,
            Err(e) => {
                eprintln!("Error!, Unable to read the Vertex Path File -> {:?} ",e);
                String::from("Error!")
            }
        };

        let fragment_code = match fs::read_to_string(fragment_path) {
            Ok( f_code ) => f_code,
            Err(e) => {
                eprintln!("Error!, Unable to read the Fragment Path File -> {:?} ",e);
                String::from("Error!")
            }
        };

        let v_shader = Self::compile_shader(&vertex_code , gl::VERTEX_SHADER);
        let f_shader = Self::compile_shader(&fragment_code, gl::FRAGMENT_SHADER);

        let shader_program = unsafe {
            gl::CreateProgram()
        };
        // Attach the Shaders and then Link it
        
        unsafe {
            gl::AttachShader(shader_program, v_shader);
            gl::AttachShader(shader_program, f_shader);
            gl::LinkProgram(shader_program);
        }

        Self::check_linking(shader_program);
        unsafe {
            gl::DeleteShader(v_shader);
            gl::DeleteShader(f_shader);
        }

        Self { id: 1 }
    }


    // compile shader
    pub fn compile_shader( shader_src:&str, shader_type:gl::types::GLenum ) -> gl::types::GLuint {
        let shader = unsafe {
            gl::CreateShader(shader_type)
        };

        unsafe {
            gl::ShaderSource( 
                shader, 
                1, 
                &shader_src.as_bytes().as_ptr().cast(), 
                null() 
            );

            gl::CompileShader(shader);
        }
        Self::check_compilation(shader);
        shader
    }

    // check compilation
    pub fn check_compilation( shader : gl::types::GLuint )  {
        let mut success = 0;
        unsafe { 
            gl::GetShaderiv(shader, gl::LINK_STATUS, &mut success); 
        }

        if success == 0 {
            let mut v1:Vec<u8> = Vec::with_capacity(1024);
            let mut log = 0_i32;
            unsafe {
                gl::GetShaderInfoLog(shader, 512, &mut log, v1.as_mut_ptr().cast() );
            }
            panic!("Error!, Shader Compilation {:?}",String::from_utf8_lossy(&v1));
        }
    }

    // check Linking
    pub fn check_linking( program: gl::types::GLuint) {
        let mut success = 0;
        unsafe {
            gl::GetShaderiv(program, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut v:Vec<u8> = Vec::with_capacity(1024);
            let mut log = 0_i32;
            unsafe {
                gl::GetShaderInfoLog(program, 512, &mut log, v.as_mut_ptr().cast());
            }

            panic!("Program Error!, --> {:?}",String::from_utf8_lossy(&v));
        }
    }
    // use Program
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
    // set uniform  <-::--::->  data transferred(CPU -> GPU)
    pub fn set_uniform4f(&self) {
        let uniform_name = CString::new("ourColor").expect("CString conversion failed");
        let uniform_location = unsafe { gl::GetUniformLocation(self.id, uniform_name.as_ptr() as *const i8) };
        let time_value = unsafe { glfwGetTime() };
        let sin_value = time_value.sin();
        let green_value = (sin_value / 2.0) + 0.5;
        
        if uniform_location != -1 {
            unsafe {
                gl::UseProgram(self.id);
                gl::Uniform4f(uniform_location, 0.0, green_value as f32, 0.7, 1.0);
            }
        }
    }
}