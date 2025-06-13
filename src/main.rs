use std::{ffi::CString, fs::File, io::Read, process, ptr::null, time::{SystemTime, UNIX_EPOCH}};
use glfw::{fail_on_errors, ffi::glfwGetTime, Action, Context, Key};
use glm::{ext::{rotate, translate}, mat4, Mat4};
use image::GenericImageView;

use crate::{lib::load_image::load_image_into_cpu, shaders::shader::Shader, textures::texture::Texture, utils::coordinates::{TexturePath, Vertices}};

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

fn main() {

    type _VertexObject = ( u32, u32, u32);
    type ShapeVerticesType = [f32; 12];
    type ColorVerticesType = [f32; 12];
    type TextureVerticesType = [f32; 8];
    type IndexType = [u32; 6];

    let mut glfw = glfw::init(fail_on_errors!()).expect("Failed to initialize GLFW");
    let (mut window, events) = glfw
        .create_window(1200, 900, "Textured Shape", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| window.get_proc_address(s));

    let vertex_shader_path = "src/glsl/texture_v.vert";
    let fragment_shader_path = "src/glsl/texture_f.frag";

    let mut vertex_code = String::new();
    File::open(vertex_shader_path)
        .expect("Failed to read vertex shader")
        .read_to_string(&mut vertex_code)
        .unwrap();

    let mut fragment_code = String::new();
    File::open(fragment_shader_path)
        .expect("Failed to read fragment shader")
        .read_to_string(&mut fragment_code)
        .unwrap();

    let shader = Shader::new(&vertex_code, &fragment_code);
    
    let mut x_offset: f32 = -1.0; // Start from the left
    let speed: f32 = 0.001; // Adjust speed

    let mut angle: f32 = 0.0; // Initial rotation angle
    let rotation_speed: f32 = 0.001; // Adjust speed

    #[derive(Debug)]
    struct ShapeVertices {
        shape_vertices: [f32; 12],
        shape_color_vertices: [f32; 12],
        shape_texture_vertices: [f32; 8],
        shape_indices: IndexType
    }

    // ------------------------------------------------   Coordinates ------------------------------------------   
    let spacing = 0.0;
    // Rectangle
    let vertices: [f32; 32] = [
        // positions      // colors       // texture coords
         0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
         0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
        -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left
    ];

    let mut shapes: Vec<ShapeVertices> = Vec::new();

    let _rectange_vertices: ShapeVerticesType = [
         -0.25, -0.25, 0.0,  // bottom left
             0.25, -0.25, 0.0,  // bottom right
             0.25,  0.25, 0.0,  // top right
            -0.25,  0.25, 0.0   // top left
    ];

    let _rectangle_color: ColorVerticesType = [
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
        1.0, 1.0, 0.0
    ];

    let _rectangle_texs: TextureVerticesType = [
        0.0, 0.0, 
        1.0, 0.0, 
        1.0, 1.0, 
        0.0, 1.0
    ];

    //REctangle
    let indices: IndexType = [
        0, 1, 2, 
        2, 3, 0
    ];

    shapes.push(ShapeVertices{
        shape_color_vertices: _rectangle_color,
        shape_texture_vertices: _rectangle_texs,
        shape_vertices: _rectange_vertices,
        shape_indices: indices
    });

    // Rhombus 
    let _rhombus_vertices: ShapeVerticesType = [
         -0.3, -0.3, 0.0,
             0.3, -0.3, 0.0,
             0.4,  0.3, 0.0,
            -0.2,  0.3, 0.0,
    ];

    let _rhombus_tex_coordinates: TextureVerticesType = [
        0.0, 0.0, 
        1.0, 0.0, 
        1.0, 1.0, 
        0.0, 1.0
    ];

    let _rhombus_colors: ColorVerticesType = [
        1.0, 0.0, 0.0,  // Red
        0.0, 1.0, 0.0,  // Green
        0.0, 0.0, 1.0,  // Blue
        1.0, 1.0, 0.0   // Yellow
    ];

    let _rhombus_indices: IndexType = [
        0, 1, 2, 
        2, 3, 0
    ];

    shapes.push(ShapeVertices{
        shape_color_vertices: _rhombus_colors,
        shape_texture_vertices: _rhombus_tex_coordinates,
        shape_vertices: _rhombus_vertices,
        shape_indices: _rhombus_indices
    });

    // Parallelogram 
    let _parallelogram_vertics: ShapeVerticesType = [
            -0.1, -0.1, 0.0,
             0.1, -0.1, 0.0,
             0.2,  0.1, 0.0,
            -0.3,  0.1, 0.0,
    ];

    let _parallelogram_tex_coordinates: TextureVerticesType = [
        0.0, 0.0, 
        1.0, 0.0, 
        1.0, 1.0, 
        0.0, 1.0
    ];

    let _parallelogram_colors: ColorVerticesType = [
        0.5, 0.0, 0.5,
        0.0, 0.5, 0.5,
        0.5, 0.5, 0.0,
        1.0, 0.0, 0.5
    ];

    let _parallelogram_indices: IndexType = [
        0, 1, 2,
        2, 3, 0,
    ];

    shapes.push(ShapeVertices{
        shape_color_vertices: _parallelogram_colors,
        shape_texture_vertices: _parallelogram_tex_coordinates,
        shape_vertices: _parallelogram_vertics,
        shape_indices: _parallelogram_indices
    });

    #[derive(Debug)]
    struct Object {
        shape_vertices_vbo: u32, 
        color_vertices_vbo: u32, 
        texture_vertices_vbo: u32, 
        shape_ebo: u32, 
        shape_vao: u32
    }

    // 3 shapes To draw and bind them to our textues
    let mut vertexbuffers:Vec<Object> = Vec::new();
    vertexbuffers.push( Object { 
        color_vertices_vbo:0, shape_vertices_vbo:0, texture_vertices_vbo:0 , shape_ebo:0, shape_vao:0
    });
    vertexbuffers.push( Object { 
        color_vertices_vbo:0, shape_vertices_vbo:0, texture_vertices_vbo:0 , shape_ebo:0, shape_vao:0  
    });
    vertexbuffers.push( Object { 
        color_vertices_vbo:0, shape_vertices_vbo:0, texture_vertices_vbo:0 , shape_ebo:0, shape_vao:0 
    });

    let total_objects = vertexbuffers.iter().fold(0, | acc , _x | acc + 1 );
    println!("Total Vertex Objects {:?}", total_objects);
    if total_objects == 3 {
        println!("Vertex Buffers Initialized Properly! -> {:?}",total_objects);
    } else {
        println!(" Vertex Buffers is not initialized properly ");
        process::exit(1);
    }
    
    let _texes = vec![ "wall.jpg", "texture.jpg" ];
    let _texture_paths = TexturePath::new(_texes);

    let _by_direct_rectangle_vertices= Vertices::new(&vertices, &indices);

    let mut index = 0;
    let mut shape_idx: usize = 0;
    // Create our buffers
    let mut obj_iter = vertexbuffers.iter_mut();
    while let Some( obj ) = obj_iter.next() {
        unsafe {
            // ---------------------------------------------- Generate Vertex Buffers -----------------------------------------------------------------------

            // Vertex Array Object
            gl::GenVertexArrays(1, &mut obj.shape_vao);
            
            // Vertex Buffer Object
            gl::GenBuffers(1, &mut obj.color_vertices_vbo);
            gl::GenBuffers(1, &mut obj.shape_vertices_vbo);
            gl::GenBuffers(1, &mut obj.texture_vertices_vbo);

            // Element Buffer Object -> EBO
            gl::GenBuffers(1, &mut obj.shape_ebo);


            // Bind Buffers
            gl::BindVertexArray(obj.shape_vao);

            // Position VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, obj.shape_vertices_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                std::mem::size_of_val( &shapes[shape_idx].shape_vertices ) as isize, 
                (&shapes[shape_idx].shape_vertices).as_ptr() as *const _, 
                gl::STATIC_DRAW
            );
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, null());
            gl::EnableVertexAttribArray(0);

            index = index + 1;
            // Color VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, obj.color_vertices_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                std::mem::size_of_val(&shapes[shape_idx].shape_color_vertices) as isize, 
                (&shapes[shape_idx].shape_color_vertices).as_ptr() as *const _, 
                gl::STATIC_DRAW
            );
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, null());
            gl::EnableVertexAttribArray(1);

            index = index + 1;
            // Texture VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, obj.texture_vertices_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                std::mem::size_of_val(&shapes[shape_idx].shape_texture_vertices) as isize, 
                (&shapes[shape_idx].shape_texture_vertices).as_ptr() as *const _, 
                gl::STATIC_DRAW
            );
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 0, null());
            gl::EnableVertexAttribArray(2);

            // Element Buffer
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, obj.shape_ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER, 
                std::mem::size_of_val(&shapes[shape_idx].shape_indices) as isize, 
                (&shapes[shape_idx].shape_indices).as_ptr() as *const _, 
                gl::STATIC_DRAW
            );

            index = index + 1;
            shape_idx = shape_idx + 1;
        }
    }

    vertexbuffers.iter().for_each( | obj | {
        println!("VertexObject -> {:#?}", obj);
    });

    let texture = Texture::new("G:\\OpenGL-YT\\openglyt\\src\\assets\\wall.jpg");
    println!("Texture id -> {:?}", texture.id );
    let texture_2 = Texture::new("G:\\OpenGL-YT\\openglyt\\src\\assets\\texture.jpg");
    println!("Texture_2 id -> {:?}", texture_2.id );


    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true);
            }
        }

        unsafe {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);

    gl::UseProgram(shader.id);

    // Ensure texture unit 0 is used
    gl::ActiveTexture(gl::TEXTURE0);
    let tex_loc = CString::new("texture1").unwrap();
    let tex_uniform = gl::GetUniformLocation(shader.id, tex_loc.as_ptr());
    gl::Uniform1i(tex_uniform, 0); // Use GL_TEXTURE0

    for (i, obj) in vertexbuffers.iter().enumerate() {
        gl::BindVertexArray(obj.shape_vao);

        // Bind texture for the shape
        if i % 2 == 0 {
            gl::BindTexture(gl::TEXTURE_2D, texture.id); // wall.jpg
        } else {
            gl::BindTexture(gl::TEXTURE_2D, texture_2.id); // texture.jpg
        }

        // Create transformation
        let x_offset = -0.7 + i as f32 * 0.7;
        let y_offset = 0.0;

        let mut transform = mat4(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        transform = translate(&transform, glm::vec3(x_offset, y_offset, 0.0));

        let get_transform_name = CString::new("transform").unwrap();
        let get_transform_location = gl::GetUniformLocation(shader.id, get_transform_name.as_ptr());
        gl::UniformMatrix4fv(get_transform_location, 1, gl::FALSE, transform.as_array_mut().as_mut_ptr() as *const _);

        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());
    }
}
        window.swap_buffers();
    }
}
