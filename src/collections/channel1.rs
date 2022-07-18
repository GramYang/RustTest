use std::{sync::mpsc, thread, time::Duration};


//异步channel
#[allow(dead_code)]
pub fn c1(){
    //1p1c
    let (tx,rx)=mpsc::channel();
    thread::spawn(move||{
        let val=String::from("hi");
        tx.send(val).unwrap(); //send会获取val的所有权，此时val已失效
    });
    let received=rx.recv().unwrap(); //recv阻塞线程直到从通道接收一个值
    println!("Got: {}",received); //Got: hi
    //多p1c
    let (tx,rx)=mpsc::channel();
    let tx1=mpsc::Sender::clone(&tx);
    thread::spawn(move ||{
       let vals = vec![String::from("hi"),String::from("from"),
                       String::from("the"),String::from("thread"),];
        for val in vals{
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move ||{
       let vals = vec![String::from("more"),String::from("messages"),
                       String::from("for"),String::from("you")];
        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx{
        println!("Got: {}",received);
    }
}

//同步channel
#[allow(dead_code)]
pub fn c2(){
    let (tx, rx) = mpsc::sync_channel::<i32>(0);
    thread::spawn(move|| {
        //这里会等待主线程开始接受
        tx.send(53).unwrap();
    });
    println!("Got: {}",rx.recv().unwrap());
}