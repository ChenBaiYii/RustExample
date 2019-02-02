// 函数定义形式

fn func_name(arg1:u32, arg2: String) -> Vec<u32> {
    /* 函数体 */
    let f = vec![1, 2, 3];
    f
}

// 用 Raw identifier 将语言关键字用作函数名字 (rust 2018)
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    println!("Hello, world!");
    assert!(r#match("foo", "foobar"));
}
