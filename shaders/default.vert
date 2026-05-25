#version 330 core
in vec4 Position;
in vec3 Color;
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out VS_OUTPUT {
    vec3 Color;
} OUT;

void main()
{
    gl_Position = projection * view * model * Position;
    OUT.Color = Color;
}
