#version 330 core
layout (location = 0) in vec3 vertexPosition;
layout (location = 1) in vec4 vertexColor;
layout (location = 2) in vec2 texCoords;

out vec4 fragmentColor;
out vec2 fragmentTexCoords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    fragmentColor = vertexColor;
    fragmentTexCoords = texCoords;
    gl_Position = projection * view * model * vec4(vertexPosition, 1);
}