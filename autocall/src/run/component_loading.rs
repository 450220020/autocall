use serde_json::{Value};
use crate::auto_config;

/// 加载预设定好的组件快速集成
#[bean]
pub fn autocall_config_loading(){
    single_get_ref_try!("autocall_loading_config_json",String,|config_json_str:&String|{
        let config_json = serde_json::from_str::<Value>(config_json_str).unwrap();
        let load_component_list = &config_json["load_component_list"];
        //加载预备的组件内容
        match load_component_list {
            Value::Array(r)=>{
                if r.contains(&Value::String(String::from("log4rs"))){
                    auto_config::log4rs_config::loading();
                }
                if r.contains(&Value::String(String::from("rbatis"))){
                    auto_config::rbatis_config::loading();
                }
            }
            _=>()
        }
    },{
        println!("autocall_loading_config_json not find");
    });
}