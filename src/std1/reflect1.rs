use std::any::{Any, TypeId};
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

//是否能判断一个实例实现了某个trait？不能
pub fn r_t2(){
    let a = Ba{a:100};
    println!("{:?} {:?}",TypeId::of::<dyn Fa>(), a.type_id());
    //TypeId { t: 4983854625466413765 } TypeId { t: 10339151103279636307 }
}

trait Fa{
    fn f(&self);
}

struct Ba{
    a:i32,
}

impl Fa for Ba{
    fn f(&self){
        println!("{}",self.a);
    }
}