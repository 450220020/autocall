use std::{path::Path};

use super::scan_path_utils;

pub fn get_auto_starter_path(){
    let path = Path::new("../");
    println!("asb_path:{:?}", path.canonicalize());

    let path_a = env!("CARGO_MANIFEST_DIR");
    println!("mat_path:{:?}",path_a);

}

pub fn get_lib_crate_path()->String{
    let sym = scan_path_utils::get_path_symbol();
    let crate_lib_path = env!("CARGO_MANIFEST_DIR");
    let crate_lib_sym_path = scan_path_utils::path_sym_cast(&crate_lib_path, &sym);
    let lib_path = for_substring!(crate_lib_sym_path,0,crate_lib_sym_path.rfind(&sym).unwrap_or(crate_lib_sym_path.len())).to_string();
    return lib_path;
}