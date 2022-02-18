use dashmap::DashMap;
use once_cell::sync::OnceCell;

use super::pointer_box::PointerBox;

type SingleKeyType = String;
#[warn(dead_code)]
type SingleType = PointerBox;

#[warn(dead_code)]
static INSTANCE: OnceCell<DashMap<SingleKeyType, SingleType>> = OnceCell::new();


#[macro_export]
macro_rules! get_map {
    () => {
        $crate::com::pointer_map::single_init()
    };
}

#[macro_export]
macro_rules! single_push {
    ($key:expr,$val:expr) => {
        get_map!().entry($key.to_string()).or_insert(pointer_box!($val))
    };
}

#[macro_export]
macro_rules! single_get_unwrap {
    ($key:expr,$b:ty) => {
        get_map!()
            .get($key)
            .unwrap()
            .cast_ref::<$b>()
    };
}

#[macro_export]
macro_rules! single_get_ref {
    ($key:expr) => {
        get_map!()
            .get($key)
            .unwrap()
    };
}

#[macro_export]
macro_rules! single_get_ref_try {
    ($key:expr,$b:ty,$fun:expr,$err:expr) => {
        match get_map!().get($key){
            Some(r)=>{
               match r.cast_ref_try::<$b>(){
                    Some(g)=>{
                       $fun(g);
                    }
                    None=>{
                        $err
                    }
                }
            }
            None=>{
                $err
            }
        }
    };
}

#[warn(dead_code)]
pub fn single_init<'a>(
) -> &'a DashMap<SingleKeyType, SingleType> {
    return INSTANCE.get_or_init(|| DashMap::new());
}
