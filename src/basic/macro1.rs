macro_rules! capture_expr_then_stringify {
    ($e:expr) => {
        stringify!($e)
    };
}

macro_rules! capture_then_match_tokens {
    ($e:expr) => {
        match_tokens!($e)
    };
}

macro_rules! match_tokens {
    ($a:tt+$b:tt) => {
        "get an addition"
    };
    (($i:ident)) => {
        "get an identifier"
    };
    ($($other:tt)*) => {
        "get something else"
    };
}

macro_rules! capture_then_what_is{
    (#[$m:meta])=>{what_is!(#[$m])};
}

macro_rules! what_is{
    (#[no_mangle])=>{"no_mangle attribute"};
    (#[inline])=>{"inline attribute"};
    ($($tts:tt)*)=>{concat!("something else (",stringify!($($tts)*), ")")};
}

macro_rules! what_is1 {
    (self) => {
        "the keyword `self`"
    }; //self是关键字，但是走这个
    ($i:ident) => {
        concat!("the identifier `", stringify!($i), "`")
    };
}

macro_rules! call_with_ident {
    ($c:ident($i:ident)) => {
        $c!($i)
    }; //这里又把self匹配成关键字了，诡异
}

macro_rules! double_method{
    ($self_:ident, $body:expr)=>{
        fn double(mut $self_)->Dummy{
            $body
        }
    };
}

struct Dummy(i32);

impl Dummy {
    double_method! {self ,{
        self.0*=2;
        self
    }}
}

//macro_rules使用细节
//从别的mod提取和使用宏倒也罢了，本mod中定义使用宏，宏定义必须在宏使用代码之前
#[allow(dead_code)]
pub fn m1() {
    println!("{:?}", stringify!(dumy(2 * 1(1 + (3))))); //"dummy ( 2 * ( 1 + ( 3 ) ) )"
    println!("{:?}", capture_expr_then_stringify!(dummy(2 * 1(1 + (3))))); //"dummy(2 * (1 + (3)))"
    println!(
        "{}\n{}\n{}\n",
        match_tokens!((caravan)),
        match_tokens!(3 + 6),
        match_tokens!(5)
    );
    // get an identifier
    // get an addition
    // get something else
    println!(
        "{}\n{}\n{}",
        capture_then_match_tokens!((caravan)),
        capture_then_match_tokens!(3 + 6),
        capture_then_match_tokens!(5)
    );
    // get something else
    // get something else
    // get something else
    println!(
        "{}\n{}\n{}\n{}",
        what_is!(#[no_mangle]),
        what_is!(#[inline]),
        capture_then_what_is!(#[no_mangle]),
        capture_then_what_is!(#[inline]),
    );
    // no_mangle attribute
    // inline attribute
    // something else (# [no_mangle])
    // something else (# [inline])
    //**************self既是关键字又是标识符******************
    println!("{}", what_is1!(self));
    println!("{}", call_with_ident!(what_is1(self)));
    // the keyword `self`
    // the keyword `self`
    println!("{:?}", Dummy(4).double().0); //8
}

macro_rules! call_with_larch {
    ($callback:ident) => {
        $callback!(larch)
    };
}

macro_rules! _expand_to_larch {
    () => {
        larch
    };
}

macro_rules! recognise_tree {
    (larch) => {
        println!("#1, 落叶松。")
    };
    (redwood) => {
        println!("#2, THE巨红杉。")
    };
    (fir) => {
        println!("#3, 冷杉。")
    };
    (chestnut) => {
        println!("#4, 七叶树。")
    };
    (pine) => {
        println!("#5, 欧洲赤松。")
    };
    ($($other:tt)*) => {
        println!("不懂，可能是种桦树？")
    };
}

macro_rules! callback {
    ($callback:ident($($args:tt)*)) => {
        $callback!($($args)*)
    };
}

//递归宏
macro_rules! mixed_rules {
    () => {};
    (trace $name:ident; $($tail:tt)*) => {
    //用$($tail:tt)*捕获分号后面的句子
        {
            println!(concat!(stringify!($name), " = {:?}"), $name);
            //stringify!很有趣，简单来说就是把宏内的一切东西都以字符串格式输出
            mixed_rules!($($tail)*); //递归调用自己
        }
    };
    (trace $name:ident = $init:expr; $($tail:tt)*) => {
        {
            let $name = $init;
            println!(concat!(stringify!($name), " = {:?}"), $name);
            mixed_rules!($($tail)*);
        }
    };
}

macro_rules! foo {
    (@as_expr $e:expr) => {$e};
    ($($tts:tt)*) => {
        foo!(@as_expr $($tts)*)
    };
}

macro_rules! init_array {
    (@accum (0, $_e:expr) -> ($($body:tt)*))
        => {init_array!(@as_expr [$($body)*])};
    (@accum (1, $e:expr) -> ($($body:tt)*))
        => {init_array!(@accum (0, $e) -> ($($body)* $e,))};
    (@accum (2, $e:expr) -> ($($body:tt)*))
        => {init_array!(@accum (1, $e) -> ($($body)* $e,))};
    (@accum (3, $e:expr) -> ($($body:tt)*))
        => {init_array!(@accum (2, $e) -> ($($body)* $e,))};
    (@as_expr $e:expr) => {$e};
    [$e:expr; $n:tt] => {
        {
            let e = $e;
            init_array!(@accum ($n, e.clone()) -> ())
        }
    };
}

//($tup_tys)虽然没用到，但是不能删，因为重复语法匹配
//Default::default()这里没有加[derive(Default)]就能运行是因为tup_tys都是基本类型
//其实可以直接用$tup_tys::default()
macro_rules! tuple_default {
    ($($tup_tys:ty),*) => {
        (
            $(
                replace_expr!(
                    ($tup_tys)
                    Default::default()
                ),
            )*
        )
    };
}

macro_rules! _replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

//处理尾部分隔符
//常见匹配方式有两种($($exprs:expr),*和$($exprs:expr,)*)；一种可处理无尾部分隔符的情况，一种可处理有的情况；但没办法同时匹配到。
//不过，在主重复的尾部放置一个$(,)*重复，则可以匹配到任意数量(包括0或1)的尾部分隔符。
macro_rules! _match_exprs {
    ($($exprs:expr),* $(,)*) => {...};
}

macro_rules! call_a_or_b_on_tail {
    ((a: $a:expr, b: $b:expr), 调a $($tail:tt)*) => {
        $a(stringify!($($tail)*))
    };

    ((a: $a:expr, b: $b:expr), 调b $($tail:tt)*) => {
        $b(stringify!($($tail)*))
    };

    ($ab:tt, $_skip:tt $($tail:tt)*) => {
        call_a_or_b_on_tail!($ab, $($tail)*)
    };
}

fn compute_len(s: &str) -> Option<usize> {
    Some(s.len())
}

fn show_tail(s: &str) -> Option<usize> {
    println!("tail: {:?}", s);
    None
}

//匹配pub struct name，甚至可以匹配pub pub pub struct name
macro_rules! struct_name {
    ($(pub)* struct $name:ident $($rest:tt)*) => {
        stringify!($name)
    };
}

macro_rules! newtype_new {
    (struct $name:ident($t:ty);) => { newtype_new! { () struct $name($t); } };
    (pub struct $name:ident($t:ty);) => { newtype_new! { (pub) struct $name($t); } };

    (($($vis:tt)*) struct $name:ident($t:ty);) => {
        as_item! {
            impl $name {
                $($vis)* fn new(value: $t) -> Self {
                    $name(value)
                }
            }
        }
    };
}

macro_rules! as_item {
    ($i:item) => {
        $i
    };
}

#[derive(Debug, Eq, PartialEq)]
struct Dummy1(i32);

//这里执行了上面定义的宏，给结构体生成了一个new方法
newtype_new! { struct Dummy1(i32);}

macro_rules! abacus {
    ((- $($moves:tt)*) -> (+ $($count:tt)*)) => {
        abacus!(($($moves)*) -> ($($count)*))
    };
    ((- $($moves:tt)*) -> ($($count:tt)*)) => {
        abacus!(($($moves)*) -> (- $($count)*))
    };
    ((+ $($moves:tt)*) -> (- $($count:tt)*)) => {
        abacus!(($($moves)*) -> ($($count)*))
    };
    ((+ $($moves:tt)*) -> ($($count:tt)*)) => {
        abacus!(($($moves)*) -> (+ $($count)*))
    };
    (() -> ()) => { true };
    (() -> ($($count:tt)+)) => { false };
}

macro_rules! _abacus1 {
    (() -> ()) => {0};
    (() -> (- $($count:tt)*)) => {
        {(-1i32) $(- replace_expr!($count 1i32))*}
    };
    (() -> (+ $($count:tt)*)) => {
        {(1i32) $(+ replace_expr!($count 1i32))*}
    };
}

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

//macro_rules使用模式
#[allow(dead_code)]
pub fn m2() {
    //****************回调********************
    recognise_tree!(expand_to_larch!()); //宏没有这么智能，不会先展开里面的宏
    call_with_larch!(recognise_tree); //只能这么写
                                      // 不懂，可能是种桦树？
                                      // #1, 落叶松。
    callback!(callback(println("Yes, this *was* unnecessary."))); //Yes, this *was* unnecessary.
                                                                  //*********************递归宏*******************
    let a = 42;
    let b = "Ho-dee-oh-di-oh-di-oh!";
    let c = (false, 2, 'c');
    mixed_rules!(
        trace a;
        trace b;
        trace c;
        trace b = "They took her where they put the crazies.";
        trace b;
    );
    // a = 42
    // b = "Ho-dee-oh-di-oh-di-oh!"
    // c = (false, 2, 'c')
    // b = "They took her where they put the crazies."
    // b = "They took her where they put the crazies."
    //*********************@as_expr导入宏***************************
    println!("{}", foo!(42)); //42
                              //***********************下推累积*******************************
    let strings: [String; 3] = init_array![String::from("hi!"); 3];
    println!("{:?}", strings); //["hi!", "hi!", "hi!"]
                               //************************重复替代******************************
    assert_eq!(tuple_default!(i32, bool, String), (0, false, String::new()));
    //************************标记树聚束****************************
    assert_eq!(
        call_a_or_b_on_tail!(
            (a: compute_len, b: show_tail),
            规则的 递归部分 将 跳过 所有这些 标记
            它们 并不 关心 我们究竟 调b 还是 调a
            只有 终结规则 关心
        ),
        None
    ); //关键在于词条：这里调b在前面所以调用的是show_tail而返回的是None
    assert_eq!(
        call_a_or_b_on_tail!(
        (a: compute_len, b: show_tail),
        而现在 为了 显式 可能的路径 有两条
        我们也 调a 一哈: 它的 输入 应该
        自我引用 因此 我们给它 一个 72),
        Some(72)
    ); //这里的词条是：调a，因此返回Some(72)
       //tail: "还是 调a 只有 终结规则 关心"
       //*************************可见性*******************************
    assert_eq!(
        struct_name!(
            pub struct Jim;
        ),
        "Jim"
    );
    assert_eq!(Dummy1::new(42), Dummy1(42));
    //*************************临时措施*******************************
    let equals_zero = abacus!((++-+-+++--++---++----+) -> ());
    assert_eq!(equals_zero, true);
}

macro_rules! as_expr {
    ($e:expr) => {
        $e
    };
}
macro_rules! as_item {
    ($i:item) => {
        $i
    };
}
macro_rules! as_pat {
    ($p:pat) => {
        $p
    };
}
macro_rules! as_stmt {
    ($s:stmt) => {
        $s
    };
}

as_item! {struct _Dummy2;}

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

macro_rules! count_tts {
    ($($tts:tt)*) => {0usize $(+ replace_expr!($tts 1usize))*};
}

macro_rules! count_tts1 {
    () => {0usize};
    ($_head:tt $($tail:tt)*) => {1usize + count_tts1!($($tail)*)};
}

//切片长度
macro_rules! count_tts2 {
    ($($tts:tt)*) => {<[()]>::len(&[$(replace_expr!($tts ())),*])};
}

//创建一个enum，里面的项默认就是u32类型，可以用as直接转
macro_rules! count_idents {
    ($($idents:ident),* $(,)*) => {
        {
            #[allow(dead_code, non_camel_case_types)]
            enum Idents { $($idents,)* __CountIdentsLast }
            const COUNT: u32 = Idents::__CountIdentsLast as u32;
            COUNT
        }
    };
}

macro_rules! parse_unitary_variants {
    (@as_expr $e:expr) => {$e};
    (@as_item $($i:item)+) => {$($i)+};
    (
        @collect_unitary_variants ($callback:ident ( $($args:tt)* )),
        ($(,)*) -> ($($var_names:ident,)*)
    ) => {
        parse_unitary_variants! {
            @as_expr
            $callback!{ $($args)* ($($var_names),*) }
        }
    };
    (
        @collect_unitary_variants ($callback:ident { $($args:tt)* }),
        ($(,)*) -> ($($var_names:ident,)*)
    ) => {
        parse_unitary_variants! {
            @as_item
            $callback!{ $($args)* ($($var_names),*) }
        }
    };
    (
        @collect_unitary_variants $fixed:tt,
        (#[$_attr:meta] $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        parse_unitary_variants! {
            @collect_unitary_variants $fixed,
            ($($tail)*) -> ($($var_names)*)
        }
    };
    (
        @collect_unitary_variants $fixed:tt,
        ($var:ident $(= $_val:expr)*, $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        parse_unitary_variants! {
            @collect_unitary_variants $fixed,
            ($($tail)*) -> ($($var_names)* $var,)
        }
    };
    (
        @collect_unitary_variants $fixed:tt,
        ($var:ident $_struct:tt, $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        const _error: () = "cannot parse unitary variants from enum with non-unitary variants";
    };
    (enum $name:ident {$($body:tt)*} => $callback:ident $arg:tt) => {
        parse_unitary_variants! {
            @collect_unitary_variants
            ($callback $arg), ($($body)*,) -> ()
        }
    };
}

//可复用宏代码
#[allow(dead_code)]
pub fn m3() {
    //***************************AST强转******************************
    as_stmt!(let as_pat!(_) = as_expr!(42));
    //***************************计数*********************************
    assert_eq!(count_tts!(0 1 2 3), 4);
    assert_eq!(count_tts1!(0 1 2 3), 4);
    assert_eq!(count_tts2!(0 1 2 3), 4);
    const COUNT: u32 = count_idents!(A, B, C);
    assert_eq!(COUNT, 3);
    //***************************枚举解析******************************
    assert_eq!(
        parse_unitary_variants!(
            enum Dummy { A, B, C }
            => stringify(variants:)
        ),
        "variants : (A, B, C)"
    );
}
