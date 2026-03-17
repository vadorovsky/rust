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

With `btf_relocations` enabled, users can mark external BTF types with
`#[repr(Btf)]` and query field metadata through BTF-aware macros.

This feature is only meaningful when compiling for BPF targets and when
debuginfo generation is enabled (for example, `-C debuginfo=2`), because the
backend relies on the extra debug metadata to materialize BTF relocation
records.

On other targets, or without debuginfo, this feature does not do anything.

## `#[repr(Btf)]`

A `#[repr(Btf)]` `struct` or `union` models an external BTF type whose field
layout should be queried through BTF-aware operations.

```rust
#![feature(btf_relocations)]

#[repr(Btf)]
struct Header {
    len: u32,
    flags: u32,
}
```

Direct field access and `core::mem::offset_of!` are rejected for `#[repr(Btf)]`
types when they would silently produce non-relocatable code. Use the BTF query
macros instead.

## Field metadata queries

The `core::btf` module provides macros for relocatable field metadata queries:

```rust
#![feature(btf_relocations)]

#[repr(Btf)]
struct Header {
    len: u32,
    flags: u32,
}

fn header_len() -> Option<(usize, usize)> {
    if core::btf::field_exists!(Header, len) {
        Some((
            core::btf::field_byte_offset!(Header, len),
            core::btf::field_byte_size!(Header, len),
        ))
    } else {
        None
    }
}
```

Nested field paths are supported:

```rust
core::btf::field_byte_offset!(Outer, inner.field)
```

On targets or backends without BTF relocation support, these macros fall back to
the ordinary layout-computed result for the current compilation unit.
