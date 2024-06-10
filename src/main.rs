use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};
mod smartponiter;
fn main() {
    println!("Hello, world!");

    // 定义了一个原始指针
    let a: i64 = 42;
    let a_ptr: *const i64 = &a as *const i64;
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };
    println!("a: {} ({:p})... 0x{:x}", a, a_ptr, a_addr + 7);

    // 宽指针用于引用动态大小类型（DST），如 trait 对象和切片，它们包含额外的元数据。
    // 通过使用宽指针，可以有效地处理 Sized 和非 Sized 类型之间的差异。
    // Box 和 Arc 都支持宽指针，因此它们能够存储动态大小类型。

    let slice: &[i32] = &[1, 2, 3]; // slice 是一个宽指针，包含指向数组起始位置的指针和数组的长度。
    let boxed_slice: Box<[i32]> = Box::new([1, 2, 3]); // boxed_slice 是一个 Box 包装的切片，它内部包含宽指针。
    let arc_slice: Arc<[i32]> = Arc::new([1, 2, 3]); // arc_slice 是一个 Arc 包装的切片，它内部包含宽指针。

    //静态分派：在编译时确定具体实现，编译器可以进行优化，适用于库中以提高性能。
    //动态分派：在运行时确定具体实现，通过 vtable 查找方法，适用于二进制中以提高编译速度和代码简洁性，但以性能为代价。

    let a: &str = "Rust";

    // 分别打印出切片长度、切片变量地址、切片对应值的地址

    // 和切片对应值的大小，它和切片中的len属性大小一致

    // println!(
    //     "size of a:{} , address of a:{:p}, value address of a:{:p} , size of data:{}",
    //     a.len(),
    //     &a,
    //     &(*a),
    //     size_of_val(&(*a))
    // );

    let shared_map: Rc<RefCell<HashMap<&str, i32>>> = Rc::new(RefCell::new(HashMap::new()));

    println!("strong count :{}", Rc::strong_count(&shared_map));

    {
        let shared_map2 = shared_map.clone();

        println!("strong count :{}", Rc::strong_count(&shared_map));

        let mut map = shared_map.borrow_mut();

        map.insert("lucky", 1);

        map.insert("regan", 2);

        map.insert("lily", 3);

        map.insert("pop", 4);
    }

    println!("strong count :{}", Rc::strong_count(&shared_map));

    let q = 5;
    let w = q;
    let e = w;

    println!("{},{},{}", q, w, e);

    // 闭包中的生命周期
    // 编译错误
    //let closure = |x:&i32|->&i32{x};
    // 使用Fn
    fn fn_closure<T, F: Fn(&T) -> &T>(f: F) -> F {
        f
    }

    let closure = fn_closure(|x: &i32| -> &i32 { x });

    println!("fn result:{}", closure(&32));
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
