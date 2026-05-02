fn main() {
    let a: i32 = 10;
    let b = || -> i32 {
        a + 1
    };
    let c = |x, y| x + y;
    println!("{}", b());
    println!("{}", c(1, 2));

    let mut string_a = String::from("Hello");
    let a1 = || println!("{string_a}");
    a1();

    let mut a2 = || {
        string_a.push(' ');
        string_a.push_str("World");
        println!("{string_a}")
    };
    a2();

    let mut a3 = move || {
        if string_a == String::from("Hello World") {
            println!("string_a will be borrowed: {}", string_a);
        } else {
            string_a.push(' ');
            string_a.push_str("World");
            println!("string_a will be borrowed: {}", string_a);
        }
    };
    a3();
    // The string_a are borrowed.
    // println!("{}", string_a);

    let op: Option<i32> = Some(12);
    let op_re: i32 = op.unwrap_or_else(|| {
        println!("执行闭包");
        12
    });
    println!("{op_re}");
}
