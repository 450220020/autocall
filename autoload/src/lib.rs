#![feature(core_intrinsics)]
#![feature(proc_macro_span)]
#![feature(proc_macro_def_site)]
#[warn(unused_imports)]
#[macro_use]
extern crate singlemap;
extern crate proc_macro;
extern crate proc_macro2;
extern crate once_cell;
extern crate pest;
extern crate quote;
#[macro_use]
extern crate pest_derive;
use proc_macro::TokenStream;
mod com;
use com::component_scan;




#[allow(warnings)]
#[proc_macro_attribute]
pub fn autowired(_attr: TokenStream, _input: TokenStream) -> TokenStream {
    return com::autowired::impl_autowired(_attr, _input.clone());
}



/// # Overview

/// `autoload` 基于 singlemap 实现 ioc aop autowired,更方便的集成库之间的使用:
/// 包含 #[bean] #[aop] #[component_scan]

/// ```rust
/// // 展开
/// // macro_rules! defsingle {
/// //     () => {
/// //         single_get_unwrap!( "defsingle", EntityObj)
/// //     };
/// // }
/// #[autowired]
///  static defsingle: Option<EntityObj> = None;


/// //扫描 atesta::ioca路径下 scan_macro="bean" 使用bean宏标记的函数，扫描的内容会根据 调用宏component_scan的crate位置作为基础坐标，不用担心发布的crate扫描路径不正确的情况
/// #[component_scan(scan_path="atesta::ioca",scan_suffix=".rs",scan_macro="bean")]
/// pub fn lading(){
///     println!("crate_ioc_path!() test");
///     let a = single_get_unwrap!("get_bec",String);
///     println!("输出 参数:{:?}",a);
/// }

/// //加载了一个bean
/// #[bean]
/// pub fn get_bec()->String{
///     println!("加载了一个组件");
///     String::from("99999")
/// }

/// //标记切入的bean
/// #[aop(first_bean="set_aopa",last_bean="set_aopb")]
/// pub fn set_aohhhhh(a:String,b:String)->String{
///     println!("set_aohhhhh:{:?}",a);
///     return String::from("888899");
/// }

/// //位于函数执行前获取到参数
/// #[bean]
/// pub fn set_aopa()->Box<(dyn Fn(Vec<Box<dyn Any>>) + Send + Sync )>{
///     return Box::new(|r:Vec<Box<dyn Any>>|{
///         println!("test bibao");
///     });
/// }

/// //位于函数执行后获取到参数
/// #[bean]
/// pub fn set_aopb()->Box<(dyn Fn(Vec<Box<dyn Any>>) + Send + Sync )>{
///     return Box::new(|r:Vec<Box<dyn Any>>|{
///         println!("test bibaobbbbb");
///     });
/// }
/// ```

/// More patterns and use-cases are in the [docs](https://docs.rs/autoload/)!

/// # Related crates
/// * [dashMap](https://crates.io/crates/dashMap)
/// * [once_cell](https://crates.io/crates/once_cell)
/// * [singlemap](https://crates.io/crates/singlemap)

#[allow(warnings)]
#[proc_macro_attribute]
pub fn bean(_attr: TokenStream, _input: TokenStream) -> TokenStream {
    return _input;
}

#[proc_macro_attribute]
pub fn component_scan(_attr: TokenStream, _input: TokenStream) -> TokenStream {
    let  ast_path = component_scan::get_caller_path();
    let work_path = component_scan::get_work_path();
    println!("ast_path:{:?}",ast_path);
    println!("work_path:{:?}",work_path);
    if ast_path.trim()==""{
        return _input;
    }
    //return _input;
    //panic!("ast_path:{:?}","hhhh");
    component_scan::impl_component_scan(&_attr,&_input,&ast_path)
}


#[proc_macro_attribute]
pub fn aop(_attr: TokenStream, _input: TokenStream) -> TokenStream {
    return com::aop::impl_aop(&_attr,&_input);
}

