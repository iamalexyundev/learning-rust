fn main() {
    let mut num = String::from("hello");
    let r1 = &raw const num;
    let r2 = &raw mut num;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {:?}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {}
