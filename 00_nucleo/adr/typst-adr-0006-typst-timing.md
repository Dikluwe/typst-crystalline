# ⚖️ ADR-0006: Remoção de `typst_timing` de L1

**Status**: `PROPOSTO`
**Data**: 2026-03-23

---

## Contexto

O diagnóstico do Passo 4 revelou que as três funções de entrada do
parser (`parse`, `parse_code`, `parse_math`) contêm chamadas a
`typst_timing::TimingScope`. Esta crate instrumenta o pipeline de
compilação com cronómetros para profiling.

`typst_timing` é infraestrutura de observabilidade — não contribui
para o output semântico do parser. A sua presença em L1 viola a
pureza do domínio: L1 passa a conhecer que existe um sistema de
medição, o que é conhecimento de infraestrutura.

Uma trait de telemetria em L1 (`SyntaxTelemetry`) foi considerada
e rejeitada: instrumentação não é domínio. Criar um contrato de
domínio para observabilidade introduz acoplamento desnecessário e
obriga L3 a satisfazer um contrato que não é estrutural.

---

## Decisão

`typst_timing` é removido de `01_core/rules/parse.rs` e
`01_core/rules/lexer.rs`.

Cada ocorrência de `TimingScope` é substituída por uma macro interna
de L1 que expande para nada:

```rust
// 01_core/src/utils.rs
/// Ponto de instrumentação sem implementação.
/// ADR-0006: substituição de typst_timing::TimingScope.
/// Ligação a telemetria real: Passo 10 (isolamento de comemo/infra).
macro_rules! timing_scope {
    ($name:expr) => {
        ()
    };
}
pub(crate) use timing_scope;
```

Cada local onde `TimingScope` era criado recebe um comentário
`// ADR-0006: timing removed — ver 00_nucleo/DEBT.md`.

Um ficheiro `00_nucleo/DEBT.md` regista os três pontos de
instrumentação perdidos:

```markdown
# Dívida de instrumentação — ADR-0006

Os seguintes pontos de timing foram removidos para manter L1 puro.
Religação prevista no Passo 10.

| Função       | Nome do scope original |
|--------------|------------------------|
| parse()      | "parse"                |
| parse_code() | "parse-code"           |
| parse_math() | "parse-math"           |
```

---

## O que esta ADR não decide

- Como ligar telemetria real no Passo 10 — decisão adiada para
  quando `comemo` for isolado em L3 (ADR-0001 Opção B)
- Se a macro `timing_scope!` será eventualmente ligada a um
  provider injectado ou substituída por outra abordagem

---

## Prompts afectados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/rules/parse.md` | Documentar ausência de timing; referenciar ADR-0006 e DEBT.md |

---

## Consequências

**Positivas**: L1 sem dependência de observabilidade; `parse()` é
função pura sem efeitos de instrumentação.

**Negativas**: Perda temporária de visibilidade de performance nas
três funções de entrada do parser durante a migração.

**Neutras**: A macro `timing_scope!` serve de marcador — quando o
Passo 10 chegar, os pontos de religação estão identificados sem
pesquisa no histórico git.

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Trait `SyntaxTelemetry` em L1 | Telemetria injectável | Instrumentação não é domínio — contrato errado |
| Autorizar `typst_timing` em `[l1_allowed_external]` | Zero trabalho | Infraestrutura de observabilidade em L1 |
| Remoção sem marcador | Simples | Perda de rastreabilidade — Passo 10 não sabe o que religar |

---

## Referências

- ADR-0001 — isolamento de `comemo` em L3 no Passo 10
- Diagnóstico Passo 4 — três ocorrências de `TimingScope` em parse.rs
