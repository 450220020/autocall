extern crate dashmap;
extern crate once_cell;
pub mod com;



#[test]
fn test_map(){
    single_push!("a",Box::new("aaaa".to_string()));
    let straa = single_get_unwrap!("a",String);
    println!("rustl:{:?}",straa);
}