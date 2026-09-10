#version 330 core


layout(location = 1) in vec3 Normal;

out vec4 Color;

void main()
{
    Color = vec4(Normal, 1.);
}
