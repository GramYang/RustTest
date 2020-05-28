use std::panic;

//catch_unwind执行一个闭包，如果闭包没有panic就返回Ok，如果闭包panic就返回Err(cause)
//该函数主要用于在其他语言如C中调用rust
pub fn p_t1(){
    let result = panic::catch_unwind(|| {
        println!("hello!");
    });
    assert!(result.is_ok());
    let result = panic::catch_unwind(|| {
        panic!("oh no!");//thread 'main' panicked at 'oh no!'
    });
    assert!(result.is_err());
}

//resume_unwind将一个错误传递到C层
//resume_unwind是会执行panic hook的，这一点和文档上面描述的有出入
pub fn p_t2(){
    let result = panic::catch_unwind(|| {
        panic!("oh no!");//thread 'main' panicked at 'oh no!'
    });
    if let Err(err) = result {
        panic::resume_unwind(err);//Passing it to the next layer?
        println!("reach here");//并没有输出
    }
}

//set_hook注册一个panic hook，会替换之前注册的所有hooker
//panic hooker会在线程panic时在panic之前调用，默认的hook会打印消息到stderr并且产生backtrace
//panic hooker是全局资源
pub fn p_t3(){
    panic::set_hook(Box::new(|_| {
        println!("Custom panic hook");//只打印这个
    }));
    panic!("Normal panic");
}

//take_hook解注册当前的panic hook并返回
//多次调用take_hook并不会报错，也不会解注册默认hook
pub fn p_t4(){
    panic::set_hook(Box::new(|_| {
        println!("Custom panic hook");
    }));
    let _ = panic::take_hook();
    panic!("Normal panic");//thread 'main' panicked at 'Normal panic'
}