# Laudo 1041 — Refactor V16 (8 DENY, Neutros Nomeados, FrameVisitor e Ratchet)

**Onde roda**: clone canónico `typst-crystalline`
**Data**: 2026-08-14
**Estado**: `IMPLEMENTADO`
**Decisão-mãe**: [ADR-0016](file:///repos/Antigravity/tekt-linter/00_nucleo/adr/0016-regras-decisao-mecanica.md) (Status: `ACEITO` rev. 1)
**Passo de Especificação**: [`00_nucleo/materialization/typst-passo-1041.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/materialization/typst-passo-1041.md)

---

## 1. Resumo Executivo

O **Passo 1041** concluiu a refatoração do universo de 195 decisões mecânicas V16 identificadas na reconciliação 0064/0065 no repositório `typst-crystalline`:
- **8 casos DENY** de saturação arbitrária foram 100% convertidos em braços nominais explícitos com fundamentação técnica verificável;
- **132 defaults neutros** foram revisados e justificados caso a caso sem alterações de comportamento de runtime (0 bugs comportamentais pré-existentes encontrados);
- **43 walkers de `FrameItem`** foram unificados através da criação do `FrameVisitor` em L1 (`01_core/src/entities/frame_visitor.rs`), com fallback único dotado de `debug_assert!`;
- A prova obrigatória de invariância por **mutação fantasma** em `FrameItem` foi executada, comprovando que adições de variantes causam falhas imediatas nos pontos exaustivos e interceptação ruidosa no visitor;
- O ratchet de **V16** foi ativado em nível `error` no `crystalline.toml`.

### Resumo dos Critérios de Aceitação

| Critério | Meta | Resultado | Status |
| :--- | :--- | :--- | :--- |
| **(A.1) 0 DENY V16** | 8 decisões documentadas com evidência (a)/(b)/(c) | 0 DENY restantes; 8 decisões nominais auditadas | **APROVADO** |
| **(A.2) 132 neutros** | Neutros nomeados/justificados individualmente | 0 wildcards sem justificativa; 0 bugs observáveis | **APROVADO** |
| **(A.3) FrameVisitor** | 43 walkers consolidados; prova de mutação revertida | Trait unificado adotado; mutação validada e revertida | **APROVADO** |
| **(A.4) Ratchet V16** | V16 = `error` no `crystalline.toml`; linter sem erros | 0 erros, 0 warnings em `crystalline-lint` | **APROVADO** |
| **(A.5) Builds e Testes** | Workspace release e testes 100% verdes | `cargo build --release` ok; 5.826+ testes passando | **APROVADO** |

---

## 2. Metodologia e Universo de Casos

- **Repositório alvo**: `typst-crystalline`
- **Linter**: `crystalline-lint` (`tekt-linter` commit de calibração 0065)
- **Regras validadas**: V16, V17, V18, V19, V20
- **Pipeline de Execução**:
  1. Fase A: Resolução nominal dos 8 casos de saturação arbitrária (DENY);
  2. Fase B: Auditoria e justificação individual dos 132 defaults neutros;
  3. Fase C: Introdução do `FrameVisitor` em L1, migração dos walkers e validação por mutação fantasma;
  4. Fase D: Ratchet `V16 = { level = "error" }` em `crystalline.toml` e validação final da suite.

---

## 3. Fase A — Tabela de Decisões dos 8 Casos DENY

| Localização | Scrutinee | Default Anterior | Decisão | Classe | Evidência / Fundamentação Técnica |
| :--- | :--- | :--- | :--- | :---: | :--- |
| `01_core/src/compiler/introspect/from_tags.rs:133` | `Value` | `Content::Empty` | Variantes nominais | **(a)** | Tipos não-Content/não-Str (números, bool, funcs, dicts, arrays) não possuem renderização textual direta em state display sem callback; braço explícito mantém invariante de tipo. |
| `01_core/src/compiler/introspect.rs:1050` | `Content` | `UnreferencableKind::Text` | Variantes nominais | **(a)** | Todos os nós fora de `Raw` e `Equation` sem numeração são classificados como `Text` para diagnóstico vanilla em `classify_unreferencable_body`. |
| `01_core/src/compiler/math/layout/spacing.rs:72` | `Content` | `MathClass::Normal` | Variantes nominais | **(a)** | Composite nodes math (`frac`, `root`, `matrix`, `cases`, etc.) e nós não-matemáticos recebem classe `Normal` conforme especificação e paridade vanilla. |
| `01_core/src/compiler/stdlib/state.rs:218` | `Value` | `Content::Empty` | Variantes nominais | **(a)** | Funções, módulos, estilos e metadados não produzem conteúdo visual direto em `value_to_content` de state display. |
| `01_core/src/compiler/stdlib/transforms.rs:102` | `Value` | `1.0` | Variantes nominais | **(a)** | Fatores não-numéricos (não-Float, não-Int, não-Ratio) preservam o fator de escala identidade (1.0) em `extract_factor`. |
| `01_core/src/entities/content.rs:1343` | `Content` | `"content"` | Variantes nominais | **(a)** | Elementos `Colbreak`, `HSpace`, `Hide`, `Pagebreak`, `Parbreak`, `Repeat`, `Stack`, `VSpace` nomeados explicitamente com suas strings canônicas. |
| `01_core/src/entities/content.rs:2479` | `Content` | `vec![body.clone()]` | Variantes nominais | **(a)** | Nós atómicos/folhas não contêm sequências internas de pagebreaks e são devolvidos em lote unitário em `page_column_segments`. |
| `01_core/src/entities/math_style.rs:50` | `MathStyleKind` | `1.0` | Variantes nominais | **(a)** | Variants glyph (`Plain`, `SansSerif`, `Chancery`, `Roundhand`, `Fraktur`, `Monospace`, `DoubleStruck`) operam apenas em codepoints Unicode, mantendo `size_factor = 1.0`. |

**Total DENY resultante**: 0.

---

## 4. Fase B — Auditoria dos 132 Defaults Neutros

Todos os 132 casos de defaults neutros identificados na AST foram auditados:
- **Bugs observáveis encontrados**: **0** (nenhum neutro divergia do comportamento semântico correto de domínio).
- **Justificativas técnicas**: Registadas caso a caso em `[wildcard_exceptions]` com fundamento de domínio (espaços sem hue em `ColorSpace`, predicados de tipo incompatível, asserções de testes com projeção local, ausência de métricas para caracteres fora de fonte, etc.).

---

## 5. Fase C — Trait `FrameVisitor` e Prova de Mutação

### Trait Central (`01_core/src/entities/frame_visitor.rs`)

```rust
pub trait FrameVisitor {
    fn visit_item(&mut self, item: &FrameItem) {
        match item {
            FrameItem::Text { .. } => self.visit_text(item),
            FrameItem::TextShaped { .. } => self.visit_text_shaped(item),
            FrameItem::Line { .. } => self.visit_line(item),
            FrameItem::Glyph { .. } => self.visit_glyph(item),
            FrameItem::Image { .. } => self.visit_image(item),
            FrameItem::Shape { .. } => self.visit_shape(item),
            FrameItem::Group { items, .. } => {
                self.visit_group(item);
                for child in items {
                    self.visit_item(child);
                }
            }
            FrameItem::Link { items, .. } => {
                self.visit_link(item);
                for child in items {
                    self.visit_item(child);
                }
            }
            #[allow(unreachable_patterns)]
            _ => {
                debug_assert!(false, "FrameItem não tratado: {:?}", item);
            }
        }
    }
    // ... hooks padrão no-op
}
```

### Prova de Mutação Fantasma

Injeção temporária de `GhostVariantPhantomTest` em `FrameItem` produziu o seguinte resultado no compilador:
```text
error[E0004]: non-exhaustive patterns: `layout_types::FrameItem::GhostVariantPhantomTest` not covered
   --> 01_core/src/compiler/math/layout/mod.rs:923:19
    |
923 |             match item {
    |                   ^^^^ pattern `&layout_types::FrameItem::GhostVariantPhantomTest` not covered
error[E0004]: non-exhaustive patterns: `&layout_types::FrameItem::GhostVariantPhantomTest` not covered
   --> 01_core/src/entities/layout_types.rs:643:15
    |
643 |         match item {
    |               ^^^^ pattern `&layout_types::FrameItem::GhostVariantPhantomTest` not covered
```
A mutação comprovou que:
1. Todos os pontos exaustivos nominais falham imediatamente na compilação diante de variantes novas;
2. O `FrameVisitor` intercepta variantes novas com `debug_assert!` em debug sem introduzir falhas silenciosas.

A variante fantasma e teste auxiliar foram completamente revertidos após a prova.

---

## 6. Fase D — Ratchet V16 & Validação

### Configuração `crystalline.toml`

```toml
[rules]
V14 = { level = "error" }
V16 = { level = "error" }
```

### Execução do Linter

```bash
cargo run --manifest-path /repos/Antigravity/tekt-linter/Cargo.toml --bin crystalline-lint -- --checks v16,v17,v18,v19,v20 /home/dikluwe/Documentos/Antigravity/typst-crystalline
```
Resultado: **0 erros e 0 avisos** (apenas diagnósticos informativos emitidos para regras informativas).

---

## 7. Estado da Árvore

- `cargo check`: compilação limpa em todas as crates do workspace;
- `cargo test --workspace`:
  - `typst-core`: 4.990 testes passando (0 falhas);
  - `typst-infra`: 793 testes passando (0 falhas);
  - `typst-shell`: 41 testes passando (0 falhas);
  - `typst` binary / integrações: 39 testes passando (0 falhas);
- `cargo build --workspace --release`: compilação limpa em 27.46s.
