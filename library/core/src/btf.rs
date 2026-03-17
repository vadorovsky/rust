//! BTF relocation support.

/// Returns the BTF-relocatable byte offset of a field path when supported.
///
/// On targets or backends without BTF field relocation support, this may fall back to
/// the ordinary layout-computed field offset.
#[unstable(feature = "btf_relocations", issue = "none")]
#[diagnostic::on_unmatch_args(
    note = "this macro expects a container type and a field path, like `field_byte_offset!(Type, field)`"
)]
#[allow_internal_unstable(builtin_syntax)]
pub macro field_byte_offset($Container:ty, $($fields:expr)+ $(,)?) {
    builtin # btf_field_byte_offset($Container, $($fields)+)
}

/// Returns the BTF-relocatable byte size of a field path when supported.
///
/// On targets or backends without BTF field relocation support, this may fall back to
/// the ordinary layout-computed field size.
#[unstable(feature = "btf_relocations", issue = "none")]
#[diagnostic::on_unmatch_args(
    note = "this macro expects a container type and a field path, like `field_byte_size!(Type, field)`"
)]
#[allow_internal_unstable(builtin_syntax)]
pub macro field_byte_size($Container:ty, $($fields:expr)+ $(,)?) {
    builtin # btf_field_byte_size($Container, $($fields)+)
}

/// Returns whether a field path exists according to BTF relocation information when supported.
///
/// On targets or backends without BTF field relocation support, this may conservatively
/// fall back to the field being present in the current compilation unit's layout.
#[unstable(feature = "btf_relocations", issue = "none")]
#[diagnostic::on_unmatch_args(
    note = "this macro expects a container type and a field path, like `field_exists!(Type, field)`"
)]
#[allow_internal_unstable(builtin_syntax)]
pub macro field_exists($Container:ty, $($fields:expr)+ $(,)?) {
    builtin # btf_field_exists($Container, $($fields)+)
}
