use naga::{FastHashMap, front, Module, ShaderStage};
use naga::valid::{Capabilities, ModuleInfo, ValidationFlags, Validator};

pub(crate) fn validate(module: &Module) -> ModuleInfo {
    Validator::new(
        ValidationFlags::default(), // TODO: custom validation flags
        Capabilities::default(),    // TODO: custom capabilities
    )
    .validate(module)
    .expect("Failed to validate module")
}

pub(crate) fn parse_stage(input: &str) -> ShaderStage {
    match input {
        "vertex" => ShaderStage::Vertex,
        "fragment" => ShaderStage::Fragment,
        "compute" => ShaderStage::Compute,
        _ => panic!("Invalid shader stage: {}", input),
    }
}

pub(crate) fn parse_from(
    input: &str,
    lang: &str,
    stage: ShaderStage,
    defines: Option<FastHashMap<String, String>>,
) -> Module {
    match lang {
        #[cfg(feature = "from-glsl")]
        "glsl" => front::glsl::Frontend::default()
            .parse(
                &if let Some(defines) = defines {
                    front::glsl::Options { stage, defines }
                } else {
                    front::glsl::Options::from(stage)
                },
                input,
            )
            .expect("Failed to parse input as GLSL Module"),

        #[cfg(feature = "from-spv")]
        "spv" => front::spv::parse_u8_slice(
            input.as_bytes(),
            &front::spv::Options::default(), // TODO: custom options
        )
        .expect("Failed to parse input as SPIR-V Module"),

        #[cfg(feature = "from-wgsl")]
        "wgsl" => front::wgsl::parse_str(input).expect("Failed to parse input as WGSL Module"),

        _ => panic!("Unsupported shader language: {:?}", lang),
    }
}
