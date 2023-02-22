use serde_json::Value;

use crate::package_json::get_package_json;

pub struct NpmProcessContext {
    pub name: String,
    pub dir: String,
    pub package_json_path: String,
    pub script: String,
    pub package_json: Value,
}

impl NpmProcessContext {
    pub fn new(dir_entry: walkdir::DirEntry, script: String) -> Self {
        let dir = str::replace(dir_entry.path().to_str().unwrap(), "package.json", "");
        let package_json = get_package_json(dir_entry.path().to_str().unwrap());
        let package_json_path = String::from(format!("{}package.json", dir));
        let name = package_json.get("name").expect(
            &format!("Could not find project name at file {}", package_json_path)
        );
        let name = match name {
            serde_json::Value::String(name) => name.clone(),
            _ => panic!("Project name at file {} was not a string", package_json_path)
        };

        Self {
            name,
            dir,
            package_json_path,
            script,
            package_json,
        }
    }

    pub fn contains_script(&self) -> bool {
        let scripts = self.package_json.get("scripts");

        if let Some(scripts) = scripts {
            let choosen_script = scripts.get(format!("hammer:{}", self.script));
            if let Some(_) = choosen_script {
                return true;
            }
            return false;
        }
        return false;
    }
}