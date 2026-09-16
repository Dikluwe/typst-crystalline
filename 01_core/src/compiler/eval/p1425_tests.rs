//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/p1425-tests.md
//! @prompt-hash 263e6728
//! @layer L1
#![cfg(test)]
//!
//! Teste A/B independente da relação observacional P1425.

use super::*;

fn carrier(inline_explicit: bool) -> Content {
    Content::math_limits_override(Content::text("x"), true, true, inline_explicit)
}

#[test]
fn p1425_observation_relation_observa_presenca_inline() {
    let Content::MathLimitsOverride(omitted) = carrier(false) else { unreachable!() };
    let Content::MathLimitsOverride(omitted_twin) = carrier(false) else {
        unreachable!()
    };
    let Content::MathLimitsOverride(explicit_true) = carrier(true) else {
        unreachable!()
    };
    let Content::MathLimitsOverride(explicit_true_twin) = carrier(true) else {
        unreachable!()
    };

    assert_eq!(
        omitted.observation_relation(&omitted_twin),
        ObservationRelation::Same,
        "identical complete omitted carriers must be Same"
    );
    assert_eq!(
        explicit_true.observation_relation(&explicit_true_twin),
        ObservationRelation::Same,
        "identical complete explicit carriers must be Same"
    );
    assert_eq!(
        omitted.observation_relation(&explicit_true),
        ObservationRelation::Different,
        "omission and explicit true must be observably Different"
    );
    assert_eq!(
        explicit_true.observation_relation(&omitted),
        ObservationRelation::Different,
        "the observation relation must be symmetric for presence"
    );
}
