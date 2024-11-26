pub fn run() {
    main();
}
// Traits - 特性
// 注意 - 这个traits使用self而不是 &self, 因此我们要实现Copy
trait Signed {
    fn is_strictly_negative(self) -> bool;
}

#[derive(Debug, Clone, Copy)]
struct Number {
    odd: bool,
    value: i32,
}

/* 
// 手动实现 Copy/Clone - 本质上一样的
impl Copy for Number { }

impl Clone for Number {
    fn clone(&self) -> MyStruct {
        *self
    }
}
*/ 

impl Signed for Number {
    fn is_strictly_negative(self) -> bool {
        self.value < 0
    }
}

// Our trait on a foreign type (a primitive type, even) - 
// 也可以实现在外部结构上，包括基本类型
impl Signed for i32 {
    fn is_strictly_negative(self) -> bool {
        self < 0
    }
}

// The Self type - Self表示类型
// 标准库里面的Neg也仅仅使用self - 取对应的相反数
impl std::ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            value: -self.value,
            odd: self.odd,
        }        
    }
}

fn main() {
    let n = Number { odd: false, value: -44 };
    // 我们声明了Debug，因此可以使用 {:?}，但并没有实现Display，不能直接使用 {n}
    println!("{:?}: {}", n, n.is_strictly_negative()); // prints "true"

    let n: i32 = -44;
    println!("{n}: {}", n.is_strictly_negative()); // prints "true"
}

// Marker traits - Marker特性
/*
以下是关于Rust中Marker traits的一些要点：

空实现：Marker traits通常没有任何方法或关联项。它们是空的trait定义。
编译时检查：它们用于在编译时进行类型检查和保证。
语义信息：为实现了这些traits的类型提供额外的语义信息。
常见例子：Rust标准库中的一些常见Marker traits包括：

Send：表示类型可以安全地在线程间传递所有权。
Sync：表示类型可以安全地在线程间共享引用。
Copy：表示类型可以通过简单的位复制来复制。
Sized：表示类型在编译时有已知的大小。

自动实现：某些Marker traits可以被编译器自动实现，如果类型满足特定条件。
泛型约束：它们经常用在泛型约束中，以确保类型具有某些特性。
*/

// Trait method receivers - Trait方法接收器 - 本质上就是结构的实例方法
/*
// 我们前面已经实现了
impl std::clone::Clone for Number {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}
impl std::marker::Copy for Number {}
*/

// note: `Copy` requires that `Clone` is implemented too
// Copy 方法需要 Clone - 想定义Copy，需要先定义Clone

// Deriving traits - derive也是一个特性
// Some traits are so common, they can be implemented automatically by using the derive attribute:
// 因为这些特性非常common，使用deriving可以被编译器自动实现