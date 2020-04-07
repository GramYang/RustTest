use futures::executor::block_on;
use std::thread;
use std::time::Duration;

//async/.await初步
//async将代码块或函数实现了Future trait。
//在async fn函数中，你可以使用.await来等待其他实现了Future特质的类型完成。
// hello, world!
// learn song
// sing song
// dance
pub fn test1(){
    let future = hello_world();
    block_on(future);
    block_on(async_main());
}



async fn hello_world() {
    println!("hello, world!");
}

struct Song;

async fn learn_song() ->Song{
    println!("learn song");
    Song{}
}

async fn sing_song(song:Song){
    // thread::sleep(Duration::from_secs(3)); //这是不能达成async sleep的，需要使用futures-timer
    println!("sing song");
}

async fn dance() {
    // thread::sleep(Duration::from_secs(1));
    println!("dance");
}

async fn learn_and_sing(){
    let song=learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1=learn_and_sing();
    let f2=dance();
    // `join!` 类似于 `.await` ，但是可以等待多个 future 并发完成
    // 如果learn_and_sing的时候有了短暂的阻塞，dance，dance
    // learn_and_sing将会返回来接管线程。如果两个futures都是阻塞的，
    // 这个‘async_main'函数就会变成阻塞状态，并让位给执行程序
    futures::join!(f1,f2);
}