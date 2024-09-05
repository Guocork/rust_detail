#[cfg(test)]
mod test {
    // 补码 与 016b 表示
    #[test]
    fn test() {
        let a: u16 = 50115;   // 正常二进制表示
        let b: i16 = -15421;  // 补码来表示  正数取反 +1 

        println!("a: {:016b} {}",a, a); // 格式化字符串语法 0表示不足宽度时 用0补齐 16表示输出宽度是16进制 b表示 二进制 格式输出
        println!("a: {:016b} {}",b, b); 
    }

    // unsafe不能绕过借用检查 只是不能保证内存安全
    #[test]
    fn test1() {
        let a: f32 = 42.42;
        let frankentype: u32 = unsafe {
            std::mem::transmute(a)
        };

        println!("{}", frankentype);
        println!("{:032b}", frankentype);

        let b: f32 = unsafe {
            std::mem::transmute(frankentype)
        };

        println!("{}", b);
        assert_eq!(a, b);
    }

    // 整数溢出
    #[test]
    fn test2() {
        let mut i: u16 = 0;
        print!("{}..", i);

        loop {
            i += 1000;
            print!("{}..", i);
            if i % 10000 == 0 {
                print!("\n")
            }
        }
    }

    // 二进制字面量
    #[test]
    fn test3() {
        let zero: u16 = 0b0000_0000_0000_0000;
        let one: u16 = 0b0000_0000_0000_0001;
        let two: u16 = 0b0000_0000_0000_0010;

        let sixtyfivethousand_533: u16 = 0b1111_1111_1111_1101;
        let sixtyfivethousand_534: u16 = 0b1111_1111_1111_1110;
        let sixtyfivethousand_535: u16 = 0b1111_1111_1111_1111;

        print!("{}, {}, {}, ...,", zero, one, two);
        println!("{}, {}, {}", sixtyfivethousand_533, sixtyfivethousand_534,sixtyfivethousand_535);
    }

    #[test]
    #[allow(arithmetic_overflow)]
    fn test4() {
        let (a, b) = (200, 200);
        let c: u8 = a + b;
        println!("200 + 200 = {}", c);
    }
}