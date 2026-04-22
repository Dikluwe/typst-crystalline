# ADR-0017 — Adiamento de eval() e estratégia typst-library

**Status**: `IMPLEMENTADO`
**Data**: 2026-03-26

## Contexto

O diagnóstico do Passo 6 confirmou que `typst-eval/src/lib.rs` depende
de `typst-library` através de 7 imports directos:

```
typst_library::World
typst_library::diag::{SourceResult, bail}
typst_library::engine::{Engine, Route, Sink, Traced}
typst_library::foundations::{Context, Module, NativeElement, Scope, Scopes, Value}
typst_library::introspection::{EmptyIntrospector, Introspector}
typst_library::math::EquationElem
typst_library::routines::Routines
```

A assinatura real de `eval()` requer `Routines`, `Traced`, `Sink`,
`Route` — todos de `typst-library::engine`. Estes tipos têm dependências
de I/O e rendering que impedem a migração para L1.

## Decisão

**`eval()` não migra neste passo.**

O erro de classificação a evitar: concluir que `eval()` deve ir para L3
por ter muitas dependências. `eval()` é o motor central do compilador —
lógica de domínio, não I/O. Pertence a L1 quando os seus tipos
dependentes estiverem migrados.

O trabalho em falta antes de `eval()` poder migrar para L1:

1. Análise completa de `typst-library/src/foundations/`
2. Separação dos tipos de domínio puro (L1) das implementações (L3)
3. `Module`, `Value`, `Content` reais em L1 sem dependências externas
4. `Engine`, `Route`, `Sink`, `Traced` — análise e decisão de camada
5. Só então `eval()` pode ser implementado em `01_core/rules/eval.rs`

## Stubs criados neste passo

Para desbloquear o pipeline e satisfazer V2 (cobertura de testes):

| Stub | Localização | Estado |
|------|-------------|--------|
| `Module(())` | `01_core/src/entities/module.rs` | Stub opaco |
| `Value(())` | `01_core/src/entities/value.rs` | Stub opaco |
| `SourceDiagnostic` | `01_core/src/entities/source_result.rs` | Migrado (domínio puro) |
| `Tracepoint` | `01_core/src/entities/source_result.rs` | Migrado (domínio puro) |
| `SourceResult<T>` | `01_core/src/entities/source_result.rs` | Migrado |

## Tracepoint — variantes reais (diagnóstico)

O original tem 4 variantes (não 3 como o plano estimava):

```rust
pub enum Tracepoint {
    Call(Option<EcoString>),   // → Call(Option<String>) — ADR-0015
    Show(EcoString),           // → Show(String)
    Import(EcoString),         // → Import(String)
    Include(EcoString),        // → Include(String) — variante extra
}
```

Migração directa para L1 com `EcoString` → `String` (ADR-0015).

## Consequências

- `World::source()` em `01_core/contracts/world.rs` usa `SourceResult<Source>` — compila
- `Module` e `Value` como stubs satisfazem os tipos de retorno até que a migração real aconteça
- O número de passos até `eval()` funcionar é indeterminado — depende da análise de `typst-library`
