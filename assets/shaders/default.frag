#version 330 core

in vec3 FragmentColor;

out vec4 Color;

void main()
{
    Color = vec4(FragmentColor, 1.f);
}
