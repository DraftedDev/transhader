use naga::back::hlsl::{Options, Writer};
use naga::Module;

use crate::util::validate;

pub(crate) fn to_hlsl(module: Module) -> String {
    let mut out = String::new();

    Writer::new(
        &mut out,
        &Options::default(), // TODO: custom options
    )
    .write(&module, &validate(&module))
    .expect("Failed to write HLSL Module");

    out
}
