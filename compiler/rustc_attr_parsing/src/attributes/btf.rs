use rustc_feature::AttributeStability;

use super::prelude::*;

pub(crate) struct BtfRelocatableParser;

impl NoArgsAttributeParser for BtfRelocatableParser {
    const PATH: &[Symbol] = &[sym::btf_relocatable];
    const ALLOWED_TARGETS: AllowedTargets<'_> =
        AllowedTargets::AllowList(&[Allow(Target::Struct), Allow(Target::Union)]);
    const STABILITY: AttributeStability = unstable!(btf_relocations);
    const CREATE: fn(Span) -> AttributeKind = AttributeKind::BtfRelocatable;
}
