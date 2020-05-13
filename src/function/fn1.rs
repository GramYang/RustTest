pub fn fn_test1() {
    let a = [1, 2, 3, 4, 8, 9];
    println!("There is 7 in the array: {}", find(7, &a)); //false
    println!("There is 8 in the array: {}", find(8, &a)); //true
}

fn find(n: i32, a: &[i32]) -> bool {
    for i in a {
        if *i == n {
            return true;
        }
    }
    false
}

//空返回类型测试。函数无返回类型也是可以return的，返回的是()，return时会执行附带的函数或者宏。
pub fn fn_test2(){
    t1();
}

fn t1(){
    let x = 5;
    match x{
        0..=5 => return println!("bingo"), //bingo
        _ => println!("miss"),
    }
    println!("没有返回！");
    return panic!("用来返回!");
}