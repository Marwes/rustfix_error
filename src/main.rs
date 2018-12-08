#[macro_use]
extern crate serde_derive_state;
extern crate serde_state;

#[derive(SerializeState)]
#[serde(serialize_state = "::Test")]
struct Test {
    x: i32,
}

fn main() {
    let _ = Test { x: 1 };
    println!("Hello, world!");
}
