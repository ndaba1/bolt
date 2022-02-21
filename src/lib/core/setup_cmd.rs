use std::path::Path;

use super::super::utils;

use colored::Colorize;

pub fn setup_cmd(proj_name: &str) -> (String, utils::ProjectConfig) {
    let root_config = utils::get_workspace_config();
    let src_name = &root_config.source[0];
    let proj_path = format!("{}/{}/", src_name, &proj_name);

    let results = utils::read_file_tree(Path::new(src_name.as_str())).unwrap();
    let target: Vec<_> = results
        .iter()
        .filter(|r| match r.to_ascii_lowercase().to_str() {
            Some(val) => val == proj_name,
            None => false,
        })
        .collect();

    let config: utils::ProjectConfig;
    if target.is_empty() {
        match utils::resolve_alias(src_name, proj_name, results) {
            Some(val) => {
                config = utils::get_project_config(&Path::new(val.as_str()));

                return (val, config);
            }
            None => {
                let msg = format!(
                    "The passed value: {}, could not be resolved as a directory or an alias!",
                    &proj_name.yellow()
                );
                println!("{}", msg.red());
                std::process::exit(1);
            }
        }
    } else {
        config = utils::get_project_config(&Path::new(proj_path.as_str()));

        (proj_path, config)
    }
}
