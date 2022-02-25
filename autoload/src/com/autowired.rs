use pest::Parser;
use proc_macro::TokenStream;
use quote::quote;

#[derive(Parser)]
#[grammar = "./pestf/autowired_input.pest"]
pub struct AutowiredParser;

#[warn(dead_code)]
pub fn impl_autowired(_attr: TokenStream, _input: TokenStream) -> TokenStream {
    let input_code = _input.to_string();
    let mut macro_name  = String::new();
    let mut type_name = String::new();
    let file = AutowiredParser::parse(Rule::file, input_code.as_str())
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::attr_content => {
                let mut inner_rules = line.into_inner();
                let attr_name_char = inner_rules.next().unwrap().as_str();
                let attr_type_char= inner_rules.next().unwrap().as_str();
                macro_name = attr_name_char.trim().clone().to_string();
                type_name = attr_type_char.clone().to_string().trim().to_string();
                type_name = for_substring!(type_name,1, type_name.len()-1).to_string();
            }
            Rule::EOI => (),
            _ => (),
        }
    }
    let  macro_name_quote = macro_name.parse::<proc_macro2::TokenStream>().unwrap();
    let  type_name_quote = type_name.parse::<proc_macro2::TokenStream>().unwrap();
    let q = quote!{
        macro_rules! #macro_name_quote{
            () => {
                single_get_unwrap!(#macro_name,#type_name_quote)
            };
        }
    };
    let result_quote = TokenStream::from(q);
    return result_quote;
}

