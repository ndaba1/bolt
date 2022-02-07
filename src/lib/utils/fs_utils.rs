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

pub fn get_config(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        let children = read_file_tree(dir)?;

        let target: &Vec<_> = &children
            .iter()
            .filter(|c| c.to_ascii_lowercase().to_str() == Some("boltconfig.json"))
            .collect();

        if target.is_empty() {
            println!(
                "Could not find a boltconfig.json in dir: {:?}",
                Path::new(dir).to_str().unwrap()
            )
        }
    }

    Ok(())
}
