use core::panic;
use std::{ str::FromStr};
use pest::Parser;
use proc_macro::TokenStream;

use super::component_scan::attr_split_to_map;

#[derive(Debug, Clone)]
pub struct FunContent {
    pub fun_name: String,
    pub fun_param: String,
    pub fun_head_end_group: String,
    pub fun_for_content: String,
}

#[derive(Parser)]
#[grammar = "./pestf/aop_celler_input.pest"]
pub struct AopCellerParser;

pub fn read_this_parset(unparsed_file: String) -> Option<FunContent> {
    let file = AopCellerParser::parse(Rule::file, unparsed_file.as_str())
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::scan_macro_fun_content => {
                let mut inner_rules = line.into_inner();
                let fun_name_group = inner_rules.next().unwrap().as_str().to_string();
                println!("fun_name_group:{:?}", fun_name_group);

                let fun_brackets_group = inner_rules.next().unwrap().as_str().to_string();
                println!("fun_brackets_group:{:?}", fun_brackets_group);

                let fun_head_end_group = inner_rules.next().unwrap().as_str().to_string();
                println!("fun_head_end_group:{:?}", fun_head_end_group);
                let fun_for_content = inner_rules.next().unwrap().as_str().to_string();
                println!("fun_for_content:{:?}", fun_for_content);
                return Some(FunContent {
                    fun_name: fun_name_group,
                    fun_param: fun_brackets_group,
                    fun_head_end_group: fun_head_end_group,
                    fun_for_content: fun_for_content,
                });
            }
            Rule::EOI => (),
            _ => (),
        }
    }
    return None;
}


pub fn impl_aop(_attr: &TokenStream, _input: &TokenStream) -> TokenStream {
    let attr_str = _attr.clone().to_string();
    let input_str = _input.clone().to_string();
    let param = attr_split_to_map(&attr_str);
    let mut first_bean = String::new();
    if let Some(r) = param.get("first_bean") {
        first_bean = r.clone();
    }
    let mut last_bean = String::new();
    if let Some(r) = param.get("last_bean") {
        last_bean = r.clone();
    }
    let fun_content_op = read_this_parset(input_str);

    match fun_content_op {
        Some(celler_fun) => {
            let mut fun_for_content = celler_fun.fun_for_content;
            fun_for_content = for_substring!(
                &fun_for_content,
                fun_for_content.find("{").unwrap() + 1,
                fun_for_content.rfind("}").unwrap()
            )
            .to_string();

            let source_code_str = format!(
                "pub fn aop_{}({}){}{{{}}}",
                celler_fun.fun_name,
                celler_fun.fun_param,
                celler_fun.fun_head_end_group,
                fun_for_content
            );
            println!("fun_head_end_group:{:?}", celler_fun.fun_head_end_group);
            let first_set_param_str =
                format!("let mut aop_param_vec = Vec::<Box<dyn Any>>::new();");
            let last_set_param_str = format!("let mut aop_last_param_vec = Vec::<Box<dyn Any>>::new();aop_last_param_vec.push(Box::new(aop_result.clone()));");
            let mut insert_param_str = String::new();
            let mut aop_set_param_str = String::new();
            for rely_str in &celler_fun.fun_param.split(",").collect::<Vec<&str>>() {
                if rely_str.is_empty() {
                    continue;
                }
                match rely_str.split_once(":") {
                    Some((key, _)) => {
                        aop_set_param_str += &(key.trim().to_string() + ",");
                        let insert_str =
                            format!("aop_param_vec.push(Box::new({}.clone()));", key.trim());
                        insert_param_str = insert_param_str + &insert_str;
                    }
                    None => {
                        continue;
                    }
                }
            }
            aop_set_param_str =
                for_substring!(aop_set_param_str, 0, aop_set_param_str.rfind(",").unwrap())
                    .to_string();

            let mut aop_first_str = String::new();
            if !first_bean.is_empty() {
                aop_first_str = format!(
                    "single_get_unwrap!({:?},{})(aop_param_vec);",
                    first_bean, "Box<(dyn Fn(Vec<Box<dyn Any>>) + Send + Sync )>"
                );
            }

            println!("aop_first_str:{:?}", aop_first_str);
            let run_source_code_str = format!(
                "let aop_result = aop_{}({});",
                celler_fun.fun_name, aop_set_param_str
            );
            let mut aop_last_str = String::new();

            if !last_bean.is_empty() {
                aop_last_str = format!(
                    "{} single_get_unwrap!({:?},{})(aop_last_param_vec);",
                    last_set_param_str,
                    last_bean,
                    "Box<(dyn Fn(Vec<Box<(dyn Any)>>) + Send + Sync )>"
                );
            }
            println!("aop_last_str:{:?}", aop_last_str);
            let content_concat_str = format!(
                "{}{}{}{}{}",
                first_set_param_str,
                insert_param_str,
                aop_first_str,
                run_source_code_str,
                aop_last_str
            );
            let code_str = format!(
                "pub fn {}({}){}{{{}  return aop_result; }} {}",
                celler_fun.fun_name,
                celler_fun.fun_param,
                celler_fun.fun_head_end_group,
                content_concat_str,
                source_code_str
            );

            println!("rs_code_str:{:?}", code_str);
            let result_token_stream = proc_macro2::TokenStream::from_str(&code_str).unwrap();

            return TokenStream::from(result_token_stream);
        }
        None => {
            panic!("parset this error");
        }
    }
}
