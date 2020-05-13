//综合测试，if let
pub fn p_test1(){
    struct Car;
    struct Computer;
    struct Person {
        name: String,
        car: Option<Car>,
        computer: Option<Computer>,
        age: u8,
    }
    let person = Person {
        name: String::from("John"),
        car: Some(Car),
        computer: None,
        age: 19,
    };
    if let
    Person {
        car: Some(_),//匹配Some
        age: person_age @ 13..=19, //匹配13到19
        name: ref person_name,
        ..
    } = person
    {
        println!("{} has a car and is {} years old.", person_name, person_age);
        //John has a car and is 15 years old.
    }
}

//match对比变量
pub fn p_test2(){
    enum Message {
        Quit,
        WriteString(String),
        Move { x: i32, y: i32 },
        ChangeColor(u8, u8, u8),
    }
    let message = Message::Quit;
    match message {
        Message::Quit => println!("Quit"), //Quit
        Message::WriteString(write) => println!("{}", &write),
        Message::Move{ x, y: 0 } => println!("move {} horizontally", x),
        Message::Move{ .. } => println!("other move"),
        Message::ChangeColor { 0: red, 1: green, 2: _ } => {
            println!("color change, red: {}, green: {}", red, green);
        }
    };
}

//可反驳性，如果一个都不匹配就什么也不输出
pub fn p_test3(){
    if let (a, 3) = (1, 2) {
        println!("Matched ({}, 3)", a);
    } else if let (a, 4) = (3, 4) {
        println!("Matched ({}, 4)", a); //Matched (3, 4)
    }
}

//文字，输出：
// Matched none of the arms
// It's minus one
// Matched none of the arms
// It's a one
// It's either a two or a four
// Matched none of the arms
// It's either a two or a four
pub fn p_test4(){
    for i in -2..5 {
        match i {
            -1 => println!("It's minus one"),
            1 => println!("It's a one"),
            2|4 => println!("It's either a two or a four"),
            _ => println!("Matched none of the arms"),
        }
    }
}

//分隔符
pub fn p_test5(){
    let x = 2;
    match x {
        e @ 1 ..= 5 => println!("got a range element {}", e), //got a range element 2
        _ => println!("anything"),
    }
    let a = Some(10);
    match a {
        None => (),
        Some(value) => println!("{}",value), //10
    }

    match a {
        None => (),
        Some(ref value) => println!("{}",*value), //10
    }
    struct Person {
        name: String,
        age: u8,
    }
    let value = Person{ name: String::from("John"), age: 23 };
    if let Person{name: ref person_name, age: 18..=150} = value {
        println!("ok"); //ok
    }
    let x: &Option<i32> = &Some(3);
    if let Some(y) = x {
        // y was converted to `ref y` and its type is &i32
        println!("ok");//ok
    }
}

//通配符
pub fn p_test6(){
    let x = 20;
    let (a, _) = (10, x);
    assert_eq!(a, 10);
    let real_part = |a: f64, _: f64| { a };
    struct RGBA {
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    }
    let color = RGBA{r: 0.4, g: 0.1, b: 0.9, a: 0.5};
    let RGBA{r: red, g: green, b: blue, a: _} = color;
    assert_eq!(color.r, red);
    assert_eq!(color.g, green);
    assert_eq!(color.b, blue);
    let x = Some(10);
    if let Some(_) = x {}
}

//范围
pub fn p_test7(){
    let c = 'f';
    let valid_variable = match c {
        'a'..='z' => true,
        'A'..='Z' => true,
        'α'..='ω' => true,
        _ => false,
    };
    let ph = 10;
    println!("{}", match ph {
        0..=6 => "acid",
        7 => "neutral",
        8..=14 => "base", //base
        _ => unreachable!(),
    });
    const TROPOSPHERE_MIN : u8 = 6;
    const TROPOSPHERE_MAX : u8 = 20;
    const STRATOSPHERE_MIN : u8 = TROPOSPHERE_MAX + 1;
    const STRATOSPHERE_MAX : u8 = 50;
    const MESOSPHERE_MIN : u8 = STRATOSPHERE_MAX + 1;
    const MESOSPHERE_MAX : u8 = 85;
    let altitude = 70;
    println!("{}", match altitude {
        TROPOSPHERE_MIN..=TROPOSPHERE_MAX => "troposphere",
        STRATOSPHERE_MIN..=STRATOSPHERE_MAX => "stratosphere",
        MESOSPHERE_MIN..=MESOSPHERE_MAX => "mesosphere", //mesosphere
        _ => "outer space, maybe",
    });
    pub mod binary {
        pub const MEGA : u64 = 1024*1024;
        pub const GIGA : u64 = 1024*1024*1024;
    }
    let n_items = 20_832_425;
    let bytes_per_item = 12;
    if let size @ binary::MEGA..=binary::GIGA = n_items * bytes_per_item {
        println!("It fits and occupies {} bytes", size); //It fits and occupies 249989100 bytes
    }
    trait MaxValue {
        const MAX: u64;
    }
    impl MaxValue for u8 {
        const MAX: u64 = (1 << 8) - 1;
    }
    impl MaxValue for u16 {
        const MAX: u64 = (1 << 16) - 1;
    }
    impl MaxValue for u32 {
        const MAX: u64 = (1 << 32) - 1;
    }
    println!("{}", match 0xfacade {
        0 ..= <u8 as MaxValue>::MAX => "fits in a u8",
        0 ..= <u16 as MaxValue>::MAX => "fits in a u16",
        0 ..= <u32 as MaxValue>::MAX => "fits in a u32", //fits in a u32
        _ => "too big",
    });
}

//引用
pub fn p_test8(){
    let int_reference = &3;
    let a = match *int_reference { 0 => "zero", _ => "some" };
    let b = match int_reference { &0 => "zero", _ => "some" };
    assert_eq!(a, b);
}

//组
pub fn p_test9(){
    let int_reference = &3;
    match int_reference {
        &(0..=5) => (),
        _ => (),
    }
}

//切片
pub fn p_test10(){
    let arr = [1, 2, 3];
    match arr {
        [1, _, _] => println!("starts with one"), //starts with one
        [a, b, c] => println!("starts with something else"),
    };
    let v = vec![1, 2, 3];
    match v[..] {
        [a, b] => println!("a, b"),
        [a, b, c] => println!("a, b, c"), //a, b, c
        _ => (),
    };
}