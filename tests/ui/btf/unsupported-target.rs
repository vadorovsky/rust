#![feature(btf_relocations)]

#[btf_relocatable]
struct KernelType {
    field: u32,
}

fn queries() {
    let _: Option<usize> = core::btf::field_byte_offset!(KernelType, field);
    //~^ ERROR BTF field relocation queries are only supported for BPF targets
    let _: Option<usize> = core::btf::field_byte_size!(KernelType, field);
    //~^ ERROR BTF field relocation queries are only supported for BPF targets
}

fn main() {}
