use std::{any::Any, alloc::{alloc,Layout, dealloc}, ptr};


//Box
#[allow(dead_code)]
pub fn b1(){
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
    let _x = unsafe { Box::from_raw(ptr) };
    let x = Box::new(String::from("Hello"));
    let ptr = Box::into_raw(x);
    let _x = unsafe { Box::from_raw(ptr) };
    unsafe {
        let ptr = alloc(Layout::new::<i32>()) as *mut i32;
        *ptr = 5;
        let _x = Box::from_raw(ptr);
    }
    let x = Box::new(String::from("Hello"));
    let p = Box::into_raw(x);
    unsafe {//手动drop和释放内存
        ptr::drop_in_place(p);
        dealloc(p as *mut u8, Layout::new::<String>());
    }
}

//释放Box指向的堆内存
//因为Box实例在离开作用域时会被释放，这个释放过程作用于 box 本身（位于栈上）和它所指向的数据（位于堆上）。
//所以使用Box封装是一种非常简单的内存释放方式
#[allow(dead_code)]
pub fn b2(){
    let x=Box::new(100);
    let p=Box::into_raw(x);
    op1(p);
    println!("{:p}",p);//0x11dbc10
    println!("{:?}",unsafe{*p})//100，虽然这里已经打印出了100，但是该指针的值已经不在堆里面了
}

fn op1(p:*mut i32){
    unsafe{
        let _=Box::from_raw(p);
    }
}