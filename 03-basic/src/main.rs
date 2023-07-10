fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
        // The value of x in the inner scope is: 12
    }

    println!("The value of x is: {x}");
    // The value of x is: 6

    let spaces = "      ";
    let spaces = spaces.len();
    // 这样就不用起spaces_str和spaces_num之类的名字了
    // 不能改变变量的类型，一如既往
    println!("{}", spaces);

    println!("function return: {}", 复合类型_compound_type());

    condition();

    loop_control();

}

fn 复合类型_compound_type() -> i32 {
    let tuple: (i32, i32) = (1, 2);
    println!("{}", tuple.1);

    let arr: [i32; 5] = [1, 2, 3, 4, 6];
    println!("{}", arr[0]);

    // 表达式：最后一行没有；
    let y = {
        let x = 3;
        x + 1
    };

    // return y;
    y
}

fn condition() {
    let b = true;
    if b {
        let b: i32 = if b { 1 } else { 2 };
        println!("{}", b);
    } else {
        println!("false");
    }
}

fn loop_control() {
    let mut count = 0;
    let end_loop = 'out_loop: loop {
        let count_in_loop = 'in_loop: loop {
            count += 1;
            if count % 5 == 0 {
                break 'in_loop count;
            } else if count >= 100 {
                break 'out_loop count;
            }
        };
        println!("count in loop {}", count_in_loop);
        count *= 2;
    };
    println!("count * 2: {}", end_loop);

    while count < 300 {
        count += 10;
    }

    for i in count..320 {
        println!("{}", i);
    }
}
