
// #[warn(unused_doc_comments)]
fn main() {
    /**
     * 变量默认是不可改变的（immutable）
     * 当变量不可变时，一旦值被绑定一个名称上，你就不能改变这个值
    */
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    // if let x = 5 will throw error: cannot assign twice to immutable variable
    // let mut x = 5 will ok.so mut show variable can change
    println!("The value of x is: {}", x);

    /**
     * 常量是绑定到一个名称的不允许改变的值，不过常量与变量还是有一些区别。
     * 首先，不允许对常量使用 mut。常量不光默认不能变，它总是不能变。
     * 声明常量使用 const 关键字而不是 let，并且 必须 注明值的类型
     * Rust 常量的命名规范是使用下划线分隔的大写字母单词，并且可以在数字字面值中插入下划线来提升可读性
    */
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_OPINTS is: {}", MAX_POINTS);

    /**
     * 我们可以定义一个与之前变量同名的新变量，而新变量会 隐藏 之前的变量
     * 隐藏与将变量标记为 mut 是有区别的.
     * 当不小心尝试对变量重新赋值时，如果没有使用 let 关键字，就会导致编译时错误
     * 当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，但复用这个名字
    */
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
