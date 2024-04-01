use naga::back::spv::{Options, Writer};
use naga::Module;

use crate::util::validate;

pub(crate) fn to_spv(module: Module) -> Vec<u32> {
    let mut out: Vec<u32> = Vec::new();

    Writer::new(&Options::default())
        .expect("Failed to create SPIR-V Writer")
        .write(
            &module,
            &validate(&module),
            None,  // TODO: custom options
            &None, // TODO: custom debug info
            &mut out,
        )
        .expect("Failed to write SPIR-V Module");

    out
}
