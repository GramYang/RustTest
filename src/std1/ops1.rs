use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::{Deref, DerefMut, Index, IndexMut, RangeBounds};

#[allow(dead_code)]
pub fn o1() {
    //Bound操作范围
    assert_eq!((..100).start_bound(), Unbounded);
    assert_eq!((1..12).start_bound(), Included(&1));
    assert_eq!((1..12).end_bound(), Excluded(&12));
    for i in 0..=5{
        print!("{}",i)
    }//012345
    for i in 0..5{
        print!("{}",i)
    }//01234
    let mut map = BTreeMap::new();
    map.insert(3, "a");
    map.insert(5, "b");
    map.insert(8, "c");
    for (key, value) in map.range((Excluded(3), Included(8))) {
        println!("{}: {}", key, value);
    }
    assert_eq!(Some((&3, &"a")), map.range((Unbounded, Included(5))).next());
    //Index不可变下标寻址
    enum Nucleotide {
        A,
        C,
        G,
        T,
    }
    struct NucleotideCount {
        a: usize,
        c: usize,
        g: usize,
        t: usize,
    }
    impl Index<Nucleotide> for NucleotideCount {
        type Output = usize;

        fn index(&self, nucleotide: Nucleotide) -> &Self::Output {
            match nucleotide {
                Nucleotide::A => &self.a,
                Nucleotide::C => &self.c,
                Nucleotide::G => &self.g,
                Nucleotide::T => &self.t,
            }
        }
    }
    let nucleotide_count = NucleotideCount {
        a: 14,
        c: 9,
        g: 10,
        t: 12,
    };
    assert_eq!(nucleotide_count[Nucleotide::A], 14);
    assert_eq!(nucleotide_count[Nucleotide::C], 9);
    assert_eq!(nucleotide_count[Nucleotide::G], 10);
    assert_eq!(nucleotide_count[Nucleotide::T], 12);
    //IndexMut可变下标寻址
    #[derive(Debug)]
    enum Side {
        Left,
        Right,
    }
    #[derive(Debug, PartialEq)]
    enum Weight {
        Kilogram(f32),
        Pound(f32),
    }
    struct Balance {
        pub left: Weight,
        pub right: Weight,
    }
    impl Index<Side> for Balance {
        type Output = Weight;

        fn index(&self, index: Side) -> &Self::Output {
            println!("Accessing {:?}-side of balance immutably", index);
            match index {
                Side::Left => &self.left,
                Side::Right => &self.right,
            }
        }
    }
    impl IndexMut<Side> for Balance {
        fn index_mut(&mut self, index: Side) -> &mut Self::Output {
            println!("Accessing {:?}-side of balance mutably", index);
            match index {
                Side::Left => &mut self.left,
                Side::Right => &mut self.right,
            }
        }
    }
    let mut balance = Balance {
        right: Weight::Kilogram(2.5),
        left: Weight::Pound(1.5),
    };
    assert_eq!(balance[Side::Right], Weight::Kilogram(2.5));
    balance[Side::Left] = Weight::Kilogram(3.0);
    //Deref 不可变解引用操作
    struct DerefExample<T> {
        value: T,
    }
    impl<T> Deref for DerefExample<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    let x = DerefExample {
        value: String::from("nmsl"),
    };
    assert_eq!("nmsl".to_string(), *x);
    //DerefMut 可变解引用操作
    struct DerefMutExample<T> {
        value: T,
    }
    impl<T> Deref for DerefMutExample<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    impl<T> DerefMut for DerefMutExample<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.value
        }
    }
    let mut x = DerefMutExample { value: 'a' };
    *x = 'b';
    assert_eq!('b', *x);
}
