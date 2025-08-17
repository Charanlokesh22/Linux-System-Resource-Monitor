fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(&["proto/system.proto"], &["proto"])?;
    println!("cargo:rerun-if-changed=proto/system.proto");
    Ok(())
}
