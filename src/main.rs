use lib::*;
use lib1::*;

// #[derive(MyMacro)]
// struct Yeet {
//     a: fn() -> u32,
//     b: fn() -> [u8; 1024],
//     c: fn() -> Box<u16>,
// }

// struct _asdasd where Box<u32>: std::marker::Copy;

// fn foo() -> u32 {
//     8 as u32
// }

// fn bar() -> [u8; 1024] {
//     [0 as u8; 1024]
// }

// fn car() -> Box<u16> {
//     Box::new(0)
// }

// fn main() {
    
//     let yeet = Yeet {
//         a: foo,
//         b: bar,
//         c: car,
//     };
//     println!("Hello, world!");
// }

// #[AssertReturnSync]
// fn failed() -> *const u8 { unimplemented!() }

// #[AssertReturnSync]
// fn success() -> u8 { unimplemented!() }

#[derive(SerdePacked)]
struct Foo {
    a: u8,
    b: bool,
    c: Vec<u8>,
}

fn main() {

}