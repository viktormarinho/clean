pub fn get_package_json(file_path: &str) -> serde_json::Value {
    let file_data = std::fs::read_to_string(file_path).expect(
        &format!("Could not read file {file_path}")
    );
    serde_json::from_str(&file_data).expect(
        &format!("Could not parse json file {file_path}")
    )
}