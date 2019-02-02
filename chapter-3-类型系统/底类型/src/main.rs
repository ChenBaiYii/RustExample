// 底类型


fn bottom() -> ! {
    panic!("l")
}


fn main() {
    let i = if false {
        bottom()
    } else {
        100
    };
    println!("i value: {:?}", i);
//    assert_eq!(i, 100);
}