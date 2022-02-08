use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::Path;

pub fn read_file_tree(path: &Path) -> io::Result<Vec<OsString>> {
    let mut entries: Vec<OsString> = vec![];
    if path.is_dir() {
        entries = fs::read_dir(path)?
            .map(|res| res.map(|e| e.file_name()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        entries.sort()
    }
    Ok(entries)
}
