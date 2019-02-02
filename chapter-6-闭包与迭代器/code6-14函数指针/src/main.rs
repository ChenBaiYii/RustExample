fn hello() {
    println!("hello function pointer");
}


fn main() {
    println!("Hello, world!");
    let fn_ptr: fn() = hello;
    println!("fn_ptr pointer address: {:p}", fn_ptr);

    let other_fn = hello;  // 必需显式声明 : fn() 才是函数指针
//    println!("{:p}", other_fn)
    hello();
    other_fn();
    fn_ptr();
    (fn_ptr)();
}
