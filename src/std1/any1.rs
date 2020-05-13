use std::any::{Any, TypeId};

//Any测试
pub fn test1(){
    assert_eq!(is_string1(&0), false);
    assert_eq!(is_string1(&"cookie monster".to_string()), true);
    is_string2(&0); //Not a string...
    is_string2(&"cookie monster".to_string()); //It's a string!
    print_if_string(&0); //Not a string...
    print_if_string(&"cookie monster".to_string()); //It's a string(14): 'cookie monster'
    let mut x = 10u32;
    let mut s = "starlord".to_string();
    modify_if_u32(&mut x);
    modify_if_u32(&mut s);
    assert_eq!(x, 42);
    assert_eq!(&s, "starlord");
}

fn is_string1(s: &dyn Any) -> bool {
    TypeId::of::<String>() == s.type_id()
}

fn is_string2(s: &dyn Any) {
    if s.is::<String>() {
        println!("It's a string!");
    } else {
        println!("Not a string...");
    }
}

fn print_if_string(s: &dyn Any) {
    if let Some(string) = s.downcast_ref::<String>() {
        println!("It's a string({}): '{}'", string.len(), string);
    } else {
        println!("Not a string...");
    }
}

fn modify_if_u32(s: &mut dyn Any) {
    if let Some(num) = s.downcast_mut::<u32>() {
        *num = 42;
    }
}