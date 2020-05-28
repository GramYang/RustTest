
//#[repr(usize)]将枚举转换为usize
pub fn dt1(){
    let l1 = Level::Off;
    let l2 = Level::Trace;
    println!("{} {}",l1 as usize, l2 as usize);//0 6
}

#[repr(usize)]
enum Level{
    Off,
    Error = 2,
    Warn,
    Info,
    Debug,
    Trace,
}