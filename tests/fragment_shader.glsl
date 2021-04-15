#version 330 core
in vec4 fragmentColor;
in vec2 fragmentTexCoords;

uniform sampler2D texture0;

out vec4 FragColor;

void main() {
    FragColor = texture(texture0, fragmentTexCoords) * fragmentColor;
}