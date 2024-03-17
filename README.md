# rm_dir

## Why?
This is just a simple cli that calls std::fs::remove_dir_all so that I can use the same command to remove on Windows and Linux. std::fs::canonicalize handles path resolution and has the addeded benefit of err on file does/doesn't exist.

## Usage
`rm-dir ./path/to/dir`
