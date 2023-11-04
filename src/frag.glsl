#version 330 core
out vec4 frag_colour;
  
in vec4 vert_colour;

void main()
{
    frag_colour = vert_colour;
} 