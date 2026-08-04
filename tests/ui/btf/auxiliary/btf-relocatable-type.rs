#![feature(btf_relocations)]
#![crate_type = "lib"]

#[btf_relocatable]
pub struct KernelType {
    pub field: u32,
}
