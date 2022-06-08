extern crate dashmap;
extern crate once_cell;
pub mod com;




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