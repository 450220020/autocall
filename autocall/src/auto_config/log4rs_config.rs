use log4rs;

pub fn loading() {
    let config_path = single_get_ref!("log4rs_config_path");
    let config_path_str = config_path.cast_ref::<String>();
    info!("loading log4rs path:{}",&config_path_str);
    let result = log4rs::init_file(config_path_str, Default::default()).unwrap();
    info!("loading log4rs result:{:?}",result);
}