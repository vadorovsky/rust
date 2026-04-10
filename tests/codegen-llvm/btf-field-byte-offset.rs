//@ only-bpf
//@ ignore-endian-big
//@ needs-llvm-components: bpf
//@ compile-flags: --target bpfel-unknown-none -Cdebuginfo=2

#![feature(relocatable_types, core_intrinsics)]
#![no_std]
#![no_main]

#[relocatable]
#[repr(C)]
pub struct Inner {
    pub x: u32,
    pub y: u32,
}

#[relocatable]
#[repr(C)]
pub struct Outer {
    pub pad: u32,
    pub inner: Inner,
}

// CHECK-DAG: @"llvm.Outer:0:4$0:1" = external global i32, !llvm.preserve.access.index

// CHECK-LABEL: define{{.*}} @field_offset(
#[no_mangle]
pub fn field_offset() -> usize {
    // CHECK: load i32, ptr @"llvm.Outer:0:4$0:1"
    // CHECK-NEXT: tail call i32 @llvm.bpf.passthrough.i32.i32(i32
    // CHECK-NEXT: zext i32 %{{.*}} to i64
    core::intrinsics::btf_field_byte_offset::<Outer>(0, 1)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
