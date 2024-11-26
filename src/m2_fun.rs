pub fn run() {
    block();
}

// fn declares a function - 定义一个函数
fn greet() {
    println!("Hi there!");
}

// 返回值
fn fair_dice_roll() -> i32 {
    4
}

// Blocks - 块，使用；使用brackets, 有自己的scope
// A pair of brackets declares a block, which has its own scope:
// 此处不会shadowing
fn block() {
    let x = "out";
    {
        // this is a different `x`
        let x = "in";
        println!("{}", x);
    }
    println!("{}", x);
}

// Blocks are expressions - 块也是表达式
fn block2() {
    // this:
    let x = 42;

    // is equivalent to this:
    let x = { 42 };

    // block里面可以多行
    let x = {
        let y = 1; // first statement
        let z = 2; // second statement
        y + z // this is the *tail* - what the whole block will evaluate to
    };
}

// Implicit return - 隐式return
fn fair_dice_roll_2() -> i32 {
    4
    // return 4; // 两者是等价的，更习惯不用return
}

// Everything is an expression - 一切皆为表达式
// if conditionals are also expressions - if 表达式
fn fair_dice_roll_3(feeling_lucky: bool) -> i32 {
    if feeling_lucky {
        6
    } else {
        4
    }
}

// A match is also an expression - match也是一个表达式
fn fair_dice_roll_4(feeling_lucky: bool) -> i32 {
    match feeling_lucky {
        true => 6,
        false => 4,
    }
}
