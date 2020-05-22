//match使用
pub fn p_test1(){
    //match对比变量
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
    //文字，输出：
    // Matched none of the arms
    // It's minus one
    // Matched none of the arms
    // It's a one
    // It's either a two or a four
    // Matched none of the arms
    // It's either a two or a four
    for i in -2..5 {
        match i {
            -1 => println!("It's minus one"),
            1 => println!("It's a one"),
            2|4 => println!("It's either a two or a four"),
            _ => println!("Matched none of the arms"),
        }
    }
    //分隔符
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
    //通配符
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
    //范围
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
    // println!("{}", match 0xfacade {
    //     0 ..= <u8 as MaxValue>::MAX => "fits in a u8",
    //     0 ..= <u16 as MaxValue>::MAX => "fits in a u16",
    //     0 ..= <u32 as MaxValue>::MAX => "fits in a u32", //fits in a u32
    //     _ => "too big",
    // });
    //引用
    let int_reference = &3;
    let a = match *int_reference { 0 => "zero", _ => "some" };
    let b = match int_reference { &0 => "zero", _ => "some" };
    assert_eq!(a, b);
    //组
    let int_reference = &3;
    match int_reference {
        &(0..=5) => (),
        _ => (),
    }
    //切片
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

//match是否会夺取结构体实例中域的所有权？会，只有基本类型不会。所以最好还是全部换成引用。
//顺便研究一下方法中使用match的问题。会占用所有权，因此所有值都需要copy！
pub fn p_t2(){
    let w =Wrap{a:100,b:String::from("114514"),c:Some(200),d:Some(String::from("1919"))};
    match &w.a{
        100 => {
            println!("{}",w.a);
            println!("{:?}",w);
        },
        _ => {},
    }
    match &w.b{
        String => {
            println!("{}",w.b);
            println!("{:?}",w);
        },
    }
    match &w.c{
        Some(e) => {
            println!("{}",e);
            println!("{:?}",w);
        },
        None => {},
    }
    match &w.d{
        Some(e) => {
            println!("{}",e);
            println!("{:?}",w);
        },
        None => {},
    }
    println!("{:?}",w);
    w.op1();
    w.op2();
    w.op3();
    w.op4();
}

#[derive(Debug)]
struct Wrap{
    a:i32,
    b:String,
    c:Option<i32>,
    d:Option<String>,
}

impl Wrap{
    fn op1(&self){
        let a1 = self.a.clone();
        match a1{//match会占用self，一个方法里面只能有一个match
            100 => {
                println!("{}",a1);
                println!("{:?}",self);
            },
            _ => {},
        }
    }

    fn op2(&self){
        let b1 = self.b.clone();
        match b1{
            String => {
                println!("{}",self.b);
                println!("{:?}",self);
            },
        }
    }

    fn op3(&self){
        let c1 = self.c.clone();
        match c1{
            Some(e) => {
                println!("{}",e);
                println!("{:?}",self);
            },
            None => {},
        }
    }

    fn op4(&self){
        let d1 = self.d.clone();
        match d1{
            Some(e) => {
                println!("{}",e);
                println!("{:?}",self);
            },
            None => {},
        }
    }
}

//专门测试if let用法
pub fn p_t3(){
    //解结构体
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
    //可反驳性，如果一个都不匹配就什么也不输出
    if let (a, 3) = (1, 2) {
        println!("Matched ({}, 3)", a);
    } else if let (a, 4) = (3, 4) {
        println!("Matched ({}, 4)", a); //Matched (3, 4)
    }
    //解结构体
    struct Person1 {
        name: String,
        age: u8,
    }
    let value = Person1{ name: String::from("John"), age: 23 };
    if let Person1{name: ref person_name, age: 18..=150} = value {
        println!("{}",person_name);//John
    }
    //解Option
    let x: &Option<i32> = &Some(3);
    if let Some(y) = x {
        // y was converted to `ref y` and its type is &i32
        println!("{}",*y);//3
    }
    let x = Some(10);
    if let Some(_) = x {
        println!("it's a Some");//it's a Some
    }
    //配合@
    pub mod binary {
        pub const MEGA : u64 = 1024*1024;
        pub const GIGA : u64 = 1024*1024*1024;
    }
    let n_items = 20_832_425;
    let bytes_per_item = 12;
    if let size @ binary::MEGA..=binary::GIGA = n_items * bytes_per_item {
        println!("It fits and occupies {} bytes", size); //It fits and occupies 249989100 bytes
    }
    //赋值
    let x = Some(3);
    let a = if let Some(1) = x {
        1
    } else if x == Some(2) {
        2
    } else if let Some(y) = x {
        y
    } else {
        -1
    };
    assert_eq!(a, 3);
    //解枚举
    enum E {
        X(u8),
        Y(u8),
        Z(u8),
    }
    let v = E::Y(12);
    if let E::X(n) | E::Y(n) = v {
        assert_eq!(n, 12);
    }
}