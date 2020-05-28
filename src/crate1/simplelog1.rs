use simplelog::*;
use std::fs::File;

//simplelog使用
pub fn t1(){
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed).unwrap(),
            WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("asset/my_rust_binary.log").unwrap()),
        ]
    ).unwrap();
    error!("Bright red error");
    info!("This only appears in the log file {} {}",123,321);
    debug!("This level is currently not enabled for any logger");
}

//simplelog输出完整信息
pub fn t2(){
    let mut c = ConfigBuilder::new();
    c.set_time_format(String::from("%Y-%m-%d %H:%M:%S"));
    c.set_time_to_local(true);
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, c.build(), TerminalMode::Mixed).unwrap(),
        ]
    ).unwrap();
    error!("Bright red error");
    warn!("this is warning");
    info!("This only appears in the log file {} {}",123,321);
    debug!("This level is currently not enabled for any logger");
}