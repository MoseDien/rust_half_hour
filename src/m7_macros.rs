pub fn run() {
    macros();
    panic();
    main_enum();
}

fn macros() {
    // println!
    println!("{}", "Hello there!");

    // 等价
    use std::io::{self, Write};
    io::stdout().lock().write_all(b"Hello there!\n").unwrap();
}

fn panic() {
    // panic!("This panics");

    let o1: Option<i32> = Some(128);
    o1.unwrap(); // this is fine

    let o2: Option<i32> = None;
    // o2.unwrap(); // this panics!
}

// Option is enum - 自定义，会覆盖掉 std::option::Option
enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    fn unwrap(self) -> T {
        // enums variants can be used in patterns:
        match self {
            Self::Some(t) => t,
            Self::None => panic!(".unwrap() called on a None option"),
        }
    }
}

use self::Option::{None, Some};

fn main_enum() {
    let o1: Option<i32> = Some(128);
    o1.unwrap(); // this is fine

    let o2: Option<i32> = None;
    // o2.unwrap(); // this panics!
}

// Result 也是枚举
enum Result<T, E> {
    Ok(T),
    Err(E),
}