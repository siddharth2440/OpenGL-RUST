#version 330 core
out vec4 FragColor;
uniform vec4 llgm_uniform_color;
void main() {
    // FragColor = vec4(0.9, 0.0, 0.0, 1.0);
    FragColor = llgm_uniform_color;
}