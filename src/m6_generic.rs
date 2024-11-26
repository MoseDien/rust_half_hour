pub fn run() {
    main();
    main_2();
    main_3();
}
// Generic functions - 范型
// 范型函数以及参数
/*
// rust不支持函数重载，因此函数名称不能一样
fn foobar<T>(arg: T) {
    // do something with `arg`
}
*/
fn foobar<L, R>(left: L, right: R) {
    // do something with `left` and `right`
}

use std::fmt::Display;
use std::fmt::Debug;
// Type parameter constraints (trait bounds) - 类型参数限制
fn print<T: Display>(value: T) {
    println!("value = {}", value);
}
// 另外一种写法
/*
fn print<T>(value: T)
where
    T: Display,
{
    println!("value = {}", value);
}

fn print<T: Debug>(value: T) {
    println!("value = {:?}", value);
}
*/

// 为什么推荐 where？ -- 需要满足多个traits的时候看上去更自然
fn compare<T>(left: T, right: T)
where
    T: Debug + PartialEq,
{
    println!("{:?} {} {:?}", left, if left == right { "==" } else { "!=" }, right);
}

fn main() {
    compare("tea", "coffee");
    // prints: "tea" != "coffee"

    // Monomorphization - 单态化
    // 范型方法可以认为是一个namespace，使用::
    // This is lovingly called turbofish syntax, because ::<> looks like a fish.
    use std::any::type_name;
    println!("{}", type_name::<i32>()); // prints "i32"
    println!("{}", type_name::<(f64, char)>()); // prints "(f64, char)"
}
// type_name是一个范型函数 - 如下定义
/*
pub fn type_name<T>() -> &'static str
where
    T: ?Sized,
*/

// Generic structs - 范型结构
struct Pair<T> {
    a: T,
    b: T,
}

fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main_2() {
    let p1 = Pair { a: 3, b: 9 };
    let p2 = Pair { a: true, b: false };
    print_type_name(&p1); // prints "Pair<i32>"
    print_type_name(&p2); // prints "Pair<bool>"
}

fn main_3() {
    let mut v1 = Vec::new();
    v1.push(1);
    let mut v2 = Vec::new();
    v2.push(false);
    print_type_name(&v1); // prints "Vec<i32>"
    print_type_name(&v2); // prints "Vec<bool>"
}