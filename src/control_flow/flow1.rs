

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
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}