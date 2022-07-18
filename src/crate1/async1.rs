use std::{thread, time::Duration};

use futures::executor::block_on;


#[allow(dead_code)]
pub fn a1(){
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

async fn sing_song(_song:Song){
    thread::sleep(Duration::from_secs(3)); //这是不能达成async sleep的，需要使用futures-timer
    println!("sing song");
}

async fn dance() {
    // thread::sleep(Duration::from_secs(1));
    println!("dance");
}

//为什么这里learn_song必然在sing_song之前执行呢？
// 因为sing_song等待的是learn_song的返回值，sing_song在得不到返回值时必然阻塞让位与learn_song
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