// Build script for compiling protobuf files using tonic_prost_build.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile the protobuf files
    tonic_prost_build::compile_protos("proto/helloworld.proto")?;

    // End the process
    Ok(())
}
