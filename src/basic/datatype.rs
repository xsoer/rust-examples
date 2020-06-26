fn main() {
    /**
     * 每一个值都属于某一个 数据类型（data type）
     * 两类数据类型子集：标量（scalar）和复合（compound）。
     */
    // cannot infer type for `_`
    let guess: u32 = "42".parse().expect("Not a number!");

    /**
     * 标量（scalar）类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。
     */
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;

    // 取余
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // 显式指定类型注解

    /**
     * char 由单引号指定，不同于字符串使用双引号
     * char 类型的大小为四个字节(four bytes)，并代表了一个 Unicode 标量值（Unicode Scalar Value），这意味着它可以比 ASCII 表示更多内容
     */
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    /**
     * 复合类型（Compound types）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。
     *
     * 元组是一个将多个其他类型的值组合进一个复合类型的主要方式。元组长度固定：一旦声明，其长度不会增大或缩小.
     * 除了使用模式匹配解构外，也可以使用点号（.）后跟值的索引来直接访问它们。
     *
     * 与元组不同，数组中的每个元素的类型必须相同。Rust 中的数组与一些其他语言中的数组不同，因为 Rust 中的数组是固定长度的：一旦声明，它们的长度不能增长或缩小。
     * 当你想要在栈（stack）而不是在堆（heap）上为数据分配空间，或者是想要确保总是有固定数量的元素时，数组非常有用。但是数组并不如 vector 类型灵活。vector 类型是标准库提供的一个 允许 增长和缩小长度的类似数组的集合类型
     */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}
