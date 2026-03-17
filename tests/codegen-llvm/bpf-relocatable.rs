//@ only-bpf
//@ ignore-endian-big
//@ needs-llvm-components: bpf
//@ compile-flags: --target bpfel-unknown-none -Cdebuginfo=2

#![feature(relocatable_types)]
#![no_std]
#![no_main]

#[relocatable]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Inner {
    pub x: u32,
    pub y: u32,
}

#[relocatable]
#[repr(C)]
pub union Mid {
    pub inner: Inner,
    pub raw: [u32; 2],
}

#[relocatable]
#[repr(C)]
pub struct Outer {
    pub pad: u32,
    pub mid: Mid,
    pub arr: [u32; 4],
}

// CHECK-DAG: @"llvm.Outer:0:8$0:1:0:1" = external global i64, !llvm.preserve.access.index
// CHECK-DAG: @"llvm.Outer:0:20$0:2:2" = external global i64, !llvm.preserve.access.index

// CHECK-LABEL: define{{.*}} @access_ptr(
#[no_mangle]
pub fn access_ptr(p: *const Outer) -> u32 {
    // CHECK: load i64, ptr @"llvm.Outer:0:8$0:1:0:1"
    // CHECK-NEXT: getelementptr i8, ptr %p, i64
    // CHECK-NEXT: call ptr @llvm.bpf.passthrough.p0.p0(i32
    // CHECK: load i64, ptr @"llvm.Outer:0:20$0:2:2"
    // CHECK-NEXT: getelementptr i8, ptr %p, i64
    // CHECK-NEXT: call ptr @llvm.bpf.passthrough.p0.p0(i32
    unsafe { (*p).mid.inner.y + (*p).arr[2] }
}

// CHECK-LABEL: define{{.*}} @access_ref(
#[no_mangle]
pub fn access_ref(p: &Outer) -> u32 {
    // CHECK: load i64, ptr @"llvm.Outer:0:8$0:1:0:1"
    // CHECK-NEXT: getelementptr i8, ptr %p, i64
    // CHECK-NEXT: call ptr @llvm.bpf.passthrough.p0.p0(i32
    // CHECK: load i64, ptr @"llvm.Outer:0:20$0:2:2"
    // CHECK-NEXT: getelementptr i8, ptr %p, i64
    // CHECK-NEXT: call ptr @llvm.bpf.passthrough.p0.p0(i32
    unsafe { p.mid.inner.y + p.arr[2] }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
