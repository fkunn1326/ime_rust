use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(
        &[
            "proto/common.proto",
            "proto/tsf.proto",
            "proto/window.proto",
            "proto/converter.proto",
        ],
        &["proto/"],
    )?;
    Ok(())
}
