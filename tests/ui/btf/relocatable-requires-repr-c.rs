#![feature(relocatable_types)]

#[relocatable]
//~^ ERROR `#[relocatable]` is only supported on `#[repr(C)]` structs and unions
struct Header {
    len: u32,
}

fn main() {}
