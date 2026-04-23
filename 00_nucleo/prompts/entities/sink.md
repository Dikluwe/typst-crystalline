# Prompt L0 — Sink
Hash do Código: a31e1f86

## Módulo
`01_core/src/entities/sink.rs`

## Propósito

Colector de diagnósticos não-fatais (warnings) durante `eval()`.
Fundação para emitir warnings estruturados a partir do eval quando
o caminho de produção encontra construções não suportadas, deprecadas
ou ambíguas.

ADR-0042 (Passo 104) formaliza a forma actual.

## Contrato

### Struct `Sink`

```rust
pub struct Sink {
    diagnostics: Vec<SourceDiagnostic>,
    seen: FxHashSet<(Span, String)>,
}
```

- `diagnostics` preserva ordem de inserção.
- `seen` indexa `(span, message)` para dedup.

### API

- `Sink::new() -> Self` / `Sink::default()` — ambos vazios.
- `Sink::warn(&mut self, diag: SourceDiagnostic)` — acumula se
  `(span, message)` é novo; ignora se duplicado.
- `Sink::is_empty(&self) -> bool`.
- `Sink::into_diagnostics(self) -> Vec<SourceDiagnostic>` — consome
  e devolve; ordem preservada.

### Dedup

Chave: `(Span, String)` — apenas `span` + `message`. `severity`,
`hints` e `trace` não participam.

### `#[comemo::track]`

O bloco `#[comemo::track] impl Sink {}` é deixado vazio. A API real
vive num `impl Sink` não-tracked. Integração comemo adiada até
`SourceDiagnostic` ser `Hash`-able ou até um wrapper tracked ser
decidido.

## Invariantes

- Sem I/O (pureza L1).
- Ordem preservada: `Vec` mantém insertion order; dedup preserva
  primeiro.
- Dedup estrito: dois warnings idênticos em `(span, message)`
  contam como um.
- `into_diagnostics` consome `self` — Sink não deve ser reutilizado
  após extracção.

## Consumidores

- Pilot consumer pendente (DEBT-49). Este passo não migra.
- `eval()` recebe `_sink: TrackedMut<Sink>` desde Passo 12 — ainda
  não propagado para consumidores internos.

## Referências

- ADR-0042 — formalização.
- ADR-0017 — `Sink` listado como stub inicial.
- ADR-0036 — DEBT-49 propagará `sink: &mut Sink` pelas eval_*.
