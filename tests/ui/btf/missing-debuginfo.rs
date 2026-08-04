//@ only-bpf
//@ needs-llvm-components: bpf

#![feature(btf_relocations)]
#![no_std]
#![no_main]

#[btf_relocatable]
struct KernelType {
    field: u32,
}

#[unsafe(no_mangle)]
fn field_offset() -> Option<usize> {
    core::btf::field_byte_offset!(KernelType, field)
    //~^ ERROR BTF field relocation queries require debug info
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
