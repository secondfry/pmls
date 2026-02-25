use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Portage",
        command: "emerge",
        category: Category::System,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "/etc/portage/make.conf",
            "/etc/portage/package.use/",
            "/etc/portage/package.mask/",
        ],
        env_vars: &["PORTAGE_TMPDIR", "PORTDIR", "DISTDIR", "PKGDIR"],
        packages_dir: Some(|_env| Some(("/var/db/pkg".to_string(), "default"))),
        list_cmd: None,
    }
}
