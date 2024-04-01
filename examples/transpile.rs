
fn main() {
    #[cfg(all(feature = "from-glsl", feature = "to-hlsl"))]
    {
        println!(
            "\n\nGLSL-to-HLSL:\n{}",
            transhader::transpile!(
                stage: "vertex",
                source: r#"
                #version 450

                void main() {
                    gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
                }
            "#,
                from: "glsl",
                to: "hlsl",
                defines: None,
                entry_point: None,
            )
        );
    }

    #[cfg(all(feature = "from-glsl", feature = "to-spv"))]
    {
        println!(
            "\n\nGLSL-to-SPIRV:\n{:?}",
            transhader::transpile!(
                stage: "vertex",
                source: r#"
                #version 450

                void main() {
                    gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
                }
            "#,
                from: "glsl",
                to: "spv",
                defines: None,
                entry_point: None,
            )
        );
    }

    #[cfg(all(feature = "from-glsl", feature = "to-msl"))]
    {
        println!(
            "\n\nGLSL-to-MSL:\n{}",
            transhader::transpile!(
                stage: "vertex",
                source: r#"
                #version 450

                void main() {
                    gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
                }
            "#,
                from: "glsl",
                to: "msl",
                defines: None,
                entry_point: None,
            )
        );
    }
}
