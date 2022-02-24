use std::{env, path::Path};

use crate::com::{toml_read, path_utils};

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
        println!("test work_path:{}",work_path);
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
    let work_path = get_work_path();
    if call_path.starts_with("src/"){
        return work_path.clone()+"/"+&call_path;
    }
    let first_str = for_substring!(call_path,0,call_path.find("/src").unwrap_or(0));
    let last_str = for_substring!(call_path,call_path.find("/src").unwrap_or(0),call_path.len());
    let crate_name = for_substring!(first_str,first_str.rfind(&sym).unwrap_or(0),first_str.len());
    if let Some(_) = work_path.strip_suffix(crate_name){
        return work_path.clone()+"/"+&call_path;
    }
    println!("first_str:{}",first_str);
    println!("last_str:{}",last_str);
    println!("crate_name:{}",crate_name);
    return call_path;
}



/// 读取文件夹指定后缀文件
pub fn read_dir_file(path_vec: Vec<String>, suffix_vec: Vec<&str>) -> Vec<String> {
    let mut all_path_ls = Vec::<String>::new();
    for path_str in path_vec {
        let path_rs = Path::new(&path_str);
        if path_rs.exists() && path_rs.is_dir() {
            let file_rs = path_rs.read_dir();
            if let Ok(r) = file_rs {
                for dir_entity_rs in r {
                    if let Ok(dir_entity) = dir_entity_rs {
                        if let Some(name_path) = dir_entity.path().to_str() {
                            let mut suffix_exists_vec = Vec::<&str>::new();
                            for suffix_str in &suffix_vec {
                                if suffix_exists_vec.contains(suffix_str) {
                                    continue;
                                }
                                if let Some(_) = name_path.strip_suffix(suffix_str) {
                                    all_path_ls.push(path_sym_cast(name_path, &get_path_symbol()));
                                    suffix_exists_vec.push(suffix_str);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return all_path_ls;
}

///
/// 获取当前地址所有文件夹
pub fn read_path_all_dir(
    path_str: &str,
    exclude_path_str: &str,
    all_path_ls: &mut Vec<String>,
    current_level: usize,
    max_level: usize,
) {
    let exclude_sp = exclude_path_str.split(",").collect::<Vec<&str>>();
    if exclude_sp.contains(&path_str) {
        return;
    }
    if current_level > max_level + 1 {
        return;
    }
    let path_rs = Path::new(&path_str);
    if path_rs.exists() && path_rs.is_dir() {
        all_path_ls.push(path_sym_cast(path_str, &get_path_symbol()));
        let dir_rs = path_rs.read_dir();
        if let Ok(r) = dir_rs {
            for dir_entity_rs in r {
                if let Ok(dir_entity) = dir_entity_rs {
                    let dir_entity_path = dir_entity.path();
                    if dir_entity_path.is_dir() {
                        read_path_all_dir(
                            dir_entity_path.to_str().unwrap_or_else(|| {
                                panic!("dir_entity_path is null");
                            }),
                            exclude_path_str,
                            all_path_ls,
                            current_level + 1,
                            max_level,
                        );
                    }
                }
            }
        }
    }
}


//获取有效的文件夹
pub fn get_effective_dir(path_list: &str, exclut_path: &str, lib_path: &str) -> Vec<String> {
    let mut all_dir_path_ls = Vec::new();
    //引用包实际目录
    println!("path_list:{:?}", path_list);
    let act_path_vec = lib_path_link(path_list, lib_path);
    println!("act_path_vec:{:?}", act_path_vec);
    //排除包实际目录
    let mut act_exclude_path_concat_str = String::new();
    if !exclut_path.is_empty() {
        let act_exclude_path_vec = lib_path_link(exclut_path, lib_path);
        for act_exclude_path_str in act_exclude_path_vec {
            act_exclude_path_concat_str = act_exclude_path_concat_str + "," + &act_exclude_path_str;
        }
    }

    for path_act_str in act_path_vec {
        let mut defaul_target_str = path_act_str.clone() + "\\target";
        if !act_exclude_path_concat_str.is_empty() {
            defaul_target_str = act_exclude_path_concat_str.clone()
        }
        read_path_all_dir(
            &path_act_str,
            &defaul_target_str,
            &mut all_dir_path_ls,
            0,
            4,
        );
    }
    all_dir_path_ls
}



///set_lib_path = 实际包地址 env!("CARGO_MANIFEST_DIR")
/// set_lib_path = 调用宏实际包地址
pub fn lib_path_link(path_list: &str, set_lib_path: &str) -> Vec<String> {
    let sym = get_path_symbol();
    let up_sym = "..".to_string() + &sym;
    //调用包地址
    let lib_config_path = path_sym_cast(set_lib_path, &sym);
    //读取调用宏的包依赖
    let toml_path = lib_config_path.clone() + &sym + "Cargo.toml";
    let mut toml_ver_map = toml_read::read_path_toml_lib_ver(&toml_path);
    //调用宏的包
    let current_lib_name = for_substring!(
        &lib_config_path,
        lib_config_path.rfind(&sym).unwrap_or(0) + 1,
        lib_config_path.len()
    )
    .to_string();
    println!("current_lib_name:{:?}", current_lib_name);
    let current_lib_path = "../".to_string() + &current_lib_name;
    toml_ver_map.insert(current_lib_name.clone(), (current_lib_path.clone(), true));
    println!("toml_ver_map:{:?}", toml_ver_map);
    let check_lib_path = path_list.replace("::", &sym).clone();
    let lib_path_split = check_lib_path.split(",");
    let mut path_parent_list: Vec<String> = vec![];
    for path_str in lib_path_split {
        if path_str.is_empty() {
            continue;
        }
        println!("path_str:{:?}", path_str);
        let mut frist_mod_path;
        let the_path_str = path_str.replace("::", &sym).replace(&up_sym, "");
        let last_mod_path = for_substring!(
            the_path_str,
            the_path_str.find(&sym).unwrap_or(the_path_str.len() - 1) + 1,
            the_path_str.len()
        )
        .to_string();
        println!("last_mod_path:{:?}", last_mod_path);
        println!("the_path_str:{:?}", the_path_str);
        let lib_name = the_path_str.clone().split(&sym).collect::<Vec<&str>>()[0].to_string();
        println!("lib_name:{:?}", lib_name);
        //是否引用依赖
        let lib_attr_opt = toml_ver_map.get(&lib_name);
        //实际地址
        match lib_attr_opt {
            Some(val) => {
                println!("val:{:?}", val);
                if val.1 {
                    println!("lib_config_path:{:?}", lib_config_path);
                    frist_mod_path = get_jump_folder(
                        &lib_config_path,
                        &sym,
                        &val.0.split("../").collect::<Vec<&str>>().len() - 1,
                    ) + &sym
                        + &val.0.replace("../", "");
                } else {
                    frist_mod_path = path_utils::get_lib_crate_path()+ &sym
                        + &lib_name.replace("_", "-")
                        + "-"
                        + &val.0;
                }
            }
            None => {
                continue;
            }
        }
        println!("frist_mod_path:{:?}", frist_mod_path);
        if Path::new(&(frist_mod_path.clone() + &sym + "src")).exists() {
            let frist_src = frist_mod_path.clone() + &(sym.clone() + "src");
            if Path::new(&(frist_src.clone() + &sym + &last_mod_path)).exists() {
                frist_mod_path += &(sym.clone() + "src");
            }
        }
        let result_path = frist_mod_path + &sym + &last_mod_path;
        println!("result_path:{:?}", result_path);
        path_parent_list.push(result_path);
    }
    return path_parent_list;
}

///set_lib_path = 实际包地址 env!("CARGO_MANIFEST_DIR")
/// set_lib_path = 调用宏实际包地址
/// 将实际文件路径转换为包路径
pub fn path_lib_link(path_list: &str, set_lib_path: &str, work_lib_path: &str) -> Vec<String> {
    let sym = get_path_symbol();
    let lib_config_path = path_sym_cast(set_lib_path, &sym);
    //C:\\Users\\45022\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\extends-rs-0.1.6
    //读取工作目录依赖文件信息 找到对应版本依赖文件扫描读取

    let toml_path = lib_config_path.clone() + &sym + "Cargo.toml";
    let mut toml_ver_map = toml_read::read_path_toml_lib_ver(&toml_path);
    let current_lib_name = for_substring!(
        lib_config_path,
        lib_config_path.rfind(&sym).unwrap_or(0) + 1,
        lib_config_path.len()
    )
    .to_string();
    let current_lib_path = "../".to_string() + &current_lib_name;
    toml_ver_map.insert(current_lib_name.clone(), (current_lib_path.clone(), true));
    println!("toml_ver_map:{:?}", toml_ver_map);
    //let check_lib_path = path_list.replace(&sym, "::").clone();
    let lib_path_split = path_list.clone().split(",");
    let mut path_parent_list: Vec<String> = vec![];
    for path_str in lib_path_split {
        if path_str.is_empty() {
            continue;
        }
        println!("path_str:{:?}", path_str);
        let mut the_path_str = String::new();
        //如果有后缀 去掉
        if let Some(r) = path_str.rfind(".") {
            if path_str.rfind(&sym).unwrap() < r {
                the_path_str =
                    for_substring!(path_str, 0, path_str.rfind(".").unwrap()).to_string();
            }
        }

        if let None = the_path_str.rfind("src") {
            panic!("the path not find src")
        }
        let jump_dir_vec = the_path_str
            .split(&sym)
            .map(|f| f.to_string())
            .collect::<Vec<String>>();
        println!("jump_dir_vec:{:?}", jump_dir_vec);
        let mut crate_path_str = String::new();
        let mut jump_record_str = the_path_str.clone();
        let max_idx = jump_dir_vec.len() - 1;
        for x in 0..max_idx {
            let jump = jump_dir_vec[max_idx - x].clone();
            crate_path_str = jump.clone() + "::" + &crate_path_str;
            println!("crate_path_str+=:{:?}", crate_path_str);
            jump_record_str = get_jump_folder(&jump_record_str, &sym, 1);
            println!("jump_record_str+=:{:?}", jump_record_str);
            if Path::new(&(jump_record_str.clone() + &sym + "Cargo.toml")).exists() {
                let crate_dir_name;
                match jump_record_str.rfind(&sym) {
                    Some(r) => {
                        crate_dir_name =
                            for_substring!(jump_record_str, r + 1, jump_record_str.len())
                                .to_string();
                    }
                    None => {
                        crate_dir_name = jump_record_str;
                    }
                }
                crate_path_str = crate_dir_name + "::" + &crate_path_str;
                break;
            }
        }
        crate_path_str = for_substring!(crate_path_str, 0, crate_path_str.rfind("::").unwrap())
            .replace("src::", "");

        println!("crate_path_str:{:?}", &crate_path_str);

        let crate_name =
            for_substring!(crate_path_str, 0, crate_path_str.find("::").unwrap()).to_string();
        if crate_name.contains("-") {
            crate_path_str = crate_path_str.replace(
                &crate_name,
                for_substring!(crate_name, 0, crate_name.find("-").unwrap()),
            );
        }
        //执行目录文件下省略引用
        let crate_name_main = crate_name.clone() + "::main";
        let crate_name_lib = crate_name.clone() + "::lib";
        if crate_path_str.starts_with(&crate_name_main) {
            crate_path_str = crate_path_str
                .replace(&crate_name_main, &crate_name.to_string())
                .to_string();
        }
        if crate_path_str.starts_with(&crate_name_lib) {
            crate_path_str = crate_path_str
                .replace(&crate_name_lib, &crate_name.to_string())
                .to_string();
        }
        let work_dir_name = for_substring!(
            work_lib_path,
            work_lib_path.rfind(&sym).unwrap(),
            work_lib_path.len()
        )
        .replace(&sym, "");
        println!("work_dir_name:{:?}", work_dir_name);
        println!("crate_name:{:?}", crate_name);
        //包与工作目录一致修改为crate
        if work_dir_name.eq(&crate_name) {
            crate_path_str = crate_path_str.replace(&crate_name, "crate");
        }
        //去除尾部mod.rs
        if let Some(_) = crate_path_str.strip_suffix("mod") {
            crate_path_str = for_substring!(
                crate_path_str,
                0,
                crate_path_str
                    .rfind("::mod")
                    .unwrap_or(crate_path_str.len())
            )
            .to_string();
        }
        println!("crate_path_str rest:{:?}", crate_path_str);
        path_parent_list.push(crate_path_str);
    }
    return path_parent_list;
}

///上级目录
pub fn get_jump_folder(path_str_the: &str, lev_str: &str, set_lev_size: usize) -> String {
    let mut path_str_result = path_str_the.to_string();
    if let None = path_str_result.find(lev_str) {
        path_str_result = path_str_the.replace("\\", lev_str).replace("/", lev_str);
    }
    let mut up_lev_size = set_lev_size;
    if up_lev_size > 0 {
        let up_size_max = path_str_result.split(lev_str).collect::<Vec<&str>>().len() - 1;
        if up_lev_size > up_size_max {
            up_lev_size = up_size_max;
        }
        for _ in 0..up_lev_size {
            path_str_result = for_substring!(
                path_str_result,
                0,
                path_str_result
                    .rfind(lev_str)
                    .unwrap_or(path_str_result.len())
            )
            .to_string();
        }
    }
    path_str_result
}
