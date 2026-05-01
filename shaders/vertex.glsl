#version 330 core
in vec4 Position;
in vec3 Color;
uniform mat4 Mvp;

out VS_OUTPUT {
    vec3 Color;
} OUT;

void main()
{
    gl_Position = Mvp * Position;
    OUT.Color = Color;
}
