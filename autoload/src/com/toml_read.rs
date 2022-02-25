use pest::Parser;
use std::{collections::HashMap, fs};

#[derive(Parser)]
#[grammar = "./pestf/toml_lib_version_input.pest"]
pub struct TomlLibVersionInput;

//读取工作目录依赖版本信息
pub fn read_path_toml_lib_ver(path_str: &str) -> HashMap<String, (String, bool)> {
    let mut toml_lib_ver_map = HashMap::new();
    let mut unparsed_file = String::new();
    if let Ok(r) = fs::read_to_string(&path_str) {
        unparsed_file = r.clone();
    }
    if !unparsed_file.is_empty() {
        let file = TomlLibVersionInput::parse(Rule::file, unparsed_file.as_str())
            .expect("unsuccessful parse")
            .next()
            .unwrap();
        for line in file.into_inner() {
            match line.as_rule() {
                Rule::toml_ver_more_path_content => {
                    let mut inner_rules = line.into_inner();
                    let lib_name = inner_rules.next().unwrap().as_str().to_string();
                    let _ = inner_rules.next().unwrap().as_str().to_string();
                    let lib_ver = inner_rules.next().unwrap().as_str().to_string();
                    toml_lib_ver_map.insert(lib_name, (lib_ver, true));
                }
                Rule::toml_ver_more_content => {
                    let mut inner_rules = line.into_inner();
                    let lib_name = inner_rules.next().unwrap().as_str().to_string();
                    let _ = inner_rules.next().unwrap().as_str().to_string();
                    let lib_ver = inner_rules.next().unwrap().as_str().to_string();
                    toml_lib_ver_map.insert(lib_name, (lib_ver, false));
                }
                Rule::toml_ver_content => {
                    let mut inner_rules = line.into_inner();
                    let lib_name = inner_rules.next().unwrap().as_str().to_string();
                    let lib_ver = inner_rules.next().unwrap().as_str().to_string();
                    toml_lib_ver_map.insert(lib_name, (lib_ver, false));
                }
                Rule::EOI => (),
                _ => (),
            }
        }
    }
    toml_lib_ver_map
}
