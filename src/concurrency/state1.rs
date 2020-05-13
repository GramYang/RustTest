use std::{thread,time};
use std::sync::{Arc, Mutex, Condvar, Barrier,RwLock};

//Mutex+Arc
pub fn stat_test1(){
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

//测试mutex的用法和golang的区别，可以看出，mutex和golang的用法一样，在golang的mux的基础上还可以含有一个值
pub fn stat_test1_1(){
    let m = Arc::new(Mutex::new(0));
    let b = Arc::new(Barrier::new(3));
    let mut handles =vec![];
    println!("start");
    for i in 0..3{
        let b1 = b.clone();
        let m1 = m.clone();
        let handle = thread::spawn(move||{
            println!("mutex {} not locked",i);
            m1.lock();
            println!("mutex {} locked",i);
            b1.wait();
            println!("mutex {} unlocked",i);
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
}

//测试LocalKey
pub fn stat_test2(){
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

//Condvar条件变量的使用
pub fn stat_test3(){
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

//Barrier的使用
//输出十次before wait再输出十次after wait
pub fn stat_test4(){
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

//rwlock使用
pub fn stat_test5(){
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
