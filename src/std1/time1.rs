use std::time::{Duration,Instant,SystemTime};
use std::thread::sleep;

//std::time使用
pub fn t_t1(){
    //Duration使用
    let five_seconds = Duration::new(5, 0);
    let five_seconds_and_five_nanos = five_seconds + Duration::new(0, 5);
    assert_eq!(five_seconds_and_five_nanos.as_secs(), 5);
    assert_eq!(five_seconds_and_five_nanos.subsec_nanos(), 5);
    println!("{:?}",five_seconds_and_five_nanos);//5.000000005s
    //Instant只能配合Duration使用
    let now = Instant::now();
    sleep(Duration::new(2, 0));
    println!("{:?}", now.elapsed());//2.0119975s
    println!("{}", now.elapsed().as_secs());//2
    //SystemTime，与Instant的区别是可以与外部沟通例如文件系统和其他进程
    let now = SystemTime::now();
    sleep(Duration::new(2, 0));
    match now.elapsed() {
        Ok(elapsed) => {
            println!("{}", elapsed.as_secs());//2
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}