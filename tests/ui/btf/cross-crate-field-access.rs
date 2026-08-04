//@ aux-build: btf-relocatable-type.rs

#![feature(btf_relocations)]

extern crate btf_relocatable_type;

use btf_relocatable_type::KernelType;

fn direct(value: &KernelType) -> u32 {
    value.field
    //~^ ERROR cannot access fields of a `#[btf_relocatable]` type directly
}

fn offset() -> usize {
    core::mem::offset_of!(KernelType, field)
    //~^ ERROR cannot use `offset_of!` with a `#[btf_relocatable]` type
}

fn main() {}
