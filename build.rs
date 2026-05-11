use std::env;
use std::error::Error;
use vergen::Emitter;

fn main() -> Result<(), Box<dyn Error>> {
    let target = env::var("TARGET").unwrap();

    if !target.contains("netbsd") {
        Emitter::default().emit()?;
    }

    Ok(())
}
