use std::any::{ self};


///
/// 类似于 https://crates.io/crates/anybox
/// 该作者好像不再维护，使用率也很低
/// 所以在原来的基础上 尝试与继续维护
/// 大致内容是通过 修改指针的表示类型，从而得到需要类型的引用，从而达到转换类型的目的  所以重新命名为指针盒子
/// 警告！本人对于rust的认知不知道是否存在 大于u8大小的指针
/// 
#[derive(Debug,Clone)] 
pub struct  PointerBox {
    pub type_name:String,
    pub type_id: any::TypeId,
    pub value_box: Box<u8>,
}


#[macro_export]
macro_rules! pointer_box {
    ($val:expr) => {
        $crate::com::pointer_box::PointerBox::new($val)
    };
}



impl PointerBox {
    pub fn new<'a,T: 'static>(value: T) -> Self {
        let type_id = any::TypeId::of::<T>();
        let value_box = Box::new(value);
        let value_box = unsafe { std::mem::transmute::<Box<T>, Box<u8>>(value_box) };
        let type_name = any::type_name::<T>();
        Self { type_id, value_box ,type_name:type_name.to_string()}
    }

    pub fn cast_ref_try<'a, T: 'static>(&'a self) -> Option<&'a T> {
        let ty = any::TypeId::of::<T>();
        if ty != self.type_id {
            return None;
        }
        let value = unsafe { std::mem::transmute::<&u8, &T>(self.value_box.as_ref()) };
        Some(value)
    }

    pub fn cast_ref<'a, T: 'static>(&'a self) -> &T {
        match self.cast_ref_try::<T>() {
            None => {
                let full_name = any::type_name::<T>();
                panic!("the {} not cast {}",self.type_name, full_name);
            }
            Some(t) => t,
        }
    }
}