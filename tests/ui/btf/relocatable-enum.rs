#![feature(relocatable_types)]

#[relocatable]
//~^ ERROR `#[relocatable]` is only supported on structs and unions
#[repr(C)]
enum Tag {
    A,
    B,
}

fn main() {}
