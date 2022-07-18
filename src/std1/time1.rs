use std::{
    thread::sleep,
    time::{Duration, Instant, SystemTime},
};

#[allow(dead_code)]
pub fn t1() {
    //Duration
    let five_seconds = Duration::new(5, 0);
    let five_seconds_and_five_nanos = five_seconds + Duration::new(0, 5);
    assert_eq!(five_seconds_and_five_nanos.as_secs(), 5);
    assert_eq!(five_seconds_and_five_nanos.subsec_nanos(), 5);
    println!("{:?}", five_seconds_and_five_nanos); //5.000000005
                                                   //Instant
    let now = Instant::now();
    sleep(Duration::new(2, 0));
    println!("{:?}", now.elapsed());
    println!("{}", now.elapsed().as_secs());
    sleep(Duration::new(2, 0));
    println!("{:?}", Instant::now() - now);
    //SystemTime
    let now = SystemTime::now();
    sleep(Duration::new(2, 0));
    match now.elapsed() {
        Ok(elapsed) => {
            println!("{}", elapsed.as_secs());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
