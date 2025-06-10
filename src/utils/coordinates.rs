// Vertices

pub struct Vertices{
    pub vertices: [f32; 32],
    pub indices: [u32; 6]
}

impl Vertices {
    pub fn new( vertices: &[f32;32], indices: &[u32;6] ) -> Self {
        Self { vertices: *vertices, indices: *indices }
    }
}

// Textures
pub struct TexturePath{
    pub texture_paths: Vec<String>
}

impl TexturePath {
    pub fn new( texture_names: Vec<&str> ) -> Self {
        let texture_base_path:&str = "G:\\OpenGL-YT\\openglyt\\src\\assets\\";
        let paths = texture_names
                        .iter()
                        .map( | texture_name | {
                            let tex = texture_name.to_string();
                            texture_base_path.to_string().push_str(&tex);
                            tex
                        }  )
                        .collect::<Vec<String>>();
        
        return Self {
            texture_paths: paths,
        };
    }
}