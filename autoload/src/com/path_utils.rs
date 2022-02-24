use std::{path::Path};

pub fn get_auto_starter_path(){
    let path = Path::new("../libs");
    println!("asb_path:{:?}", path.canonicalize().unwrap())
}