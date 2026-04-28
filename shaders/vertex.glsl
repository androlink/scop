#version 330 core
in vec3 Position;
in vec3 Color;
uniform mat4 Mvp;

out VS_OUTPUT {
    vec3 Color;
} OUT;

void main()
{
    gl_Position = Mvp * vec4(Position, 1.0);
    OUT.Color = Color;
}
