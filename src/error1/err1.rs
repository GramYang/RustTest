//panic!使用测试
pub fn t1(){
    panic!();
    panic!("this is a terrible mistake!");
    panic!(4); // panic with the value of 4 to be collected elsewhere
    panic!("this is a {} {message}", "fancy", message = "message");
    let a=3;
    panic!("123 {}", a);//123 3
}