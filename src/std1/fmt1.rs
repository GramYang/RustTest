//format!
#[allow(dead_code)]
pub fn f1() {
    println!("{}", format!("Hello")); // => "Hello"
    println!("{}", format!("Hello, {}!", "world")); // => "Hello, world!"
    println!("{}", format!("The number is {}", 1)); // => "The number is 1"
    println!("{}", format!("{:?}", (3, 4))); // => "(3, 4)"
    println!("{}", format!("{value}", value = 4)); // => "4"
    println!("{}", format!("{} {}", 1, 2)); // => "1 2"
    println!("{}", format!("{:04}", 42)); // => "0042" with leading zeros
    println!("{}", format!("{1} {} {0} {}", 1, 2)); //"2 1 1 2"
    println!("{}", format!("{argument}", argument = "test")); // => "test"
    println!("{}", format!("{name} {}", 1, name = 2)); // => "2 1"
    println!("{}", format!("{a} {c} {b}", a = "a", b = 'b', c = 3)); // => "a 3 b"
                                                                     // All of these print "Hello x    !"
    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);
    println!("Hello {1:0$}!", 5, "x");
    println!("Hello {:width$}!", "x", width = 5);
    //<>^分别表示居左，居右，居中
    assert_eq!(format!("Hello {:<5}!", "x"), "Hello x    !");
    assert_eq!(format!("Hello {:-<5}!", "x"), "Hello x----!");
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
    //+-表示正负，#表示内部格式：#?表示Debug格式，#x #X #b #o表示0x 0x 0b 0o，05表示一共5位数
    assert_eq!(format!("Hello {:+}!", 5), "Hello +5!");
    assert_eq!(format!("{:#x}!", 27), "0x1b!");
    assert_eq!(format!("Hello {:05}!", 5), "Hello 00005!");
    assert_eq!(format!("Hello {:05}!", -5), "Hello -0005!"); //注意这里，-也占了一位
    assert_eq!(format!("{:#010x}!", 27), "0x0000001b!");
    //下面都输出Hello x is 0.01000
    println!("Hello {0} is {1:.5}", "x", 0.01);
    println!("Hello {1} is {2:.0$}", 5, "x", 0.01);
    println!("Hello {0} is {2:.1$}", "x", 5, 0.01);
    println!("Hello {} is {:.*}", "x", 5, 0.01);
    println!("Hello {} is {2:.*}", "x", 5, 0.01);
    println!("Hello {} is {number:.prec$}", "x", prec = 5, number = 0.01);
    println!(
        "{}, `{name:.*}` has 3 fractional digits",
        "Hello",
        3,
        name = 1234.56
    );
    println!(
        "{}, `{name:.*}` has 3 characters",
        "Hello",
        3,
        name = "1234.56"
    );
    println!(
        "{}, `{name:>8.*}` has 3 right-aligned characters",
        "Hello",
        3,
        name = "1234.56"
    );
    // Hello, `1234.560` has 3 fractional digits
    // Hello, `123` has 3 characters
    // Hello, `     123` has 3 right-aligned characters
    //如何输出大括号
    assert_eq!(format!("Hello {{}}"), "Hello {}");
    assert_eq!(format!("{{ Hello"), "{ Hello");
}

//format!是否会占用所有权？不会占用所有权
#[allow(dead_code)]
pub fn f2() {
    let e = Entity {
        a: 100,
        b: String::from("110"),
        c: Some(String::from("123")),
    };
    println!("{}", format!("{0} {1} {2:?}", e.a, e.b, e.c)); //100 110 Some("123")
    println!("{:?}", e); //Entity { a: 100, b: "110", c: Some("123") }
}

#[derive(Debug)]
struct Entity {
    a: i32,
    b: String,
    c: Option<String>,
}
