use rbatis::{core::db::DBPoolOptions, rbatis::Rbatis};
use futures::executor::block_on;
use serde_json::{Value};

pub fn loading() {
    single_get_ref_try!("rbatis_config_json",String,|config_json_str:&String|{
    block_on(async{
            info!("start loading rbatis");
            info!("rbatis config:{:?}",config_json_str);
            let config_json = serde_json::from_str::<Value>(config_json_str).unwrap();
            match  config_json{
                Value::Object(r)=>{
                    let keys = r.keys();
                    for key in keys{
                        let config = &r[key].as_object().unwrap();
                        let rb = Rbatis::new();
                        let mut opt =DBPoolOptions::new();
                        opt.max_connections=100;
                        let con = rb.link_opt(config["url"].as_str().unwrap(),opt).await;
                        match con {
                            Ok(_)=>{
                                info!("rbatis link :{:?} ok",config["url"]);
                                single_push!(key,rb);
                            }
                            Err(r)=>{
                                error!("rbatis link error:{:?}",r);
                            }
                        }
                    }
                }
                _=>()
            }
            info!("end loading rbatis");
        
    })},{
    });
}