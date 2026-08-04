#![feature(btf_relocations)]

#[btf_relocatable]
#[repr(C)]
struct Inner {
    value: u32,
}

#[btf_relocatable]
#[repr(C)]
struct Outer {
    inner: Inner,
}

#[repr(C)]
struct Wrapper {
    inner: Inner,
}

fn direct(outer: &Outer) -> u32 {
    outer.inner.value
    //~^ ERROR cannot access fields of a `#[btf_relocatable]` type directly
}

fn nested_direct(wrapper: &Wrapper) -> u32 {
    wrapper.inner.value
    //~^ ERROR cannot access fields of a `#[btf_relocatable]` type directly
}

fn offset() -> usize {
    core::mem::offset_of!(Outer, inner.value)
    //~^ ERROR cannot use `offset_of!` with a `#[btf_relocatable]` type
}

fn nested_offset() -> usize {
    core::mem::offset_of!(Wrapper, inner.value)
    //~^ ERROR cannot use `offset_of!` with a `#[btf_relocatable]` type
}

fn main() {}
