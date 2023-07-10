#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    age: u32,
    width: u32,
    height: u32,
}

impl User {
    // &self 是 self: &Self 的缩写
    fn get_area(&self) -> u32 {
        self.width * self.height
    }
}

impl User {
    // &self 是 self: &Self 的缩写
    fn isChild(&self) -> bool {
        self.age < 18
    }

    // 不是方法的关联函数
    fn square(n: u32) -> Self {
        Self{
            width: n,
            height: n,
            active: false,
            username: "".to_owned(),
            email: "".to_owned(),
            age: 1,
        }
    }
}

// 没有任何字段的类单元结构体
struct AlwaysEqual;

fn main() {
    let mut u1 = User {
        active: true,
        username: "mik".to_owned(),
        email: String::from("some@some.org"), //这里不能直接用&str，具体尚未知
        age: 12,
        width: 10,
        height: 30,
    };

    u1.active = false;

    // ..u1表示其余的使用了u1的东西，
    // 在这个过程中，使用了=赋值，无法再使用u1了
    let u2 = User {
        username: String::from("some"),
        ..u1
    };

    println!("Hello, {}! {:#?}", u2.email, User::square(32));

    // 元组结构体
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    let c1 = Color(12, 32, 52);
    print!("{:#?}", c1);
    dbg!(c1);

    // 5.1总是相同
    let subject = AlwaysEqual;

    println!("user width * height: {}", area(&u2));
    println!("user: {:#?}", u2);
    println!("user get_area width * height: {}", u2.get_area());
}

fn area(user: &User) -> u32 {
    user.width * user.height
}
