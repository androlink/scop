#version 330 core

uniform sampler2D Texture;

in vec3 FragmentColor;
in vec2 uv;

out vec4 Color;

void main()
{
    Color = texture(Texture, uv) * vec4(FragmentColor, 1.);
}
