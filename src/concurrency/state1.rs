use std::sync::{Mutex,Arc};
use std::thread;

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