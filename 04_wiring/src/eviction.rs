//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/eviction.md
//! @prompt-hash 7ac7b48b
//! @layer L4
//! @updated 2026-05-06
//!
//! **P204E (M8)** — Wrapper `crystalline_evict` sobre
//! `comemo::evict` per ADR-0073 (política de invalidação
//! tracking-based intra-compilation + `evict()` exposed
//! para callers).
//!
//! Paridade vanilla: `lab/typst-original/crates/typst-cli/src/watch.rs:81`
//! invoca `comemo::evict(10)` directamente em watch mode.
//! Cristalino expõe wrapper paralelo em L4 wiring para futura
//! integração CLI / watch mode (não materializada em P204E —
//! reservada para pós-M8).
//!
//! L4 = composição pura (CLAUDE.md). Wrapper é delegate
//! trivial — sem policy adicional além da exposta por comemo.

/// Evict cache memoizado de comemo cristalino.
///
/// Remove memoized results cuja `age >= max_age`. Age cresce
/// em 1 por chamada `evict`; reset a 0 em cache hit. `max_age
/// = 0` clears entire cache.
///
/// **Uso típico**:
/// - Watch mode futuro: `crystalline_evict(10)` periodicamente
///   para evitar memory growth ilimitado.
/// - Tests / debug: `crystalline_evict(0)` para forçar
///   recomputação.
///
/// **Cross-references**:
/// - ADR-0073 (PROPOSTO) §P204E plano de materialização.
/// - P204A C6 (política invalidação).
/// - Vanilla: `comemo::evict(10)` em CLI watch.
///
/// **Nota**: `#[allow(dead_code)]` aplicado porque P204E expõe
/// API para integração CLI / watch mode futura (não exercida
/// em main.rs actual). Sentinel tests confirmam que função
/// permanece compilável.
#[allow(dead_code)]
pub fn crystalline_evict(max_age: usize) {
    comemo::evict(max_age);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p204e_crystalline_evict_existe() {
        // Sentinel: confirma que `crystalline_evict` está
        // disponível como wrapper sobre `comemo::evict`.
        // Falha de compilação se função for removida.
        // Chama com max_age = 0 (clear all) — semanticamente
        // safe em test (não há outros tests em paralelo
        // dependendo de cache deste binary).
        crystalline_evict(0);
    }

    #[test]
    fn p204e_crystalline_evict_aceita_max_age_parametro() {
        // Sentinel: confirma assinatura `(max_age: usize)`
        // — falha de compilação se signature mudar.
        crystalline_evict(10);
        crystalline_evict(usize::MAX);
    }
}
