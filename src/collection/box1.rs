//Box使用
pub fn b_t1(){
    let five = Box::new(5);
    println!("{} {} {:p}",five,*five,five);//5 5 0xc4030
}