use std::any::Any;

//Box使用
pub fn b_t1(){
    //new
    let five = Box::new(5);
    println!("{} {} {:p}",five,*five,five);//5 5 0xc4030
    //leak
    let x = Box::new(41);
    let static_ref: &'static mut usize = Box::leak(x);
    *static_ref += 1;
    assert_eq!(*static_ref, 42);
    let x = vec![1, 2, 3].into_boxed_slice();
    let static_ref = Box::leak(x);
    static_ref[0] = 4;
    assert_eq!(*static_ref, [4, 2, 3]);
    //downcast
    fn print_if_string(value: Box<dyn Any + Send>) {
        if let Ok(string) = value.downcast::<String>() {
            println!("String ({}): {}", string.len(), string);
        }
    }
    let my_string = "Hello World".to_string();
    print_if_string(Box::new(my_string));//String (11): Hello World
    print_if_string(Box::new(0i8));
}