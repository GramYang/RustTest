
//控制流loop, while, while let, for range, 三种迭代器
pub fn flow_test1() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result); //The result is 20
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        // 3!
        // 2!
        // 1!
        number = number - 1;
    }
    println!("LIFTOFF!!!");
    let mut stack=Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top)=stack.pop() {
        println!("{}",top);
        // 3
        // 2
        // 1
    }
    //遍历i32数组
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
            String => println!("1ok"),
        }
    }
    for name in names.into_iter(){
        match name.clone(){
            String => println!("2ok"),
        }
    }
    let mut names1=vec!["Bob".to_string(),"Frank".to_string(),"Ferris".to_string()];
    for name in names1.iter_mut(){
        match name.clone(){
            mut String => name.push_str("bbb"),
        }
    }
    println!("{:?}",names1);
    //遍历Vec<i32>
    let mut numbers = vec![1,2,3,4,5];
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
}

//测试一下控制流操作符对所有权的占据
pub fn f_t2(){
    let mut w = Wrap::new(1, String::from("a"), 2, String::from("b"),
                          3, String::from("c"));
    println!("{}",w.op1());//20
    println!("{}",w.op2());//true
}

struct Wrap{
    a:i32,
    b:String,
    c:Vec<i32>,
    d:Vec<String>,
    e:Option<i32>,
    f:Option<String>,
}

impl Wrap{
    fn new(x1:i32,y1:String,x2:i32,y2:String,x3:i32,y3:String)-> Self{
        Wrap{
            a:x1,b:y1,c:vec![x2],d:vec![y2],e:Some(x3),f:Some(y3),
        }
    }

    //loop
    fn op1(&mut self) -> i32{
        let s = loop{
            self.a +=1;
            if self.a == 10{
                break self.a*2
            }
        };
        s
    }

    //if
    fn op2(&mut self) -> bool{
        if self.b.as_str() == "a"{//字符串的比较一定要是&str，String不能比较
            return true;
        }
        return false;
    }
}

//continue使用
// rust没有go里面的label，你需要用match枚举完所有的条件
pub fn f_t3(){
    for i in 0..=5{
        for j in 0..=5{
            match j {
                // e @ 4..=5 => continue,
                4|5 => continue,
                _ => {},
            }
            println!("i: {}, j: {}",i,j);
        }
    }
}