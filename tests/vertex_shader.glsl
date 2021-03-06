#version 330 core
layout (location = 0) in vec3 vertexPosition;
layout (location = 1) in vec4 vertexColor;
layout (location = 2) in vec2 texCoords;

out vec4 fragmentColor;
out vec2 fragmentTexCoords;

void main() {
    fragmentColor = vertexColor;
    fragmentTexCoords = texCoords;
    gl_Position = vec4(vertexPosition, 1);
}