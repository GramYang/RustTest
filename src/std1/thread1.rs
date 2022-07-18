use std::{thread, time::{Duration, Instant}, cell::RefCell};


//thread
#[allow(dead_code)]
pub fn t1(){
    //park+unpark
    let parked_thread = thread::Builder::new()
        .spawn(|| {
            println!("Parking thread");
            thread::park();
            println!("Thread unparked");
        })
        .unwrap();
    thread::sleep(Duration::from_millis(10));
    println!("Unpark the thread");
    parked_thread.thread().unpark();
    parked_thread.join().unwrap();
    //park_timeout
    thread::park_timeout(Duration::from_secs(2));
    //id+current
    let other_thread = thread::spawn(|| {
        thread::current().id()
    });
    let other_thread_id = other_thread.join().unwrap();
    assert!(thread::current().id() != other_thread_id);
    //name，线程名称如果不指定的话就是None
    //Builder可以构造线程具体参数
    let builder = thread::Builder::new();
    let handler = builder.spawn(|| {
        assert!(thread::current().name().is_none());
    }).unwrap();
    handler.join().unwrap();
    let builder = thread::Builder::new()
        .name("foo".into()).stack_size(32 * 1024);
    let handler = builder.spawn(|| {
        assert_eq!(thread::current().name(), Some("foo"))
    }).unwrap();
    handler.join().unwrap();
    //sleep让当前线程睡眠至少指定时间
    let ten_millis = Duration::from_millis(10);
    let now = Instant::now();
    thread::sleep(ten_millis);
    assert!(now.elapsed() >= ten_millis);
}


//LocalKey
#[allow(dead_code)]
pub fn t2(){
    thread_local!(static FOO: RefCell<u32>  = RefCell::new(1));
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