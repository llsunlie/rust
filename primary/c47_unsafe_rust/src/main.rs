use std::slice;

fn main() {
    /* 如何从引用同时创建不可变和可变裸指针 */
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    /* 通过引用创建裸指针 */
    let address = 0x012345usize;
    let r = address as *const i32;

    /* 创建指向任意内存地址的裸指针 */
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    /* 调用不安全函数或方法 */
    unsafe {
        dangerous();
    }
    
    /* 创建不安全代码的安全抽象 */
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    /* 使用安全的 split_at_mut 函数 */
    let address = 0x01234usize;
    let r = address as *mut i32;
    let slice: &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    };

    /* 使用 extern 函数调用外部代码 */
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    /* 定义和使用一个不可变静态变量 */
    static HELLO_WORLD: &str = "Hello, world!";
    println!("name is: {}", HELLO_WORLD);

    /* 读取或修改一个可变静态变量是不安全的 */
    static mut COUNTER: u32 = 0;
    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    /* 定义并实现不安全 trait */
    unsafe trait Foo {
        // methods go here
    }
    
    unsafe impl Foo for i32 {
        // method implementations go here
    }




}

unsafe fn dangerous() {}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}