use std::{
    any::Any,
    sync::{Arc, Barrier, Condvar, Mutex, Once, RwLock},
    thread,
    time::Duration,
};

//Arc
#[allow(dead_code)]
pub fn s1() {
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
    let _weak_five = Arc::downgrade(&five);
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
    *Arc::make_mut(&mut data) += 1; // Won't clone anything
    let mut other_data = Arc::clone(&data); // Won't clone inner data
    *Arc::make_mut(&mut data) += 1; // Clones inner data
    *Arc::make_mut(&mut data) += 1; // Won't clone anything
    *Arc::make_mut(&mut other_data) *= 2; // Won't clone anything
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

//Mutex
#[allow(dead_code)]
pub fn s2() {
    //lock
    let mutex = Arc::new(Mutex::new(0));
    let c_mutex = Arc::clone(&mutex);
    thread::spawn(move || {
        *c_mutex.lock().unwrap() = 10;
    })
    .join()
    .expect("thread::spawn failed");
    assert_eq!(*mutex.lock().unwrap(), 10);
    //try_lock
    let c_mutex = Arc::clone(&mutex);
    thread::spawn(move || {
        let mut lock = c_mutex.try_lock();
        if let Ok(ref mut mutex) = lock {
            **mutex = 10;
        } else {
            println!("try_lock failed");
        }
    })
    .join()
    .expect("thread::spawn failed");
    assert_eq!(*mutex.lock().unwrap(), 10);
    //is_poisoned，所谓的poisoned就是指持有该mutex的线程panic了
    let c_mutex = Arc::clone(&mutex);
    let _ = thread::spawn(move || {
        let _lock = c_mutex.lock().unwrap();
        panic!(); // the mutex gets poisoned
    })
    .join();
    assert_eq!(mutex.is_poisoned(), true);
    //get_mut，因为是调用borrow不需要加锁
    let mut mutex = Mutex::new(0);
    *mutex.get_mut().unwrap() = 10;
    assert_eq!(*mutex.lock().unwrap(), 10);
    //into_inner
    let mutex = Mutex::new(0);
    assert_eq!(mutex.into_inner().unwrap(), 0);
}

//Condvar
#[allow(dead_code)]
pub fn s3() {
    //wait+notify_one+notify_all
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();
    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
        // cvar.notify_all();
    });
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
        //wait会自动解锁started并阻塞当前线程，直到条件变量收到通知
    }
    //wait_timeout
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    loop {
        let result = cvar
            .wait_timeout(started, Duration::from_millis(10))
            .unwrap();
        started = result.0;
        if *started == true {
            break;
        }
    }
    //wait_timeout_while
    let pair = Arc::new((Mutex::new(true), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut pending = lock.lock().unwrap();
        *pending = false;
        cvar.notify_one();
    });
    let (lock, cvar) = &*pair;
    let result = cvar
        .wait_timeout_while(
            lock.lock().unwrap(),
            Duration::from_millis(100),
            |&mut pending| pending,
        )
        .unwrap();
    if result.1.timed_out() {
        println!("time out")
    }
    //wait_while
    let pair = Arc::new((Mutex::new(true), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut pending = lock.lock().unwrap();
        *pending = false;
        cvar.notify_one();
    });
    let (lock, cvar) = &*pair;
    let _guard = cvar
        .wait_while(lock.lock().unwrap(), |pending| *pending)
        .unwrap();
}

//Barrier
#[allow(dead_code)]
pub fn s4() {
    let mut handles = Vec::with_capacity(10);
    let barrier = Arc::new(Barrier::new(10)); //barrier只能阻塞9个线程！而不是10个
    for _ in 0..10 {
        let c = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            c.wait(); //第十个线程调用wait时，解除barrier上的所有线程阻塞！
            println!("after wait");
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

//Once一般用于FFI的一次性初始化和相关函数
#[allow(dead_code)]
pub fn s5() {
    static mut VAL: usize = 0;
    static INIT: Once = Once::new();
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

//rwlock
#[allow(dead_code)]
pub fn s6() {
    //read会阻塞调用线程直到没有其他的writer持有锁，当read返回时可能有其他的reader在锁内，该方法不保证谁先获取锁
    //read的返回值可以*到共享数据
    let lock = Arc::new(RwLock::new(1));
    let n = lock.read().unwrap();
    assert_eq!(*n, 1);
    for _ in 1..10 {
        let lock1 = lock.clone();
        thread::spawn(move || {
            let c_lock = lock1.clone();
            let r = c_lock.read().unwrap();
            assert_eq!(*r, 1);
        })
        .join()
        .unwrap();
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
    })
    .join();
    assert_eq!(lock.is_poisoned(), true);
    //into_inner消耗读写锁，返回里面的值
    let lock = RwLock::new(String::new());
    {
        let mut s = lock.write().unwrap();
        *s = "modified".to_owned();
    }
    assert_eq!(lock.into_inner().unwrap(), "modified");
    //get_mut返回内部值的可变引用，如果该读写锁被毒害了方法返回error
    let mut lock = RwLock::new(0); //lock必须要是mut的
    *lock.get_mut().unwrap() = 10;
    assert_eq!(*lock.read().unwrap(), 10);
}
