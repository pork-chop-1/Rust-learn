fn main() {
    // https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html
    println!("Hello, world!");
    // 这里不能运行！
    /*
       let a = String::from("hello");
       let b = take_and_give(a);
       let c = take_and_give(b);
       println!("{}, {}, {}", a, b, c);
    */

    let mut str = String::from("value");
    // 不可对同一个变量创造一个以上的可变引用
    // 哇哦！我们 也 不能在拥有不可变引用的同时拥有可变引用。
    // 数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；Rust 避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！
    let str_ptr = &str;
    let some = slice(str_ptr);
    println!("{}", str);
    println!("{}", str_ptr);
}

fn take_and_give(take: String) -> String {
    println!("{}", take);
    take
}

// 这里不能运行！
/**
fn change(take: &String) {
    take.push_str("string")
}
*/
// mutable reference
fn change2(take: &mut String) {
    take.push_str("string")
}

fn slice(take: &String) -> &str {
    let two_letter = &take[0..2];
    println!("{}", two_letter);
    take
}
