use pest::Parser;

#[derive(Debug, Clone)]
pub struct FunContent {
    pub fun_name: String,
    pub fun_param: String,
    pub fun_head_end_group: String,
    pub fun_for_content: String,
}

#[derive(Parser)]
#[grammar = "./pestf/component_celler_input.pest"]
pub struct ComponentScanThisParser;

pub fn read_this_parset(unparsed_file: String) -> Option<FunContent> {
    let file = ComponentScanThisParser::parse(Rule::file, unparsed_file.as_str())
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::scan_macro_fun_content => {
                let mut inner_rules = line.into_inner();
                let fun_name_group = inner_rules.next().unwrap().as_str().to_string();
                let fun_brackets_group = inner_rules.next().unwrap().as_str().to_string();
                let fun_head_end_group = inner_rules.next().unwrap().as_str().to_string();
                let fun_for_content = inner_rules.next().unwrap().as_str().to_string();
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
