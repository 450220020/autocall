# autopath

`autopath`  autoload 的附属包

```rust
#[test]
fn test_path(){
    let a_p = "C:/Users/Joinz/.cargo/registry/src/github.com-1ecxxxxx9ec823/autocall-0.1.6/src/lib.rs".to_string();
    let b_p = "rscontr/src/lib.rs".to_string();
    let c_p = "src/lib.rs".to_string();
    let a= get_caller_file_path(a_p.clone());
    let b= get_caller_file_path(b_p.clone());
    let c= get_caller_file_path(c_p.clone());
    println!(" a:{} \n b:{} \n c:{}",a,b,c);


    let a= get_caller_path(a_p.clone());
    let b= get_caller_path(b_p.clone());
    let c= get_caller_path(c_p.clone());
    println!(" a:{} \n b:{} \n c:{}",a,b,c);

    let e = get_lib_crate_path();
    let f = get_work_path();

    println!("e:{} ,f:{}",e,f);
}

```

More patterns and use-cases are in the [docs](https://docs.rs/autopath/)!

# Related crates
* [dashMap](https://crates.io/crates/dashMap)
* [once_cell](https://crates.io/crates/once_cell)
* [singlemap](https://crates.io/crates/singlemap)
