use naga::back::wgsl;
use naga::back::wgsl::WriterFlags;
use naga::Module;

use crate::util::validate;

pub(crate) fn to_wgsl(module: Module) -> String {
    wgsl::write_string(
        &module,
        &validate(&module),
        WriterFlags::all(), // TODO: custom writer flags
    )
    .expect("Failed to write WGSL output")
}
