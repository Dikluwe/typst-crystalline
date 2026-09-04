# P1300 — nota canônica de API pública para P5

**Criada por:** coordenador P7, antes de qualquer patch P6
**Instante:** `2026-09-03T19:58:02,602857232-03:00`
**Motivo:** P5 demonstrou que tratar `html`/`a11y-extras` como Cargo features é
inválido e não tinha, na allowlist inicial, a assinatura pública necessária
para materializar os quatro perfis runtime exigidos pelo L0.

Esta nota transfere somente assinaturas públicas do baseline; não contém lógica
interna, patch candidato, expectativa de teste nem output P6.

Fonte baseline `01_core/src/compiler/eval/mod.rs:305-321`, SHA-256
`cafdcbf690f5ad8020bbe3da4457a759397c29ac091dc1624f33e14f5127c5b4`:

```rust
pub fn eval_expression_with_features(
    world: &dyn World,
    expression: &str,
    features: crate::entities::compiler_features::Features,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>);
```

Fonte baseline `01_core/src/entities/compiler_features.rs:8-41`, SHA-256
`82955e2f3874c6dcae21c39e9b316fb7857e3141d1eead80d0d86f1a6bfcac0e`:

```rust
pub enum Feature { Html, A11yExtras }

pub struct Features { /* campos privados */ }

impl Features {
    pub const fn empty() -> Self;
    pub const fn html() -> Self;
    pub fn enable(&mut self, feature: Feature);
}
```

Construções permitidas para a matriz P5:

```rust
let default = Features::empty();
let html = Features::html();
let mut a11y = Features::empty();
a11y.enable(Feature::A11yExtras);
let mut html_a11y = Features::html();
html_a11y.enable(Feature::A11yExtras);
```

P5 pode consumir esta nota sem abrir `eval/mod.rs`, `compiler_features.rs` ou
qualquer candidato. A ampliação é apenas informacional e deve ser registrada
no receipt como capacidade pública fornecida pelo coordenador; o nível de
atestação permanece `executado sem atestação de isolamento técnico`.
