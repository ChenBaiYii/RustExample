type MathOp = fn(i32, i32) -> i32;

fn math(op: &str) -> MathOp {
    fn sum(a:i32, b:i32) -> i32 {
        a + b
    }

    fn product(a:i32, b:i32) -> i32 {
        a * b
    }

    match op {
        "sum" => sum,
        "product" => product,
        _ => {
            println!("warning: not implemented {:?} op, replace with sum", op);
            sum
        }
    }
}


fn main() {
    println!("Hello, world!");
    let (a, b) = (2, 3);
    let sum = math("sum");
    let product = math("product");

    assert_eq!(sum(a, b), 5);
    assert_eq!(product(a, b), 6);
}
