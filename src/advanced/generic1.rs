use std::fmt::Display;

trait Example {
    fn call(&self);
}

impl<T> Example for T
{
    default fn call(&self) {//比如这里，不加default那就是匹配所有的类型，加了default在合适的时候就会让位下面两个方法
        println!("most generic");
    }
}

impl<T> Example for T
    where T: Display
{
    default fn call(&self) {
        println!("generic for Display, {}", self);
    }
}

impl Example for str {
    fn call(&self) {
        println!("specialized for str, {}", self);
    }
}

//泛型特化测试：default只能用在泛型impl中，标记了default的方法，在匹配的时候会进行特化，如果有更符合要求的方法匹配，default方法就会让出来
pub fn g1(){
    let v1 = vec![1i32,2,3];
    let v2 = 1_i32;
    let v3 = "hello";
    v1.call();
    v2.call();
    v3.call();
    // most generic
    // generic for Display, 1
    // specialized for str, hello
}