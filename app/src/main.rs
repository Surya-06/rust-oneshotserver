extern crate osw_lib;

fn main() {
    println!("Hello, world!");

    let return_value = osw_lib::add(1, 2);

    osw_lib::some_other_test();

    println!("The value is {}", return_value);
}
