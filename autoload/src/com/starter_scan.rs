use crate::com::component_celler_read;
use crate::com::scan_path_utils;
use crate::com::toml_read;
use core::panic;
use pest::Parser;
pub use proc_macro::TokenStream;
use quote::quote;
use std::{collections::HashMap, env, fs, path::Path, str::FromStr};


#[derive(Parser)]
#[grammar = "./pestf/starter_scan_input.pest"]
pub struct StarterScanParser;

const SYMBOL_STR: &str = "/";

fn get_path_symbol() -> String {
    String::from(SYMBOL_STR)
}

pub fn impl_starter_scan(
    _attr: &TokenStream,
    _input: &TokenStream,
    celler_path: &String,
) -> TokenStream {
    println!("impl_starter_scan _input:{:?}", _input.to_string());
    let attr_str = _attr.to_string();
    let sym = get_path_symbol();
    let sym_src_lib_str = sym.clone() + "src/lib.rs";
    let mut scan_path = String::new();
    let mut lib_name = String::new();
    let mut scan_suffix = String::from(".rs");
    let param = attr_split_to_map(&attr_str);

    if let Some(r) = param.get("scan_path") {
        scan_path = r.clone().trim().to_string();
    }
    if let Some(r) = param.get("lib_name") {
        lib_name = r.clone().trim().to_string();
    }
    let mut true_file_paht_vec = vec![];
    if !lib_name.is_empty() {
        // let run_load_code  = "
        // single_get_ref_try!(\"autocall_loading_config_json\",String,|config_json_str:&String|{
        //     let config_json = serde_json::from_str::<Value>(config_json_str).unwrap();
        //     let load_component_list = &config_json[\"load_component_list\"];
        //     match load_component_list {
        //         Value::Array(r)=>{
        //             if r.contains(&Value::String(String::from(\"log4rs\"))){
        //                 auto_config::log4rs_config::loading();
        //             }
        //             if r.contains(&Value::String(String::from(\"rbatis\"))){
        //                 auto_config::rbatis_config::loading();
        //             }
        //         }
        //         _=>()
        //     }
        // },{
        //     println!(\"autocall_loading_config_json not find\");
        // });
        // ";

        //需要扫描的文件路径
        let file_path_vec = get_effective_file(&celler_path, &scan_path, &scan_suffix);
        let crate_name_sp = lib_name.split(",");
        for name_str in crate_name_sp {
            let suffix_str = name_str.to_string() + &sym_src_lib_str;
            for file_paht_str in &file_path_vec {
                if let Some(_) = file_paht_str.strip_suffix(&suffix_str) {
                    true_file_paht_vec.push((name_str.to_string(), file_paht_str.clone()));
                }
            }
        }
    }
    let caller_file_path = scan_path_utils::get_caller_file_path();
    //生成调用代码
    let mut loading_code_str = String::new();
    let crate_path_vec = path_lib_link(&caller_file_path, &celler_path, &celler_path);
    let crate_path_str = &crate_path_vec[0];
    println!("crate_path_straaaa:{:?}", crate_path_str);
    let mut mod_code_str = String::new();
    for (name_str, path_str) in true_file_paht_vec {
        let crate_lib_name = name_str.replace("-", "_");
        if let Ok(r) = fs::read_to_string(&path_str) {
            let up_r = dorp_select_parset(r.clone());
            mod_code_str += &format!("\n pub mod {}{{{}}}", &crate_lib_name, &up_r);
            loading_code_str += &(crate_path_str.clone() + "::" + &crate_lib_name + "::loading();");
        } else {
            continue;
        }
    }

    println!("mod_code_str:{}", mod_code_str);

    println!("loading_code_str:{}", loading_code_str);

    let celler_fun =
        crate::com::starter_celler_read::read_this_parset(_input.clone().to_string()).unwrap();
    let mut fun_for_content = celler_fun.fun_for_content;
    fun_for_content = for_substring!(
        &fun_for_content,
        fun_for_content.find("{").unwrap() + 1,
        fun_for_content.rfind("}").unwrap()
    )
    .to_string();
    let result_code = format!(
        "pub fn {}({}){}{{{}{}}}  {}",
        celler_fun.fun_name,
        celler_fun.fun_param,
        celler_fun.fun_head_end_group,
        fun_for_content,
        loading_code_str,
        mod_code_str
    );
    println!("result_code:{}", result_code);
    let result_token_stream = proc_macro2::TokenStream::from_str(&result_code).unwrap();
    return TokenStream::from(result_token_stream);
}





pub fn dorp_select_parset(unparsed_file:String)->String {
    let mut up_unparsed_file =unparsed_file.clone();
    let rd_unparsed_file = unparsed_file.clone();
    let file = StarterScanParser::parse(Rule::file, &rd_unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::extern_content => {
                let  inner_rules = line.as_str();
                println!("inner_rules:{}",inner_rules);
                up_unparsed_file = up_unparsed_file.replace(inner_rules, "");
            }
            Rule::extern_content=>{

            }
            Rule::EOI => (),
            _ => (),
        }
    }
    up_unparsed_file = up_unparsed_file.replace("#[macro_use]","");
    return up_unparsed_file;
}

///
/// 获取有效的扫描文件
fn get_effective_file(
    celler_path: &String,
    scan_path: &String,
    scan_suffix: &String,
) -> Vec<String> {
    //最终需要扫描的所有目录
    let path_list = scan_path.clone();
    let lib_path = celler_path.clone();
    let exculde_path = String::from("");
    let all_dir_path_ls = get_effective_dir(&path_list, &exculde_path, &lib_path);

    println!("all_dir_path_ls:{:?}", all_dir_path_ls);
    //需要扫描的文件后缀
    let mut scan_suffix_vec = Vec::<&str>::new();

    if !scan_suffix.trim().is_empty() {
        let scan_suffix_sp = scan_suffix.split(",");
        for scan_suffix_str in scan_suffix_sp {
            scan_suffix_vec.push(scan_suffix_str.trim());
        }
    } else {
        scan_suffix_vec.push(".rs");
    }

    //需要扫描的文件路径
    let file_path_vec = read_dir_file(all_dir_path_ls, scan_suffix_vec);
    return file_path_vec;
}

//获取有效的文件夹
fn get_effective_dir(path_list: &str, exclut_path: &str, lib_path: &str) -> Vec<String> {
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

//分解宏属性为map
pub fn attr_split_to_map(attr_str: &str) -> HashMap<String, String> {
    attr_str
        .replace("/", "")
        .replace('\n', "")
        .replace('\\', "")
        .trim()
        .to_string();
    let attr_split = attr_str.split("\",").map(|s| s.replace("\"", ""));
    let mut attr_map = HashMap::<String, String>::new();
    for attr_mate in attr_split {
        let attr_sp = attr_mate.split_once("=");
        if let Some((key, val)) = attr_sp {
            attr_map.insert(key.trim().to_string(), val.trim().to_string());
        }
    }
    return attr_map;
}

///
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
        all_path_ls.push(path_str.to_string());
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

fn path_sym_cast(path_str: &str, sym: &str) -> String {
    path_str.replace("\\", sym).replace("/", sym)
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
                    frist_mod_path = get_jump_folder(&lib_config_path, &sym, 1)
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
