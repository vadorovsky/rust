#[btf_relocatable]
//~^ ERROR the `btf_relocatable` attribute is an experimental feature
struct KernelType {
    field: u32,
}

fn main() {}
