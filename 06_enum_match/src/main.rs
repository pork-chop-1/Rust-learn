
enum IpAddrKind {
    ipv4,
    ipv6
}
fn main() {
    let four = IpAddrKind::ipv4;
    let six = IpAddrKind::ipv6;

    struct IpAddr {
        kind: IpAddrKind,
        address: String
    }

    let home = IpAddr{
        kind: four,
        address: String::from("22.22.22.22")
    };

    // 可以将数据附加到枚举中
    enum ipAddrKind2 {
        ipV4(String),
        ipV6(String)
    }

    let loopAddress = ipAddrKind2::ipV4(String::from("0.0.0.0"));

    // 空值假设：枚举
    // let some_number: Option<i32> = None;
    let some_number: Option<i32> = Some(123);
    // some_number = 13;
    println!("Hello, world!{}", some_number.expect("som1e"));

    //----------- match------------
    println!("{}", match_test(home.kind));

    println!("{}", match_option(Some(14)).expect("msg"));

    // ---------let if ------------
    let config_max: Option<u8> = Some(3u8);
    // if let 是可以看做match的一个语法糖，它不会检索是否全部匹配
    if let Some(max) = config_max {
        println!("max is {}", max)
    }
}

fn match_test(value: IpAddrKind) -> i8 {
    match value {
        IpAddrKind::ipv4 => 1,
        IpAddrKind::ipv6 => 2
    }
}

// 匹配option
fn match_option(value: Option<i8>) -> Option<i8> {
    match value {
        // None => None,
        Some(i) => Some(i+1),
        // 其他匹配
        other => other
        // 当不使用的时候可以_
        // _ => None
    }
}
