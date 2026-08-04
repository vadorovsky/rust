//! BPF Type Format (BTF) relocation support.
//!
//! This module contains metadata queries for fields of types that model external BTF types.

/// Returns the BTF-relocatable byte offset of a field path.
///
/// The result is [`Some`] when the field exists in the target BTF and [`None`] otherwise. The
/// offset is relative to the root carrier type, including for nested field paths.
///
/// This macro is only available when compiling for a BPF target with debug info enabled.
#[unstable(feature = "btf_relocations", issue = "none")]
#[diagnostic::on_unmatched_args(
    note = "this macro expects a carrier type and a field path, like `field_byte_offset!(Type, field)`"
)]
#[allow_internal_unstable(builtin_syntax)]
#[diagnostic::opaque]
pub macro field_byte_offset($Carrier:ty, $($fields:expr)+ $(,)?) {{
    if builtin # btf_field_exists($Carrier, $($fields)+) {
        $crate::option::Option::Some(builtin # btf_field_byte_offset($Carrier, $($fields)+))
    } else {
        $crate::option::Option::None
    }
}}

/// Returns the BTF-relocatable byte size of a field path.
///
/// The result is [`Some`] when the field exists in the target BTF and [`None`] otherwise. For a
/// nested path, the returned size is the size of the terminal field.
///
/// This macro is only available when compiling for a BPF target with debug info enabled.
#[unstable(feature = "btf_relocations", issue = "none")]
#[diagnostic::on_unmatched_args(
    note = "this macro expects a carrier type and a field path, like `field_byte_size!(Type, field)`"
)]
#[allow_internal_unstable(builtin_syntax)]
#[diagnostic::opaque]
pub macro field_byte_size($Carrier:ty, $($fields:expr)+ $(,)?) {{
    if builtin # btf_field_exists($Carrier, $($fields)+) {
        $crate::option::Option::Some(builtin # btf_field_byte_size($Carrier, $($fields)+))
    } else {
        $crate::option::Option::None
    }
}}
