
//循环
#[allow(dead_code, irrefutable_let_patterns)]
pub fn cf1() {
    //while
    let mut i = 0;
    while i < 3 {
        print!("hello ");
        i = i + 1;
    }
    let mut x = vec![1, 2, 3];
    while let Some(y) = x.pop() {
        print!("{} ", y);
    }
    while let _ = 5 {
        println!("Irrefutable patterns are always true");
        break;
    }
    let mut vals = vec![2, 3, 1, 2, 2];
    while let Some(v @ 1) | Some(v @ 2) = vals.pop() {
        print!("{} ", v); //2 2 1, 3不符合条件就break了
    }
    //for
    for n in 1..10{
        if n%15==0{
            println!("fizzbuzz");
        } else if n%3==0{
            println!("fizz");
        } else if n%5==0{
            println!("buzz");
        } else {
            println!("{}",n);
        }
    }
    for n in 1..=10{
        if n%30==0{
            println!("fizzbuzz");
        } else if n%6==0{
            println!("fizz");
        } else if n%10==0{
            println!("buzz");
        } else {
            println!("{}",n);
        }
    }
    //for + if遍历Vec<String>
    let names=vec!["Bob".to_string(),"Frank".to_string(),"Ferris".to_string()];
    for name in  names.iter(){
        if name.as_str() == "Bob"{
            println!("find Bob");
        } else{
            println!("not find Bob");
        }
    }
    for name in names.into_iter(){
        if name.as_str() == "Bob"{
            println!("find Bob");
        } else{
            println!("not find Bob");
        }
    }
    let mut names1=vec!["Bob".to_string(),"Frank".to_string(),"Ferris".to_string()];
    for name in names1.iter_mut(){
        if name.as_str() == "Bob"{
            println!("find Bob");
            name.push_str("bbb");
        } else{
            println!("not find Bob");
        }
    }
    println!("{:?}",names1);
    //遍历Vec<&str>
    let names=vec!["Bob","Frank","Ferris"];
    for name in names.iter(){
        match name{
            &"Ferris"=>println!("there is a rustacean among us!"),
            _=>println!("Hello {}", name),
        }
    }
    for name in names.into_iter(){//这里好像不要into_iter也可以
        match name{
            "Ferris"=>println!("there is a rustacean among us!"),
            _=>println!("Hello {}", name),
        }
    }
    let mut names1=vec!["Bob","Frank","Ferris"];
    for name in names1.iter_mut(){//不能和不可变引用共存
        *name=match name{
            &mut "Ferris"=>"there is a rustacean among us!",
            _=>"hello",
        }
    }
    println!("{:?}",names1);
    //遍历Vec<String>，对于String这种Box，for遍历后不应该与match搭配，因为match会move所有权，应该于if搭配
    let names=vec!["Bob".to_string(),"Frank".to_string(),"Ferris".to_string()];
    for name in  names.iter(){
        match name.clone(){//这里只能clone，因为name会被move，但是clone后的类型就不是&String了
            _s => println!("1ok"),
        }
    }
    for name in names.into_iter(){
        match name.clone(){
            _s => println!("2ok"),
        }
    }
    let mut names1=vec!["Bob".to_string(),"Frank".to_string(),"Ferris".to_string()];
    for name in names1.iter_mut(){
        match name.clone(){
            mut _s => name.push_str("bbb"),
        }
    }
    println!("{:?}",names1);
    //遍历Vec<i32>
    let numbers = vec![1,2,3,4,5];
    for v in numbers.into_iter(){//这里也是不加into_iter也可以
        match v {
            1 => println!("1"),//1
            _=>{},
        }
    }
    let mut numbers = vec![1,2,3,4,5];
    for v in numbers.iter_mut(){
        *v=match v {
            &mut 1 => 2,
            _=>0,
        }
    }
    println!("{}",numbers.get(0).unwrap());//2
    //loop
    let (mut a, mut b) = (1, 1);
    let result = loop {
        if b > 10 {
            break b;
        }
        let c = a + b;
        a = b;
        b = c;
    };
    assert_eq!(result, 13);
}

//match
#[allow(dead_code)]
pub fn cf2() {
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
        Message::Move { x, y: 0 } => println!("move {} horizontally", x),
        Message::Move { .. } => println!("other move"),
        Message::ChangeColor {
            0: red,
            1: green,
            2: _,
        } => {
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
            2 | 4 => println!("It's either a two or a four"),
            _ => println!("Matched none of the arms"),
        }
    }
    //分隔符
    let x = 2;
    match x {
        e @ 1..=5 => println!("got a range element {}", e), //got a range element 2
        _ => println!("anything"),
    }
    let a = Some(10);
    match a {
        None => (),
        Some(value) => println!("{}", value), //10
    }
    match a {
        None => (),
        Some(ref value) => println!("{}", *value), //10
    }
    //范围
    let c = 'f';
    let _valid_variable = match c {
        'a'..='z' => true,
        'A'..='Z' => true,
        'α'..='ω' => true,
        _ => false,
    };
    let ph = 10;
    println!(
        "{}",
        match ph {
            0..=6 => "acid",
            7 => "neutral",
            8..=14 => "base", //base
            _ => unreachable!(),
        }
    );
    const TROPOSPHERE_MIN: u8 = 6;
    const TROPOSPHERE_MAX: u8 = 20;
    const STRATOSPHERE_MIN: u8 = TROPOSPHERE_MAX + 1;
    const STRATOSPHERE_MAX: u8 = 50;
    const MESOSPHERE_MIN: u8 = STRATOSPHERE_MAX + 1;
    const MESOSPHERE_MAX: u8 = 85;
    let altitude = 70;
    println!(
        "{}",
        match altitude {
            TROPOSPHERE_MIN..=TROPOSPHERE_MAX => "troposphere",
            STRATOSPHERE_MIN..=STRATOSPHERE_MAX => "stratosphere",
            MESOSPHERE_MIN..=MESOSPHERE_MAX => "mesosphere", //mesosphere
            _ => "outer space, maybe",
        }
    );
    //引用
    let int_reference = &3;
    let a = match *int_reference {
        0 => "zero",
        _ => "some",
    };
    let b = match int_reference {
        &0 => "zero",
        _ => "some",
    };
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
        [_a, _b, _c] => println!("starts with something else"),
    };
    let v = vec![1, 2, 3];
    match v[..] {
        [_a, _b] => println!("a, b"),
        [_a, _b, _c] => println!("a, b, c"), //a, b, c
        _ => (),
    };
}

//if let
#[allow(dead_code)]
pub fn cf3() {
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
    if let Person {
        car: Some(_),              //匹配Some
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
    let value = Person1 {
        name: String::from("John"),
        age: 23,
    };
    if let Person1 {
        name: ref person_name,
        age: 18..=150,
    } = value
    {
        println!("{}", person_name); //John
    }
    //解Option
    let x: &Option<i32> = &Some(3);
    if let Some(y) = x {
        // y was converted to `ref y` and its type is &i32
        println!("{}", *y); //3
    }
    let x = Some(10);
    if let Some(_) = x {
        println!("it's a Some"); //it's a Some
    }
    //配合@
    pub mod binary {
        pub const MEGA: u64 = 1024 * 1024;
        pub const GIGA: u64 = 1024 * 1024 * 1024;
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