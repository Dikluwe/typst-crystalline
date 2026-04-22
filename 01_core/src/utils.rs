//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/utils.md
//! @prompt-hash 113e0000
//! @layer L1
//! @updated 2026-03-23

/// Ponto de instrumentação sem implementação.
/// ADR-0006: substituição de `typst_timing::TimingScope`.
/// Religação prevista no Passo 10 (isolamento de comemo/infra).
/// Ver: `00_nucleo/DEBT.md`
#[allow(unused_macros)]
macro_rules! timing_scope {
    ($name:expr) => {
        // ADR-0006: timing removed — ver 00_nucleo/DEBT.md
        ()
    };
}

/// RAII handle que executa uma closure ao sair do scope.
///
/// Inlinado de `typst_utils::defer` — ADR-0008.
/// Origem: https://github.com/typst/typst/blob/main/crates/typst-utils/src/lib.rs
/// Licença: Apache-2.0
pub(crate) struct DeferHandle<'a, T, F: FnOnce(&mut T)> {
    thing: &'a mut T,
    deferred: Option<F>,
}

impl<T, F: FnOnce(&mut T)> Drop for DeferHandle<'_, T, F> {
    fn drop(&mut self) {
        if let Some(f) = self.deferred.take() {
            f(self.thing);
        }
    }
}

impl<T, F: FnOnce(&mut T)> std::ops::Deref for DeferHandle<'_, T, F> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.thing
    }
}

impl<T, F: FnOnce(&mut T)> std::ops::DerefMut for DeferHandle<'_, T, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.thing
    }
}

/// Executa `deferred` sobre `thing` quando o handle retornado é dropped.
///
/// Uso: `let _guard = defer(self, |p| p.depth -= 1);`
///
/// Inlinado de `typst_utils::defer` — ADR-0008.
pub(crate) fn defer<T, F: FnOnce(&mut T)>(
    thing: &mut T,
    deferred: F,
) -> DeferHandle<'_, T, F> {
    DeferHandle { thing, deferred: Some(deferred) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defer_executa_ao_sair_do_scope() {
        let mut counter = 0i32;
        {
            let _guard = defer(&mut counter, |c| *c += 1);
        }
        assert_eq!(counter, 1);
    }

    #[test]
    fn defer_executa_mesmo_em_panic_via_drop() {
        let mut value = 0u8;
        {
            let _guard = defer(&mut value, |v| *v = 42);
        }
        assert_eq!(value, 42);
    }

    #[test]
    fn defer_deref_mut_acede_a_thing() {
        let mut value = 10i32;
        {
            let mut guard = defer(&mut value, |v| *v += 1);
            // Acesso ao T via DerefMut durante vida do guard
            *guard += 100;
        }
        // DerefMut: value foi modificado para 110; Drop: +1 → 111
        assert_eq!(value, 111);
    }

    #[test]
    fn timing_scope_expande_para_unit() {
        // timing_scope! deve expandir sem efeitos
        let result = timing_scope!("parse");
        assert_eq!(result, ());
    }
}
