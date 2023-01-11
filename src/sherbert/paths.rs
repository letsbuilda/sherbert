//! Path helpers.

use std::path::{Path, PathBuf};

/// Search for file.
#[must_use]
pub fn find_file(starting_directory: PathBuf, filename: &str) -> Option<PathBuf> {
    let mut path: PathBuf = starting_directory;
    let file = Path::new(filename);

    loop {
        path.push(file);

        if path.is_file() {
            break Some(path);
        }

        if !(path.pop() && path.pop()) {
            // remove file && remove parent
            break None;
        }
    }
}
