// Owned types vs reference types - 拥有类型+引用类型
// Strings: String is owned, &str is a reference.
// Paths: PathBuf is owned, &Path is a reference.
// Collections: Vec<T> is owned, &[T] is a reference. 

pub fn run() {
    main_slice();
    main_range();
    main_slice_borrow();
    main_static_array();
    main_string_slice();
    main_result();
    main_deref();
    main_fn();
    main_apply();
    main_loop();
    main_return_closure();
}

// Slices - 切片，切片是引用
// 同时也是操作符 [] 的重载 - Index/IndexMut
fn main_slice() {
    let v = vec![1, 2, 3, 4, 5];
    let v2 = &v[2..4];
    println!("v2 = {:?}", v2);
}

// Operator overloading - 操作符重载
// range符号 .. - [左闭右开) , ..= [左闭右闭-open-ended]
fn main_range() {
    // 0 or greater - [0,
    println!("{:?}", (0..).contains(&100)); // true
    // strictly less than 20 - [0, 20)
    println!("{:?}", (..20).contains(&20)); // false
    // 20 or less than 20 - [0, 20]
    println!("{:?}", (..=20).contains(&20)); // true
    // only 3, 4, 5 - [3, 6)
    println!("{:?}", (3..6).contains(&4)); // true
}

// Borrowing rules and slices - 
fn tail(s: &[u8]) -> &[u8] {
    &s[1..] 
}

// 两者是等价的
fn tail2<'a>(s: &'a [u8]) -> &'a [u8] {
    &s[1..] 
}
  
fn main_slice_borrow() {
    let x = &[1, 2, 3, 4, 5];
    let y = tail(x);
    println!("y = {:?}", y);
    let y = tail2(x);
    println!("y = {:?}", y);
}

fn main_static_array() {
    let y = {
        let x = &[1, 2, 3, 4, 5]; // 静态生命周期
        tail(x)
    };
    println!("y = {:?}", y);
}

/*
fn main_dyn_array() {
    let y = {
        let v = vec![1, 2, 3, 4, 5]; // 非静态，局部
        tail(&v)
        // error: `v` does not live long enough
    };
    println!("y = {:?}", y);
}
*/

// String slices (&str) - String切片
/*
// last返回的是个切片引用
pub const fn last(&self) -> Option<&T>
Returns the last element of the slice, or None if it is empty.
*/
fn file_ext(name: &str) -> Option<&str> {
    // this does not create a new string - it returns
    // a slice of the argument. 
    // split 返回的是对原字符串的引用，不会创建新的字符串。
    // 返回值：返回一个 Split 迭代器，其中包含分割后的子字符串。
    name.split(".").last()
}

fn main_string_slice() {
    let name = "Read me. Or don't.txt";
    if let Some(ext) = file_ext(name) {
        println!("file extension: {}", ext);
    } else {
        println!("no file extension");
    }

    /*
    let ext = {
        let name = String::from("Read me. Or don't.txt");
        file_ext(&name).unwrap_or("")
        // error: `name` does not live long enough
    };
    println!("extension: {:?}", ext);
    */
}

// Fallible functions (Result<T, E>) - 可失败的函数
fn main_result() {
    let s = std::str::from_utf8(&[240, 159, 141, 137]);
    println!("{:?}", s);
    // prints: Ok("🍉")

    let s = std::str::from_utf8(&[195, 40]);
    println!("{:?}", s);
    // prints: Err(Utf8Error { valid_up_to: 0, error_len: Some(1) })

    // 如果出错则直接panic
    // let s = std::str::from_utf8(&[195, 40]).unwrap();
    // expect可以输出错误消息 - 包括trace
    // let s = std::str::from_utf8(&[195, 40]).expect("valid utf-8");

    // if let
    if let Ok(s) = std::str::from_utf8(&[240, 159, 141, 137]) {
        println!("{}", s);
    }
}

// Or you can bubble up the error - 把错误往上传递
fn main_bubble_up_error() -> Result<(), std::str::Utf8Error> {
    match std::str::from_utf8(&[240, 159, 141, 137]) {
        Ok(s) => println!("{}", s),
        Err(e) => return Err(e),
    }
    Ok(())
}

// 简写 - 使用？
fn main_bubble_up_error_2() -> Result<(), std::str::Utf8Error> {
    let s = std::str::from_utf8(&[240, 159, 141, 137])?;
    println!("{}", s);
    Ok(())
}

// Dereferencing - 解引用
#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn negate(p: Point) -> Point {
    Point {
        x: -p.x,
        y: -p.y,
    }
}

fn main_deref() {
    let p = Point { x: 1.0, y: 3.0 };
    let p_ref = &p;
    // 非primitive类型 编译器自动解引用
    println!("({}, {})", p_ref.x, p_ref.y);

    // 此处用到了Copy，如果没有 derive-copy，则无法通过编译
    negate(*p_ref); // ...and now this works
}

// Function types, closures
fn for_each_planet<F>(f: F)
    where F: Fn(&'static str)
{
    f("Earth");
    f("Mars");
    f("Jupiter");
}

fn for_each_planet_static<F>(f: F)
    where F: Fn(&'static str) + 'static // `F` must now have "'static" lifetime
{
    f("Earth");
    f("Mars");
    f("Jupiter");
}


fn main_fn() {
    let greeting = String::from("Good to see you");
    for_each_planet(|planet| println!("{}, {}", greeting, planet));
    // our closure borrows `greeting`, so it cannot outlive it

    // may outlive borrowed value `greeting`
    // F的生命周期是static，但参数的周期是local
    // for_each_planet_static(|planet| println!("{}, {}", greeting, planet));

    // move 之后，greeting的生命周期和F一样了
    for_each_planet_static(move |planet| println!("{}, {}", greeting, planet));
}

// FnMut
fn foobar<F>(mut f: F)
    where F: FnMut(i32) -> i32
{
    // second mutable borrow occurs here
    // println!("{}", f(f(2))); 

    // 这个是合法的
    let tmp = f(2);
    println!("{}", f(tmp)); 
}
 
fn main_fuMut() {
    foobar(|x| x * 2);
}

// 带两个参数的closure
fn apply_something<F>(x: i32, y: i32, is_greater: F)
    where F: Fn(i32, i32) -> bool
{
    let (greater, smaller) = if is_greater(x, y) {
        (x, y)
    } else {
        (y, x)
    };
    println!("{} is greater than {}", greater, smaller);
}

// 
fn countdown<F>(count: usize, tick: F)
    where F: Fn(usize)
{
    for i in (1..=count).rev() {
        tick(i);
    }
}
 
fn main_apply() {
    apply_something(32, 64, |x, y| x > y);

    // 甚至可以panic
    // apply_something(32, 64, |_, _| panic!("Comparing is futile!"));

    countdown(3, |i| println!("tick {}...", i));

    // toilet closure - 
    // 这个closure什么也不做
    countdown(3, |_| ());
}

// Loops, iterators - 循环和迭代器
fn main_loop() {
    for i in vec![52, 49, 21] {
        println!("I like the number {}", i);
    }

    for i in &[52, 49, 21] {
        println!("I like the number {}", i);
    }

    // note: `&str` also has a `.bytes()` iterator.
    // Rust's `char` type is a "Unicode scalar value"
    // &str 有 .bytes() 和 .chars() 方法
    for c in "rust".chars() {
        println!("Give me a {}", c);
    }

    // + filter + map
    for c in "SuRPRISE INbOUND"
        .chars()
        .filter(|c| c.is_lowercase())
        .flat_map(|c| c.to_uppercase()) // map也可以
    {
        print!("{}", c);
    }
    println!();
}

// 返回一个闭包
fn make_tester(answer: String) -> impl Fn(&str) -> bool {
    move |challenge| {
        challenge == answer
    }
}

// Capturing into a closure - move a reference
// 甚至可以把一个引用move到闭包里面
fn make_tester2<'a>(answer: &'a str) -> impl Fn(&str) -> bool + 'a {
    move |challenge| {
        challenge == answer
    }
}

// 2和3等价
fn make_tester3(answer: &str) -> impl Fn(&str) -> bool + '_ {
    move |challenge| {
        challenge == answer
    }
}

// 如下版本无法通过编译 - 
/*
fn make_tester4(answer: &str) -> impl Fn(&str) -> bool {
    move |challenge| {
        challenge == answer
    }
}
*/
fn main_return_closure() {
    // you can use `.into()` to perform conversions
    // between various types, here `&'static str` and `String`
    let test = make_tester("hunter2".into());
    println!("{}", test("******"));
    println!("{}", test("hunter2"));

    // 
    let test = make_tester2("hunter2");
    println!("{}", test("*******"));
    println!("{}", test("hunter2"));
}
