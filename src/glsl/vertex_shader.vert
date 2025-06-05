#version 330 core
layout (location = 0) in vec3 aPos;
out vec4 vColor;
void main() {
    gl_Position = vec4(aPos, 1.0);
    vColor = vec4(0.9, 0.0, 0.0, 1.0);
}