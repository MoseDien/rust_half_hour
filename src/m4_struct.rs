pub fn run() {
    main_1();
    main_2();
    main_method();
}

// Structs are declared with the struct keyword - 结构的定义
// 最后那个逗号不能少
struct Vec2 {
    x: f64, // 64-bit floating point, aka "double precision"
    y: f64,
}

fn main_1() {
    // They can be initialized using struct literals - 使用结构字面初始化
    let v1 = Vec2 { x: 1.0, y: 3.0 };
    let v2 = Vec2 { y: 2.0, x: 4.0 };

    // Struct update syntax - update语法
    let v3 = Vec2 {
        x: 14.0,
        ..v2
    };

    let v4 = Vec2 { ..v3 };
}

fn main_2() {
    // Destructuring structs - 结构的解构
    let v = Vec2 { x: 3.0, y: 6.0 };
    let Vec2 { x, y } = v;
    // `x` is now 3.0, `y` is now `6.0` - 可以直接获取这两个值
    println!("{x}, {y}");

    // this throws away `v.y` - 丢掉y
    let Vec2 { x, .. } = v;
}

// Patterns and destructuring - 模式与解构
struct Number {
    odd: bool,
    value: i32,
}

fn main_3() {
    let one = Number { odd: true, value: 1 };
    let two = Number { odd: false, value: 2 };
    print_number(one);
    print_number(two);
}

// if let 来匹配其中的字段
// 注意 - 如下的print_number使用的都不是 &Number, 而是 Number，
// 因此所有权转移了 - 因为我们没有实现Copy/Clone
fn print_number(n: Number) {
    if let Number { odd: true, value } = n {
        println!("Odd number: {}", value);
    } else if let Number { odd: false, value } = n {
        println!("Even number: {}", value);
    }
}

// 通过match
fn print_number2(n: Number) {
    match n {
        Number { odd: true, value } => println!("Odd number: {}", value),
        Number { odd: false, value } => println!("Even number: {}", value),
    }
}

// Exhaustive matches - 详尽匹配 - 
fn print_number3(n: Number) {
    match n {
        Number { value: 1, .. } => println!("One"),
        Number { value: 2, .. } => println!("Two"),
        Number { value, .. } => println!("{}", value), // 必须
        // if that last arm didn't exist, we would get a compile-time error
    }
}

fn print_number4(n: Number) {
    match n.value {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("{}", n.value), // 必须
    }
}

// Method - 定义Number的方法
impl Number {
    fn is_strictly_positive(self) -> bool {
        self.value > 0
    }
}

fn main_method() {
    let minus_two = Number {
        odd: false,
        value: -2,
    };
    println!("positive? {}", minus_two.is_strictly_positive());
    // this prints "positive? false"

    // Immutability - 变量绑定默认是不可变的，因此不能修改
    let n = Number {
        odd: true,
        value: 17,
    };
    /*
    n = Number {
        odd: false,
        value: 22,
    }; // error: cannot assign twice to immutable variable `n`
    */
    let mut n = Number {
        odd: true,
        value: 17,
    }; // 这个分号不能少
    n.value = 19; // all good
}