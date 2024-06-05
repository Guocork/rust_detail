fn main() {
    println!("Hello, world!");

    // 定义了一个原始指针
    let a: i64 = 42;
    let a_ptr: *const i64 = &a as *const i64;
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };
    println!("a: {} ({:p})... 0x{:x}", a, a_ptr, a_addr + 7);
}


//  AsRef<T> trait
fn is_strong<T: AsRef<str>>(password: T) -> bool {
    password.as_ref().len() > 6
}

// Into<T> trait
fn is_stron<T: Into<String>>(password: T) -> bool {
    let password_str = password.into();
    password_str.len() > 6
}

