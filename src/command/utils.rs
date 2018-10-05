//! Utility functions for commands.

use emoji;
use error::Error;
use progressbar::Step;
use std::fs;
use std::path::{Path, PathBuf};
use PBAR;

/// If an explicit path is given, then use it, otherwise assume the current
/// directory is the crate path.
pub fn set_crate_path(path: Option<PathBuf>) -> Result<PathBuf, Error> {
    let crate_path = match path {
        Some(p) => p,
        None => PathBuf::from("."),
    };
    canonicalize_path(crate_path)
}

/// Construct our `pkg` directory in the crate.
pub fn create_pkg_dir(out_dir: &Path, step: &Step) -> Result<(), Error> {
    let msg = format!("{}Creating a pkg directory...", emoji::FOLDER);
    PBAR.step(step, &msg);
    fs::create_dir_all(&out_dir)?;
    Ok(())
}

/// Locates the pkg directory from a specific path
/// Returns None if unable to find the 'pkg' directory
pub fn find_pkg_directory(path: &Path) -> Option<PathBuf> {
    if is_pkg_directory(path) {
        return Some(path.to_owned());
    }

    path.read_dir().ok().and_then(|entries| {
        entries
            .filter_map(|x| x.ok().map(|v| v.path()))
            .find(|x| is_pkg_directory(&x))
    })
}

fn is_pkg_directory(path: &Path) -> bool {
    path.exists() && path.is_dir() && path.ends_with("pkg")
}

cfg_if! {
    if #[cfg(windows)] {
        /// Strips UNC from canonical path on Windows.
        /// See https://github.com/rust-lang/rust/issues/42869 for why this is needed.
        pub fn canonicalize_path(path: PathBuf) -> Result<PathBuf, Error> {
            use std::ffi::OsString;
            use std::os::windows::prelude::*;
            let canonical = path.canonicalize()?;
            let vec_chars = canonical.as_os_str().encode_wide().collect::<Vec<u16>>();
            if vec_chars[0..4] == [92, 92, 63, 92] {
                Ok(Path::new(&OsString::from_wide(&vec_chars[4..])).to_owned())
            }
            else {
                Ok(canonical)
            }
        }
    }
    else {
        /// Strips UNC from canonical path on Windows.
        pub fn canonicalize_path(path: PathBuf) -> Result<PathBuf, Error> {
            let canonical = path.canonicalize()?;
            Ok(canonical)
        }
    }
}
