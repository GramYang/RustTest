//字符串的转换
pub fn str_test1() {
    //创建一个空String
    let mut s=String::new();
    s.push_str("bar");
    s.push('y');
    println!("{}",s);
    //&str切换String
    let data="initial contents";
    let _=data.to_string();
    let _="initial contents".to_string();
    //String切换&str
    let a = String::from("1234");
    let _= &a[..];
    //&&str类型
    let b = &"asd";
    //拼接
    let s1=String::from("tic");
    let s2=String::from("tac");
    let s3=String::from("toe");
    let s4=s1+"-"+&s2+"-"+&s3;
    println!("{}",s4);
    // let h=s4[0]; //错误，String不支持内部索引
}

//String使用
pub fn str_test2(){
    //指定String容量来避免多次重新分配内存
    let mut s = String::with_capacity(25);
    println!("{} {}", s.capacity(), s.len());//25
    for _ in 0..5 {
        s.push_str("hello");
        println!("{} {}", s.capacity(),s.len());//返回容量和长度
    }//输出5次25
    //from_utf8
    let sparkle_heart = vec![240, 159, 146, 150];
    println!("{}",String::from_utf8(sparkle_heart).unwrap());//💖
    //匹配from_utf8，as_bytes将字符串转换为&[u8]
    let s = String::from("hello");
    assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());
    //from_utf8_lossy，在from_utf8的基础上同时转换不确定字符
    let input = b"Hello \xF0\x90\x80World";
    println!("{}",String::from_utf8_lossy(input));//Hello �World
    //reserve(n)将String的容量提升自大于等于n，如果n<=容量-长度，那就什么都不做
    let mut s = String::new();
    s.reserve(10);
    println!("{}",s.capacity());//10
    //shrink_to_fit降低容量匹配其长度
    let mut s = String::from("foo");
    s.reserve(100);
    assert!(s.capacity() >= 100);//100了
    s.shrink_to_fit();
    assert_eq!(3, s.capacity());//变成3了
    //truncate截断String，如果指定的长度大于String本来的长度就无操作
    let mut s = String::from("hello");
    s.truncate(2);
    assert_eq!("he", s);
    //String添加
    let mut hello = String::from("Hello, ");
    hello.push('w');//推入字符
    hello.push_str("orld!");//推入字符串
    println!("{}",hello);//Hello, world!
    let mut s = String::with_capacity(3);
    s.insert(0, 'f');//指定下标存入字符，这也是一个O(n)的耗时操作
    s.insert(1, 'o');
    s.insert(2, 'o');
    assert_eq!("foo", s);
    let mut s = String::from("bar");
    s.insert_str(0, "foo");//指定下标存入字符串，这也是一个O(n)的耗时操作
    assert_eq!("foobar", s);
    //String移除
    let mut s = String::from("foo");
    assert_eq!(s.pop(), Some('o'));//移除并返回String最后一个字符
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('f'));
    assert_eq!(s.pop(), None);
    let mut s = String::from("foo");
    assert_eq!(s.remove(0), 'f');//按下标删除String中的字符并返回，该操作为O(n)，会拷贝缓存中的每个元素
    assert_eq!(s.remove(1), 'o');
    assert_eq!(s.remove(0), 'o');
    let mut s = String::from("f_o_ob_ar");
    s.retain(|c| c != '_');//保留字符串中指定数组
    assert_eq!(s, "foobar");
    //is_empty判断String是否为空
    let mut v = String::new();
    println!("{} {}",v.capacity(),v.len());//0 0
    assert!(v.is_empty());
    v.push('a');
    assert!(!v.is_empty());
    //split_off字符串拆分[0, at)、[at, len)
    let mut hello = String::from("Hello, World!");
    let world = hello.split_off(7);
    assert_eq!(hello, "Hello, ");
    assert_eq!(world, "World!");
    //clear清空该字符串
    let mut s = String::from("foo");
    s.clear();
    assert!(s.is_empty());
    assert_eq!(0, s.len());
    assert_eq!(3, s.capacity());
    //drain指定返回截取字符串返回，范围也是[0, at)、[at, len)
    let mut s = String::from("α is alpha, β is beta");
    let beta_offset = s.find('β').unwrap_or(s.len());
    let t: String = s.drain(..beta_offset).collect();
    assert_eq!(t, "α is alpha, ");
    assert_eq!(s, "β is beta");
    s.drain(..);
    assert_eq!(s, "");
    //replace_range替换字符串中范围内的内容
    let mut s = String::from("α is alpha, β is beta");
    let beta_offset = s.find('β').unwrap_or(s.len());
    s.replace_range(beta_offset.., "Α is capital alpha; ");
    println!("{}",s);//α is alpha, Α is capital alpha;
}