extern crate proc_macro;

use proc_macro::{Literal, TokenStream, TokenTree};
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
    entry_point: Option<&'a str>,
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

    input.insert(0, '(');
    input.push(')');

    let input: Transpile =
        ron::from_str(input.as_str()).expect("Failed to parse input as RON struct");

    let stage = util::parse_stage(input.stage);

    let from = util::parse_from(
        input.source,
        input.from,
        stage,
        input
            .defines
            .map_or(None, |some| Some(FastHashMap::from_iter(some.into_iter()))),
    );

    match input.to {
        #[cfg(feature = "to-hlsl")]
        "hlsl" => TokenStream::from(TokenTree::Literal(Literal::string(
            hlsl::to_hlsl(from).as_str(),
        ))),

        #[cfg(feature = "to-wgsl")]
        "wgsl" => TokenStream::from(TokenTree::Literal(Literal::string(
            wgsl::to_wgsl(from).as_str(),
        ))),

        #[cfg(feature = "to-spv")]
        "spv" => format!("{:?}", spv::to_spv(from)).parse().unwrap(),

        #[cfg(feature = "to-glsl")]
        "glsl" => TokenStream::from(TokenTree::Literal(Literal::string(
            glsl::to_glsl(
                from,
                stage,
                input.entry_point.expect("GLSL requires an entry point"),
            )
            .as_str(),
        ))),

        #[cfg(feature = "to-msl")]
        "msl" => TokenStream::from(TokenTree::Literal(Literal::string(
            msl::to_msl(from).as_str(),
        ))),

        _ => panic!("Unsupported output language: {}", input.to),
    }
}
