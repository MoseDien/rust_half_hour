// Modules, use syntax - 模块语法
// In this example: let least = std::cmp::min(3, 8); // this is 3
// std is a crate (~ a library), - 函数库，独立的编译包
// cmp is a module (~ a source file),  - module等价一个文件
// and min is a function - 一个module里面的一个方法

/*
// this works:
use std::cmp::min;
use std::cmp::max;

// this also works:
use std::cmp::{min, max};

// this also works!
use std::{cmp::min, cmp::max};

// A wildcard (*) - 通配符
use std::cmp::*;
*/
pub fn run() {
    let least = std::cmp::min(3, 8); // this is 3
}

// Types are namespaces too - 类型也是命名空间
// 在rust里面，struct的方法本质上就是第一个参数为&self的函数
fn ns_fun() {
    let x = "amos".len(); // this is 4
    let x = str::len("amos"); // this is also 4
}

// The libstd prelude - prelude - 标准库提供的一个简单办法直接include多个modules
// use std::prelude::v1::*;