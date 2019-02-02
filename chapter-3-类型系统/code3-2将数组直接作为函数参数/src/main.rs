//不可确定的类型会异常

//fn reset(mut arr: [u32]) {
//    arr[0] = 5;
//    arr[1] = 4;
//    arr[2] = 3;
//    arr[3] = 2;
//    arr[4] = 1;
//    println!("reset array {:?}", arr);
//}
//
//fn main() {
//    let arr: [u32] = [1, 2, 3, 4, 5];
//    reset(arr);
//    println!("origin arr {:?}", arr);
//}

// 使用指定长度类型 [type; len]

//fn reset(mut arr: [u32; 5]) {
//    arr[0] = 5;
//    arr[1] = 4;
//    arr[2] = 3;
//    arr[3] = 2;
//    arr[4] = 1;
//    println!("reset array {:?}", arr);
//}
//
//fn main() {
//    let arr: [u32; 5] = [1, 2, 3, 4, 5];
//    reset(arr);
//    println!("origin arr {:?}", arr);
//}

// 使用&mut[u32]作为参数类型


fn reset(arr: &mut [u32]) {
    arr[0] = 5;
    arr[1] = 4;
    arr[2] = 3;
    arr[3] = 2;
    arr[4] = 1;
    println!("arr length {:?}", arr.len());
    println!("reset array {:?}", arr);
}

fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    println!("origin array {:?}", arr);

    reset(& mut arr);

    print!("mem use: {:?}", std::mem::size_of::<&[u32; 5]>());
    print!("胖指针 mem use: {:}", std::mem::size_of::<&mut[u32]>());

}



