use toml::value::{Map, Value};

pub fn add_rustc_wrapper(path: &str, sccache_path: &str) {
    let cargo_contents = std::fs::read_to_string(path).unwrap();
    let toml_contents = toml::from_str::<toml::Value>(&cargo_contents).unwrap();

    let toml_table = toml_contents.as_table();

    if let Some(t_contents) = toml_table {
        let mut map = Map::new();
        map.insert(
            String::from("rustc-wrapper"),
            toml::Value::String(sccache_path.to_string()),
        );

        let mut t_contents_clone = t_contents.clone();
        t_contents_clone.insert(String::from("build"), Value::Table(map));

        let toml_value = toml::Value::Table(t_contents_clone);

        let toml_string = toml_value.to_string();
        std::fs::write(path, toml_string).unwrap();
    }
}
