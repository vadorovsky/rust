// Checks that results larger than one register are returned indirectly
//@ add-minicore
//@ needs-llvm-components: bpf
//@ compile-flags: --target bpfel-unknown-none

#![feature(lang_items, no_core)]
#![no_core]
#![no_std]
#![no_main]

extern crate minicore;
use minicore::Result::*;
use minicore::*;

#[no_mangle]
fn outer(a: u64) -> u64 {
    let v = match inner_res(a) {
        Ok(v) => v,
        Err(()) => 0,
    };

    inner_big(v).a[0] as u64
}

// CHECK-LABEL: define {{.*}} @_R{{.*}}inner_res(
// CHECK-SAME:   ptr{{[^,]*}},
// CHECK-SAME:   i64{{[^)]*}}
#[inline(never)]
fn inner_res(a: u64) -> Result<u64, ()> {
    if a == 0 { Err(()) } else { Ok(a + 1) }
}

struct Big {
    a: [u16; 32],
    b: u64,
}

// CHECK-LABEL: define {{.*}} @_R{{.*}}inner_big(
// CHECK-SAME:   ptr{{[^,]*}},
// CHECK-SAME:   i64{{[^)]*}}
#[inline(never)]
fn inner_big(a: u64) -> Big {
    Big { a: [a as u16; 32], b: 42 }
}
