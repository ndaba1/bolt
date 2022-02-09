use super::super::utils::{Policy, ProjectConfig};

pub fn resolve_policy(cfg: ProjectConfig, val: String) -> (Option<String>, String, String) {
    let cmd: Vec<&Policy> = cfg.policies.iter().filter(|p| p.name == val).collect();
    let policy = &cmd.first().unwrap();

    let pre_load = if policy.depends_on.is_empty() {
        None
    } else if policy.depends_on == "^" || policy.depends_on == "install" {
        Some(String::from("install"))
    } else {
        Some(policy.depends_on.clone())
    };

    let target = &policy.map_to.value;

    match target.as_str() {
        "cli" => {
            let target_cli = cfg.info.env.cli.unwrap().clone();
            let target_cmd = policy.map_to.cmd.clone().unwrap();

            (
                pre_load,
                format!("{} {}", target_cli, target_cmd),
                "🚀 Starting your application...".to_owned(),
            )
        }
        "pkg_mgr" => {
            let pkgr = cfg.info.env.pkg_mgr;
            let value = pkgr.value;
            let cmd = pkgr.cmds.add;

            (
                pre_load,
                format!("{} {}", value, cmd),
                "➕ Checking dependencies...".to_owned(),
            )
        }
        _ => (None, "".to_owned(), "".to_owned()),
    }
}
