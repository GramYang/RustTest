use std::any::Any;
use std::alloc::{alloc,Layout, dealloc};
use std::ptr;

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
    //from_raw和into_raw
    let x = Box::new(5);
    let ptr = Box::into_raw(x);
    let x = unsafe { Box::from_raw(ptr) };
    let x = Box::new(String::from("Hello"));
    let ptr = Box::into_raw(x);
    let x = unsafe { Box::from_raw(ptr) };
    unsafe {
        let ptr = alloc(Layout::new::<i32>()) as *mut i32;
        *ptr = 5;
        let x = Box::from_raw(ptr);
    }
    let x = Box::new(String::from("Hello"));
    let p = Box::into_raw(x);
    unsafe {//手动drop和释放内存
        ptr::drop_in_place(p);
        dealloc(p as *mut u8, Layout::new::<String>());
    }
}