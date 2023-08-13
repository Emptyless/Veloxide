fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("src/interfaces/protos/helloworld.proto")?;
    Ok(())
}
