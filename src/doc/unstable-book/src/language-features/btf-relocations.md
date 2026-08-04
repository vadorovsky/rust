# `btf_relocations`

The tracking issue for this feature is: none.

The `btf_relocations` feature provides BPF Type Format (BTF) Compile Once, Run Everywhere
(CO-RE) field metadata queries for eBPF programs.

Types that model external BTF types can be marked with `#[btf_relocatable]`. The attribute is
accepted on structs and unions. Ordinary field projection and `offset_of!` are rejected for these
types because they would silently produce fixed, non-relocatable offsets.

```rust,ignore (requires a BPF target)
#![feature(btf_relocations)]

#[btf_relocatable]
struct task_struct {
    pid: i32,
    tgid: i32,
}
```

The `core::btf::field_byte_offset!` and `core::btf::field_byte_size!` macros accept a carrier type
and a dot-separated field path. They return `Option<usize>`: `None` indicates that the target BTF
does not contain the requested field.

```rust,ignore (requires a BPF target)
let pid_offset = core::btf::field_byte_offset!(task_struct, pid);
let pid_size = core::btf::field_byte_size!(task_struct, pid);
```

The queries require a BPF target and debug info (`-C debuginfo=2`).
