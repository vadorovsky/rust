#![feature(btf_relocations)]

#[btf_relocatable]
struct ValidStruct {
    field: u32,
}

#[btf_relocatable]
//~^ ERROR the `btf_relocatable` attribute cannot be used on enums
enum InvalidEnum {
    A,
}

#[btf_relocatable]
//~^ ERROR the `btf_relocatable` attribute cannot be used on functions
fn invalid_function() {}

#[btf_relocatable]
union ValidUnion {
    word: u64,
    half: u32,
}

fn main() {}
