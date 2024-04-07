fn main() {
    #[cfg(all(feature = "from-glsl", feature = "to-hlsl"))]
    {
        println!(
            "\n\nGLSL-to-HLSL:\n{}",
            transhader::transpile_file!(
                stage: "vertex",
                source: "examples/my_shader.glsl",
                from: "glsl",
                to: "hlsl",
                defines: None,
                entry_point: None,
            )
        );
    }
}
