use std::{path::{Path, self}, fs};
#[macro_use]
extern crate singlemap;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate serde_json;
pub mod auto_config;
pub mod run;


#[test]
fn copy_the_lib(){
 
    let sym = get_path_symbol();
    let crate_lib_path = env!("CARGO_MANIFEST_DIR");
    let crate_lib_sym_path = path_sym_cast(&crate_lib_path, &sym);
    let lib_path = for_substring!(crate_lib_sym_path,0,crate_lib_sym_path.rfind(&sym).unwrap_or(crate_lib_sym_path.len())).to_string();

    let copy_dir = lib_path.clone()+"/autocall/libs_test";
    println!("copy_dir:{}",copy_dir);
    let pase_dir = lib_path.clone()+"/autocall/libs";

    let mut all_path_ls:Vec<String> = vec!();
    let su_vec = vec!(".rs",".toml");
    read_path_all_dir(&copy_dir,"",&mut all_path_ls,0,4);
    println!("read_dir:{:?}",all_path_ls);
    let all_file = read_dir_file(all_path_ls, su_vec);
    println!("all_file:{:?}",all_file);
    for file_path in all_file{
        let mut dir_paht = file_path.split_at(file_path.rfind("/").unwrap()+1).0.to_string();
        dir_paht = dir_paht.replace("/libs_test/", "/libs/");
        let file_asb_path  = file_path.replace("Cargo.toml", "Cargo.to").replace("/libs_test/", "/libs/");
        println!("check path:{}",dir_paht);
        check_file(&dir_paht);
        let f_rs = fs::copy(&file_path, &file_asb_path);
        println!("copy file:{} \n to file:{}  \n {:?}",&file_path,&file_asb_path,f_rs);
    }
}


const SYMBOL_STR: &str = "/";

pub fn get_path_symbol() -> String {
    String::from(SYMBOL_STR)
}

pub fn path_sym_cast(path_str: &str, sym: &str) -> String {
    path_str.replace("\\", sym).replace("/", sym)
}


pub fn check_file(path:&str){
    let path_string = path.to_string();
    let last_x_idx = path_string.rfind("/").unwrap_or(0);
    let dir_str = &path[0..last_x_idx];
    let dir_path = path::Path::new(dir_str);
    if!dir_path.exists(){
      info!("文件夹不存在，path:{:?}",dir_str);
      let cr_fs_ex = fs::create_dir_all(dir_path);
      match cr_fs_ex {
          Ok(_)=>{info!("创建成功，path：{:?}",dir_str);}
          _=>{info!("创建失败");}
      }
    }else{
        info!("文件夹已存在");
    }
    if last_x_idx==(path_string.len()-1){
        info!("地址只指向了目录，不进行文件创建");
    }else{
        let file_path = path::Path::new(path);
        if!file_path.exists(){
            info!("文件不存在,path:{:?}",file_path);
            let file_rs  =   fs::File::create(path);
            match  file_rs{
                Ok(_)=>{
                    info!("创建成功");
                }
                _=>{
                    info!("创建失败");
                }

            }
        }else{
            info!("目标文件已存在");
        }
    }
}




/// 读取文件夹指定后缀文件
pub fn read_dir_file(path_vec: Vec<String>, suffix_vec: Vec<&str>) -> Vec<String> {
    let mut all_path_ls = Vec::<String>::new();
    for path_str in path_vec {
        let path_rs = Path::new(&path_str);
        if path_rs.exists() && path_rs.is_dir() {
            let file_rs = path_rs.read_dir();
            if let Ok(r) = file_rs {
                for dir_entity_rs in r {
                    if let Ok(dir_entity) = dir_entity_rs {
                        if let Some(name_path) = dir_entity.path().to_str() {
                            let mut suffix_exists_vec = Vec::<&str>::new();
                            for suffix_str in &suffix_vec {
                                if suffix_exists_vec.contains(suffix_str) {
                                    continue;
                                }
                                if let Some(_) = name_path.strip_suffix(suffix_str) {
                                    all_path_ls.push(path_sym_cast(name_path, &get_path_symbol()));
                                    suffix_exists_vec.push(suffix_str);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return all_path_ls;
}

///
/// 获取当前地址所有文件夹
pub fn read_path_all_dir(
    path_str: &str,
    exclude_path_str: &str,
    all_path_ls: &mut Vec<String>,
    current_level: usize,
    max_level: usize,
) {
    let exclude_sp = exclude_path_str.split(",").collect::<Vec<&str>>();
    if exclude_sp.contains(&path_str) {
        return;
    }
    if current_level > max_level + 1 {
        return;
    }
    let path_rs = Path::new(&path_str);
    if path_rs.exists() && path_rs.is_dir() {
        all_path_ls.push(path_sym_cast(path_str, &get_path_symbol()));
        let dir_rs = path_rs.read_dir();
        if let Ok(r) = dir_rs {
            for dir_entity_rs in r {
                if let Ok(dir_entity) = dir_entity_rs {
                    let dir_entity_path = dir_entity.path();
                    if dir_entity_path.is_dir() {
                        read_path_all_dir(
                            dir_entity_path.to_str().unwrap_or_else(|| {
                                panic!("dir_entity_path is null");
                            }),
                            exclude_path_str,
                            all_path_ls,
                            current_level + 1,
                            max_level,
                        );
                    }
                }
            }
        }
    }
}



///上级目录
pub fn get_jump_folder(path_str_the: &str, lev_str: &str, set_lev_size: usize) -> String {
    let mut path_str_result = path_str_the.to_string();
    if let None = path_str_result.find(lev_str) {
        path_str_result = path_str_the.replace("\\", lev_str).replace("/", lev_str);
    }
    let mut up_lev_size = set_lev_size;
    if up_lev_size > 0 {
        let up_size_max = path_str_result.split(lev_str).collect::<Vec<&str>>().len() - 1;
        if up_lev_size > up_size_max {
            up_lev_size = up_size_max;
        }
        for _ in 0..up_lev_size {
            path_str_result = for_substring!(
                path_str_result,
                0,
                path_str_result
                    .rfind(lev_str)
                    .unwrap_or(path_str_result.len())
            )
            .to_string();
        }
    }
    path_str_result
}
