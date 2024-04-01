use naga::valid::{Capabilities, ModuleInfo, ValidationFlags, Validator};
use naga::{FastHashMap, Module, ShaderStage};

#[allow(dead_code)]
pub(crate) fn validate(module: &Module) -> ModuleInfo {
    Validator::new(
        ValidationFlags::default(), // TODO: custom validation flags
        Capabilities::default(),    // TODO: custom capabilities
    )
    .validate(module)
    .expect("Failed to validate module")
}

pub(crate) fn parse_stage(_input: &str) -> ShaderStage {
    match _input {
        "vertex" => ShaderStage::Vertex,
        "fragment" => ShaderStage::Fragment,
        "compute" => ShaderStage::Compute,
        _ => panic!("Invalid shader stage: {}", _input),
    }
}

pub(crate) fn parse_from(
    _input: &str,
    _stage: ShaderStage,
    _defines: Option<FastHashMap<String, String>>,
    lang: &str,
) -> Module {
    match lang {
        #[cfg(feature = "from-glsl")]
        "glsl" => naga::front::glsl::Frontend::default()
            .parse(
                &if let Some(defines) = _defines {
                    naga::front::glsl::Options {
                        stage: _stage,
                        defines,
                    }
                } else {
                    naga::front::glsl::Options::from(_stage)
                },
                _input,
            )
            .expect("Failed to parse input as GLSL Module"),

        #[cfg(feature = "from-spv")]
        "spv" => naga::front::spv::parse_u8_slice(
            _input.as_bytes(),
            &naga::front::spv::Options::default(), // TODO: custom options
        )
        .expect("Failed to parse input as SPIR-V Module"),

        #[cfg(feature = "from-wgsl")]
        "wgsl" => {
            naga::front::wgsl::parse_str(_input).expect("Failed to parse input as WGSL Module")
        }

        _ => panic!("Unsupported shader language: {:?}", lang),
    }
}
