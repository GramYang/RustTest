
//生命周期注解函数
pub fn l_test1() {
    //函数
    let s1=String::from("abcd");
    let s2="xyz";
    let result1=longest(s1.as_str(),s2);
    println!("{}",result1);
    //跨作用域
    let result2;
    {
        result2=longest1(s1.as_str(),String::from("xyz").as_str());
    }
    println!("{}",result2); //abcd，result2跨作用域存活
    // let _=longest2(&s1,s2); //该函数不能编译
    //结构体
    let novel=String::from("Call me Ishmael. Some years ago...");
    let first_sentence=novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let _=ImportantExcerpt{part:first_sentence};
}

//指定x和y的生命周期一样
fn longest<'a>(x:&'a str,y:&'a str)->&'a str{
    if x.len()>y.len(){
        x
    } else{
        y
    }
}

//x和返回值同生命周期
fn longest1<'a>(x:&'a str, y:& str)->&'a str{
    x
}

//声明周期注解只能表示生命周期的关系，并不能指定生命周期。如下的函数不能编译
// fn longest2<'a>(x:&str,y:&str)->&'a str{
//     let result=String::from("asd123");
//     result.as_str()
// }

struct ImportantExcerpt<'a>{
    part:&'a str,
}

impl<'a> ImportantExcerpt<'a>{
    fn level(&self)->i32{ //生命周期注解省略了
        3
    }
    fn announce_and_return_part(&self, announcement:&str)->&str{
        println!("Attention please: {}", announcement);
        self.part
    }
}
