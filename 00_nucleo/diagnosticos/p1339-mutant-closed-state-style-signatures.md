# P1339 — assinaturas baseline de cadeia, componente do lote 1

Executor `/root/p1336_tests`; preparação declarativa, sem execução nem expectativa.
Fonte `01_core/src/entities/style_chain.rs`, SHA-256
`02a1a8bb1016bf33a384f841026d70f0d7306ed30b4275204728c990bfacbd19`.
As declarações integrais dos campos já pertencem ao inventário ancestral.
As assinaturas abaixo são existentes; não constituem novas APIs.

```rust
// crate::entities::style_chain::StyleDelta
pub struct StyleDelta {
    // demais campos no inventário, não omitidos do contrato
    pub size: Option<f64>, // pontos tipográficos, linha 53
}
impl StyleDelta {
    pub const fn empty() -> Self;
}

// crate::entities::style_chain::StyleChain
pub struct StyleChain(Option<Arc<StyleNode>>); // linha 291
impl StyleChain {
    pub const fn empty() -> Self;                       // linha 295
    pub fn default_chain() -> Self;                     // linha 304
    pub fn push(&self, delta: StyleDelta) -> Self;       // linha 319
    pub fn push_styles(&self, styles: &Styles) -> Self;  // linha 331
    pub fn collapse(&self) -> StyleDelta;               // linha 354
    pub fn size(&self) -> f64;                          // linha 487
}
```

Construção declarativa disponível: `StyleDelta { size: Some(valor),
..StyleDelta::empty() }`, seguida de `cadeia.push(delta)`. Não existe necessidade
de inventar `apply`: `push` retorna outra `StyleChain`; atribuição ao campo
`Engine.styles`/cadeia do teste usa a assinatura real já publicada. Este documento
não escolhe valores, sequência de leituras, resultado ou regra de comparação.
