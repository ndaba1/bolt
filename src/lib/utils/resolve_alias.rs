use std::ffi::OsString;
use std::path::Path;

use super::super::utils;

pub fn resolve_alias(root: &str, alias: &str, results: Vec<OsString>) -> Option<String> {
    let wp_cfg = utils::get_workspace_config();
    let target: Vec<_> = results
        .iter()
        .filter(|r| {
            let str_version = r.to_str().unwrap();
            match &wp_cfg.registered {
                Some(vector) => {
                    if vector.contains(&str_version.to_owned()) {
                        let proj_path: String = format!("{}/{}/", root, str_version);
                        let config = utils::get_project_config(Path::new(proj_path.as_str()));

                        return config.info.alias == alias;
                    }
                    false
                }
                None => {
                    let proj_path: String = format!("{}/{}/", root, str_version);
                    let config = utils::get_project_config(Path::new(proj_path.as_str()));

                    config.info.alias == alias
                }
            }
        })
        .collect();

    if target.is_empty() {
        return None;
    }

    let resolved = target[0].to_str().unwrap();
    let resolved_path = format!("{}/{}/", root, resolved);

    Some(resolved_path)
}
