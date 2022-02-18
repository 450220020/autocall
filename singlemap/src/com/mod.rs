pub mod single_loader;
pub mod map;
pub mod pointer_box;
pub mod pointer_map;


#[macro_export]
macro_rules! for_substring {
    ($str:expr,$first:expr,$last:expr) => {
        $str.split_at($first).1.split_at($last-$first).0
    };
}
