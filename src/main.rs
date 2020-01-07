use lib::MyMacro;

#[derive(MyMacro)]
struct Yeet {
    a: fn() -> u32,
    b: fn() -> [u8; 1024],
    c: fn() -> Box<u16>,
}

// struct _asdasd where Box<u32>: std::marker::Copy;

fn foo() -> u32 {
    8 as u32
}

fn bar() -> [u8; 1024] {
    [0 as u8; 1024]
}

fn car() -> Box<u16> {
    Box::new(0)
}

fn main() {
    
    let yeet = Yeet {
        a: foo,
        b: bar,
        c: car,
    };
    println!("Hello, world!");
}
