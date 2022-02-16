# singlemap
rust singlemap 


```rust
#[macro_use]
extern crate singlemap;

#[test]
fn test_map(){
    single_push!("a",Box::new("aaaa".to_string()));
    let straa = single_get_unwrap!("a",String);
    println!("rustl:{:?}",straa);
}
```