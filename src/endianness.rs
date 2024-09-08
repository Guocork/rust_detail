use std::mem::transmute;

// 大端序和小端序
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
        let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];
        let a: u32 = unsafe {
            transmute(big_endian)
        };

        let b: u32 = unsafe {
            transmute(little_endian)
        };
        println!("{} vs {}", a, b);
        let hex_value: u32 = 0xAABBCCDD;
        println!("0xAABBCCDD in decimal is: {}", hex_value);  // 果然 整数在计算机中是以小端序来进行存储的
    }

    // >> 位移操作符 提取符号位
    #[test]
    fn test1() {
        let n: f32 = 42.42;
        let n_bits: u32 = n.to_bits();
        let sign_bits = n_bits >> 31;
    }

    // 提取指数 从一个f32中分离并解码出指数
    #[test]
    fn test2() {
        let n: f32 = 42.42;
        let n_bits: u32 = n.to_bits();
        let exponent_ = n_bits >> 23;
        let exponent_ = exponent_ & 0xff;
        let exponent = (exponent_ as i32) - 127;
    }

    // 分离解码尾数
    #[test]
    fn test3() {
        let n: f32 = 42.42;
        let n_bits: u32 = n.to_bits();
        let mut mantissa: f32 = 1.0;

        for i in 0..23 {
            let mask = 1 << i;
            let one_at_bit_i = n_bits & mask;
            if one_at_bit_i != 0 {
                let i_ = i as f32;
                let weight = 2_f32.powf(i_ - 23.0);
                mantissa += weight;
            }
        }
        println!("{}", mantissa);
    }
}