
//String
#[allow(dead_code)]
pub fn s1(){
    //&str和String相互转换
    let s = String::from("foo");
    assert_eq!("foo", s.as_str());
    let s1="initial contents";
    let _s2=s1.to_string();
    let _s3="initial contents".to_string();
    let _s4=String::from(s1);
    //指定String容量来避免多次重新分配内存
    let mut s = String::with_capacity(25);
    println!("{} {}", s.capacity(), s.len());//25
    for _ in 0..5 {
        s.push_str("hello");
        println!("{} {}", s.capacity(),s.len());//返回容量和长度
    }
    //from_utf8
    let sparkle_heart = vec![240, 159, 146, 150];
    println!("{}",String::from_utf8(sparkle_heart).unwrap());
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
    let v = String::from("");
    println!("{}",v.is_empty());//""也是空字符串
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
    //drain指定返回截取字符串返回，范围也是[0, at)、[at, len)，drain会修改自己
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
    //clone测试，String和Rc很相似
    let s = String::from("114514");
    let s1 = &s.clone();
    let s2 = s.clone();
    println!("{:p}",&s);//0x11df7d0
    drop(s);
    println!("{} {} {} {:p} {:p}",*s1,s1,s2,s1,&s2);//114514 114514 114514 0x11df7f0 0x11df808
}


//&str
//测试String和&str的隐式转换？不能隐式转换，常量&str也不行
//唯一的转换：&String可以是&str，因为String实现了Deref<Target=str>，因此String继承了str的所有方法
//String继承的str方法如下
#[allow(dead_code)]
pub fn s2(){
    const AA:&'static str = "123";//这不是String
    //as_bytes
    let bytes = "bors".as_bytes();
    assert_eq!(b"bors", bytes);
    //as_bytes_mut
    let mut s = String::from("Hello");
    let bytes = unsafe { s.as_bytes_mut() };
    assert_eq!(b"Hello", bytes);
    //bytes遍历的是byte，不能遍历char
    let mut bytes = "bors".bytes();
    assert_eq!(Some(b'b'), bytes.next());
    assert_eq!(Some(b'o'), bytes.next());
    assert_eq!(Some(b'r'), bytes.next());
    assert_eq!(Some(b's'), bytes.next());
    assert_eq!(None, bytes.next());
    //char_indices下标遍历char
    let word = "goodbye";
    let count = word.char_indices().count();
    assert_eq!(7, count);
    let mut char_indices = word.char_indices();
    assert_eq!(Some((0, 'g')), char_indices.next());
    assert_eq!(Some((1, 'o')), char_indices.next());
    assert_eq!(Some((2, 'o')), char_indices.next());
    assert_eq!(Some((3, 'd')), char_indices.next());
    assert_eq!(Some((4, 'b')), char_indices.next());
    assert_eq!(Some((5, 'y')), char_indices.next());
    assert_eq!(Some((6, 'e')), char_indices.next());
    assert_eq!(None, char_indices.next());
    let yes = "y̆es";
    let mut char_indices = yes.char_indices();
    assert_eq!(Some((0, 'y')), char_indices.next()); // not (0, 'y̆')
    assert_eq!(Some((1, '\u{0306}')), char_indices.next());
// note the 3 here - the last character took up two bytes
    assert_eq!(Some((3, 'e')), char_indices.next());
    assert_eq!(Some((4, 's')), char_indices.next());
    assert_eq!(None, char_indices.next());
    //chars非下标遍历char
    let word = "goodbye";
    let count = word.chars().count();
    assert_eq!(7, count);
    let mut chars = word.chars();
    assert_eq!(Some('g'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some('d'), chars.next());
    assert_eq!(Some('b'), chars.next());
    assert_eq!(Some('y'), chars.next());
    assert_eq!(Some('e'), chars.next());
    assert_eq!(None, chars.next());
    let y = "y̆";
    let mut chars = y.chars();
    assert_eq!(Some('y'), chars.next()); // not 'y̆'
    assert_eq!(Some('\u{0306}'), chars.next());
    assert_eq!(None, chars.next());
    //contains判断是否包含此子字符串
    let bananas = "bananas";
    assert!(bananas.contains("nana"));
    assert!(!bananas.contains("apples"));
    //encode_utf16将字符串转换为[u16]
    let text = "Zażółć gęślą jaźń";
    let utf8_len = text.len();
    let utf16_len = text.encode_utf16().count();
    assert!(utf16_len <= utf8_len);
    //ends_with判断结尾的子字符串，字符也可以
    let bananas = "bananas";
    assert!(bananas.ends_with("anas"));
    assert!(!bananas.ends_with("nana"));
    assert!(bananas.ends_with('s'));
    //starts_with
    let bananas = "bananas";
    assert!(bananas.starts_with("bana"));
    assert!(!bananas.starts_with("nana"));
    //eq_ignore_ascii_case不区分大小写比较是否相等
    assert!("Ferris".eq_ignore_ascii_case("FERRIS"));
    assert!("Ferrös".eq_ignore_ascii_case("FERRöS"));
    assert!(!"Ferrös".eq_ignore_ascii_case("FERRÖS"));
    //find寻找首字符下标，注意，find的参数pat并不会占用其所有权，不过你传入引用也是可以的
    let s = "Löwe 老虎 Léopard";
    assert_eq!(s.find('L'), Some(0));
    assert_eq!(s.find('é'), Some(14));
    assert_eq!(s.find("Léopard"), Some(13));
    let s = "Löwe 老虎 Léopard";
    assert_eq!(s.find(char::is_whitespace), Some(5));
    assert_eq!(s.find(char::is_lowercase), Some(1));
    assert_eq!(s.find(|c: char| c.is_whitespace() || c.is_lowercase()), Some(1));
    assert_eq!(s.find(|c: char| (c < 'o') && (c > 'a')), Some(4));
    let s = "Löwe 老虎 Léopard";
    let x: &[_] = &['1', '2'];
    assert_eq!(s.find(x), None);
    //rfind寻找尾字符下标
    let s = "Löwe 老虎 Léopard";
    assert_eq!(s.rfind('L'), Some(13));
    assert_eq!(s.rfind('é'), Some(14));
    let s = "Löwe 老虎 Léopard";
    assert_eq!(s.rfind(char::is_whitespace), Some(12));
    assert_eq!(s.rfind(char::is_lowercase), Some(20));
    let s = "Löwe 老虎 Léopard";
    let x: &[_] = &['1', '2'];
    assert_eq!(s.rfind(x), None);
    //get返回子字符串&str，get只能获取range参数而不能指定某一个下标
    let v = String::from("🗻∈🌏");
    let _v1 = v.get(0..4);
    assert_eq!(Some("🗻"), v.get(0..4));
// indices not on UTF-8 sequence boundaries
    assert!(v.get(1..).is_none());
    assert!(v.get(..8).is_none());
// out of bounds
    assert!(v.get(..42).is_none());
    let v = String::from("田所浩二1145141919810");
    assert_eq!(v.get(0..12),Some("田所浩二"));
    assert_eq!(v.get(12..18),Some("114514"));
    assert_eq!(v.get(18..),Some("1919810"));
    //get_mut返回子字符串&mut str
    let mut v = String::from("hello");
    let _v1 = v.get_mut(0..5);
// correct length
    assert!(v.get_mut(0..5).is_some());
// out of bounds
    assert!(v.get_mut(..42).is_none());
    assert_eq!(Some("he"), v.get_mut(0..2).map(|v| &*v));
    //这里必须这么写，猜测&*是用来去掉&mut str中的mut
    assert_eq!("hello", v);
    {
        let s = v.get_mut(0..2);
        let s = s.map(|s| {
            s.make_ascii_uppercase();
            &*s
        });
        assert_eq!(Some("HE"), s);
    }
    assert_eq!("HEllo", v);
    //get_unchecked返回的是unchecked子字符串，返回值没有用Option包裹
    let v = "🗻∈🌏";
    unsafe {
        assert_eq!("🗻", v.get_unchecked(0..4));
        assert_eq!("∈", v.get_unchecked(4..7));
        assert_eq!("🌏", v.get_unchecked(7..11));
    }
    //is_ascii判断该字符串是否全用的ascii
    let ascii = "hello!\n";
    let non_ascii = "Grüße, Jürgen ❤";
    assert!(ascii.is_ascii());
    assert!(!non_ascii.is_ascii());
    //is_char_boundary判断下标是否是char的第一个或最后一个字节
    let s = "Löwe 老虎 Léopard";
    assert!(s.is_char_boundary(0));
// start of `老`
    assert!(s.is_char_boundary(6));
    assert!(s.is_char_boundary(s.len()));
// second byte of `ö`
    assert!(!s.is_char_boundary(2));
// third byte of `老`
    assert!(!s.is_char_boundary(8));
    //is_empty
    let s = "";
    assert!(s.is_empty());
    let s = "not empty";
    assert!(!s.is_empty());
    //lines返回一行的子字符串
    let text = "foo\r\nbar\n\nbaz\n";
    let mut lines = text.lines();
    assert_eq!(Some("foo"), lines.next());
    assert_eq!(Some("bar"), lines.next());
    assert_eq!(Some(""), lines.next());
    assert_eq!(Some("baz"), lines.next());
    assert_eq!(None, lines.next());
    let text = "foo\nbar\n\r\nbaz";
    let mut lines = text.lines();
    assert_eq!(Some("foo"), lines.next());
    assert_eq!(Some("bar"), lines.next());
    assert_eq!(Some(""), lines.next());
    assert_eq!(Some("baz"), lines.next());
    assert_eq!(None, lines.next());
    //make_ascii_lowercase将字符串的ascii换成小写
    let mut s = String::from("GRÜßE, JÜRGEN ❤");
    s.make_ascii_lowercase();
    assert_eq!("grÜße, jÜrgen ❤", s);
    //make_ascii_uppercase与上面相反
    let mut s = String::from("Grüße, Jürgen ❤");
    s.make_ascii_uppercase();
    assert_eq!("GRüßE, JüRGEN ❤", s);
    //match_indices收集自字符串出现的头下标
    let v: Vec<_> = "abcXXXabcYYYabc".match_indices("abc").collect();
    assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);
    let v: Vec<_> = "1abcabc2".match_indices("abc").collect();
    assert_eq!(v, [(1, "abc"), (4, "abc")]);
    let v: Vec<_> = "ababa".match_indices("aba").collect();
    assert_eq!(v, [(0, "aba")]); // only the first `aba`
    //rmatch_indices和上面相反
    let v: Vec<_> = "abcXXXabcYYYabc".rmatch_indices("abc").collect();
    assert_eq!(v, [(12, "abc"), (6, "abc"), (0, "abc")]);
    let v: Vec<_> = "1abcabc2".rmatch_indices("abc").collect();
    assert_eq!(v, [(4, "abc"), (1, "abc")]);
    let v: Vec<_> = "ababa".rmatch_indices("aba").collect();
    assert_eq!(v, [(2, "aba")]); // only the last `aba`
    //matches收集字符串中所有的指定子字符串
    let v: Vec<&str> = "abcXXXabcYYYabc".matches("abc").collect();
    assert_eq!(v, ["abc", "abc", "abc"]);
    let v: Vec<&str> = "1abc2abc3".matches(char::is_numeric).collect();
    assert_eq!(v, ["1", "2", "3"]);
    //rmatches集合的顺序相反
    let v: Vec<&str> = "abcXXXabcYYYabc".rmatches("abc").collect();
    assert_eq!(v, ["abc", "abc", "abc"]);
    let v: Vec<&str> = "1abc2abc3".rmatches(char::is_numeric).collect();
    assert_eq!(v, ["3", "2", "1"]);
    //parse字符串解析成其他类型
    let four: u32 = "4".parse().unwrap();
    assert_eq!(4, four);
    let four = "4".parse::<u32>();
    assert_eq!(Ok(4), four);
    //repeat重复字符串
    assert_eq!("abc".repeat(4), String::from("abcabcabcabc"));
    //replace创建新字符串，将引用的字符串copy进去，如果匹配不上就什么都不做
    let s = "this is old";
    assert_eq!("this is new", s.replace("old", "new"));
    let s = "this is old";
    assert_eq!(s, s.replace("cookie monster", "little lamb"));
    //replacen将前count个匹配字符串替换，如果匹配不上就什么都不做
    let s = "foo foo 123 foo";
    assert_eq!("new new 123 foo", s.replacen("foo", "new", 2));
    assert_eq!("faa fao 123 foo", s.replacen('o', "a", 3));
    assert_eq!("foo foo new23 foo", s.replacen(char::is_numeric, "new", 1));
    let s = "this is old";
    assert_eq!(s, s.replacen("cookie monster", "little lamb", 10));
    //split指定条件拆分字符串
    let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
    assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);
    let v: Vec<&str> = "".split('X').collect();
    assert_eq!(v, [""]);
    let v: Vec<&str> = "lionXXtigerXleopard".split('X').collect();
    assert_eq!(v, ["lion", "", "tiger", "leopard"]);
    let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
    assert_eq!(v, ["lion", "tiger", "leopard"]);
    let v: Vec<&str> = "abc1def2ghi".split(char::is_numeric).collect();
    assert_eq!(v, ["abc", "def", "ghi"]);
    let v: Vec<&str> = "lionXtigerXleopard".split(char::is_uppercase).collect();
    assert_eq!(v, ["lion", "tiger", "leopard"]);
    let v: Vec<&str> = "abc1defXghi".split(|c| c == '1' || c == 'X').collect();
    assert_eq!(v, ["abc", "def", "ghi"]);
    let x = "||||a||b|c".to_string();
    let d: Vec<_> = x.split('|').collect();
    assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
    let x = "(///)".to_string();
    let d: Vec<_> = x.split('/').collect();
    assert_eq!(d, &["(", "", "", ")"]);
    let d: Vec<_> = "010".split("0").collect();
    assert_eq!(d, &["", "1", ""]);
    let f: Vec<_> = "rust".split("").collect();
    assert_eq!(f, &["", "r", "u", "s", "t", ""]);
    let x = "    a  b c".to_string();
    let d: Vec<_> = x.split(' ').collect();
    assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
    //rsplit
    let v: Vec<&str> = "Mary had a little lamb".rsplit(' ').collect();
    assert_eq!(v, ["lamb", "little", "a", "had", "Mary"]);
    let v: Vec<&str> = "".rsplit('X').collect();
    assert_eq!(v, [""]);
    let v: Vec<&str> = "lionXXtigerXleopard".rsplit('X').collect();
    assert_eq!(v, ["leopard", "tiger", "", "lion"]);
    let v: Vec<&str> = "lion::tiger::leopard".rsplit("::").collect();
    assert_eq!(v, ["leopard", "tiger", "lion"]);
    let v: Vec<&str> = "abc1defXghi".rsplit(|c| c == '1' || c == 'X').collect();
    assert_eq!(v, ["ghi", "def", "abc"]);
    //split_at指定下标将字符串一分为二，不会修改自己
    let s = "Per Martin-Löf";
    let (first, last) = s.split_at(3);
    assert_eq!("Per", first);
    assert_eq!(" Martin-Löf", last);
    //split_at_mut, 分开后的字符串的修改能映射到原来的字符串
    let mut s = "Per Martin-Löf".to_string();
    {
        let (first, last) = s.split_at_mut(3);
        first.make_ascii_uppercase();
        assert_eq!("PER", first);
        assert_eq!(" Martin-Löf", last);
    }
    assert_eq!("PER Martin-Löf", s);
    //split_terminator
    let v: Vec<&str> = "A.B.".split_terminator('.').collect();
    assert_eq!(v, ["A", "B"]);
    let v: Vec<&str> = "A..B..".split_terminator(".").collect();
    assert_eq!(v, ["A", "", "B", ""]);
    let v: Vec<&str> = "A...B...".split_terminator('.').collect();
    assert_eq!(v, ["A", "", "", "B", "", ""]);
    //rsplit_terminator
    let v: Vec<&str> = "A.B.".rsplit_terminator('.').collect();
    assert_eq!(v, ["B", "A"]);
    let v: Vec<&str> = "A..B..".rsplit_terminator(".").collect();
    assert_eq!(v, ["", "B", "", "A"]);
    //split_whitespace
    let mut iter = "A few words".split_whitespace();
    assert_eq!(Some("A"), iter.next());
    assert_eq!(Some("few"), iter.next());
    assert_eq!(Some("words"), iter.next());
    assert_eq!(None, iter.next());
    let mut iter = " Mary   had\ta\u{2009}little  \n\t lamb".split_whitespace();
    assert_eq!(Some("Mary"), iter.next());
    assert_eq!(Some("had"), iter.next());
    assert_eq!(Some("a"), iter.next());
    assert_eq!(Some("little"), iter.next());
    assert_eq!(Some("lamb"), iter.next());
    assert_eq!(None, iter.next());
    //splitn在split的基础上指定拆分的子字符串数量，只能用于&str
    let v: Vec<&str> = "Mary had a little lambda".splitn(3, ' ').collect();
    assert_eq!(v, ["Mary", "had", "a little lambda"]);
    let v: Vec<&str> = "lionXXtigerXleopard".splitn(3, "X").collect();
    assert_eq!(v, ["lion", "", "tigerXleopard"]);
    let v: Vec<&str> = "abcXdef".splitn(1, 'X').collect();
    assert_eq!(v, ["abcXdef"]);
    let v: Vec<&str> = "".splitn(1, 'X').collect();
    assert_eq!(v, [""]);
    let v: Vec<&str> = "abc1defXghi".splitn(2, |c| c == '1' || c == 'X').collect();
    assert_eq!(v, ["abc", "defXghi"]);
    //rsplitn
    let v: Vec<&str> = "Mary had a little lamb".rsplitn(3, ' ').collect();
    assert_eq!(v, ["lamb", "little", "Mary had a"]);
    let v: Vec<&str> = "lionXXtigerXleopard".rsplitn(3, 'X').collect();
    assert_eq!(v, ["leopard", "tiger", "lionX"]);
    let v: Vec<&str> = "lion::tiger::leopard".rsplitn(2, "::").collect();
    assert_eq!(v, ["leopard", "lion::tiger"]);
    let v: Vec<&str> = "abc1defXghi".rsplitn(2, |c| c == '1' || c == 'X').collect();
    assert_eq!(v, ["ghi", "abc1def"]);
    //trim去除头尾的空格
    let s = " Hello\tworld\t";
    assert_eq!("Hello\tworld", s.trim());
    //trim_end只去除尾部空格
    let s = " Hello\tworld\t";
    assert_eq!(" Hello\tworld", s.trim_end());
    let s = "  English  ";
    assert!(Some('h') == s.trim_end().chars().rev().next());
    let s = "  עברית  ";
    assert!(Some('ת') == s.trim_end().chars().rev().next());
    //trim_end_matches去除尾部指定字符
    assert_eq!("11foo1bar11".trim_end_matches('1'), "11foo1bar");
    assert_eq!("123foo1bar123".trim_end_matches(char::is_numeric), "123foo1bar");
    let x: &[_] = &['1', '2'];
    assert_eq!("12foo1bar12".trim_end_matches(x), "12foo1bar");
    assert_eq!("1fooX".trim_end_matches(|c| c == '1' || c == 'X'), "1foo");
    //trim_matches去除头尾的指定字符
    assert_eq!("11foo1bar11".trim_matches('1'), "foo1bar");
    assert_eq!("123foo1bar123".trim_matches(char::is_numeric), "foo1bar");
    let x: &[_] = &['1', '2'];
    assert_eq!("12foo1bar12".trim_matches(x), "foo1bar");
    assert_eq!("1foo1barXX".trim_matches(|c| c == '1' || c == 'X'), "foo1bar");
    //trim_start
    let s = " Hello\tworld\t";
    assert_eq!("Hello\tworld\t", s.trim_start());
    let s = "  English  ";
    assert!(Some('E') == s.trim_start().chars().next());
    let s = "  עברית  ";
    assert!(Some('ע') == s.trim_start().chars().next());
    //trim_start_matches
    assert_eq!("11foo1bar11".trim_start_matches('1'), "foo1bar11");
    assert_eq!("123foo1bar123".trim_start_matches(char::is_numeric), "foo1bar123");
    let x: &[_] = &['1', '2'];
    assert_eq!("12foo1bar12".trim_start_matches(x), "foo1bar12");
}

//rust没有字符串常量池
#[allow(dead_code)]
pub fn s3(){
    let s1 = String::from("abcddef");
    let s2 = String::from("abcddef");
    println!("{} {}", s1.as_str() == s2.as_str(),s1.as_ptr() == s2.as_ptr());
}