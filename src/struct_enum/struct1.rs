struct User {
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool,
}
struct Color(i32,i32,i32); //元组结构体
struct Point(i32,i32,i32);

struct Rectangle{
    width:u32,
    height:u32,
}

impl Rectangle{
    fn area(&self)->u32{
        self.width*self.height
    }
    fn square(size:u32)->Rectangle{
        Rectangle{width:size,height:size}
    }
}

//结构体的实例化，赋值
pub fn struct_test1() {
    let mut user1=User{
        email:String::from("someone@example.com"),
        username:String::from("someusername123"),
        active:true,
        sign_in_count:1,
    };
    user1.email=String::from("anotheremail@example.com");
    let user2=User{
        email:String::from("111@example.com"),
        ..user1
    };
    println!("{}",user2.email); //111@example.com
    let black=Color(0,0,0);
    let origin=Point(0,0,0);
    println!("{} {}",black.0,origin.0); //0 0
    println!("{}",Rectangle::square(3).height)
}

//方法
pub fn struct_test2() {
    let rect1=Rectangle{width:30,height:50};
    println!("{}",rect1.area()); //注意调用方式
}