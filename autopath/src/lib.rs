use std::{path::Path, env};

macro_rules! for_substring {
    ($str:expr,$first:expr,$last:expr) => {
        $str.split_at($first).1.split_at($last-$first).0
    };
}

const SYMBOL_STR: &str = "/";

pub fn get_path_symbol() -> String {
    String::from(SYMBOL_STR)
}

pub fn path_sym_cast(path_str: &str, sym: &str) -> String {
    path_str.replace("\\", sym).replace("/", sym)
}

///
/// 获取.cargo\registry\src\githubxxxxxxxx,当前包所述文件夹
pub fn get_lib_crate_path()->String{
    let sym = get_path_symbol();
    let crate_lib_path = env!("CARGO_MANIFEST_DIR");
    let crate_lib_sym_path = path_sym_cast(&crate_lib_path, &sym);
    let lib_path = for_substring!(crate_lib_sym_path,0,crate_lib_sym_path.rfind(&sym).unwrap_or(crate_lib_sym_path.len())).to_string();
    return lib_path;
}


///
/// 获取工作目录   src 上级目录
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


// 获取调用宏的项目路径
pub fn get_caller_path(call_macro_path:String )->String{
    let sym = get_path_symbol();
    let sym_src = sym.clone()+"src";
    let src_sym = "src".to_string()+&sym;
    let call_path = path_sym_cast(&call_macro_path,&sym);
    let work_path = get_work_path();
    match call_path.rfind(&src_sym){
        Some(r)=>{
            let first_str = call_path.split_at(r).0.to_string();
            if let Some(z)=first_str.rfind("."){
                if let Some(l)=call_path.rfind("-"){
                    if l<z&&z<r{
                        return for_substring!(call_path,0,call_path.rfind(&sym_src).unwrap_or(0)).to_string();
                    }
                }
            }
        }
        None=>{
            panic!("call_macro_path is error:{}",call_macro_path);
        }
    }
    let mut crate_name = for_substring!(call_path,0,call_path.rfind(&sym_src).unwrap_or(0)).to_string();
    if !crate_name.is_empty(){
        crate_name = sym.clone()+&crate_name;
    }
    return work_path.clone()+&crate_name;
}

//获取调用宏的文件路径
pub fn get_caller_file_path(call_macro_path:String)->String{
    let sym = get_path_symbol();
    let sym_src = sym.clone()+"src";
    let src_sym = "src".to_string()+&sym;
    let call_path = path_sym_cast(&call_macro_path,&sym);
    let work_path = get_work_path();
    match call_path.rfind(&src_sym){
        Some(r)=>{
            let first_str = call_path.split_at(r).0.to_string();
            if let Some(z)=first_str.rfind("."){
                if let Some(l)=call_path.rfind("-"){
                    if l<z&&z<r{
                        return call_path;
                    }
                }
            }
        }
        None=>{
            panic!("call_macro_path is error:{}",call_macro_path);
        }
    }
    return work_path.clone()+&sym+&call_path;
}


#[test]
fn test_path(){
    let a_p = "C:/Users/Joinz/.cargo/registry/src/github.com-1ecxxxxx9ec823/autocall-0.1.6/src/lib.rs".to_string();
    let b_p = "rscontr/src/lib.rs".to_string();
    let c_p = "src/lib.rs".to_string();
    let a= get_caller_file_path(a_p.clone());
    let b= get_caller_file_path(b_p.clone());
    let c= get_caller_file_path(c_p.clone());
    println!(" a:{} \n b:{} \n c:{}",a,b,c);


    let a= get_caller_path(a_p.clone());
    let b= get_caller_path(b_p.clone());
    let c= get_caller_path(c_p.clone());
    println!(" a:{} \n b:{} \n c:{}",a,b,c);

    let e = get_lib_crate_path();
    let f = get_work_path();

    println!("e:{} ,f:{}",e,f);
}
