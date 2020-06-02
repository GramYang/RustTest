use std::sync::{Arc, Mutex, Barrier, Condvar, RwLock, Once};
use std::any::Any;
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

//Arc使用测试
pub fn arc1(){
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

//Arc例子，这两个例子都会在输出几个值后报panic at cannot access stdout during shutdownthread
//虽然这两个例子是官方文档上的，但是他们都是错误的示范
pub fn arc2(){
    let five = Arc::new(5);
    for _ in 0..10 {
        let five = Arc::clone(&five);
        thread::spawn(move || {
            println!("{:?}", five);
        });
    }
    let val = Arc::new(AtomicUsize::new(5));
    for _ in 0..10 {
        let val = Arc::clone(&val);
        thread::spawn(move || {
            let v = val.fetch_add(1, Ordering::SeqCst);
            println!("{:?}", v);
        });
    }
}

//Mutex+Arc例子1
pub fn arc3(){
    let counter=Arc::new(Mutex::new(0));
    let mut handles =vec![];
    for _ in 0..10{
        let counter1=Arc::clone(&counter);
        let handle = thread::spawn(move||{
            let mut num = counter1.lock().unwrap();
            *num+=1;
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
    println!("{}",*counter.lock().unwrap()); //10
}

//Condvar条件变量例子
pub fn cv1(){
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();
    thread::spawn(move|| {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();//通知一个被条件变量阻塞的线程解除阻塞
    });
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
        //wait会自动解锁started并阻塞当前线程，直到条件变量收到通知
    }
}

//Condvar使用
pub fn cv2(){
    //notify_all唤醒该condvar阻塞的所有线程
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();
    thread::spawn(move|| {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_all();
    });
// Wait for the thread to start up.
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
// As long as the value inside the `Mutex<bool>` is `false`, we wait.
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    //wait_timeout
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();
    thread::spawn(move|| {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    });
// wait for the thread to start up
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
// as long as the value inside the `Mutex<bool>` is `false`, we wait
    loop {
        let result = cvar.wait_timeout(started, Duration::from_millis(10)).unwrap();
        // 10 milliseconds have passed, or maybe the value changed!
        started = result.0;
        if *started == true {
            // We received the notification and the value has been updated, we can leave.
            break
        }
    }
    //wait_timeout_while
    let pair = Arc::new((Mutex::new(true), Condvar::new()));
    let pair2 = pair.clone();
    thread::spawn(move|| {
        let (lock, cvar) = &*pair2;
        let mut pending = lock.lock().unwrap();
        *pending = false;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    });
// wait for the thread to start up
    let (lock, cvar) = &*pair;
    let result = cvar.wait_timeout_while(
        lock.lock().unwrap(),
        Duration::from_millis(100),
        |&mut pending| pending,
    ).unwrap();
    if result.1.timed_out() {
        // timed-out without the condition ever evaluating to false.
    }
// access the locked mutex via result.0
    //wait_while
    let pair = Arc::new((Mutex::new(true), Condvar::new()));
    let pair2 = pair.clone();
    thread::spawn(move|| {
        let (lock, cvar) = &*pair2;
        let mut pending = lock.lock().unwrap();
        *pending = false;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    });
// Wait for the thread to start up.
    let (lock, cvar) = &*pair;
// As long as the value inside the `Mutex<bool>` is `true`, we wait.
    let _guard = cvar.wait_while(lock.lock().unwrap(), |pending| { *pending }).unwrap();
}

//Barrier的使用
//输出十次before wait再输出十次after wait
pub fn b1(){
    let mut handles = Vec::with_capacity(10);
    let barrier = Arc::new(Barrier::new(10)); //barrier只能阻塞9个线程！而不是10个
    for _ in 0..10 {
        let c = barrier.clone();
        handles.push(thread::spawn(move|| {
            println!("before wait");
            c.wait();//第十个线程调用wait时，解除barrier上的所有线程阻塞！
            println!("after wait");
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

//Once使用，一般用于FFI的一次性初始化和相关函数
pub fn o1(){
    //call_once
    static mut VAL: usize = 0;
    static INIT: Once = Once::new();
    // Accessing a `static mut` is unsafe much of the time, but if we do so
// in a synchronized fashion (e.g., write once or read all) then we're
// good to go!
//
// This function will only call `expensive_computation` once, and will
// otherwise always return the value returned from the first invocation.
//     fn get_cached_val() -> usize {
//         unsafe {
//             INIT.call_once(|| {
//                 VAL = expensive_computation();
//             });
//             VAL
//         }
//     }
    // fn expensive_computation() -> usize {
    //     // ...
    // }
    //is_completedl如果call_once执行成功则返回true；如果call_once没有调用或者没有完成，或者Once实例被毒害，则返回false
    static INIT1: Once = Once::new();
    assert_eq!(INIT1.is_completed(), false);
    INIT1.call_once(|| {
        assert_eq!(INIT1.is_completed(), false);
    });
    assert_eq!(INIT1.is_completed(), true);
    static INIT2: Once = Once::new();
    assert_eq!(INIT2.is_completed(), false);
    let handle = thread::spawn(|| {
        INIT2.call_once(|| panic!());
    });
    assert!(handle.join().is_err());
    assert_eq!(INIT2.is_completed(), false);
}

//rwlock例子，rwlock想和Arc配合来并发修改对象值，就必须用rwlock包裹住这个对象
//一个rwlock的例子见vecdeque1中的pool
pub fn rw1(){
    //例子1
    let lock = RwLock::new(5);
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    }
}

//rwlock使用
pub fn rw2(){
    //read会阻塞调用线程直到没有其他的writer持有锁，当read返回时可能有其他的reader在锁内，该方法不保证谁先获取锁
    //read的返回值可以*到共享数据
    let lock = Arc::new(RwLock::new(1));
    let n = lock.read().unwrap();
    assert_eq!(*n, 1);
    for _ in 1..10{
        let lock1 = lock.clone();
        thread::spawn(move ||{
            let c_lock = lock1.clone();
            let r = c_lock.read().unwrap();
            assert_eq!(*r,1);
        }).join().unwrap();
    }
    //try_read尝试获取读连接，该方法不会阻塞
    let lock = RwLock::new(1);
    match lock.try_read() {
        Ok(n) => assert_eq!(*n, 1),
        Err(_) => unreachable!(),
    };
    //write阻塞线程直到获取写锁，在有其他writer或reader时会阻塞
    let lock = RwLock::new(1);
    let mut n = lock.write().unwrap();
    *n = 2;
    assert!(lock.try_read().is_err());
    //try_write
    let lock = RwLock::new(1);
    let n = lock.read().unwrap();
    assert_eq!(*n, 1);
    assert!(lock.try_write().is_err());
    //is_poisoned所谓的毒害，猜测就是该持有锁的线程panic了
    let lock = Arc::new(RwLock::new(0));
    let c_lock = lock.clone();
    let _ = thread::spawn(move || {
        let _lock = c_lock.write().unwrap();
        panic!(); // the lock gets poisoned
    }).join();
    assert_eq!(lock.is_poisoned(), true);
    //into_inner消耗读写锁，返回里面的值
    let lock = RwLock::new(String::new());
    {
        let mut s = lock.write().unwrap();
        *s = "modified".to_owned();
    }
    assert_eq!(lock.into_inner().unwrap(), "modified");
    //get_mut返回内部值的可变引用，如果该读写锁被毒害了方法返回error
    let mut lock = RwLock::new(0);//lock必须要是mut的
    *lock.get_mut().unwrap() = 10;
    assert_eq!(*lock.read().unwrap(), 10);
}
