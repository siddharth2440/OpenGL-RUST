use gl;

pub struct Texture {
    pub id: gl::types::GLuint
}

impl Texture {
    fn _new() {
        let mut texture:u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S , gl::MIRRORED_REPEAT as i32 );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T , gl::MIRRORED_REPEAT as i32 );
            
        }

    }
}