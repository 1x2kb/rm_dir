use core::panic;
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

    /// Flag to force delete without confirmation.
    #[arg(short, long, action)]
    force: bool,
}

fn main() {
    let opts = Cli::parse();

    let dir_to_remove = std::fs::canonicalize(opts.source_dir).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let confirmation = get_user_confirmation(
        &dir_to_remove,
        opts.force,
        &mut stdin().lock(),
        &mut stdout(),
    )
    .trim()
    .to_lowercase();

    let _result = handle_confirmation(&confirmation, &dir_to_remove);
}

fn get_user_confirmation(
    source_dir: &Path,
    force: bool,
    input: &mut impl BufRead,
    output: &mut impl Write,
) -> String {
    if force {
        println!("Running delete without confirmation.");
        println!(
            "Deleting all files and folders in {}.",
            source_dir.to_string_lossy()
        );
        return "y".to_string();
    }

    let prompt = format!(
        "Are you sure you want to delete all files and folders in {}? (y/n) ",
        source_dir.to_string_lossy()
    );

    write!(output, "{prompt}").unwrap_or_else(|e| panic!("Failed to write prompt Error: {}", e));

    output
        .flush()
        .unwrap_or_else(|e| panic!("Failed to flush output. Error: {}", e));

    let mut user_input = String::new();
    input
        .read_line(&mut user_input)
        .unwrap_or_else(|e| panic!("Failed to read user input {}", e));

    user_input.trim().to_string()
}

fn handle_confirmation(confirmation: &str, dir_to_remove: &Path) -> Result<(), std::io::Error> {
    if confirmation != "y" {
        println!("Aborting as user input '{confirmation}' was not 'y'");
        return Ok(());
    }

    let now = Instant::now();
    let result = remove_dir_all(dir_to_remove);

    match &result {
        Ok(_) => println!(
            "Removed all files and folders from {}",
            dir_to_remove.to_string_lossy()
        ),
        Err(e) => println!("Error: {}", e),
    }

    println!("Done in {}s", now.elapsed().as_secs_f32());

    result
}

fn remove_dir_all(dir_to_remove: &Path) -> Result<(), std::io::Error> {
    std::fs::remove_dir_all(dir_to_remove)
}

#[cfg(test)]
mod get_user_confirmation_should {
    use super::*;

    #[test]
    fn return_user_input() {
        let mut input = "y\n".as_bytes();
        let mut output = Vec::new();
        let dir = PathBuf::from("./test-dir");

        let confirmation = get_user_confirmation(&dir, false, &mut input, &mut output);
        assert_eq!(confirmation, "y");

        let output = String::from_utf8(output).unwrap();
        let expected = format!(
            "Are you sure you want to delete all files and folders in {}? (y/n) ",
            dir.to_string_lossy()
        );
        assert_eq!(output, expected);
    }

    #[test]
    fn return_y_when_force() {
        let mut input = "n\n".as_bytes();
        let mut output = Vec::new();

        let dir = PathBuf::from("./test-dir-other");

        let confirmation = get_user_confirmation(&dir, true, &mut input, &mut output);
        assert_eq!(confirmation, "y"); // Is y, even though we gave n

        let output = String::from_utf8(output).unwrap();
        assert_eq!(
            output, "",
            "Output should be empty since prompting gets skipped when force is true"
        );
    }

    #[test]
    fn return_n_when_given_n() {
        let mut input = "n\n".as_bytes();
        let mut output = Vec::new();

        let dir = PathBuf::from("./test-dir");
        let confirmation = get_user_confirmation(&dir, false, &mut input, &mut output);
        assert_eq!(confirmation, "n");
    }
}

#[cfg(test)]
mod handle_confirmation_should {
    use std::sync::atomic::AtomicU8;

    use super::*;

    static UNIQUE_IDENTIFIER: AtomicU8 = AtomicU8::new(0);

    #[test]
    fn remove_dir_when_confirmation() {
        let unique = UNIQUE_IDENTIFIER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let dir = format!("./data/test-dir-{unique}");

        let files_to_remove = ["file1.txt", "file2.txt", "file3.txt"];

        std::fs::create_dir_all(&dir).unwrap();
        let canonicalized = std::fs::canonicalize(&dir).unwrap();

        for file in files_to_remove.iter() {
            let file_path = format!("{dir}/{file}");
            std::fs::File::create(&file_path).unwrap();
            assert!(
                std::fs::canonicalize(&file_path).is_ok(),
                "File was not created"
            );
        }

        let result = handle_confirmation("y", &canonicalized);
        assert!(result.is_ok(), "Error when removing dir");
        assert!(std::fs::canonicalize(&dir).is_err(), "Dir was not removed");
    }

    #[test]
    fn do_not_remove_dir_when_n() {
        let unique = UNIQUE_IDENTIFIER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let dir = format!("./data/test-dir-{unique}");

        std::fs::create_dir_all(&dir).unwrap();
        let canonicalized = std::fs::canonicalize(&dir).unwrap();

        let result = handle_confirmation("n", &canonicalized);
        assert!(result.is_ok(), "Error when removing dir");
        assert!(
            std::fs::canonicalize(&dir).is_ok(),
            "Dir was removed in confirmation n"
        );

        std::fs::remove_dir(&dir).unwrap();
    }

    #[test]
    fn error_when_dir_does_not_exist() {
        let unique = UNIQUE_IDENTIFIER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let dir = format!("./data/test-dir-{unique}");

        assert!(
            handle_confirmation("y", &PathBuf::from(&dir)).is_err(),
            "Deleted a folder that does not exist???"
        );
    }
}
