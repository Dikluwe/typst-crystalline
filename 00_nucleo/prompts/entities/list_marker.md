# Prompt L0 — `entities/list_marker` — `ListMarker`
Hash do Código: (a calcular após implementação)

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
