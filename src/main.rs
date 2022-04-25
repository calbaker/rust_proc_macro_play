#[derive(Debug)]
struct MyStruct {
    x: i32,
}

impl MyStruct {
    fn double_x(&self) -> i32 {
        self.x * 2
    }
}

pub fn main() {
    let x = MyStruct { x: 5 };
    println!("{:?}", x);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn double_x() {
        let s = MyStruct { x: 1 };
        assert_eq!(s.double_x(), 2);
    }
}
