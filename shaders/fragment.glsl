#version 330 core

out vec4 FragColor;

uniform float time;

void main()
{
    // Determine the position of the fragment (pixel) on the screen
    vec2 uv = gl_FragCoord.xy / vec2(800.0, 600.0); // Replace 800 and 600 with your window dimensions

    // Add time-based animation to create a wiggle effect
    float wiggle = sin(time) * 0.1; // Adjust the amplitude (0.1) for the wiggle effect

    // Apply the wiggle effect to the UV coordinates
    uv += vec2(wiggle, wiggle);

    // Calculate the color components based on the modified UV coordinates
    float red = abs(sin(uv.x * 3.14159 * 2.0));
    float green = abs(sin((uv.x + uv.y) * 3.14159 * 2.0));
    float blue = abs(sin(uv.y * 3.14159 * 2.0));

    // Output the final color
    FragColor = vec4(red, green, blue, 1.0);
}
