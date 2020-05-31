use std::{thread, time};
use std::time::{Duration, Instant};

//Thread使用
pub fn t1(){
    //park和unpark
    let parked_thread = thread::Builder::new()
        .spawn(|| {
            println!("Parking thread");
            thread::park();//park的是当前线程，这里是子线程
            println!("Thread unparked");
        })
        .unwrap();
// Let some time pass for the thread to be spawned.
    thread::sleep(Duration::from_millis(10));
    println!("Unpark the thread");
    parked_thread.thread().unpark();//返回&Thread，调用其unpark
    parked_thread.join().unwrap();//join
    //id，使用thread::current().id()可以输出子线程和主线程的id
    let other_thread = thread::spawn(|| {
        thread::current().id()
    });
    let other_thread_id = other_thread.join().unwrap();
    assert!(thread::current().id() != other_thread_id);
    //name，线程名称如果不指定的话就是None
    let builder = thread::Builder::new();
    let handler = builder.spawn(|| {
        assert!(thread::current().name().is_none());
    }).unwrap();
    handler.join().unwrap();
    let builder = thread::Builder::new()
        .name("foo".into());
    let handler = builder.spawn(|| {
        assert_eq!(thread::current().name(), Some("foo"))
    }).unwrap();
    handler.join().unwrap();
}

//LocalKey使用
pub fn lk1(){
    use std::cell::RefCell;
    use std::thread;
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });
    let t = thread::spawn(move|| {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });
    t.join().unwrap();
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });
}

//Builder使用，构建Thread
pub fn b1(){
    //new,new,stack_size
    let builder = thread::Builder::new()
        .name("foo".into())//线程名称
        .stack_size(32 * 1024);//栈尺寸，单位字节，如果平台指定了更大的栈尺寸那就以平台为准
    let handler = builder.spawn(|| {
        // thread code
    }).unwrap();
    handler.join().unwrap();
    //spawn生产
    let builder = thread::Builder::new();
    let handler = builder.spawn(|| {
        // thread code
    }).unwrap();
    handler.join().unwrap();
}

//JoinHandle，专门用来join线程
pub fn jh1(){
    //join等待关联Thread结束
    let builder = thread::Builder::new();
    let join_handle: thread::JoinHandle<_> = builder.spawn(|| {
        // some work here
    }).unwrap();
    join_handle.join().expect("Couldn't join on the associated thread");
    //thread返回JoinHandle中的Thread引用
    let builder = thread::Builder::new();
    let join_handle: thread::JoinHandle<_> = builder.spawn(|| {
        // some work here
    }).unwrap();
    let thread = join_handle.thread();
    println!("thread id: {:?}", thread.id());
}

//测试thread包里的函数
pub fn func1(){
    //current返回当前的Thread实例
    let handler = thread::Builder::new()
        .name("named thread".into())
        .spawn(|| {
            let handle = thread::current();
            assert_eq!(handle.name(), Some("named thread"));
        })
        .unwrap();
    handler.join().unwrap();
    //panicking判断当前线程是否因为panic而unwinding
    struct SomeStruct;
    impl Drop for SomeStruct {
        fn drop(&mut self) {
            if thread::panicking() {
                println!("dropped while unwinding");
            } else {
                println!("dropped while not unwinding");
            }
        }
    }
    {
        print!("a: ");
        let a = SomeStruct;
    }
    {
        print!("b: ");
        let b = SomeStruct;
        panic!()
    }
    //park_timeout
    let timeout = Duration::from_secs(2);
    let beginning_park = Instant::now();
    let mut timeout_remaining = timeout;
    loop {
        thread::park_timeout(timeout_remaining);
        let elapsed = beginning_park.elapsed();
        if elapsed >= timeout {
            break;
        }
        println!("restarting park_timeout after {:?}", elapsed);
        timeout_remaining = timeout - elapsed;
    }
    //sleep让当前线程睡眠至少指定时间
    let ten_millis = time::Duration::from_millis(10);
    let now = time::Instant::now();
    thread::sleep(ten_millis);
    assert!(now.elapsed() >= ten_millis);
}