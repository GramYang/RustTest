use std::thread;
use std::time::Duration;

//线程使用，join
pub fn t_test1(){
    let handle =thread::spawn(||{
       for i in 1..10{
           println!("{} from the spawned thread!",i);
           thread::sleep(Duration::from_millis(1));
       }
    });
    for i in 1..5{
        println!("{} from main thread!",i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

//线程使用闭包
pub fn t_test2() {
    let v=vec![1,2,3];
    let handle =thread::spawn(move||{
        println!("vector: {:?}",v); //vector: [1, 2, 3]
    });
    handle.join().unwrap();
}