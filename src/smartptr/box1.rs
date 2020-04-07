
//Box使用
pub fn b_test1(){
    println!("{}",Box::new(5)); //5
    let a = Box::new(6);
    println!("{}", *a); //6，这里有一个重要的概念，Box<i32>等同于&i32，而*是可以把Box<i32>解引用成i32
}