use std::ffi::OsString;
use std::path::Path;

use super::super::utils;

pub fn resolve_alias(root: &str, alias: &str, results: Vec<OsString>) -> Option<String> {
    let target: Vec<_> = results
        .iter()
        .filter(|r| {
            let proj_path: String = format!("{}/{}/", root, r.to_str().unwrap());
            let config = utils::get_project_config(&Path::new(proj_path.as_str()));

            config.info.alias == alias
        })
        .collect();

    if target.is_empty() {
        return None;
    }

    let resolved = target[0].to_str().unwrap();
    let resolved_path = format!("{}/{}/", root, resolved);

    Some(resolved_path)
}
