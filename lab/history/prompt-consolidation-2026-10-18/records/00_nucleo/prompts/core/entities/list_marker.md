# Prompt L0 — `entities/list_marker` — `ListMarker`
Hash do Código: 2a887cd0

**Camada**: L1 · **Alvo**: `01_core/src/entities/list_marker.rs`
**Origem**: P470 — Marcadores configuráveis de `list`.

---

## Tipo

```rust
use ecow::EcoString;

/// Marcador de item de lista não ordenada. Subset minimal P470:
/// bullet padrão ou string customizada.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ListMarker {
    /// Bullet padrão (`•` U+2022).
    Default,
    /// Marcador customizado (ex: `"→"`, `"-"`, `"*"`).
    Custom(EcoString),
}

impl Default for ListMarker {
    fn default() -> Self { Self::Default }
}

impl ListMarker {
    /// Devolve o marcador como `&str` para render.
    pub fn render(&self) -> &str {
        match self {
            Self::Default    => "•",
            Self::Custom(s)  => s.as_str(),
        }
    }
}
```

## Scope-out explícito (P470)

- Marcadores por nível (`([•], [–], [·])`) — requer infraestrutura de
  nível de aninhamento. Futuro.
- `ListMarker::Content(Content)` — marcador como bloco arbitrário.

## Critério

- `ListMarker::Default.render()` == `"•"`.
- `ListMarker::Custom("→".into()).render()` == `"→"`.
- `Default::default()` == `ListMarker::Default`.
- `PartialEq`: dois `Default` iguais; `Custom("x") != Custom("y")`.

## Default derivado com valor preservado — 2026-10-13

### Medição anterior à decisão

`01_core/src/entities/list_marker.rs:23` contém a implementação manual de Default sinalizada pelo Clippy. O corpo existente retorna a mesma variante ou inicializa cada campo pelo seu default. O tipo não é genérico, portanto a derivação não acrescenta bounds a parâmetros públicos. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; fontes, owners, spans, UTC, status e diffstat estão em `lab/results/construction/diagnostics/expressoes-defaults-ci-2026-10-13/before.json.gz`. É evidência de implementação, não nova medição de paridade ou desempenho.

### Decisão e aceitação

Substituir somente a implementação manual de Default por derive(Default), conservando o trait já implementado e seu valor. ListMarker mantém a variante Default, anotada com #[default], e o bullet correspondente. Preservar Custom, Array e render. Os exemplos anteriores de impl Default explicitam o valor exigido; a derivação equivalente passa a ser a forma desta implementação.

Não alterar assinatura pública, campos, variantes, comportamento padrão do produto ou fase. Correção mecânica interna em fluxo contínuo ADR-0127. Preservar testes e expectativas; validar todos os targets do workspace, suíte core, registro de testes, build, rustdoc, Clippy e linhagem.
