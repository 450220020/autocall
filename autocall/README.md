# autocall


```rust


//扫描自动组件下的目录 自动初始化选中的内容 autocall_loading_config_json:load_component_list=["log4rs","rabtis"]
#[component_scan(scan_path="autocall::run"scan_macro="bean")]
pub fn lading(){
    println!("crate_ioc_path!() test");
    single_push!("autocall_loading_config_json","{
        \"load_component_list\":[
            \"log4rs\",\"rbatis\"
        ]
    }".to_string());
    let  work_path_str = autocall::auto_config::auto_path::get_work_path();
    //log4rs 配置路径
    single_push!("log4rs_config_path",work_path_str.clone()+"/log4rs.yaml");
    //rbatis 加载的配置
    single_push!("rbatis_config_json","{\"abas\":{\"url\":\"sqlite://E:/tsdb/ab.db\"}}".to_string());
}

```

More patterns and use-cases are in the [docs](https://docs.rs/autoload/)!

# Related crates
* [dashMap](https://crates.io/crates/dashMap)
* [once_cell](https://crates.io/crates/once_cell)
* [singlemap](https://crates.io/crates/singlemap)
