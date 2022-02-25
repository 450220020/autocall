use crate::com::path_utils::{get_caller_file_path, get_path_symbol};
use crate::com::scan_path_utils::path_lib_link;
use crate::com::scan_path_utils::{self, attr_split_to_map};
use pest::Parser;
pub use proc_macro::TokenStream;
use std::{fs, str::FromStr};

#[derive(Parser)]
#[grammar = "./pestf/starter_scan_input.pest"]
pub struct StarterScanParser;

pub fn impl_starter_scan(
    _attr: &TokenStream,
    _input: &TokenStream,
    celler_path: &String,
) -> TokenStream {
    let attr_str = _attr.to_string();
    let sym = get_path_symbol();
    let sym_src_lib_str = sym.clone() + "src/lib.rs";
    let mut scan_path = String::new();
    let mut lib_name = String::new();
    let scan_suffix = String::from(".rs");
    let param = attr_split_to_map(&attr_str);

    if let Some(r) = param.get("scan_path") {
        scan_path = r.clone().trim().to_string();
    }
    if let Some(r) = param.get("lib_name") {
        lib_name = r.clone().trim().to_string();
    }
    let mut true_file_paht_vec = vec![];
    if !lib_name.is_empty() {
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
    let caller_file_path = get_caller_file_path();
    //生成调用代码
    let mut loading_code_str = String::new();
    let crate_path_vec = path_lib_link(&caller_file_path, &celler_path, &celler_path);
    let crate_path_str = &crate_path_vec[0];
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
    println!("starter_scan_code:\n{}", result_code);
    let result_token_stream = proc_macro2::TokenStream::from_str(&result_code).unwrap();
    return TokenStream::from(result_token_stream);
}

pub fn dorp_select_parset(unparsed_file: String) -> String {
    let mut up_unparsed_file = unparsed_file.clone();
    let rd_unparsed_file = unparsed_file.clone();
    let file = StarterScanParser::parse(Rule::file, &rd_unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::extern_content => {
                let inner_rules = line.as_str();
                up_unparsed_file = up_unparsed_file.replace(inner_rules, "");
            }
            Rule::EOI => (),
            _ => (),
        }
    }
    up_unparsed_file = up_unparsed_file.replace("#[macro_use]", "");
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
    let all_dir_path_ls = scan_path_utils::get_effective_dir(&path_list, &exculde_path, &lib_path);
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
    let file_path_vec = scan_path_utils::read_dir_file(all_dir_path_ls, scan_suffix_vec);
    return file_path_vec;
}
