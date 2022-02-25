
pub fn get_lib_crate_path()->String{
    return autopath::get_lib_crate_path();
}


pub fn get_path_symbol() -> String {
    autopath::get_path_symbol()
}

pub fn path_sym_cast(path_str: &str, sym: &str) -> String {
    autopath::path_sym_cast(path_str,sym)
}

pub fn get_caller_path()->String{
    let sym = get_path_symbol();
    let call_site_span = proc_macro::Span::call_site();
    let call_path = path_sym_cast(call_site_span
        .source_file()
        .path()
        .to_str()
        .unwrap_or(""),&sym);
        println!("call_path:{:?}",call_path);
    return autopath::get_caller_path(call_path);
}

pub fn get_caller_file_path()->String{
    let sym = get_path_symbol();
    let call_site_span = proc_macro::Span::call_site();
    
    let call_path = path_sym_cast(call_site_span
        .source_file()
        .path()
        .to_str()
        .unwrap_or(""),&sym);
        println!("call_file_path:{:?}",call_path);
    return autopath::get_caller_file_path(call_path);
}
