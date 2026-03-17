#[repr(btf)]
//~^ ERROR BTF relocations are experimental
struct Task {
    pid: u32,
}

fn main() {}
