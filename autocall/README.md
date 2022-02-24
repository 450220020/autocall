# autocall


```rust


//扫描自动组件下的目录 自动初始化选中的内容 autocall_loading_config_json:load_component_list=["log4rs","rabtis"]
//如果大家有写好的配置也可以发送到autocall这个项目的github 我会根据情况加入到自动配置目录当中
//##当前在构想一个0依赖的自动配置的实现方式，诱因是因为自动配置不断增长所引用的内容会逐渐变大 下载的内容逐渐增多，cargo.htlm 引用自动下载包成为了一个麻烦，希望获得建议
//上面的这个问题解决了通过autoload 实现了引入自动配置的方式
//autocall 内的libs 下的包是用来扫描的配置内容,从而快速配置使用
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

//这是非强制引用加载包的方式,主要通过扫描autocall下的libs的starter包 加载需要自动装配代码实现快速集成
#[starter_scan(scan_path="autocall::libs",lib_name="log4rs-starter,rbatis-starter,actix-web-starter")]
pub fn scan_auto_loading(){
    //用来承载扫描内容的载体
}

```

More patterns and use-cases are in the [docs](https://docs.rs/autoload/)!

# Related crates
* [dashMap](https://crates.io/crates/dashMap)
* [once_cell](https://crates.io/crates/once_cell)
* [singlemap](https://crates.io/crates/singlemap)
