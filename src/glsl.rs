use naga::{Module, ShaderStage};
use naga::back::glsl::{Options, PipelineOptions, Writer};
use naga::proc::BoundsCheckPolicies;

use crate::util::validate;

pub(crate) fn to_glsl(module: Module, stage: ShaderStage, entry_point: &str) -> String {
    let mut out = String::new();

    Writer::new(
        &mut out,
        &module,
        &validate(&module),
        &Options::default(), // TODO: custom options
        &PipelineOptions {
            shader_stage: stage,
            entry_point: entry_point.to_string(),
            multiview: None, // TODO: customize multiview
        },
        BoundsCheckPolicies::default(), // TODO: custom policies
    )
    .expect("Failed to generate GLSL output")
    .write()
    .expect("Failed to write GLSL output");

    out
}
