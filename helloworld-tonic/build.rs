fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("build.rs");
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}