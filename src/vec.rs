#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let mut num1: Vec<i32> = Vec::with_capacity(10);
        println!(
            "扩容前数据内存地址:{:p} 容量:{} 数组长度:{}",
            &(*num1),
            num1.capacity(),
            num1.len()
        );

        num1.extend([1, 3, 5, 33, 34, 99, 88, 77, 66]);

        num1.reserve(20);

        println!(
            "扩容后数据内存地址:{:p} 容量:{} 数组长度:{}",
            &(*num1),
            num1.capacity(),
            num1.len()
        );
        num1.shrink_to_fit();
        println!("释放容量后容量:{} 数组长度:{}", num1.capacity(), num1.len());

        assert!(!num1.is_empty());

        num1.insert(1, 899999);
        assert!(899999 == *num1.get(1).unwrap());
        num1.remove(1);

        let tail_num = num1.pop();
        println!("tail num:{:?}", tail_num);

        num1.retain_mut(|x| *x > 10);
        println!("删除小于等于10的数据后:{:?}", num1);

        let del_eles: Vec<i32> = num1.drain(0..2).collect();

        println!("被删除的元素:{:?}", del_eles);


        let slice: &[i32] = &num1[0..2];
        assert!(slice == &[99, 88]);

        num1.clear();
    }
}
