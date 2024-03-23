mod executor;

use clap::Parser as clapParser;

#[derive(clapParser)]
struct Args {
    /// Path to a file
    file_path: std::path::PathBuf,

    /// Path to an svg or png image
    image_path: std::path::PathBuf,

    /// Height
    height: u32,

    /// Width
    width: u32,
}

fn main() -> Result<(), ()> {
    let args: Args = Args::parse();

    // Access the parsed arguments
    let file_path = args.file_path;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;

    match file_path.extension().map(|s| s.to_str()).flatten() {
        Some("lg") => {}
        _ => {
            eprintln!("source file extension not supported");
            return Err(());
        }
    }

    let mut executor = executor::ExecutorFactory::create_turtle(100, 100, image_path);
    executor.pen_down();
    executor.foreward(100.0);
    Ok(())
}
