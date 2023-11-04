#version 330 core
in vec3 vert_pos;

out vec4 vert_colour;

void main()
{
    gl_Position = vec4(vert_pos, 1.0);
    vert_colour = vec4(0.5, 0.0, 0.0, 1.0);
}