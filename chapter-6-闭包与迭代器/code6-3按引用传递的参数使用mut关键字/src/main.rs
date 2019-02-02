//fn modify(v: &mut [u32]) {
fn modify(v: &mut Vec<u32>) {
    v.reverse();
}

fn main() {
    println!("Hello, world!");
    let mut v = vec![1, 2, 3];
    modify(&mut v);
    println!("after {:?}", v);
}
