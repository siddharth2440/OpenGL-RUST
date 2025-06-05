use gl;

use crate::lib::load_image::load_image_into_cpu;

pub struct Texture {
    pub id: gl::types::GLuint
}

impl Texture {
    pub fn new( image_path: &str ) -> Self {
        let mut texture:u32 = 0;
        unsafe {

            // load image
            let ( width, height, image_data ) = load_image_into_cpu(image_path);
            println!("Width -> {:?}",width );
            println!("Height -> {:?}",height );
            // println!("Image Data -> {:?}",image_data );

            // Generate A Textures
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            // Configure the S-T axis
            Self::config_s_t_axis();

            // Configure the minificaion and magification
            Self::min_mag_configs();

            // Save image into our CPU
            Self::set_img_in_our_texture(width, height, image_data);

            // Generate MipMaps
            gl::GenerateMipmap(gl::TEXTURE_2D);

            Self {
                id: texture
            }
        }
    }

    pub fn min_mag_configs() {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
    }

    pub fn config_s_t_axis() {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        }
    }

    pub fn set_img_in_our_texture( width: i32,height: i32, data:Vec<u8>) {
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D, 
                0,
                gl::RGBA as i32, 
                width, 
                height, 
                0, 
                gl::RGBA, 
                gl::UNSIGNED_BYTE, 
                data.as_ptr() as *const _);
        }
    }
}