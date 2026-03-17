#[relocatable]
//~^ ERROR the `#[relocatable]` attribute is experimental
#[repr(C)]
struct S;

fn main() {}
