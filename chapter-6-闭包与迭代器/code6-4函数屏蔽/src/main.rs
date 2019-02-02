fn f() {
    println!("in 1")
}

fn main() {
    println!("Hello, world!");
    f();
    println!("over");
    {
        f();
        fn f() {println!("in 3")}
    }

    f();
    fn f() {println!("in 2")}
}
