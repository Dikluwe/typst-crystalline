//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout_counters.md
//! @prompt-hash dd2f700b
//! @layer L1
//! @updated 2026-04-13

use crate::entities::counter_state_legacy::{CounterAction, CounterStateLegacy};

/// Braço `SetHeadingNumbering` — activa/desactiva numeração de headings.
/// Não produz output visual.
pub(super) fn layout_set_heading_numbering(counter: &mut CounterStateLegacy, active: bool) {
    counter.numbering_active.insert("heading".to_string(), active);
}

/// Braço `SetEquationNumbering` (P199B) — activa/desactiva numeração de
/// equations. Análoga a `layout_set_heading_numbering`. Não produz output
/// visual. Mutação legacy é write paralelo M5: walk arm Equation +
/// compute_labelled Equation arm (P195D) lêem `state.numbering_active`
/// durante walk; cleanup orgânico em M6.
pub(super) fn layout_set_equation_numbering(counter: &mut CounterStateLegacy, active: bool) {
    counter.numbering_active.insert("equation".to_string(), active);
}

/// Braço `CounterUpdate` — avança ou força um contador.
/// Não produz output visual.
pub(super) fn layout_counter_update(
    counter: &mut CounterStateLegacy,
    key: &str,
    action: &CounterAction,
) {
    match action {
        CounterAction::Step => {
            if key == "heading" {
                counter.step_hierarchical("heading", 1);
            } else {
                counter.step_flat(key);
            }
        }
        CounterAction::Update(val) => {
            counter.update_flat(key, *val);
        }
    }
}

/// Braço `CounterDisplay` — lê o estado actual e retorna texto formatado.
/// Delega em `CounterStateLegacy::display_value` (Passo 66).
pub(super) fn format_counter_display(counter: &CounterStateLegacy, kind: &str) -> String {
    counter.display_value(kind)
}

