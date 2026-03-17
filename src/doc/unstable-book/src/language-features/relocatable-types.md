# `relocatable_types`

This feature does not yet have a tracking issue.

------------------------

The `relocatable_types` feature gate allows users to emit BTF relocations for
individual type definitions.

BTF, the BPF Type Format, encodes type information for both the running Linux
kernel and compiled eBPF programs. An eBPF object can carry relocation records
that describe field and aggregate accesses in terms of BTF types instead of
fixed offsets; at load time, the loader compares the program's BTF with the
kernel's BTF and rewrites those accesses to the correct offsets for the target
kernel. These relocations live in the `BTF.ext` ELF section.

With `relocatable_types` enabled, users can request emission of these
relocations.

This feature is only meaningful when compiling for BPF targets and when
debuginfo generation is enabled (for example, `-C debuginfo=2`), because the
backend relies on the extra debug metadata to materialize BTF relocation
records.

On other targets, or without debuginfo, this feature does not do anything.

## `#[relocatable]`

Annotating a `#[repr(C)]` `struct` or `union` with this attribute asks the
compiler to emit the relocation for field and aggregate accesses involving that
type.

```rust
#![feature(relocatable_types)]

#[relocatable]
#[repr(C)]
struct Header {
    len: u32,
    flags: u32,
}
```
