#version 330 core

uniform sampler2D Texture;

in s_frag {
    vec3 color;
    vec2 uv;
    vec4 position;
} FRAG;

out vec4 Color;

void main()
{
    Color = texture(Texture, FRAG.uv);
    // Color = texture(Texture, FRAG.uv) * vec4(FRAG.color, 1.0);
    // Color = vec4(vec3((int((FRAG.uv.x * 2.)) ^ int((FRAG.uv.y * 2.))) / 2.), 1.);
}
