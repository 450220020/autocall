use core::panic;
use std::{collections::HashMap, env, fs, path::Path, str::FromStr};
use pest::Parser;
pub use proc_macro::TokenStream;
use crate::com::toml_read;
use crate::com::component_celler_read;
use quote::quote;


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

const SYMBOL_STR: &str = "/";

fn get_path_symbol() -> String {
    String::from(SYMBOL_STR)
}






pub fn impl_component_scan(_attr: &TokenStream, _input: &TokenStream,celler_path:&str) -> TokenStream {
    println!("_input:{:?}",_input.to_string());
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

    println!("file_path_vec:{:?}",file_path_vec);
    let  fun_vec = read_macro_file_path(&file_path_vec,&scan_macro_vec);
    println!("fun_vec:{:?}",fun_vec);
    let loading_code_str = fun_model_to_code(fun_vec,celler_path);

    let celler_fun_code_op = component_celler_read::read_this_parset(_input.clone().to_string());
    
    if let None =celler_fun_code_op{
        panic!("not parset:{}",_input.clone().to_string());
    }
    let celler_fun = celler_fun_code_op.unwrap();
    let mut fun_for_content = celler_fun.fun_for_content;
    println!("fun_for_contentaaa:{:?}",fun_for_content);
    println!("substring(fun_{:?}",fun_for_content.find("{").unwrap());
    println!("fun_for_con{:?}",fun_for_content.rfind("}").unwrap()-3);
    println!("fun_for_con{:?}",fun_for_content.len());
    
    fun_for_content  = for_substring!(&fun_for_content,fun_for_content.find("{").unwrap()+1, fun_for_content.rfind("}").unwrap()).to_string();
  
    let result_code = format!("pub fn {}({}){}{{{}{}}}",celler_fun.fun_name,celler_fun.fun_param,celler_fun.fun_head_end_group,fun_for_content,loading_code_str);
    println!("result_code:{}",result_code);
    //let result_token_stream = proc_macro2::TokenStream::from_str(&result_code).unwrap();
    
    return TokenStream::from(quote!(result_code));
}


///识别的函数集转换为执行代码
pub fn  fun_model_to_code(fun_vec:Vec<FunModel>,celler_path:&str)->String{
     for model in fun_vec{
        let attr_map = param_attr_split_to_map(&model.fun_param);
        println!("attr_map:{:?}-{:?}",attr_map,&model.fun_param);
        let mut rely_vec = vec!();
        let mut rely_name_type = vec!();
        for rely_str in &model.fun_param.split(",").collect::<Vec<&str>>(){
            if rely_str.is_empty(){
                continue;
            }
            match rely_str.split_once(":") {
                Some((key,_))=>{
                    rely_vec.push(key.to_string());
                    rely_name_type.push(rely_str.to_string());
                    
                }
                None=>{
                    continue;
                }
            }
        }
        let crate_path_str = path_lib_link(&model.file_path,&celler_path,&celler_path);
        //导入所有依赖
        push_loader!(model.fun_name.clone(),(false,model.fun_name.clone(),rely_vec,crate_path_str[0].clone(),rely_name_type,model.fun_return));
    }
    let sort_result = sort_loader!();
    println!("sort_result:{:?}",sort_result);
    let rely_sort;
    match sort_result{
        Ok(r)=>{
            rely_sort = r;
        }
        Err(r)=>{
            panic!("Error rely :{:?}",r);
        }
    }
    let mut insert_code_line = String::new();
    //按照依赖顺序装载
    for rely in rely_sort{
        let fun_name = rely.1;
        let fun_rely_vec = rely.4;
        let fun_crate_path = rely.3;
        let fun_return = rely.5;
        let run_tokens ;
        if fun_rely_vec.len()>0{
            println!("fun_rely_vec:{:?}",fun_rely_vec);
            let mut rely_str = String::new();
            for rely_name_type in fun_rely_vec{
                let (key,val) = rely_name_type.split_once(":").unwrap();
                let mut val_str = val.trim().clone();
                if val_str.starts_with("&"){
                    val_str = val_str.split_at(1).1;
                }
                rely_str=rely_str+"single_get_unwrap!(\""+&key+"\","+val_str+"),";
            }
            rely_str = for_substring!(rely_str,0,rely_str.len()-1).to_string();
            if fun_return.is_empty(){
                run_tokens = format!("{}::{}({});",fun_crate_path,fun_name,rely_str);
            }else{
                run_tokens = format!("let {} = {}::{}({});single_push!({:?},{});",fun_name,fun_crate_path,fun_name,rely_str,fun_name,fun_name);
            }
            
        }else{
            //run_tokens = format!("let {} = {}::{}();single_push!({:?},{});",fun_name,fun_crate_path,fun_name,fun_name,fun_name);
            //run_tokens = format!("let {} = {}::{}();",fun_name,fun_crate_path,fun_name);
            if fun_return.is_empty(){
                run_tokens = format!("{}::{}();",fun_crate_path,fun_name);
            }else{
                run_tokens = format!("let {} = {}::{}();single_push!({:?},{});",fun_name,fun_crate_path,fun_name,fun_name,fun_name);
            }
        }
        insert_code_line = format!("{}{}",insert_code_line,run_tokens);
        let rely_tokens = proc_macro2::TokenStream::from_str(&run_tokens).unwrap();
        println!("rely_tokens:{:#?}",rely_tokens.to_string());
    }
    println!("insert_code_line:{:#?}",insert_code_line);
    return insert_code_line;
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
        if let None = call_path.find(&sym_src){
            return String::new();
        }
        let work_dir_name = work_path.split_at(work_path.rfind(&sym).unwrap()).1;
        let call_dir_name = call_path.split_at(call_path.find(&sym_src).unwrap()).0;
        if !work_dir_name.eq(call_dir_name){
            ast_path = work_path+&sym + call_dir_name;
        }else{
            ast_path = get_work_path();
        }
    }else{
        
        let call_ast_path = path_sym_cast(&call_path, &sym);
        ast_path =  for_substring!(call_ast_path,0,call_ast_path.rfind(&sym_src).unwrap_or(call_ast_path.len())).to_string();
    }
    return ast_path;
}


#[derive(Debug,Clone)]
pub struct FunModel{
    macro_name:String,
    fun_name:String,
    fun_param:String,
    file_path:String,
    fun_return:String,
}


// #[derive(Debug,Clone)]
// pub struct FunContent{
//     pub fun_name:String,
//     pub fun_param:String,
//     pub fun_head_end_group:String,
//     pub fun_for_content:String,
// }

//读取文件并识别Fun
#[allow(dead_code)]
pub fn read_macro_file_path(file_path_vec: &Vec<String>, scan_macro_vec: &Vec<&str>)->Vec<FunModel> {
    println!("扫面文件:{:?}", file_path_vec);
    println!("扫描注解:{:?}", scan_macro_vec);
    let mut fun_model_vec = vec!();
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
                    println!("识别的内容");
                    let mut inner_rules = line.into_inner();
                    let macro_char_group = inner_rules.next().unwrap().as_str().to_string();
                    println!("macro_char_group:{:?}", macro_char_group);
                    let macro_name_sp = macro_char_group.split("(");
                    let mut macro_name = String::new();
                    for macro_name_str in macro_name_sp {
                        macro_name = macro_name_str.trim().to_string();
                        break;
                    }
                    let fun_name_group = inner_rules.next().unwrap().as_str().to_string();
                    println!("fun_name_group:{:?}", fun_name_group);

                    let fun_brackets_group = inner_rules.next().unwrap().as_str().to_string();
                    println!("fun_brackets_group:{:?}",fun_brackets_group);

                    let fun_return_group = inner_rules.next().unwrap().as_str().to_string();

            
                    if scan_macro_vec.contains(&macro_name.as_str()) {
                        let fun_model = FunModel{
                            macro_name:macro_name,
                            fun_name:fun_name_group,
                            fun_param:fun_brackets_group,
                            file_path:file_path_str.clone(),
                            fun_return:fun_return_group,
                        };
                        fun_model_vec.push(fun_model.clone());
                        println!("fun_model:{:?}", fun_model);
                    }
                }
                Rule::EOI => (),
                _ => (),
            }
        }
    }
    return fun_model_vec;
}

///按文件地址识别

///
/// 获取有效的扫描文件
fn get_effective_file(config_vo: ComponentScanVO) -> Vec<String> {
    let config = config_vo;
    //最终需要扫描的所有目录
    println!("config:{:?}",config);
    let path_list = config.scan_path.unwrap();
    let lib_path = config.lib_path.unwrap();
    let exculde_path = config.exculde_path.unwrap();
    let all_dir_path_ls = get_effective_dir(&path_list, &exculde_path, &lib_path);

    println!("all_dir_path_ls:{:?}",all_dir_path_ls);
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

//获取有效的文件夹
fn get_effective_dir(path_list: &str, exclut_path: &str, lib_path: &str) -> Vec<String> {
    let mut all_dir_path_ls = Vec::new();
    //引用包实际目录
    println!("path_list:{:?}",path_list);
    let act_path_vec = lib_path_link(path_list, lib_path);
    println!("act_path_vec:{:?}",act_path_vec);
    //排除包实际目录
    let act_exclude_path_vec = lib_path_link(exclut_path, lib_path);
    let mut act_exclude_path_concat_str = String::new();
    for act_exclude_path_str in act_exclude_path_vec {
        act_exclude_path_concat_str = act_exclude_path_concat_str + "," + &act_exclude_path_str;
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


pub fn param_attr_split_to_map(attr_str: &str) -> HashMap<String, String> {
    attr_str
        .replace("/", "")
        .replace('\n', "")
        .replace('\\', "")
        .trim()
        .to_string();
    let attr_split = attr_str.split(",");
    let mut attr_map = HashMap::<String, String>::new();
    for attr_mate in attr_split {
        let attr_sp = attr_mate.split_once(":");
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
    println!("通过目录:{:?}", path_str);
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
                            dir_entity_path.to_str().unwrap_or_else(||{panic!("dir_entity_path is null");}),
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
    let current_lib_name =  for_substring!(&lib_config_path,lib_config_path.rfind(&sym).unwrap_or(0)+1, lib_config_path.len()).to_string();
    println!("current_lib_name:{:?}",current_lib_name);
    let current_lib_path = "../".to_string()+&current_lib_name;
    toml_ver_map.insert(current_lib_name.clone(), (current_lib_path.clone(),true));
    println!("toml_ver_map:{:?}",toml_ver_map);
    let check_lib_path = path_list.replace("::", &sym).clone();
    let lib_path_split = check_lib_path.split(",");
    let mut path_parent_list: Vec<String> = vec![];
    for path_str in lib_path_split {
        if path_str.is_empty(){
            continue;
        }
        println!("path_str:{:?}",path_str);
        let mut frist_mod_path;
        let  the_path_str = path_str.replace("::", &sym).replace(&up_sym, "");
        let last_mod_path = for_substring!( the_path_str,the_path_str.find(&sym).unwrap_or(the_path_str.len()-1)+1 , the_path_str.len())
            .to_string();
            println!("last_mod_path:{:?}",last_mod_path);
            println!("the_path_str:{:?}",the_path_str);
        let lib_name = the_path_str.clone().split(&sym).collect::<Vec<&str>>()[0].to_string();
        println!("lib_name:{:?}",lib_name);
        //是否引用依赖
        let lib_attr_opt = toml_ver_map.get(&lib_name);
        //实际地址
        match lib_attr_opt {
            Some(val) => {
                println!("val:{:?}",val);
                if val.1 {
                    println!("lib_config_path:{:?}",lib_config_path);
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
        println!("frist_mod_path:{:?}",frist_mod_path);
        if Path::new(&(frist_mod_path.clone() + &sym + "src")).exists() {
            frist_mod_path += &(sym.clone() + "src");
        }
        let result_path = frist_mod_path + &sym + &last_mod_path;
        println!("result_path:{:?}",result_path);
        path_parent_list.push(result_path);
    }
    return path_parent_list;
}




///set_lib_path = 实际包地址 env!("CARGO_MANIFEST_DIR")
/// set_lib_path = 调用宏实际包地址
/// 将实际文件路径转换为包路径
#[warn(unused_must_use)]
pub fn path_lib_link(path_list: &str, set_lib_path: &str,work_lib_path:&str) -> Vec<String> {
    let sym = get_path_symbol();
    let lib_config_path = path_sym_cast(set_lib_path, &sym);
    //C:\\Users\\45022\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\extends-rs-0.1.6
    //读取工作目录依赖文件信息 找到对应版本依赖文件扫描读取

    let toml_path = lib_config_path.clone() + &sym + "Cargo.toml";
    let mut toml_ver_map = toml_read::read_path_toml_lib_ver(&toml_path);
    let current_lib_name = for_substring!(lib_config_path,lib_config_path.rfind(&sym).unwrap_or(0)+1, lib_config_path.len()).to_string();
    let current_lib_path = "../".to_string()+&current_lib_name;
    toml_ver_map.insert(current_lib_name.clone(), (current_lib_path.clone(),true));
    println!("toml_ver_map:{:?}",toml_ver_map);
    //let check_lib_path = path_list.replace(&sym, "::").clone();
    let lib_path_split = path_list.clone().split(",");
    let mut path_parent_list: Vec<String> = vec![];
    for path_str in lib_path_split {
        if path_str.is_empty(){
            continue;
        }
        println!("path_str:{:?}",path_str);
        let mut the_path_str =String::new();
        //如果有后缀 去掉
        if let Some(r)=path_str.rfind("."){
            if path_str.rfind(&sym).unwrap()<r{
                the_path_str  = for_substring!(path_str,0, path_str.rfind(".").unwrap()).to_string();
            }
        }
        
        if let None = the_path_str.rfind("src"){
            panic!("the path not find src")
        }
        let jump_dir_vec  = the_path_str.split(&sym).map(|f| f.to_string()).collect::<Vec<String>>();
        println!("jump_dir_vec:{:?}",jump_dir_vec);
        let mut crate_path_str = String::new();
        let mut jump_record_str = the_path_str.clone();
        let max_idx = jump_dir_vec.len()-1;
        for x in 0..max_idx{
            let jump = jump_dir_vec[max_idx-x].clone();
            crate_path_str= jump.clone()+"::"+&crate_path_str;
            println!("crate_path_str+=:{:?}",crate_path_str);
            jump_record_str  = get_jump_folder(&jump_record_str, &sym, 1);
            println!("jump_record_str+=:{:?}",jump_record_str);
           if Path::new( &(jump_record_str.clone()+&sym+"Cargo.toml")).exists(){
               let crate_dir_name = for_substring!(jump_record_str,jump_record_str.rfind(&sym).unwrap()+1, jump_record_str.len()).to_string();
               crate_path_str= crate_dir_name+"::"+&crate_path_str;
                break;
           }
        }
        crate_path_str = for_substring!(crate_path_str,0,crate_path_str.rfind("::").unwrap()).replace("src::", "");
        
        println!("crate_path_str:{:?}",&crate_path_str);
        
        let crate_name = for_substring!(crate_path_str,0, crate_path_str.find("::").unwrap()).to_string();
        if crate_name.contains("-"){
            crate_path_str = crate_path_str.replace(&crate_name,  for_substring!(crate_name,0,crate_name.find("-").unwrap()));
        }
        //执行目录文件下省略引用
        let crate_name_main = crate_name.clone()+"::main";
        let crate_name_lib = crate_name.clone()+"::lib";
        if crate_path_str.starts_with(&crate_name_main){
            crate_path_str = crate_path_str.replace(&crate_name_main,&crate_name.to_string()).to_string(); 
        }
        if crate_path_str.starts_with(&crate_name_lib){
            crate_path_str = crate_path_str.replace(&crate_name_lib,&crate_name.to_string()).to_string(); 
        }
        let work_dir_name = for_substring!(work_lib_path,work_lib_path.rfind(&sym).unwrap(), work_lib_path.len()).replace(&sym,"");
        println!("work_dir_name:{:?}",work_dir_name);
        println!("crate_name:{:?}",crate_name);
        //包与工作目录一致修改为crate
        if work_dir_name.eq(&crate_name){
            crate_path_str  = crate_path_str.replace(&crate_name, "crate");
        }
        println!("crate_path_str rest:{:?}",crate_path_str);
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
            path_str_result = for_substring!(path_str_result
                ,
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
