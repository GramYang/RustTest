use std::any::Any;
use std::fmt::Debug;

//这是rustprimer上的反射例子
pub fn r_t1(){
    let cfp = "/etc/wayslog.conf".to_string();
    assert_eq!(load_config(&cfp), vec!["/etc/wayslog.conf".to_string()]);
    let cfps = vec!["/etc/wayslog.conf".to_string(),
                    "/etc/wayslog_sec.conf".to_string()];
    assert_eq!(load_config(&cfps),
               vec!["/etc/wayslog.conf".to_string(),
                    "/etc/wayslog_sec.conf".to_string()]);
}

fn load_config<T:Any+Debug>(value:&T)->Vec<String>{
    let mut cfgs:Vec<String>=vec![];
    let value = value as &dyn Any;
    match value.downcast_ref::<String>(){//downcast_ref，如果value是String类型就返回&String，否则返回None
        Some(cfp) => cfgs.push(cfp.clone()),
        None => (),
    };
    match value.downcast_ref::<Vec<String>>() {
        Some(v) => cfgs.extend_from_slice(&v),
        None =>(),
    }

    if cfgs.len() == 0 {
        panic!("No Config File");
    }
    cfgs
}