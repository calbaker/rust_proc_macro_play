// much of this code is bowrrowed from https://blog.logrocket.com/procedural-macros-in-rust/

use proc_macro::DoubleF64;

#[derive(DoubleF64)]
struct MyStruct {
    my_string: String,
    my_number: f64,
    my_other_number: f64,
}


fn main() {
    let mystruct = MyStruct {
        my_string: "some str".to_string(),
        my_number: 2.0,
        my_other_number: 2.0,
    };
    println!("my_number * 2: {}", mystruct.double_my_number());
}
