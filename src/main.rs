// import necessary modules
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The pattern to search for.
    #[arg(short = 'p', long, value_name = "foobar")]
    pattern: String,
    /// The file path to search in || the path to the file to read.
    #[arg(short = 'f', long, value_name = "~/test.txt")]
    path: std::path::PathBuf,
}

fn main() {
    //use the clap crate to parse command-line arguments
    let args: Cli = Cli::parse();

    //open the file for reading
    let content: String = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
}
