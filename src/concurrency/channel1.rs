use std::thread;
use std::sync::mpsc;
use std::time::Duration;

//channel使用
pub fn chan_test1(){
    let (tx,rx)=mpsc::channel();
    thread::spawn(move||{
        let val=String::from("hi");
        tx.send(val).unwrap(); //send会获取val的所有权，此时val已失效
    });
    let received=rx.recv().unwrap(); //recv阻塞线程直到从通道接收一个值
    println!("Got: {}",received); //Got: hi
}

//多p模式
pub fn chan_test2() {
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