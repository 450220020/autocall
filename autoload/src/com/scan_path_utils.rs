use std::env;

const SYMBOL_STR: &str = "/";

pub fn get_path_symbol() -> String {
    String::from(SYMBOL_STR)
}

pub fn path_sym_cast(path_str: &str, sym: &str) -> String {
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

/// 
/// 获取调用宏的项目路径
pub fn get_caller_path()->String{
    let sym = get_path_symbol();
    let call_site_span = proc_macro::Span::call_site();
    let ast_path ;
    let sym_src = sym.clone()+"src";
    let src_sym = "src".to_string()+&sym;
    
    let call_path = path_sym_cast(call_site_span
        .source_file()
        .path()
        .to_str()
        .unwrap_or(""),&sym);
        println!("call_path:{:?}",call_path);
        println!("call_path:{:?}",call_path.contains("src"));
    if call_path.contains("src") {
        let work_path = get_work_path();
        if let None = work_path.rfind(&sym){
            return String::new();
        }

        let work_dir_name = work_path.split_at(work_path.rfind(&sym).unwrap()).1.to_string();
        let call_dir_name;
        match call_path.find(&sym_src){
            Some(r)=>{
                call_dir_name = call_path.split_at(r).0.to_string();
            }
            None=>{
                if call_path.starts_with(&src_sym){
                    call_dir_name = work_dir_name.clone();
                }else{
                    return String::new();
                }
            }
        }
    
        if !work_dir_name.eq(&call_dir_name){
            ast_path = work_path+&sym + &call_dir_name;
        }else{
            ast_path = get_work_path();
        }
    }else{
        
        let call_ast_path = path_sym_cast(&call_path, &sym);
        ast_path =  for_substring!(call_ast_path,0,call_ast_path.rfind(&sym_src).unwrap_or(call_ast_path.len())).to_string();
    }
    return ast_path;
}

pub fn get_caller_file_path()->String{
    let sym = get_path_symbol();
    let call_site_span = proc_macro::Span::call_site();
    
    let call_path = path_sym_cast(call_site_span
        .source_file()
        .path()
        .to_str()
        .unwrap_or(""),&sym);
        println!("call_path:{:?}",call_path);
    return call_path;
}
