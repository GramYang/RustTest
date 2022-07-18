//迭代器
#[allow(dead_code)]
pub fn i1() {
    //next
    let a = [1, 2, 3];
    let mut iter = a.iter();
    assert_eq!(Some(&1), iter.next());
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&3), iter.next());
    assert_eq!(None, iter.next());
    //all接收一个闭包后返回bool，闭包会遍历执行所有的元素，只有所有的闭包都返回true，all才会返回true
    //all只要碰到一个false就会中断返回false，所以all是一个短循环
    let a = [1, 2, 3];
    assert!(a.iter().all(|&x| x > 0));
    assert!(!a.iter().all(|&x| x > 2));
    let a = [1, 2, 3];
    let mut iter = a.iter();
    assert!(!iter.all(|&x| x != 2));
    assert_eq!(iter.next(), Some(&3));
    //any只要有一个返回true那就是true，如果全部返回false那就是false，和上面一样
    let a = [1, 2, 3];
    assert!(a.iter().any(|&x| x > 0));
    assert!(!a.iter().any(|&x| x > 5));
    let a = [1, 2, 3];
    let mut iter = a.iter();
    assert!(iter.any(|&x| x != 2));
    assert_eq!(iter.next(), Some(&2));
    //by_ref返回一个&mut Self
    let a = [1, 2, 3];
    let iter = a.iter();
    let sum: i32 = iter.take(5).fold(0, |acc, i| acc + *i);
    assert_eq!(sum, 6);
    let a = [1, 2, 3];
    let mut iter = a.iter();
    let sum: i32 = iter.by_ref().take(2).fold(0, |acc, i| acc + *i);
    assert_eq!(sum, 3);
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
    //chain链接两个迭代器，同时遍历两个集合
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];
    let mut iter = a1.iter().chain(a2.iter());
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&6));
    assert_eq!(iter.next(), None);
    let s1 = &[1, 2, 3];
    let s2 = &[4, 5, 6];
    let mut iter = s1.iter().chain(s2);
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&6));
    assert_eq!(iter.next(), None);
    //copied创建迭代器copy所有元素，只能用于可以copy的元素
    let a = [1, 2, 3];
    let v_cloned: Vec<_> = a.iter().copied().collect();
    let v_map: Vec<_> = a.iter().map(|&x| x).collect(); //其实这就是copied的原理
    assert_eq!(v_cloned, vec![1, 2, 3]);
    assert_eq!(v_map, vec![1, 2, 3]);
    //cloned创建一个迭代器clone其所有的元素，前提是其中的元素必须实现了Clone
    let a = [1, 2, 3];
    let v_cloned: Vec<_> = a.iter().cloned().collect();
    let v_map: Vec<_> = a.iter().map(|&x| x).collect(); //其实这就是cloned的原理
    assert_eq!(v_cloned, vec![1, 2, 3]);
    assert_eq!(v_map, vec![1, 2, 3]);
    // struct Bag{a:i32};//失败，Bag没有实现Clone
    // let s = [Bag{a:10},Bag{a:20},Bag{a:30}];
    // let s_c = s.iter().cloned().collect();
    // assert_eq!(s_c, vec![Bag{a:10},Bag{a:20},Bag{a:30}]);
    //cmp
    use std::cmp::Ordering;
    assert_eq!([1].iter().cmp([1].iter()), Ordering::Equal);
    assert_eq!([1].iter().cmp([1, 2].iter()), Ordering::Less);
    assert_eq!([1, 2].iter().cmp([1].iter()), Ordering::Greater);
    //collect是一个非常牛逼的方法，它获取可遍历的元素装入相关的集合
    let a = [1, 2, 3];
    let doubled: Vec<i32> = a.iter().map(|&x| x * 2).collect();
    assert_eq!(vec![2, 4, 6], doubled);
    use std::collections::VecDeque;
    let a = [1, 2, 3];
    let doubled: VecDeque<i32> = a.iter().map(|&x| x * 2).collect();
    assert_eq!(2, doubled[0]);
    assert_eq!(4, doubled[1]);
    assert_eq!(6, doubled[2]);
    let a = [1, 2, 3];
    let doubled = a.iter().map(|x| x * 2).collect::<Vec<i32>>(); //collect使用泛型来判断输出集合类型
    assert_eq!(vec![2, 4, 6], doubled);
    let a = [1, 2, 3];
    let doubled = a.iter().map(|x| x * 2).collect::<Vec<_>>();
    assert_eq!(vec![2, 4, 6], doubled);
    let chars = ['g', 'd', 'k', 'k', 'n'];
    let hello: String = chars
        .iter()
        .map(|&x| x as u8)
        .map(|x| (x + 1) as char)
        .collect(); //将char数组转换成String
    assert_eq!("hello", hello);
    let results = [Ok(1), Err("nope"), Ok(3), Err("bad")];
    let result: Result<Vec<_>, &str> = results.iter().cloned().collect();
    assert_eq!(Err("nope"), result); //collect碰到Err就会中止
    let results = [Ok(1), Ok(3)];
    let result: Result<Vec<_>, &str> = results.iter().cloned().collect();
    assert_eq!(Ok(vec![1, 3]), result);
    //count迭代器元素数量
    let a = [1, 2, 3];
    assert_eq!(a.iter().count(), 3);
    let a = [1, 2, 3, 4, 5];
    assert_eq!(a.iter().count(), 5);
    //cycle无限的循环迭代
    let a = [1, 2, 3];
    let mut it = a.iter().cycle();
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), Some(&1));
    //enumerate创建一个迭代器包含当前下标和值
    let a = ['a', 'b', 'c'];
    let mut iter = a.iter().enumerate();
    assert_eq!(iter.next(), Some((0, &'a')));
    assert_eq!(iter.next(), Some((1, &'b')));
    assert_eq!(iter.next(), Some((2, &'c')));
    assert_eq!(iter.next(), None);
    //eq判断两个迭代器的元素是否相等
    assert_eq!([1].iter().eq([1].iter()), true);
    assert_eq!([1].iter().eq([1, 2].iter()), false);
    //filter过滤掉闭包返回false的元素
    let a = [0i32, 1, 2];
    let mut iter = a.iter().filter(|x| x.is_positive());
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
    let a = [0, 1, 2];
    let mut iter = a.iter().filter(|x| **x > 1); // need two *s!
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
    let a = [0, 1, 2];
    let mut iter = a.iter().filter(|&x| *x > 1); // both & and *
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
    let a = [0, 1, 2];
    let mut iter = a.iter().filter(|&&x| x > 1); // two &s
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
    //filter_map相当于filter加拆Option
    let a = ["1", "lol", "3", "NaN", "5"];
    let mut iter = a.iter().filter_map(|s| s.parse().ok());
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);
    let a = ["1", "lol", "3", "NaN", "5"];
    let mut iter = a
        .iter()
        .map(|s| s.parse())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap()); //同时使用filter和map
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);
    //find寻找闭包返回true的元素并返回，只会返回第一个满足true的元素
    let a = [1, 2, 3];
    assert_eq!(a.iter().find(|&&x| x == 2), Some(&2));
    assert_eq!(a.iter().find(|&&x| x == 5), None);
    let a = [1, 2, 3];
    let mut iter = a.iter();
    assert_eq!(iter.find(|&&x| x == 2), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    //find_map就是find加拆Option
    let a = ["lol", "NaN", "2", "5"];
    let first_number = a.iter().find_map(|s| s.parse().ok());
    assert_eq!(first_number, Some(2));
    //flat_map
    let words = ["alpha", "beta", "gamma"];
    let merged: String = words.iter().flat_map(|s| s.chars()).collect();
    assert_eq!(merged, "alphabetagamma");
    //flatten返回迭代器合并内部结构
    let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
    let flattened = data.into_iter().flatten().collect::<Vec<u8>>();
    assert_eq!(flattened, &[1, 2, 3, 4, 5, 6]);
    let words = ["alpha", "beta", "gamma"];
    let merged: String = words.iter().map(|s| s.chars()).flatten().collect();
    assert_eq!(merged, "alphabetagamma");
    let words = ["alpha", "beta", "gamma"];
    let merged: String = words.iter().flat_map(|s| s.chars()).collect();
    assert_eq!(merged, "alphabetagamma");
    let d3 = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]];
    let d2 = d3.iter().flatten().collect::<Vec<_>>();
    assert_eq!(d2, [&[1, 2], &[3, 4], &[5, 6], &[7, 8]]);
    let d1 = d3.iter().flatten().flatten().collect::<Vec<_>>();
    assert_eq!(d1, [&1, &2, &3, &4, &5, &6, &7, &8]);
    //fold返回一个最终值
    let a = [1, 2, 3];
    let sum = a.iter().fold(0, |acc, x| acc + *x);
    assert_eq!(sum, 6);
    let numbers = [1, 2, 3, 4, 5];
    let mut result = 0;
    for i in &numbers {
        result = result + i;
    }
    let result2 = numbers.iter().fold(0, |acc, &x| acc + x);
    assert_eq!(result, result2);
    //for_each遍历所有元素执行闭包
    use std::sync::mpsc::channel;
    let (tx, rx) = channel();
    (0..5)
        .map(|x| x * 2 + 1)
        .for_each(move |x| tx.send(x).unwrap());
    let v: Vec<_> = rx.iter().collect();
    assert_eq!(v, vec![1, 3, 5, 7, 9]);
    (0..5)
        .flat_map(|x| x * 10..x * 11)
        .enumerate()
        .filter(|&(i, x)| (i + x) % 3 == 0)
        .for_each(|(i, x)| println!("{}:{}", i, x));
    //fuse创建一个迭代器，再碰到第一个None时就结束
    struct Alternate {
        state: i32,
    }
    impl Iterator for Alternate {
        type Item = i32;
        fn next(&mut self) -> Option<i32> {
            let val = self.state;
            self.state = self.state + 1;
            if val % 2 == 0 {
                Some(val)
            } else {
                None
            }
        }
    }
    let mut iter = Alternate { state: 0 };
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
    let mut iter = iter.fuse();
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    //ge大于等于传入的迭代器的元素则返回true
    assert_eq!([1].iter().ge([1].iter()), true);
    assert_eq!([1].iter().ge([1, 2].iter()), false);
    assert_eq!([1, 2].iter().ge([1].iter()), true);
    //gt大于才返回true
    assert_eq!([1].iter().gt([1].iter()), false);
    assert_eq!([1].iter().gt([1, 2].iter()), false);
    assert_eq!([1, 2].iter().gt([1].iter()), true);
    //le和ge相反
    assert_eq!([1].iter().le([1].iter()), true);
    assert_eq!([1].iter().le([1, 2].iter()), true);
    assert_eq!([1, 2].iter().le([1].iter()), false);
    //lt和gt相反
    assert_eq!([1].iter().lt([1].iter()), false);
    assert_eq!([1].iter().lt([1, 2].iter()), true);
    assert_eq!([1, 2].iter().lt([1].iter()), false);
    //inspect在每个元素被遍历时执行一个闭包，可以用来显示发生的操作
    let a = [1, 4, 2, 3];
    let sum = a
        .iter()
        .cloned()
        .filter(|x| x % 2 == 0)
        .fold(0, |sum, i| sum + i);
    println!("{}", sum);
    let sum = a
        .iter()
        .cloned()
        .inspect(|x| println!("about to filter: {}", x))
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("made it through filter: {}", x))
        .fold(0, |sum, i| sum + i);
    println!("{}", sum);
    // 6
    // about to filter: 1
    // about to filter: 4
    // made it through filter: 4
    // about to filter: 2
    // made it through filter: 2
    // about to filter: 3
    // 6
    let lines = ["1", "2", "a"];
    let sum: i32 = lines
        .iter()
        .map(|line| line.parse::<i32>())
        .inspect(|num| {
            if let Err(ref e) = *num {
                println!("Parsing error: {}", e);
            }
        })
        .filter_map(Result::ok)
        .sum();
    println!("Sum: {}", sum);
    // Parsing error: invalid digit found in string
    // Sum: 3
    //last消耗迭代器，返回最后一个元素
    let a = [1, 2, 3];
    assert_eq!(a.iter().last(), Some(&3));
    let a = [1, 2, 3, 4, 5];
    assert_eq!(a.iter().last(), Some(&5));
    //map遍历每个元素执行闭包
    let a = [1, 2, 3];
    let mut iter = a.iter().map(|x| 2 * x);
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), None);
    //max返回最大元素，如果同时有几个最大元素则返回最后一个
    let a = [1, 2, 3];
    let b: Vec<u32> = Vec::new();
    assert_eq!(a.iter().max(), Some(&3));
    assert_eq!(b.iter().max(), None);
    //max_by自定义比较函数
    let a = [-3_i32, 0, 1, 5, -10];
    assert_eq!(*a.iter().max_by(|x, y| x.cmp(y)).unwrap(), 5);
    //max_by_key把key转换一下
    let a = [-3_i32, 0, 1, 5, -10];
    assert_eq!(*a.iter().max_by_key(|x| x.abs()).unwrap(), -10);
    //min
    let a = [1, 2, 3];
    let b: Vec<u32> = Vec::new();
    assert_eq!(a.iter().min(), Some(&1));
    assert_eq!(b.iter().min(), None);
    //min_by
    let a = [-3_i32, 0, 1, 5, -10];
    assert_eq!(*a.iter().min_by(|x, y| x.cmp(y)).unwrap(), -10);
    //min_by_key
    let a = [-3_i32, 0, 1, 5, -10];
    assert_eq!(*a.iter().min_by_key(|x| x.abs()).unwrap(), 0);
    //ne判断迭代器中元素是否都不相等
    assert_eq!([1].iter().ne([1].iter()), false);
    assert_eq!([1].iter().ne([1, 2].iter()), true);
    //nth返回迭代器的第n个元素，一个元素只能调用一次，下标越界了返回None
    let a = [1, 2, 3];
    assert_eq!(a.iter().nth(1), Some(&2));
    let a = [1, 2, 3];
    let mut iter = a.iter();
    assert_eq!(iter.nth(1), Some(&2));
    assert_eq!(iter.nth(1), None);
    let a = [1, 2, 3];
    assert_eq!(a.iter().nth(10), None);
    //partial_cmp比较大小返回结果
    assert_eq!([1.].iter().partial_cmp([1.].iter()), Some(Ordering::Equal)); //这个1.是f64
    assert_eq!(
        [1.].iter().partial_cmp([1., 2.].iter()),
        Some(Ordering::Less)
    );
    assert_eq!(
        [1., 2.].iter().partial_cmp([1.].iter()),
        Some(Ordering::Greater)
    );
    assert_eq!([std::f64::NAN].iter().partial_cmp([1.].iter()), None);
    //partition消耗迭代器，返回两个集合
    let a = [1, 2, 3];
    let (even, odd): (Vec<i32>, Vec<i32>) = a.iter().partition(|&n| n % 2 == 0);
    assert_eq!(even, vec![2]);
    assert_eq!(odd, vec![1, 3]);
    //peek创建一个迭代器a的迭代器可以看a的下一个元素，但是不会消耗a
    let xs = [1, 2, 3];
    let mut iter = xs.iter().peekable();
    assert_eq!(iter.peek(), Some(&&1));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.peek(), Some(&&3));
    assert_eq!(iter.peek(), Some(&&3));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.peek(), None);
    assert_eq!(iter.next(), None);
    //position查找迭代器的一个元素并返回其下标
    let a = [1, 2, 3];
    assert_eq!(a.iter().position(|&x| x == 2), Some(1));
    assert_eq!(a.iter().position(|&x| x == 5), None);
    let a = [1, 2, 3, 4];
    let mut iter = a.iter();
    assert_eq!(iter.position(|&x| x >= 2), Some(1));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.position(|&x| x == 4), Some(0)); //因为这个时候迭代器里面只有4了
                                                     //rposition
    let a = [1, 2, 3];
    assert_eq!(a.iter().rposition(|&x| x == 3), Some(2));
    assert_eq!(a.iter().rposition(|&x| x == 5), None);
    let a = [1, 2, 3];
    let mut iter = a.iter();
    assert_eq!(iter.rposition(|&x| x == 2), Some(1));
    assert_eq!(iter.next(), Some(&1));
    //product累乘迭代器内的元素并返回值
    fn factorial(n: u32) -> u32 {
        (1..=n).product()
    }
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(5), 120);
    //rev反向迭代
    let a = [1, 2, 3];
    let mut iter = a.iter().rev();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
    //scan持有一个state，返回一个新的迭代器
    let a = [1, 2, 3];
    let mut iter = a.iter().scan(1, |state, &x| {
        *state = *state * x;
        Some(-*state)
    });
    assert_eq!(iter.next(), Some(-1));
    assert_eq!(iter.next(), Some(-2));
    assert_eq!(iter.next(), Some(-6));
    assert_eq!(iter.next(), None);
    //size_hint返回个元组，0是下限，1是上限，1是None表示没有上限或者上限比usize大
    let a = [1, 2, 3];
    let iter = a.iter();
    assert_eq!((3, Some(3)), iter.size_hint());
    let iter = (0..10).filter(|x| x % 2 == 0);
    assert_eq!((0, Some(10)), iter.size_hint());
    let iter = (0..10).filter(|x| x % 2 == 0).chain(15..20);
    assert_eq!((5, Some(15)), iter.size_hint());
    let iter = 0..;
    assert_eq!((usize::max_value(), None), iter.size_hint());
    //skip跳前n个元素，只是第一次next有效
    let a = [1, 2, 3, 4, 5];
    let mut iter = a.iter().skip(2);
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), None);
    //skip_while
    let a = [-1i32, 0, 1];
    let mut iter = a.iter().skip_while(|x| x.is_negative());
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
    let a = [-1, 0, 1];
    let mut iter = a.iter().skip_while(|x| **x < 0); // need two *s!
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
    let a = [-1, 0, 1, -2];
    let mut iter = a.iter().skip_while(|x| **x < 0);
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&-2));
    assert_eq!(iter.next(), None);
    //step_by每次都跳n个元素，和skip区分开来
    let a = [0, 1, 2, 3, 4, 5];
    let mut iter = a.iter().step_by(2);
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), None);
    //sum累加迭代器中的元素返回结果
    let a = [1, 2, 3];
    let sum: i32 = a.iter().sum();
    assert_eq!(sum, 6);
    //take创建一个迭代器获取前n个元素
    let a = [1, 2, 3];
    let mut iter = a.iter().take(2);
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
    let mut iter = (0..).take(3);
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
    //take_while不会获取所有满足条件的元素，碰到第一个不满足条件的元素时就会中断
    let a = [-1i32, 0, 1];
    let mut iter = a.iter().take_while(|x| x.is_negative());
    assert_eq!(iter.next(), Some(&-1));
    assert_eq!(iter.next(), None);
    let a = [-1, 0, 1];
    let mut iter = a.iter().take_while(|x| **x < 0); // need two *s!
    assert_eq!(iter.next(), Some(&-1));
    assert_eq!(iter.next(), None);
    let a = [-1, 0, 1, -2];
    let mut iter = a.iter().take_while(|x| **x < 0);
    assert_eq!(iter.next(), Some(&-1));
    assert_eq!(iter.next(), None);
    let a = [1, 2, 3, 4];
    let mut iter = a.iter();
    let result: Vec<i32> = iter.by_ref().take_while(|n| **n != 3).cloned().collect();
    assert_eq!(result, &[1, 2]); //迭代器过了3，所以下面只会返回4
    let result: Vec<i32> = iter.cloned().collect();
    assert_eq!(result, &[4]);
    //zip将两个迭代器合成一个键值对的新迭代器，这样你就可以同时遍历两个迭代器
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];
    let mut iter = a1.iter().zip(a2.iter());
    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &5)));
    assert_eq!(iter.next(), Some((&3, &6)));
    assert_eq!(iter.next(), None);
    let s1 = &[1, 2, 3];
    let s2 = &[4, 5, 6];
    let mut iter = s1.iter().zip(s2);
    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &5)));
    assert_eq!(iter.next(), Some((&3, &6)));
    assert_eq!(iter.next(), None);
    let enumerate: Vec<_> = "foo".chars().enumerate().collect();
    let zipper: Vec<_> = (0..).zip("foo".chars()).collect();
    assert_eq!((0, 'f'), enumerate[0]);
    assert_eq!((0, 'f'), zipper[0]);
    assert_eq!((1, 'o'), enumerate[1]);
    assert_eq!((1, 'o'), zipper[1]);
    assert_eq!((2, 'o'), enumerate[2]);
    assert_eq!((2, 'o'), zipper[2]);
    //unzip将一个键值对迭代器分解成两个容器
    let a = [(1, 2), (3, 4)];
    let (left, right): (Vec<_>, Vec<_>) = a.iter().cloned().unzip();
    assert_eq!(left, [1, 3]);
    assert_eq!(right, [2, 4]);
}

//IntoIterator定义for in和loop的行为测试
#[allow(dead_code)]
pub fn i2() {
    let mut c = MyCollection::new();
    c.add(0);
    c.add(1);
    c.add(2);
    for i in c {
        println!("{}", i);
    }
    let mut c = MyCollection::new();
    c.add(0);
    c.add(1);
    c.add(2);
    for (i, n) in c.into_iter().enumerate() {
        println!("{} {}", i as i32, n);
    }
}

#[derive(Debug)]
struct MyCollection(Vec<i32>);

impl MyCollection {
    fn new() -> MyCollection {
        MyCollection(Vec::new())
    }

    fn add(&mut self, elem: i32) {
        self.0.push(elem);
    }
}

impl IntoIterator for MyCollection {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
