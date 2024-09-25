/*
练习 2. 在 Linux 环境下编写一个会产生异常的应用程序，并简要解释操作系统的处理结果。
*/
fn main(){
    let mut ptr: *mut i32 = std::ptr::null_mut();
    println!("The address of ptr is {:?}", ptr as usize);
    
    unsafe{
        *ptr = 123;
    }

    unsafe{
        println!("The value at ptr is {}", *ptr);
    }
}
/*
解: 使用 let mut ptr: *mut i32 = std::ptr::null_mut();
创建了一个空指针，打印空指针的地址，可以发现默认为地址 0
Linux 环境下不允许用户/应用程序直接对地址0进行写入 (*ptr = 123;) 或者访问地址0处的内存内容 *ptr (解引用打印 ptr 部分的值)
产生结果: 运行时报错 Segmentation fault
*/