# singlemap
rust singlemap 

更新 通过指针转换类型获得引用 
```rust
#[macro_use]
extern crate singlemap;

#[test]
fn test_map(){
    single_push!("a",Box::new("aaaa".to_string()));
    let straa = single_get_unwrap!("a",Box<String>).clone();
    let refaa = single_get_ref!("a");
    println!("straa:{:?}",straa);
    println!("rustlaa:{:?}",refaa.cast_ref::<Box<String>>());
    single_get_ref_try!("a",Box<String>,|r:&Box<String>|{
        println!("single_get_try:{:?}",r);
    },{println!("error")});
}
```