fn main() {
    let mut x = 10;
    let mut y = 20;

    let mut r: *mut i32 = &mut x;
    println!("1 x: {}", unsafe { *r });

    let rr: *mut *mut i32 = &mut r;

    unsafe {
        **rr += 1;
        println!("2 x: {}", x);
        println!("3 x: {}", *r);
        
        *rr = &mut y;
        println!("4 y: {}", *r);
    }
}
