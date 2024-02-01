use std::path::PathBuf;

#[derive(Parser)]
#[command(author = "maiboyer (aka Maix)", version = "2.0", about = "Generate source files from defintion and templates", long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    #[arg(value_name = "INPUT_FILE", default_value = "./input.toml")]
    pub input_file: PathBuf,

    #[arg(
        short = 'o',
        long = "output",
        value_name = "OUT_DIR",
        default_value = "./output"
    )]
    pub output_dir: PathBuf,
}
