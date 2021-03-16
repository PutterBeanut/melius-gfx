#version 330 core
in vec4 fragmentColor;
in vec2 fragmentTexCoords;

uniform sampler2D texture0;
uniform float time;

void main() {
    gl_FragColor = texture(texture0, fragmentTexCoords) * fragmentColor * time;
}