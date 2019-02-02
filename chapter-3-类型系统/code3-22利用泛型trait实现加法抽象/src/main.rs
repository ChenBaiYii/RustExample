trait Add<RHS, Output> {
    fn my_add(self, rhs: RHS) -> Output;
}


impl Add<i32, i32> for i32 {
    fn my_add(self, rhs: i32) -> i32 {
        self + rhs
    }
}

impl Add<u32, u32> for u32 {
    fn my_add(self, rhs: u32) -> u32 {
        self + rhs
    }
}


fn main() {
    let (a, b, c, d) = (1i32, 2i32, 3u32, 4u32);
    let x: i32 = a.my_add(b);
    assert_eq!(x, 3);

    let b: u32 = c.my_add(d);
    assert_eq!(b, 7);
}
