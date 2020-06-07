use std::alloc::{Layout, LayoutErr, dealloc, alloc, alloc_zeroed};

//std::alloc
pub fn a1(){
    //Layout结构体
    pub fn repr_c(fields: &[Layout]) -> Result<(Layout, Vec<usize>), LayoutErr> {
        let mut offsets = Vec::new();
        let mut layout = Layout::from_size_align(0, 1)?;//我是64位系统，因此1就是64
        for &field in fields {
            let (new_layout, offset) = layout.extend(field)?;
            layout = new_layout;
            offsets.push(offset);
        }
        // Remember to finalize with `pad_to_align`!
        Ok((layout.pad_to_align(), offsets))
    }
    // test that it works
    #[repr(C)] struct S { a: u64, b: u32, c: u16, d: u32 }
    let s = Layout::new::<S>();
    let u16 = Layout::new::<u16>();
    let u32 = Layout::new::<u32>();
    let u64 = Layout::new::<u64>();
    assert_eq!(repr_c(&[u64, u32, u16, u32]), Ok((s, vec![0, 8, 12, 16])));
    //alloc函数分配内存，dealloc回收内存
    unsafe {
        let layout = Layout::new::<u16>();
        let ptr = alloc(layout);
        *(ptr as *mut u16) = 42;
        assert_eq!(*(ptr as *mut u16), 42);
        dealloc(ptr, layout);
    }
    //alloc_zeroed不仅分配内存，内存还被初始化为0值
    unsafe {
        let layout = Layout::new::<u16>();
        let ptr = alloc_zeroed(layout);
        assert_eq!(*(ptr as *mut u16), 0);
        dealloc(ptr, layout);
    }
}