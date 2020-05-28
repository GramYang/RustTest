//move测试，基础类型不会move，结构体类型才会move
pub fn o_t1(){
    let a = 12;
    let mut b = Bag{a:10,b:"ab".to_string()};
    b.a = a;
    println!("{}",b.a);
    println!("{}",a);
    let s1 = String::from("cd");
    b.b = s1;
    println!("{}",b.b);
    // println!("{}",s1);//move了
}

struct Bag{
    a:i32,b:String,
}