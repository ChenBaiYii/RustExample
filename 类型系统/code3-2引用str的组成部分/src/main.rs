fn main() {
    let str = "hello rust";
    let ptr = str.as_ptr();
    let len = str.len();
    println!("{:p}", ptr);
    println!("{:?}", len);
}
