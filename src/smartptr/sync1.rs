use std::sync::Arc;
use std::any::Any;

//Arc使用测试
pub fn s_t1(){
    //downcast
    fn print_if_string(value: Arc<dyn Any + Send + Sync>) {
        if let Ok(string) = value.downcast::<String>() {
            println!("String ({}): {}", string.len(), string);
        }
    }
    let my_string = "Hello World".to_string();
    print_if_string(Arc::new(my_string));
    print_if_string(Arc::new(0i8));
    //Arc也可以转Weak
    let five = Arc::new(5);
    let weak_five = Arc::downgrade(&five);
    //from_raw由一个裸不变指针创建Arc
    //into_raw消耗Arc，返回内部指针*const T
    let x = Arc::new("hello".to_owned());
    let x_ptr = Arc::into_raw(x);
    unsafe {
        // Convert back to an `Arc` to prevent leak.
        let x = Arc::from_raw(x_ptr);
        assert_eq!(&*x, "hello");
        // Further calls to `Arc::from_raw(x_ptr)` would be memory-unsafe.
    }
    //get_mut返回内部值的可变引用，前提是没有其他的Arc或Weak持有这个值
    let mut x = Arc::new(3);
    *Arc::get_mut(&mut x).unwrap() = 4;
    assert_eq!(*x, 4);
    let _y = Arc::clone(&x);
    assert!(Arc::get_mut(&mut x).is_none());
    //make_mut返回Arc内部值的可变引用，如果该值被多个Arc持有，则会clone该值
    let mut data = Arc::new(5);
    *Arc::make_mut(&mut data) += 1;         // Won't clone anything
    let mut other_data = Arc::clone(&data); // Won't clone inner data
    *Arc::make_mut(&mut data) += 1;         // Clones inner data
    *Arc::make_mut(&mut data) += 1;         // Won't clone anything
    *Arc::make_mut(&mut other_data) *= 2;   // Won't clone anything
// Now `data` and `other_data` point to different allocations.
    assert_eq!(*data, 8);
    assert_eq!(*other_data, 12);
    //ptr_eq
    let five = Arc::new(5);
    let same_five = Arc::clone(&five);
    let other_five = Arc::new(5);
    assert!(Arc::ptr_eq(&five, &same_five));
    assert!(!Arc::ptr_eq(&five, &other_five));
    //strong_count
    let five = Arc::new(5);
    let _also_five = Arc::clone(&five);
    assert_eq!(2, Arc::strong_count(&five));
    //weak_count
    let five = Arc::new(5);
    let _weak_five = Arc::downgrade(&five);
    assert_eq!(1, Arc::weak_count(&five));
    //try_unwrap，如果值有多个Arc则返回传入的那个Arc
    let x = Arc::new(3);
    assert_eq!(Arc::try_unwrap(x), Ok(3));
    let x = Arc::new(4);
    let _y = Arc::clone(&x);
    assert_eq!(*Arc::try_unwrap(x).unwrap_err(), 4);
}