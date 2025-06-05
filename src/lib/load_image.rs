use image::GenericImageView;

pub fn load_image_into_cpu( image_path: &str  ) -> (i32,i32 , Vec<u8>) {
    let image = image::open(image_path).expect("Unable to Load Image");
    let image_buffer = image.flipv().into_rgba8();
    let image_data = image_buffer.as_raw();

    let ( width , height  ) = image.dimensions();
    ( width as i32 , height as i32, image_data.to_owned()  )
}