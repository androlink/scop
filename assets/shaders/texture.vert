#version 330 core

layout(location = 0) in vec3 Position;
layout(location = 1) in vec3 Normal;
layout(location = 2) in vec2 Texcoord;
layout(location = 3) in vec3 Color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
out s_frag {
    vec3 color;
    vec2 uv;
    vec4 position;
} FRAG;

void main()
{
    gl_Position = projection * view * model * vec4(Position, 1.0);
    FRAG.color = Color;
    FRAG.uv = Texcoord;
    FRAG.position = gl_Position;
}
