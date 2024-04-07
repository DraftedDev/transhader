extern crate proc_macro;

use proc_macro::TokenStream;
use std::collections::HashMap;

use naga::FastHashMap;
use serde::Deserialize;

#[cfg(feature = "to-glsl")]
mod glsl;
#[cfg(feature = "to-hlsl")]
mod hlsl;
#[cfg(feature = "to-msl")]
mod msl;
#[cfg(feature = "to-spv")]
mod spv;
mod util;
#[cfg(feature = "to-wgsl")]
mod wgsl;

#[derive(Deserialize)]
struct Transpile<'a> {
    _entry_point: Option<&'a str>,
    stage: &'a str,
    source: &'a str,
    from: &'a str,
    to: &'a str,
    defines: Option<HashMap<String, String>>,
}

/// # Transpilation Macro
///
/// Procedural macro for transpiling shaders from one language to another.
///
/// For a list of supported languages see the [documentation](https://github.com/drafteddev/transhader).
///
/// Input must be valid RON (Rusty-Object-Notation).
///
/// Depending on the target language, this may return a string or a u32 vector (for SPIR-V).
///
/// ## Configuration:
///
/// - `entry_point` - Name of the entry point function of the shader. Only required for specific languages.
/// - `stage` - Shader stage.
/// - `source` - Source code to transpile.
/// - `from` - Source language.
/// - `to` - Target language to transpile to.
/// - `defines` - Defines to pass to the transpiler. Only required for certain languages and shaders.
///
/// ## Example
///
/// ```
/// #[cfg(any(feature = "from-glsl", feature = "to-hlsl"))]
/// transhader::transpile!(
///     stage: "vertex",
///     source: r#"
///     #version 450
///
///     void main() {
///         gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
///     }
///     "#,
///     from: "glsl",
///     to: "hlsl",
///     defines: None,
///     entry_point: None,
///  );
/// ```
#[proc_macro]
pub fn transpile(input: TokenStream) -> TokenStream {
    let mut input = input.to_string();

    // add parentheses to make the macro syntax less "confusing"
    input.insert(0, '(');
    input.push(')');

    let input: Transpile =
        ron::from_str(input.as_str()).expect("Failed to parse input as RON struct");

    let stage = util::parse_stage(input.stage);

    #[allow(clippy::redundant_closure)]
    let _from = util::parse_from(
        input.source,
        stage,
        input.defines.map(|some| FastHashMap::from_iter(some)),
        input.from,
    );

    match input.to {
        #[cfg(feature = "to-hlsl")]
        "hlsl" => TokenStream::from(proc_macro::TokenTree::Literal(proc_macro::Literal::string(
            hlsl::to_hlsl(_from).as_str(),
        ))),

        #[cfg(feature = "to-wgsl")]
        "wgsl" => TokenStream::from(proc_macro::TokenTree::Literal(proc_macro::Literal::string(
            wgsl::to_wgsl(_from).as_str(),
        ))),

        #[cfg(feature = "to-spv")]
        "spv" => format!("{:?}", spv::to_spv(_from)).parse().unwrap(),

        #[cfg(feature = "to-glsl")]
        "glsl" => TokenStream::from(proc_macro::TokenTree::Literal(proc_macro::Literal::string(
            glsl::to_glsl(
                _from,
                stage,
                input._entry_point.expect("GLSL requires an entry point"),
            )
            .as_str(),
        ))),

        #[cfg(feature = "to-msl")]
        "msl" => TokenStream::from(proc_macro::TokenTree::Literal(proc_macro::Literal::string(
            msl::to_msl(_from).as_str(),
        ))),

        _ => panic!("Unsupported output language: {}", input.to),
    }
}

/// # Transpilation Macro
///
/// Procedural macro for transpiling shaders from one language to another.
///
/// Unlike the [transpile] macro, this macro takes a file path as input and transpiles its content
///
/// For a list of supported languages see the [documentation](https://github.com/drafteddev/transhader).
///
/// Input must be valid RON (Rusty-Object-Notation).
///
/// Depending on the target language, this may return a string or a u32 vector (for SPIR-V).
///
/// ## Configuration:
///
/// - `entry_point` - Name of the entry point function of the shader. Only required for specific languages.
/// - `stage` - Shader stage.
/// - `source` - Source file to transpile.
/// - `from` - Source language.
/// - `to` - Target language to transpile to.
/// - `defines` - Defines to pass to the transpiler. Only required for certain languages and shaders.
///
/// ## Example
///
/// ```
/// #[cfg(any(feature = "from-glsl", feature = "to-hlsl"))]
/// transhader::transpile!(
///     stage: "vertex",
///     source: "my_shader.glsl",
///     from: "glsl",
///     to: "hlsl",
///     defines: None,
///     entry_point: None,
///  );
/// ```
#[proc_macro]
pub fn transpile_file(input: TokenStream) -> TokenStream {
    let mut input = input.to_string();

    // add parentheses to make the macro syntax less "confusing"
    input.insert(0, '(');
    input.push(')');

    let mut input: Transpile =
        ron::from_str(input.as_str()).expect("Failed to parse input as RON struct");

    let stage = util::parse_stage(input.stage);

    let file = std::fs::read_to_string(input.source).expect("Failed to read source file");

    // change input path to content
    input.source = file.as_str();

    #[allow(clippy::redundant_closure)]
    let _from = util::parse_from(
        input.source,
        stage,
        input.defines.map(|some| FastHashMap::from_iter(some)),
        input.from,
    );

    match input.to {
        #[cfg(feature = "to-hlsl")]
        "hlsl" => TokenStream::from(proc_macro::TokenTree::Literal(proc_macro::Literal::string(
            hlsl::to_hlsl(_from).as_str(),
        ))),

        #[cfg(feature = "to-wgsl")]
        "wgsl" => TokenStream::from(proc_macro::TokenTree::Literal(proc_macro::Literal::string(
            wgsl::to_wgsl(_from).as_str(),
        ))),

        #[cfg(feature = "to-spv")]
        "spv" => format!("{:?}", spv::to_spv(_from)).parse().unwrap(),

        #[cfg(feature = "to-glsl")]
        "glsl" => TokenStream::from(proc_macro::TokenTree::Literal(proc_macro::Literal::string(
            glsl::to_glsl(
                _from,
                stage,
                input._entry_point.expect("GLSL requires an entry point"),
            )
            .as_str(),
        ))),

        #[cfg(feature = "to-msl")]
        "msl" => TokenStream::from(proc_macro::TokenTree::Literal(proc_macro::Literal::string(
            msl::to_msl(_from).as_str(),
        ))),

        _ => panic!("Unsupported output language: {}", input.to),
    }
}
