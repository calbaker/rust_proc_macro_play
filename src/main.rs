// much of this code is bowrrowed from https://blog.logrocket.com/procedural-macros-in-rust/

use attr_macro::{Describe, DoubleF64};

#[derive(Describe, DoubleF64)]
struct MyStruct {
    my_string: String,
    my_enum: MyEnum,
    my_number: f64,
}

#[derive(Describe)]
struct MyTupleStruct(u32, String, i8);

#[derive(Describe)]
enum MyEnum {
    VariantA,
    VariantB,
}

#[derive(Describe)]
union MyUnion {
    unsigned: u32,
    signed: i32,
}

fn main() {
    MyStruct::describe();
    MyTupleStruct::describe();
    MyEnum::describe();
    MyUnion::describe();
    let mystruct = MyStruct {
        my_string: "some str".to_string(),
        my_enum: MyEnum::VariantA,
        my_number: 2.0,
    };
    println!("my_number * 2: {}", mystruct.double_my_number());
}
