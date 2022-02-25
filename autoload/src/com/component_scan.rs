use crate::com::component_celler_read;
use crate::com::scan_path_utils;
use crate::com::scan_path_utils::attr_split_to_map;
use crate::com::scan_path_utils::param_attr_split_to_map;
use crate::com::scan_path_utils::read_dir_file;
use crate::com::scan_path_utils::{get_effective_dir, path_lib_link};
use core::panic;
use pest::Parser;
pub use proc_macro::TokenStream;
use std::{fs, str::FromStr};

#[derive(Parser)]
#[grammar = "./pestf/component_scan_input.pest"]
pub struct ComponentScanParser;

#[derive(Debug, Clone)]
struct ComponentScanVO {
    scan_path: Option<String>,
    scan_suffix: Option<String>,
    scan_macro: Option<String>,
    exculde_path: Option<String>,
    lib_path: Option<String>,
}

pub fn impl_component_scan(
    _attr: &TokenStream,
    _input: &TokenStream,
    celler_path: &str,
) -> TokenStream {
    let attr_str = _attr.to_string();
    let mut config = ComponentScanVO {
        scan_path: None,
        scan_suffix: Some(String::from(".rs")),
        scan_macro: None,
        exculde_path: Some(String::from("")),
        lib_path: None,
    };
    let param = attr_split_to_map(&attr_str);

    if let Some(r) = param.get("scan_path") {
        config.scan_path = Some(r.clone().trim().to_string());
    }
    if let Some(r) = param.get("scan_macro") {
        config.scan_macro = Some(r.clone().trim().to_string());
    }
    if let Some(r) = param.get("scan_suffix") {
        config.scan_suffix = Some(r.clone().trim().to_string());
    }
    if let Some(r) = param.get("exculde_path") {
        config.exculde_path = Some(r.clone().trim().to_string());
    }
    if let Some(r) = param.get("lib_path") {
        config.lib_path = Some(r.clone().trim().to_string());
    }
    config.lib_path = Some(celler_path.to_string());

    if let None = param.get("scan_path") {
        panic!("scan_path is none");
    }

    //需要扫描的文件路径
    let file_path_vec = get_effective_file(config.clone());

    //需要扫描的宏
    let mut scan_macro_vec = Vec::<&str>::new();
    let scan_macro = config.scan_macro.unwrap_or("".to_string()).clone();
    if !scan_macro.trim().is_empty() {
        let scan_macro_sp = scan_macro.split(",");
        for scan_macro_str in scan_macro_sp {
            scan_macro_vec.push(scan_macro_str);
        }
    }
    let fun_vec = read_macro_file_path(&file_path_vec, &scan_macro_vec);
    let loading_code_str = fun_model_to_code(fun_vec, celler_path);

    let celler_fun_code_op = component_celler_read::read_this_parset(_input.clone().to_string());

    if let None = celler_fun_code_op {
        panic!("not parset:{}", _input.clone().to_string());
    }
    let celler_fun = celler_fun_code_op.unwrap();
    let mut fun_for_content = celler_fun.fun_for_content;

    fun_for_content = for_substring!(
        &fun_for_content,
        fun_for_content.find("{").unwrap() + 1,
        fun_for_content.rfind("}").unwrap()
    )
    .to_string();

    let result_code = format!(
        "pub fn {}({}){}{{{}{}}}",
        celler_fun.fun_name,
        celler_fun.fun_param,
        celler_fun.fun_head_end_group,
        fun_for_content,
        loading_code_str
    );
    println!("component_scan_code:\n{}", result_code);
    let result_token_stream = proc_macro2::TokenStream::from_str(&result_code).unwrap();
    return TokenStream::from(result_token_stream);
}

///识别的函数集转换为执行代码
pub fn fun_model_to_code(fun_vec: Vec<FunModel>, celler_path: &str) -> String {
    for model in fun_vec {
        let attr_map = param_attr_split_to_map(&model.fun_param);
        let mut rely_vec = vec![];
        let mut rely_name_type = vec![];
        for rely_str in &model.fun_param.split(",").collect::<Vec<&str>>() {
            if rely_str.is_empty() {
                continue;
            }
            match rely_str.split_once(":") {
                Some((key, _)) => {
                    rely_vec.push(key.to_string());
                    rely_name_type.push(rely_str.to_string());
                }
                None => {
                    continue;
                }
            }
        }
        let crate_path_str = path_lib_link(&model.file_path, &celler_path, &celler_path);
        //导入所有依赖
        push_loader!(
            model.fun_name.clone(),
            (
                false,
                model.fun_name.clone(),
                rely_vec,
                crate_path_str[0].clone(),
                rely_name_type,
                model.fun_return
            )
        );
    }
    let sort_result = sort_loader!();
    let rely_sort;
    match sort_result {
        Ok(r) => {
            rely_sort = r;
        }
        Err(r) => {
            panic!("Error rely :{:?}", r);
        }
    }
    let mut insert_code_line = String::new();
    //按照依赖顺序装载
    for rely in rely_sort {
        let fun_name = rely.1;
        let fun_rely_vec = rely.4;
        let fun_crate_path = rely.3;
        let fun_return = rely.5;
        let run_tokens;
        if fun_rely_vec.len() > 0 {
            let mut rely_str = String::new();
            for rely_name_type in fun_rely_vec {
                let (key, val) = rely_name_type.split_once(":").unwrap();
                let mut val_str = val.trim().clone();
                if val_str.starts_with("&") {
                    val_str = val_str.split_at(1).1;
                }
                rely_str = rely_str + "single_get_unwrap!(\"" + &key + "\"," + val_str + "),";
            }
            rely_str = for_substring!(rely_str, 0, rely_str.len() - 1).to_string();
            if fun_return.is_empty() {
                run_tokens = format!("{}::{}({});", fun_crate_path, fun_name, rely_str);
            } else {
                run_tokens = format!(
                    "let {} = {}::{}({});single_push!({:?},{});",
                    fun_name, fun_crate_path, fun_name, rely_str, fun_name, fun_name
                );
            }
        } else {
            if fun_return.is_empty() {
                run_tokens = format!("{}::{}();", fun_crate_path, fun_name);
            } else {
                run_tokens = format!(
                    "let {} = {}::{}();single_push!({:?},{});",
                    fun_name, fun_crate_path, fun_name, fun_name, fun_name
                );
            }
        }
        insert_code_line = format!("{}{}", insert_code_line, run_tokens);
        let rely_tokens = proc_macro2::TokenStream::from_str(&run_tokens).unwrap();
    }
    return insert_code_line;
}

#[derive(Debug, Clone)]
pub struct FunModel {
    macro_name: String,
    fun_name: String,
    fun_param: String,
    file_path: String,
    fun_return: String,
}

//读取文件并识别Fun
#[allow(dead_code)]
pub fn read_macro_file_path(
    file_path_vec: &Vec<String>,
    scan_macro_vec: &Vec<&str>,
) -> Vec<FunModel> {
    let mut fun_model_vec = vec![];
    for file_path_str in file_path_vec {
        let mut unparsed_file = String::new();
        if let Ok(r) = fs::read_to_string(&file_path_str) {
            unparsed_file = r.clone();
        } else {
            continue;
        }
        let file = ComponentScanParser::parse(Rule::file, unparsed_file.as_str())
            .expect("unsuccessful parse")
            .next()
            .unwrap();
        for line in file.into_inner() {
            match line.as_rule() {
                Rule::scan_macro_fun_content => {
                    let mut inner_rules = line.into_inner();
                    let macro_char_group = inner_rules.next().unwrap().as_str().to_string();
                    let macro_name_sp = macro_char_group.split("(");
                    let mut macro_name = String::new();
                    for macro_name_str in macro_name_sp {
                        macro_name = macro_name_str.trim().to_string();
                        break;
                    }
                    let fun_name_group = inner_rules.next().unwrap().as_str().to_string();
                    let fun_brackets_group = inner_rules.next().unwrap().as_str().to_string();
                    let fun_return_group = inner_rules.next().unwrap().as_str().to_string();

                    if scan_macro_vec.contains(&macro_name.as_str()) {
                        let fun_model = FunModel {
                            macro_name: macro_name,
                            fun_name: fun_name_group,
                            fun_param: fun_brackets_group,
                            file_path: file_path_str.clone(),
                            fun_return: fun_return_group,
                        };
                        fun_model_vec.push(fun_model.clone());
                    }
                }
                Rule::EOI => (),
                _ => (),
            }
        }
    }
    return fun_model_vec;
}

/// 获取有效的扫描文件
fn get_effective_file(config_vo: ComponentScanVO) -> Vec<String> {
    let config = config_vo;
    //最终需要扫描的所有目录
    let path_list = config.scan_path.unwrap();
    let lib_path = config.lib_path.unwrap();
    let exculde_path = config.exculde_path.unwrap();
    let all_dir_path_ls = get_effective_dir(&path_list, &exculde_path, &lib_path);
    //需要扫描的文件后缀
    let mut scan_suffix_vec = Vec::<&str>::new();
    let scan_suffix = config.scan_suffix.unwrap_or("".to_string()).clone();
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
