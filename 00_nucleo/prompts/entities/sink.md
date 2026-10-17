# Prompt L0 — Sink
Hash do Código: 55084846

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

### Métodos tracked (em `world_types.rs`, Passos 106/107/740A)

O bloco tracked vive em `01_core/src/entities/world_types.rs`
(mantém a assinatura `TrackedMut<Sink>` do `eval()`):

- `warn_note(span, message, hint)` — Passo 106/107; `hint == ""` =
  sem hint (`Option<&str>` não é aceite pela proc-macro do comemo).
- `warn_note2(span, message, hint, hint2)` — **P740A**; variante para
  warnings com dois hints (paridade vanilla
  `warn_for_discarded_content`, que junta um hint extra condicional
  sobre state/counter). Mesma convenção: string vazia = hint ausente.
  Ambos delegam em `record` (dedup partilhado, ADR-0043).

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

## Default derivado com valor preservado — 2026-10-13

### Medição anterior à decisão

`01_core/src/entities/sink.rs:73` contém a implementação manual de Default sinalizada pelo Clippy. O corpo existente retorna a mesma variante ou inicializa cada campo pelo seu default. O tipo não é genérico, portanto a derivação não acrescenta bounds a parâmetros públicos. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; fontes, owners, spans, UTC, status e diffstat estão em `00_nucleo/diagnosticos/expressoes-defaults-ci-2026-10-13/before.json.gz`. É evidência de implementação, não nova medição de paridade ou desempenho.

### Decisão e aceitação

Substituir somente a implementação manual de Default por derive(Default), conservando o trait já implementado e seu valor. Sink mantém diagnostics vazio e seen vazio, cada campo pelo seu Default existente, na mesma ordem. Preservar Clone, deduplicação, ordem e integração comemo. Os exemplos anteriores de impl Default explicitam o valor exigido; a derivação equivalente passa a ser a forma desta implementação.

Não alterar assinatura pública, campos, variantes, comportamento padrão do produto ou fase. Correção mecânica interna em fluxo contínuo ADR-0127. Preservar testes e expectativas; validar todos os targets do workspace, suíte core, registro de testes, build, rustdoc, Clippy e linhagem.
