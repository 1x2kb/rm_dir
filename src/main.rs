use std::{
    io::{stdin, stdout, BufRead, Write},
    path::{Path, PathBuf},
    time::Instant,
};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The source directory to operate on.
    #[arg(value_name = "SOURCE_PATH")]
    source_dir: String,
}

fn main() {
    let opts = Cli::parse();

    let dir_to_remove = std::fs::canonicalize(opts.source_dir).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let confirmation = get_user_confirmation(&dir_to_remove).trim().to_lowercase();

    if confirmation != "y" {
        println!("Aborting as user input '{confirmation}' was not 'y'");
        return;
    }

    let now = Instant::now();
    let result = remove_dir_all(&dir_to_remove);

    match result {
        Ok(_) => println!(
            "Removed all folders and files from {}",
            dir_to_remove.to_string_lossy()
        ),
        Err(e) => println!("Error: {}", e),
    }

    println!("Done in {}s", now.elapsed().as_secs_f32());
}

fn get_user_confirmation(source_dir: &PathBuf) -> String {
    print!(
        "Are you sure you want to delete all files and folders in {}? (y/n) ",
        source_dir.to_string_lossy()
    );

    stdout()
        .flush()
        .unwrap_or_else(|e| panic!("Failed to flush stdout {}", e));

    let mut user_input = String::new();
    stdin()
        .lock()
        .read_line(&mut user_input)
        .unwrap_or_else(|e| panic!("Failed to read user input {}", e));

    user_input
}

fn remove_dir_all(dir_to_remove: &Path) -> Result<(), std::io::Error> {
    std::fs::remove_dir_all(dir_to_remove)
}
