//@ only-bpf
//@ ignore-endian-big
//@ needs-llvm-components: bpf
//@ compile-flags: --target bpfel-unknown-none -Cdebuginfo=2

#![feature(btf_relocations)]
#![no_std]
#![no_main]

#[repr(C)]
pub struct Inner {
    pub x: u32,
    pub y: u32,
}

#[repr(C)]
pub struct Outer {
    pub pad: u32,
    pub inner: Inner,
}

// CHECK-DAG: @"llvm.Outer:0:4$0:1" = external global i32, !llvm.preserve.access.index
// CHECK-DAG: @"llvm.Outer:1:8$0:1" = external global i32, !llvm.preserve.access.index
// CHECK-DAG: @"llvm.Outer:2:1$0:1" = external global i32, !llvm.preserve.access.index
// CHECK-DAG: @"llvm.Outer:0:8$0:1:1" = external global i32, !llvm.preserve.access.index
// CHECK-DAG: @"llvm.Outer:1:4$0:1:1" = external global i32, !llvm.preserve.access.index
// CHECK-DAG: @"llvm.Outer:2:1$0:1:1" = external global i32, !llvm.preserve.access.index

// CHECK-LABEL: define{{.*}} @field_offset(
#[no_mangle]
pub fn field_offset() -> usize {
    // CHECK: load i32, ptr @"llvm.Outer:0:4$0:1"
    // CHECK-NEXT: tail call i32 @llvm.bpf.passthrough.i32.i32(i32
    // CHECK-NEXT: zext i32
    core::btf::field_byte_offset!(Outer, inner)
}

// CHECK-LABEL: define{{.*}} @field_size(
#[no_mangle]
pub fn field_size() -> usize {
    // CHECK: load i32, ptr @"llvm.Outer:1:8$0:1"
    // CHECK-NEXT: tail call i32 @llvm.bpf.passthrough.i32.i32(i32
    // CHECK-NEXT: zext i32
    core::btf::field_byte_size!(Outer, inner)
}

// CHECK-LABEL: define{{.*}} @field_exists(
#[no_mangle]
pub fn field_exists() -> bool {
    // CHECK: load i32, ptr @"llvm.Outer:2:1$0:1"
    // CHECK-NEXT: tail call i32 @llvm.bpf.passthrough.i32.i32(i32
    // CHECK-NEXT: icmp ne i32
    core::btf::field_exists!(Outer, inner)
}

// CHECK-LABEL: define{{.*}} @nested_field_offset(
#[no_mangle]
pub fn nested_field_offset() -> usize {
    // CHECK: load i32, ptr @"llvm.Outer:0:8$0:1:1"
    // CHECK-NEXT: tail call i32 @llvm.bpf.passthrough.i32.i32(i32
    // CHECK-NEXT: zext i32
    core::btf::field_byte_offset!(Outer, inner.y)
}

// CHECK-LABEL: define{{.*}} @nested_field_size(
#[no_mangle]
pub fn nested_field_size() -> usize {
    // CHECK: load i32, ptr @"llvm.Outer:1:4$0:1:1"
    // CHECK-NEXT: tail call i32 @llvm.bpf.passthrough.i32.i32(i32
    // CHECK-NEXT: zext i32
    core::btf::field_byte_size!(Outer, inner.y)
}

// CHECK-LABEL: define{{.*}} @nested_field_exists(
#[no_mangle]
pub fn nested_field_exists() -> bool {
    // CHECK: load i32, ptr @"llvm.Outer:2:1$0:1:1"
    // CHECK-NEXT: tail call i32 @llvm.bpf.passthrough.i32.i32(i32
    // CHECK-NEXT: icmp ne i32
    core::btf::field_exists!(Outer, inner.y)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
