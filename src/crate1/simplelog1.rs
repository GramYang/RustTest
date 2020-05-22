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