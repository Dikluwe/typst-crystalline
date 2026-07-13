# Prompt L0 — `pdf` — módulo de funcionalidade específica de PDF
Hash do Código: 42709f26

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/pdf.rs`, `01_core/src/rules/eval/mod.rs`
**Origem**: Passo 735 — namespace `pdf` ausente no cristalino ("unknown variable"); vanilla expõe como `module` (medido em P731/P735).
**ADRs**: ADR-0107 (paridade linguagem vs mecânica), ADR-0108 (medir antes de decidir), ADR-0029 (pureza L1).

---

## 1. Contexto e medições

O vanilla expõe `pdf` como módulo (`type(pdf)` → `module`, medido), definido em `lab/typst-original/crates/typst-library/src/lib.rs:347` + `pdf/mod.rs`. Conteúdo no binário medido:

- `pdf.attach` → `function` (medido) — `AttachElem`, embute ficheiros no PDF.
- `pdf.artifact` → `function` (medido) — `ArtifactElem`, marca conteúdo como artefacto decorativo (só afecta tagging/a11y; o render é inalterado).
- `table_summary`, `header_cell`, `data_cell` — gated em `Feature::A11yExtras`, ausentes do binário medido.

Decisão registada: o exportador PDF cristalino não suporta embedding nem tagging (scope-out global). Implementar o módulo com as duas funções, com o comportamento honesto máximo dentro desse limite:

- `pdf.attach(...)` → **erro de scope-out explícito** ("o exportador PDF cristalino não suporta ficheiros embutidos — scope-out"). Não há aproximação honesta: o efeito do vanilla é alterar o ficheiro PDF produzido.
- `pdf.artifact(body)` → **passthrough do `body`** (primeiro argumento posicional): render pixel-idêntico ao vanilla (o artefacto só altera tagging; o cristalino não produz tagging — registado). Silenciar seria errado se houvesse tagging; não havendo, o observável (pixels) é paridade.

## 2. Funções

```rust
// 01_core/src/rules/stdlib/pdf.rs
pub fn make_pdf_module() -> Value;   // Value::Module("pdf", scope) com as 2 funções
fn native_pdf_attach(...) -> SourceResult<Value>;   // Err scope-out
fn native_pdf_artifact(...) -> SourceResult<Value>; // Ok(body)
```

## 3. Registo no scope

```rust
// 01_core/src/rules/eval/mod.rs
scope.define("pdf", make_pdf_module());
```

## 4. Critérios de verificação

- `type(pdf)` → `module`; `type(pdf.attach)`/`type(pdf.artifact)` → `function`.
- `pdf.attach(...)` → erro com a mensagem de scope-out.
- `pdf.artifact[conteúdo]` → render idêntico ao conteúdo sem o wrapper.
- `cargo test --workspace` verde; `crystalline-lint .` limpo.
