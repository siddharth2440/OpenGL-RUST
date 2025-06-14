use glm::{ext::{look_at, perspective}, vec3, Mat4, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub world_up: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(position: Vec3, up: Vec3, yaw: f32, pitch: f32) -> Self {
        let mut camera = Camera {
            position,
            front: vec3(0.0, 0.0, -1.0),
            up,
            right: vec3(0.0, 0.0, 0.0),
            world_up: up,
            yaw,
            pitch,
            movement_speed: 2.5,
            mouse_sensitivity: 0.1,
            zoom: 45.0,
        };
        camera.update_camera_vectors();
        camera
    }

    fn update_camera_vectors(&mut self) {
        let front = glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );

        self.front = glm::normalize(front);
        self.right = glm::normalize(glm::cross(self.front, self.world_up));
        self.up = glm::normalize(glm::cross(self.right, self.front));
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        look_at(self.position, self.position + self.front, self.up)
    }

    pub fn get_projection_matrix(&self, aspect_ratio: f32) -> Mat4 {
        perspective(aspect_ratio, self.zoom.to_radians(), 0.1, 100.0)
    }

    pub fn process_keyboard(&mut self, direction: &str, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;
        match direction {
            "FORWARD" => self.position = self.position - (self.front * velocity),
            "BACKWARD" => self.position = self.position - (self.front * velocity),
            "LEFT" => self.position = self.position - (self.right * velocity),
            "RIGHT" => self.position = self.position + ( self.right * velocity),
            _ => {}
        }
    }

    pub fn process_mouse_movement(&mut self, xoffset: f32, yoffset: f32) {
        self.yaw += xoffset * self.mouse_sensitivity;
        self.pitch += yoffset * self.mouse_sensitivity;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        self.update_camera_vectors();
    }
}
