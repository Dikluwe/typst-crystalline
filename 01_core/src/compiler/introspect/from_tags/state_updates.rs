//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/introspect/from_tags/state_updates.md
//! @prompt-hash e3d6d8f3
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// **P191B (ADR-0071)** — slim post-pass para `StateUpdate::Func`.
///
/// Walk fn popula `intr` directamente para todos os payloads excepto
/// `StateUpdate::Func` — Funcs requerem `Engine + EvalContext` para
/// `apply_func`. Esta função processa apenas tags `Tag::Start` com
/// `ElementPayload::StateUpdate { update: StateUpdate::Func(_) }`.
///
/// **Ordem location-monotónica**: walk emite tags em ordem de
/// Locator (counter incrementado por `next()`). Func updates aqui
/// processados na mesma ordem, anexados a `intr.state` history.
/// Para keys onde walk apenas inseriu Sets em locations anteriores
/// (caso comum), value_at(loc_func) devolve o valor correcto antes
/// da aplicação da Func.
///
/// Caller: `fixpoint::run_fixpoint` (vide P174). Path legacy não
/// chama esta função — Funcs ignoradas por design.
pub fn apply_state_funcs(
    tags: &[Tag],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) -> SourceResult<()> {
    // P394: callbacks de state não têm acesso ao scope de eval; usam scope
    // vazio (closures trazem o seu captured scope como parent).
    let mut scopes = Scopes::new(None);
    for tag in tags {
        if let Tag::Start(loc, info) = tag {
            if let ElementPayload::StateUpdate { key, update } = &info.payload {
                if let StateUpdate::Func(func) = update {
                    if let Some(curr) = intr.state.value_at(key, *loc).cloned() {
                        let args = Args::positional(vec![curr]);
                        match apply_func(func.clone(), args, &mut scopes, ctx, engine) {
                            Ok(new_value) => {
                                intr.state.update(key.clone(), new_value, *loc);
                            }
                            Err(diagnostics) => return Err(diagnostics),
                        }
                    }
                    // value_at == None: defensive ignore (P171 padrão
                    // "update sem init").
                }
            }
        }
    }
    Ok(())
}
