use std::env;

pub mod compiler;
pub mod downloader;

pub const BUILD_DIRECTORY: &str = "build/";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: s++ [project_id]");
        return Ok(());
    }

    let project_id = args[1].clone();

    downloader::download(&project_id)?;
    compiler::compile(&project_id);

    Ok(())
}
