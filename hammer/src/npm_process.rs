use serde_json::Value;
use crate::args::Args;
use crate::errors::{BeautifulErrors,panic_err};

use crate::package_json::get_package_json;

pub struct NpmProcessContext {
    pub name: String,
    pub dir: String,
    pub package_json_path: String,
    pub args: Args,
    pub package_json: Value,
}

impl NpmProcessContext {
    pub fn new(dir_entry: walkdir::DirEntry, args: Args) -> Self {
        let entry_path = dir_entry.path().to_str().expect_or_err(
            &format!("Error converting directory path to string: {}", dir_entry.path().display())
        );
        let dir = str::replace(entry_path, "package.json", "");
        let package_json = get_package_json(entry_path);
        let package_json_path = String::from(format!("{}package.json", dir));
        let name = package_json.get("name").expect_or_err(
            &format!("Could not find project name at file {}", package_json_path)
        );
        let name = match name {
            serde_json::Value::String(name) => name.clone(),
            _ => panic_err(&format!("Project name at file {} was not a string", package_json_path))
        };

        Self {
            name,
            dir,
            package_json_path,
            args,
            package_json,
        }
    }

    pub fn validate_script(self) -> Option<Self> {
        let scripts = self.package_json.get("scripts");
        let script_name = format!("{}{}", {
            if self.args.no_prefix { "" } else { "hammer:" }
        }, self.args.script);

        if let Some(scripts) = scripts {
            let choosen_script = scripts.get(script_name.clone());
            if let Some(_) = choosen_script {
                return Some(Self {
                    args: Args {
                        script: script_name,
                        ..self.args
                    },
                    ..self
                });
            }
            return None;
        }
        return None;
    }
}