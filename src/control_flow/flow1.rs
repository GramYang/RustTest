
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
    for n in 1..101{
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
    for n in 1..=100{
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
    let names=vec!["Bob","Frank","Ferris"];
    for name in names.iter(){
        match name{
            &"Ferris"=>println!("there is a rustacean among us!"),
            _=>println!("Hello {}", name),
        }
    }
    for name in names.into_iter(){
        match name{
            "Ferris"=>println!("there is a rustacean among us!"),
            _=>println!("Hello {}", name),
        }
    }
    let mut names1=vec!["Bob","Frank","Ferris"];
    for name in names1.iter_mut(){
        *name=match name{
            &mut "Ferris"=>"there is a rustacean among us!",
            _=>"hello",
        }
    }
    println!("{:?}",names1);
}