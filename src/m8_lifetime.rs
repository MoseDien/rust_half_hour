pub fn run() {
    main_1();
    main_2();
    main_3();
    main_4();
    main_5();
}

// Variables bindings have a "lifetime":
// 变量绑定
// Similarly, references have a lifetime:
// 引用也有
fn main_1() {
    // `x` doesn't exist yet
    {
        let x = 42; // `x` starts existing
        let x_ref = &x; // `x_ref` starts existing - it borrows `x`
        println!("x_ref = {}", x_ref);
        // `x_ref` stops existing
        // `x` stops existing
    }
    // `x` no longer exists

    // The lifetime of a reference cannot exceed the lifetime of the variable binding it borrows:
    // 引用的生命不能超过变量绑定的生命周期 - 如下语句无法通过编译
    // ^^ borrowed value does not live long enough
    /*
    let x_ref = {
        let x = 42;
        &x
    };
    println!("x_ref = {}", x_ref);
    */
}

// A variable binding can be immutably borrowed multiple times:
// 可以存在多个借用 - 尤其是不可变借用
fn main_2() {
    let x = 42;
    let x_ref1 = &x;
    let x_ref2 = &x;
    let x_ref3 = &x;
    println!("{} {} {}", x_ref1, x_ref2, x_ref3);
}

// x在借用期间不能修改
fn main_3() {
    let mut x = 42;
    let x_ref = &x;
    // x = 13; // error: cannot assign to `x` because it is borrowed
    println!("x_ref = {}", x_ref);
}

// While immutably borrowed, a variable cannot be mutably borrowed:
// 在借用期间不能再可变借用
fn main_4() {
    let mut x = 42;
    let x_ref1 = &x;
    // let x_ref2 = &mut x; // error: cannot borrow `x` as mutable because it is also borrowed as immutable
    println!("x_ref1 = {}", x_ref1);
}
// ---- 生命周期使用的是范型的方式
// Functions generic over lifetimes - 函数的生命周期
// 函数和函数参数的生命周期并不完全一样 - 因此就出现了
// elided (non-named) lifetimes - 省略的写法
// fn print(x: &i32) {}
// named lifetimes - 完整的写法
fn print<'a>(x: &'a i32) {}
// 此处 <'a> 这是定义生命周期，而 'a 则是引用


// This allows returning references whose lifetime depend on the lifetime of the arguments:
// 返回值的生命周期==参数的生命周期
struct Number {
    value: i32,
}

// Lifetime elision
// 当只有一个输入的生命周期的时候，其实不需要显式写出来，因为函数/参数/返回值生命周期都是一样的
// 如下两种写法等价，因此第一种更好，但是编译器会给加上lifetime参数
fn number_value(num: &Number) -> &i32 {
    &num.value
}

fn number_value_0<'a>(num: &'a Number) -> &'a i32 {
    &num.value
}

fn main_5() {
    let n = Number { value: 47 };
    let v = number_value(&n);
    // `v` borrows `n` (immutably), thus: `v` cannot outlive `n`.
    // While `v` exists, `n` cannot be mutably borrowed, mutated, moved, etc.
}

// -----
// Structs generic over lifetimes - 结构带生命周期
struct NumRef<'a> {
    x: &'a i32,
}

// <'_> - 是省略写法，其实他也可以省掉
fn as_num_ref(x: &i32) -> NumRef<'_> {
    NumRef { x: &x }
}

// 省略写法，此处a不需要被引用
impl NumRef<'_> {
    fn as_i32_ref(&self) -> &i32 {
        self.x
    }
}

fn main_6() {
    // 也就是说最终 x_ref的生命周期取决于x，
    let x: i32 = 99;
    let x_ref = as_num_ref(&x);
    // `x_ref` cannot outlive `x`, etc.
}
// ---- static 生命周期
// 'static - 在整个app生命周期里面都有效
struct Person {
    name: &'static str,
}

fn main_7() {
    // 这个name本身就是静态的
    let p = Person {
        name: "fasterthanlime",
    };

    // 这个name的生命周期不够长，只是一个local
    let name = format!("fasterthan{}", "lime");
    // let p = Person { name: &name };
    // error: `name` does not live long enough
}

// 不带生命周期的版本 - name的所有权被移动到Person里面，不存在生命周期的问题
/*
struct Person {
    name: String,
}

fn main_8() {
    let name = format!("fasterthan{}", "lime");
    let p = Person { name: name };
    // `name` was moved into `p`, their lifetimes are no longer tied.
}
*/

// Struct literal assignment shorthand - struct的字面赋值简写
// when a field is set to a variable binding of the same name:
// 很奇怪的语法 - 当field名字和变量绑定的名字是一样的时候可以简写 
fn main_8() {
    let name = "Ming";
    let p = Person { name: name };
    let p = Person { name };

    let full_name = "LiMing";
    let p = Person { name: full_name };
    // error: `Person` does not have this field
    // let p = Person { full_name };
}