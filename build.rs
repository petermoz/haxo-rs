use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-env-changed=VERGEN_GIT_DESCRIBE");

    // Use vergen env if passed directly (e.g. in cross builds, where the
    // container has no usable git).
    if let Ok(describe) = std::env::var("VERGEN_GIT_DESCRIBE") {
        println!("cargo:rustc-env=VERGEN_GIT_DESCRIBE={}", describe);
    } else {
        EmitBuilder::builder().all_git().emit()?;
    }
    Ok(())
}
