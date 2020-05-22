use log::{Level, LevelFilter, Log, Metadata, Record};
use std::sync::{Arc, Mutex};

#[cfg(feature = "std")]
use log::set_boxed_logger;

#[cfg(not(feature = "std"))]
fn set_boxed_logger(logger: Box<dyn Log>) -> Result<(), log::SetLoggerError> {
    log::set_logger(Box::leak(logger))
}

struct State {
    last_log: Mutex<Option<Level>>,
}

struct Logger(Arc<State>);

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        *self.0.last_log.lock().unwrap() = Some(record.level());
    }
    fn flush(&self) {}
}

fn test(a: &State, filter: LevelFilter) {
    log::set_max_level(filter);
    error!("error");
    last(&a, t(Level::Error, filter));
    warn!("warn");
    last(&a, t(Level::Warn, filter));
    info!("info");
    last(&a, t(Level::Info, filter));
    debug!("debug");
    last(&a, t(Level::Debug, filter));
    trace!("trace");
    last(&a, t(Level::Trace, filter));

    fn t(lvl: Level, filter: LevelFilter) -> Option<Level> {
        if lvl <= filter {
            Some(lvl)
        } else {
            None
        }
    }
}

fn last(state: &State, expected: Option<Level>) {
    let lvl = state.last_log.lock().unwrap().take();
    assert_eq!(lvl, expected);
}

//log的filter
pub fn t1(){
    let me = Arc::new(State {
        last_log: Mutex::new(None),
    });
    let a = me.clone();
    set_boxed_logger(Box::new(Logger(me))).unwrap();
    test(&a, LevelFilter::Off);
    test(&a, LevelFilter::Error);
    test(&a, LevelFilter::Warn);
    test(&a, LevelFilter::Info);
    test(&a, LevelFilter::Debug);
    test(&a, LevelFilter::Trace);
}

//宏使用
pub fn t2(){
    info!("hello");
    info!("hello",);
    info!("hello {}", "cats");
    info!("hello {}", "cats",);
    info!("hello {}", "cats",);
    match "cats" {
        cats => info!("hello {}", cats),
    };
    let cats = "cats";
    info!("hello {cats}", cats = cats);
    info!("hello {cats}", cats = cats,);
    info!("hello {cats}", cats = cats,);
}