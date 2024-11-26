pub fn run() {
    m1();
    m2();
    m3();
}

fn m1() {
    // Variable bindings - 变量绑定
    let x; // declare "x"
    x = 42; // assign 42 to "x"

    // 可以一步到位
    let x = 42;

    // Type annotation - 类型标记
    let x: i32; // `i32` is a signed 32-bit integer
    x = 42;
    // 或者
    let x: i32 = 42;


    //+ Uninitialized variables - 没有初始化的变量是不能使用的
    let x;
    // foobar(x); // error: borrow of possibly-uninitialized variable: `x`
    x = 42;

    //+ Throwing values away - 不使用的量
    // this does *nothing* because 42 is a constant
    let _ = 42;

    // this calls `get_thing` but throws away its result
    let _ = get_thing();

    //+ 不使用的变量，名字前面加_, 编译器就不会警告
    // we may use `_x` eventually, but our code is a work-in-progress
    // and we just wanted to get rid of a compiler warning for now.
    let _x = 42;

    //+ Shadowing bindings - 变量遮蔽, 改变类型
    let x = 13;
    let x = x + 3;
    let x: &str = "mystr";
    // using `x` after that line only refers to the second `x`,
    //
    // although the first `x` still exists (it'll be dropped
    // when going out of scope), you can no longer refer to it.
}

fn foobar(i: i32) {
}

fn get_thing() -> String {
    "MyString".to_string()
}

// split_at() - Divides one slice into two at an index.
fn m2() {
    //+ Tuples - 元组
    let pair = ('a', 17);
    pair.0; // this is 'a'
    pair.1; // this is 17

    let pair: (char, i32) = ('a', 17);

    // Destructuring tuples - 元组的拆取
    let (some_char, some_int) = ('a', 17);
    // now, `some_char` is 'a', and `some_int` is 17
    let slice: &str = "Hello world";
    let middle: usize = 5;
    let (left, right) = slice.split_at(middle);
    let (_, right) = slice.split_at(middle);
}

fn m3() {
    // Statements - 声明语句
    // 分号是语句结束的标志
    let x = 3;
    let y = 5;
    let z = y + x;

    //+ 多行
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
    .iter()
    .map(|x| x + 3)
    .fold(0, |x, y| x + y);
}