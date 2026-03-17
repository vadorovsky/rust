#![feature(btf_relocations)]

#[repr(Btf)]
struct Task {
    pid: u32,
}

fn field_access(task: &Task) -> u32 {
    task.pid
    //~^ ERROR cannot access fields of a `#[repr(Btf)]` type directly
}

fn offset_of() -> usize {
    std::mem::offset_of!(Task, pid)
    //~^ ERROR cannot use `offset_of!` with a `#[repr(Btf)]` type
}

fn main() {}
