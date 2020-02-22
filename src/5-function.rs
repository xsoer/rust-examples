fn main() {
    println!("Hello, world!");

    /**
     * 表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值
     */
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    another_function();
    another_function2(23);
    another_function3(100,233);
}

/**
 * 函数定义以 fn 开始并在函数名后跟一对圆括号。大括号告诉编译器哪里是函数体的开始和结尾。
 * 拥有 参数（parameters），参数是特殊变量，是函数签名的一部分。当函数拥有参数（形参）时，可以为这些参数提供具体的值（实参）。
 * 函数不能重载
 */
fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}
fn another_function3(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}
