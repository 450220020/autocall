use dashmap::DashMap;
use once_cell::sync::OnceCell;
use std::any::Any;

type SingleKeyType = String;
#[warn(dead_code)]
type SingleType = (dyn Any + std::marker::Send + Sync);

#[warn(dead_code)]
static INSTANCE: OnceCell<DashMap<SingleKeyType, Box<SingleType>>> = OnceCell::new();

pub static INSTANCE_SINGLE: OnceCell<&DashMap<SingleKeyType, Box<SingleType>>> = OnceCell::new();


#[macro_export]
macro_rules! for_substring {
    ($str:expr,$first:expr,$last:expr) => {
        $str.split_at($first).1.split_at($last-$first).0
    };
}

#[macro_export]
macro_rules! get_map {
    () => {
        $crate::com::map::INSTANCE_SINGLE.get_or_init(|| $crate::com::map::single_init())
    };
}

#[macro_export]
macro_rules! single_push {
    ($key:expr,$val:expr) => {
        get_map!().entry($key.to_string()).or_insert($val)
    };
}

#[macro_export]
macro_rules! single_get_unwrap {
    ($key:expr,$b:ty) => {
        get_map!()
            .get($key)
            .unwrap()
            .downcast_ref::<$b>()
            .unwrap().clone()
    };
}

#[warn(dead_code)]
pub fn single_init<'a>(
) -> &'a DashMap<SingleKeyType, Box<(dyn Any + Sync + std::marker::Send + 'a)>> {
    return INSTANCE.get_or_init(|| DashMap::new());
}
