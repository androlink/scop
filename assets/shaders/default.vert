#version 330 core

layout(location = 0) in vec4 Position;
layout(location = 1) in vec3 Normal;
layout(location = 2) in vec3 Texture;
layout(location = 3) in vec4 Color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec4 FragmentColor;

void main()
{
    gl_Position = projection * view * model * Position;
    FragmentColor = Color;
}
