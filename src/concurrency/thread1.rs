use std::thread;
use std::time::Duration;

//线程使用，join
// 1 from main thread!
// 1 from the spawned thread!
// 2 from the spawned thread!
// 2 from main thread!
// 3 from the spawned thread!
// 3 from main thread!
// 4 from main thread!
// 4 from the spawned thread!
// 5 from the spawned thread!
// 6 from the spawned thread!
// 7 from the spawned thread!
// 8 from the spawned thread!
// 9 from the spawned thread!
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