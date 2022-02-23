use std::{env};

const SYMBOL_STR: &str = "/";

fn get_path_symbol() -> String {
    String::from(SYMBOL_STR)
}


fn path_sym_cast(path_str: &str, sym: &str) -> String {
    path_str.replace("\\", sym).replace("/", sym)
}

pub  fn get_work_path()->String{
    let sym = get_path_symbol();
    let mut config_path = String::new();
    let config_path_rs = env::current_dir();
    match config_path_rs {
        Ok(r) => {
            if let Some(s) = r.to_str() {
                config_path = path_sym_cast(s, &sym);
            }
        }
        Err(e) => {
            panic!("error:{:?}", e);
        }
    }
    return config_path;
}