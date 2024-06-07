// 动物

#[derive(Debug)]

struct Animal {
    age: u32,
}

// 龙

#[derive(Debug)]

struct Loong {
    age: u32,
}

impl From<Animal> for Loong {
    fn from(value: Animal) -> Self {
        Loong { age: value.age }
    }
}

impl Into<Animal> for Loong {
    fn into(self) -> Animal {
        Animal { age: self.age }
    }
}
