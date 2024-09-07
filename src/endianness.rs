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

    // >> 位移操作符
    #[test]
    fn test1() {
        let n: f32 = 42.42;
        let n_bits: u32 = n.to_bits();
        let sign_bits = n_bits >> 31;
    }
}