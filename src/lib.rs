// 动物

// #[derive(Debug)]

// struct Animal {
//     age: u32,
// }

// // 龙

// #[derive(Debug)]

// struct Loong {
//     age: u32,
// }

// impl From<Animal> for Loong {
//     fn from(value: Animal) -> Self {
//         Loong { age: value.age }
//     }
// }

// impl Into<Animal> for Loong {
//     fn into(self) -> Animal {
//         Animal { age: self.age }
//     }
// }

use std::cell::RefCell;
mod bit_model;
mod endianness;
#[derive(Debug)]
struct Animal {
    age: u8,
    weight: RefCell<f32>,
    name: RefCell<String>,
}

#[cfg(test)]

mod test {

    use super::*;
    #[test]
    fn test() {
        let dog = Animal {
            age: 3,
            weight: RefCell::new(10.5),
            name: RefCell::new(String::from("小憨包")),
        };

        println!("dog:{:?}", dog);

        // 修改weight 
        *dog.weight.borrow_mut() += 10.1;
        // 修改name
        *dog.name.borrow_mut() = String::from("憨憨");
        println!("dog weight:{:?}", dog);

        let a = 5;
    }
}
