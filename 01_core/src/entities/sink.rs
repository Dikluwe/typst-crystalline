//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/sink.md
//! @prompt-hash a31e1f86
//! @layer L1
//! @updated 2026-04-23
//!
//! Colector de diagnósticos não-fatais (warnings) durante `eval()`.
//! Materializado no Passo 104 conforme ADR-0042.
//!
//! Substitui o stub `Sink(())` que vivia em `world_types.rs` desde o
//! Passo 12. API mínima: `warn` + `into_diagnostics` + `is_empty`,
//! com dedup por `(span, message)` para evitar inundação em hot loops.
//!
//! Integração comemo adiada: o bloco `#[comemo::track] impl Sink {}`
//! em `world_types.rs` permanece (vazio) para preservar a assinatura
//! `TrackedMut<Sink>` do `eval()`. Os métodos reais vivem aqui num
//! `impl Sink` não-tracked.

use rustc_hash::FxHashSet;

use crate::entities::source_result::SourceDiagnostic;
use crate::entities::span::Span;

/// Colector de diagnósticos não-fatais (warnings).
///
/// Dedup: duas warnings com o mesmo `(span, message)` contam como
/// uma. `severity`, `hints` e `trace` não participam na chave.
///
/// `Clone` derivado para satisfazer o contrato de `#[comemo::track]`
/// (Passo 106, ADR-0043) — comemo precisa de clonar o estado
/// interno para rollback de chamadas tracked. Clone é O(n) no número
/// de diagnósticos acumulados; aceitável porque o número total por
/// eval é baixo (dedup garante).
#[derive(Clone)]
pub struct Sink {
    diagnostics: Vec<SourceDiagnostic>,
    seen: FxHashSet<(Span, String)>,
}

impl Sink {
    /// Cria um Sink vazio.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adiciona um `SourceDiagnostic` se ainda não foi visto. Dedup
    /// por `(span, message)` — idempotente para chamadas repetidas.
    pub fn warn(&mut self, diag: SourceDiagnostic) {
        self.record(diag);
    }

    /// Implementação partilhada entre a API não-tracked (`warn`) e o
    /// método tracked (`warn` em `#[comemo::track] impl Sink`) — garante
    /// dedup consistente (ADR-0043, Passo 106).
    pub(crate) fn record(&mut self, diag: SourceDiagnostic) {
        let key = (diag.span, diag.message.clone());
        if self.seen.insert(key) {
            self.diagnostics.push(diag);
        }
    }

    /// Retorna `true` se nenhum diagnóstico foi acumulado.
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Consome o Sink e devolve os diagnósticos na ordem de inserção.
    pub fn into_diagnostics(self) -> Vec<SourceDiagnostic> {
        self.diagnostics
    }
}

impl Default for Sink {
    fn default() -> Self {
        Self {
            diagnostics: Vec::new(),
            seen: FxHashSet::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::source_result::SourceDiagnostic;
    use crate::entities::span::Span;

    fn test_span(n: u32) -> Span {
        // Usar `Span::detached()` dá sempre o mesmo valor; precisamos de
        // spans distintos para testar dedup cross-span. Construir via
        // Span::detached + shift não é possível com a API pública — cada
        // teste que precise de spans distintos constrói via helpers.
        let _ = n;
        Span::detached()
    }

    #[test]
    fn sink_novo_esta_vazio() {
        let s = Sink::new();
        assert!(s.is_empty());
        assert_eq!(s.into_diagnostics().len(), 0);
    }

    #[test]
    fn warn_adiciona_um_diagnostico() {
        let mut s = Sink::new();
        s.warn(SourceDiagnostic::warning(Span::detached(), "um aviso"));
        assert!(!s.is_empty());
        let diags = s.into_diagnostics();
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].message, "um aviso");
    }

    #[test]
    fn warn_duplicado_nao_acumula_segundo() {
        let mut s = Sink::new();
        let sp = Span::detached();
        s.warn(SourceDiagnostic::warning(sp, "mesmo"));
        s.warn(SourceDiagnostic::warning(sp, "mesmo"));
        let diags = s.into_diagnostics();
        assert_eq!(diags.len(), 1,
            "dedup por (span, message) deve contar dois iguais como um");
    }

    #[test]
    fn warn_mesma_span_message_diferente_nao_deduplica() {
        let mut s = Sink::new();
        let sp = Span::detached();
        s.warn(SourceDiagnostic::warning(sp, "mensagem A"));
        s.warn(SourceDiagnostic::warning(sp, "mensagem B"));
        let diags = s.into_diagnostics();
        assert_eq!(diags.len(), 2,
            "mensagens distintas no mesmo span contam separadamente");
    }

    #[test]
    fn warn_preserva_ordem_de_insercao() {
        let mut s = Sink::new();
        let sp = Span::detached();
        s.warn(SourceDiagnostic::warning(sp, "primeiro"));
        s.warn(SourceDiagnostic::warning(sp, "segundo"));
        s.warn(SourceDiagnostic::warning(sp, "terceiro"));
        let diags = s.into_diagnostics();
        let msgs: Vec<&str> = diags.iter().map(|d| d.message.as_str()).collect();
        assert_eq!(msgs, vec!["primeiro", "segundo", "terceiro"],
            "ordem de inserção preservada");
    }

    #[test]
    fn warn_hint_diferente_mesmo_par_conta_como_duplicado() {
        let mut s = Sink::new();
        let sp = Span::detached();
        s.warn(
            SourceDiagnostic::warning(sp, "aviso")
                .with_hint("hint A"),
        );
        s.warn(
            SourceDiagnostic::warning(sp, "aviso")
                .with_hint("hint B"),
        );
        let diags = s.into_diagnostics();
        assert_eq!(diags.len(), 1,
            "hints não participam na chave de dedup — o primeiro ganha");
        assert_eq!(diags[0].hints, vec!["hint A".to_string()],
            "primeiro inserido preservado");
        let _ = test_span; // suprimir unused (helper para futuros tests com spans distintos)
    }

    #[test]
    fn warn_severity_diferente_mesmo_par_conta_como_duplicado() {
        let mut s = Sink::new();
        let sp = Span::detached();
        s.warn(SourceDiagnostic::warning(sp, "comum"));
        // Se severity fosse parte da chave, este error seria aceite. Não é.
        s.warn(SourceDiagnostic::error(sp, "comum"));
        let diags = s.into_diagnostics();
        assert_eq!(diags.len(), 1,
            "severity não participa na chave de dedup");
    }

    #[test]
    fn into_diagnostics_consome_sink() {
        let mut s = Sink::new();
        s.warn(SourceDiagnostic::warning(Span::detached(), "um"));
        // Após `into_diagnostics`, `s` já não está acessível.
        // Este teste valida apenas que a chamada compila e devolve vec.
        let v = s.into_diagnostics();
        assert_eq!(v.len(), 1);
    }
}
