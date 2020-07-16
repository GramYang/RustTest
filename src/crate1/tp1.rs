//本文测试threadpool crate的使用
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::sync::{Arc, Barrier};
use std::sync::atomic::{AtomicUsize, Ordering};


//使用通道同步
pub fn test1(){
    let n_workers = 4;
    let n_jobs = 8;
    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = channel();
    for _ in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move|| {
            tx.send(1).expect("channel will be there waiting for the pool");
        });
    }
    assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
}

//使用barrier同步
pub fn test2(){
    let n_workers = 42;
    let n_jobs = 23;
    let pool = ThreadPool::new(n_workers);
    let an_atomic = Arc::new(AtomicUsize::new(0));
    assert!(n_jobs <= n_workers, "too many jobs, will deadlock");
    let barrier = Arc::new(Barrier::new(n_jobs + 1));//barrier只能阻塞n_jobs个线程
    for _ in 0..n_jobs {
        let barrier = barrier.clone();
        let an_atomic = an_atomic.clone();
        pool.execute(move|| {
            an_atomic.fetch_add(1, Ordering::Relaxed);
            barrier.wait();
        });
    }
    barrier.wait(); //第n个线程调用wait，其实是去掉所有barrier阻塞。
    assert_eq!(an_atomic.load(Ordering::SeqCst), 23);
}