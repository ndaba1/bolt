use std::collections::HashMap;
use std::path::Path;

use super::super::core;
use super::super::utils::ProjectConfig;

pub fn up(vals: HashMap<String, String>, opts: HashMap<String, String>) {
    let target = vals.get("app_name").unwrap();

    let skip_deps = if opts.contains_key("skip") {
        true
    } else {
        false
    };

    // handle starting multiple apps
    if opts.contains_key("priority") {}

    let (proj_path, config) = core::setup_cmd(target);

    core::load_directives(Path::new(&proj_path.as_str()), false);
    start(config, "up".to_owned(), &proj_path, skip_deps)
}

fn start(cfg: ProjectConfig, value: String, dir: &str, skip: bool) {
    let root_path = std::env::current_dir().unwrap();
    let full_path = Path::new(&root_path).join(&dir);

    match core::resolve_policy(cfg.clone(), value) {
        (Some(val), cmd, msg) => {
            if !skip {
                start(cfg, val, dir, false);
            }
            core::execute(cmd, full_path, true, &msg);
        }
        (None, cmd, msg) => {
            core::execute(cmd, full_path, true, &msg);
        }
    }
}
