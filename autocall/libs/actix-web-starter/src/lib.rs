use actix_web::{HttpServer,web};
use dashmap::DashMap;
use std::time::{SystemTime};

pub fn loading(){
    info!("actix-web code init ok");
}

pub fn add_actix_config_service(group_url:&str,fun:Box<dyn Fn()->Box<dyn Fn(&mut  web::ServiceConfig)->()>>){
    let config_key = "actix_config_service_map";
    single_get_ref_try!(config_key,
    DashMap<String,Box<dyn Fn()->Box<dyn Fn(&mut  web::ServiceConfig)->()>>>,
    |m:&DashMap<String,Box<dyn Fn()->Box<dyn Fn(&mut  web::ServiceConfig)->()>>>|{
        m.entry(group_url.to_string()).or_insert(fun);
    },{
        let map = DashMap::<String,Box<dyn Fn()->Box<dyn Fn(&mut  web::ServiceConfig)->()>>>::new();
        single_push!(config_key,map);
        single_get_ref_try!(config_key,
        DashMap<String,Box<dyn Fn()->Box<dyn Fn(&mut  web::ServiceConfig)->()>>>,
        |m:&DashMap<String,Box<dyn Fn()->Box<dyn Fn(&mut  web::ServiceConfig)->()>>>|{
            m.entry(group_url.to_string()).or_insert(fun);
        },{
            info!("add service error");
        });
    });
}




pub async fn run()-> std::io::Result<()>{
    let mut addr_str = String::new();
    single_get_ref_try!("actix_config_addr",String,|addr:&String|{
        info!("actix addr:{}",addr);
        addr_str = addr.clone();
    },{
        error!("actix not set addr");
    }); 
    if addr_str.is_empty(){
        panic!("actix-web error addr is null");
    }
    
    HttpServer::new(move|| {
        let mut app = actix_web::App::new();
        single_get_ref_try!("actix_config_service_map_loading_true",bool,|is_loading:&bool|{
            info!("actix set config copy ing:{}",is_loading);
        },{
              info!("start add actix service:{:?}",SystemTime::now());
              single_get_ref_try!("actix_config_service_map",DashMap<String,Box<dyn Fn()->Box<dyn Fn(&mut  actix_web::web::ServiceConfig)->()>>>,
            |m:&DashMap<String,Box<dyn Fn()->Box<dyn Fn(&mut  actix_web::web::ServiceConfig)->()>>>|{
                for map in m{
                    info!("add group:{}",map.key());
                }
            },{
                error!("not find actix_config_service_map");
            });
              //设置时上锁
              single_push!("actix_config_service_map_loading_true",true);
        });

        let mut service_vec = vec!();
            single_get_ref_try!("actix_config_service_map",DashMap<String,Box<dyn Fn()->Box<dyn Fn(&mut  actix_web::web::ServiceConfig)->()>>>,
            |m:&DashMap<String,Box<dyn Fn()->Box<dyn Fn(&mut  actix_web::web::ServiceConfig)->()>>>|{
                for map in m{
                    match m.get(map.key()){
                        Some(r) => {
                            service_vec.push(r.value()());
                        },
                        None => {
                            info!("not other config");
                        }
                    }
                    
                }
            },{
                error!("not find actix_config_service_map");
            });
            for ser in service_vec{
                app = app.configure(ser);
            }
        app
    })
    .bind(&addr_str)?
    .run()
    .await
}