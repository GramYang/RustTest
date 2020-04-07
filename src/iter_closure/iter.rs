
//迭代器使用
pub fn iter_test1() {
    let v1=vec![1,2,3];
    let mut v1_iter=v1.iter();
    assert_eq!(v1_iter.next(),Some(&1));
    assert_eq!(v1_iter.next(),Some(&2));
    assert_eq!(v1_iter.next(),Some(&3));
    assert_eq!(v1_iter.next(),None);
    let v2:Vec<_>=v1.iter().map(|x|x+1).collect(); //v2的类型必不可少
    assert_eq!(v2,vec![2,3,4]);
    let total:i32=v1_iter.sum(); //调用sum()后，v1_iter被借用
    assert_eq!(total,0);
    // let _=v1_iter.next();//v1_iter已无效
    let shoes=vec![
        Shoe{size:10,style:String::from("sneaker")},
        Shoe{size:13,style:String::from("sandal")},
        Shoe{size:10,style:String::from("boot")},
    ];
    let in_my_size=shoes_in_my_size(shoes,10);
    assert_eq!(in_my_size,vec![Shoe{size:10,style:String::from("sneaker")},
    Shoe{size:10,style:String::from("boot")}]);
}

#[derive(PartialEq,Debug)]
struct Shoe{
    size:u32,
    style:String,
}

fn shoes_in_my_size(shoes:Vec<Shoe>,shoe_size:u32)->Vec<Shoe>{
    shoes.into_iter()
        .filter(|s|s.size==shoe_size)
        .collect()
}

//自定义迭代器
pub fn iter_test2() {
    let mut counter=Counter::new();
    assert_eq!(counter.next(),Some(1));
    assert_eq!(counter.next(),Some(2));
    assert_eq!(counter.next(),Some(3));
    assert_eq!(counter.next(),Some(4));
    assert_eq!(counter.next(),Some(5));
    assert_eq!(counter.next(),None);
    let sum:u32=Counter::new().zip(Counter::new().skip(1))
        .map(|(a,b)|a*b).filter(|x|x%3==0).sum();
    assert_eq!(18,sum);
}

struct Counter{
    count:u32,
}

impl Counter{
    fn new()->Counter{
        Counter{count:0}
    }
}

impl Iterator for Counter{
    type Item=u32;
    fn next(&mut self)->Option<Self::Item>{
        self.count+=1;
        if self.count<6{
            Some(self.count)
        }else {
            None
        }
    }
}

//遍历一下&str
pub fn iter_test3(){
    let ss= "123123456677";
    let ss_iter = Some(&ss).into_iter();
    for s in ss_iter{
        println!("{}",*s)
    }
}

//DoubleEndedIterator
pub fn iter_test4(){
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let mut iter = numbers.iter();
    assert_eq!(Some(&1), iter.next());
    assert_eq!(Some(&6), iter.next_back());
    assert_eq!(Some(&5), iter.next_back());
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&3), iter.next());
    assert_eq!(Some(&4), iter.next());
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next_back());
}