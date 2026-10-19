//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/html/attr_specs.md
//! @prompt-hash d39513c9
//! @layer L1
//! @updated 2026-09-30

#[derive(Clone, Copy)]
pub(super) enum AttrKind {
    Str,
    Int,
    PositiveInt,
    NonNegativeInt,
    Float,
    TrueFalse,
    Presence,
    ListStr,
    ListChar,
    Enum(&'static [&'static str]),
    EnumList(&'static [&'static str]),
    EnumOrStr(&'static [&'static str]),
    NoneEmptyOrEnum(&'static [&'static str]),
    BoolOr(&'static [&'static str]),
    BoolOrUndefined,
    NoneOrEnum(&'static [&'static str]),
    OnOffOrNoneOrEnum(&'static [&'static str]),
    OnOff,
    ContentEditable,
    Direction,
    PresenceOr(&'static [&'static str]),
    NoneOrStr,
    AutoOr(&'static [&'static str]),
    Preload,
    NoneOrRole,
    YesNo,
    Datetime,
    DurationOrDatetime,
    Color,
    FloatListComma,
    StrListComma,
    StrFloatDatetime,
    PositiveFloatOr(&'static [&'static str]),
    InputValue,
    SourceSizeList,
    ImageCandidateList,
    IconSizeList,
}

pub(super) struct AttrSpec {
    pub(super) name: &'static str,
    pub(super) kind: AttrKind,
}
