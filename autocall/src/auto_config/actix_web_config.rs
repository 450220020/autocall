use actix_web::HttpServer;
use dashmap::DashMap;


#[macro_export]
macro_rules! actix_config_service {
    ($key:expr,$($x:expr), *) => {

            let group_url = $key;
            let fun = Box::new(
                ||->Box<dyn Fn(&mut  web::ServiceConfig)->()>{
                    return Box::new(|cfg: &mut web::ServiceConfig|{
                        $(cfg.service($x);)*
                    });
                }
            );
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
        
    };
}

#[macro_export]
macro_rules! actix_web_run {
    () => {
        $crate::auto_config::actix_web_config::loading()
    };
}

pub async fn loading()-> std::io::Result<()>{
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
    HttpServer::new(move || {
        let mut app = actix_web::App::new();
        let mut service_vec = vec!();
        single_get_ref_try!("actix_config_service_map",DashMap<String,Box<dyn Fn()->Box<dyn Fn(&mut  actix_web::web::ServiceConfig)->()>>>,
        |m:&DashMap<String,Box<dyn Fn()->Box<dyn Fn(&mut  actix_web::web::ServiceConfig)->()>>>|{
            info!("start add actix service");
            for map in m{
                info!("add group:{}",map.key());
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