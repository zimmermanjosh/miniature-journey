// import necessary modules
use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};
use std::{thread, time::Duration};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The pattern to search for.
    #[arg(short = 'p', long, value_name = "foobar")]
    pattern: String,

    /// The file path to search in || the path to the file to read.
    #[arg(short = 'f', long, value_name = "~/test.txt")]
    path: std::path::PathBuf
}
fn main() -> Result<(), anyhow::Error> {//+
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    })
    .expect("Error setting Ctrl-C handler");

    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");

    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        //do_hard_work();

        let args: Cli = Cli::parse();//+

        let content = std::fs::read_to_string(&args.path)//+
            .with_context(|| format!("could not read file `{}`", args.path.display()))?;//+

        for line in content.lines() {//+
            if line.contains(&args.pattern) {//+
                println!("{}", line);//+
            }//+
        }
        pb.println(format!("[+] finished #{}", i));//+
        pb.inc(1);
    }
    pb.finish_with_message("done");
    Ok(())//+
}
