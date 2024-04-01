use naga::back::msl::{Options, PipelineOptions, Writer};
use naga::Module;

use crate::util::validate;

pub(crate) fn to_msl(module: Module) -> String {
    let mut out = String::new();

    Writer::new(&mut out)
        .write(
            &module,
            &validate(&module),
            &Options::default(),         // TODO: custom options
            &PipelineOptions::default(), // TODO: custom pipeline options
        )
        .expect("Failed to write MSL");

    out
}
