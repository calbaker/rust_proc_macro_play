// much of this code is bowrrowed from https://blog.logrocket.com/procedural-macros-in-rust/

use proc_macros::DoubleF64;

#[derive(DoubleF64)]
struct MyStruct {
    my_string: String,
    my_number: f64,
    my_other_number: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_double() {
        let mystruct = MyStruct {
            my_string: "some str".to_string(),
            my_number: 2.0,
            my_other_number: 17.0,
        };
        assert_eq!(mystruct.double_my_number(), 4.0);
        assert_eq!(mystruct.double_my_other_number(), 34.0);
    }
}
