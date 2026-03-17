# `btf_relocations`

This feature does not yet have a tracking issue.

------------------------

The `btf_relocations` feature gate allows users to emit BTF relocations for
individual type definitions.

BTF, the BPF Type Format, encodes type information for both the running Linux
kernel and compiled eBPF programs. An eBPF object can carry relocation records
that describe field and aggregate accesses in terms of BTF types instead of
fixed offsets; at load time, the loader compares the program's BTF with the
kernel's BTF and rewrites those accesses to the correct offsets for the target
kernel. These relocations live in the `BTF.ext` ELF section.

With `btf_relocations` enabled, users can request emission of these
relocations.

This feature is only meaningful when compiling for BPF targets and when
debuginfo generation is enabled (for example, `-C debuginfo=2`), because the
backend relies on the extra debug metadata to materialize BTF relocation
records.

On other targets, or without debuginfo, this feature does not do anything.

## `#[repr(btf)]`

A `#[repr(btf)]` `struct` or `union` models an external BTF type whose field
layout should be queried through BTF-aware operations.

```rust
#![feature(btf_relocations)]

#[repr(btf)]
struct Header {
    len: u32,
    flags: u32,
}
```
