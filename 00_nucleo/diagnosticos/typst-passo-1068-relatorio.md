# Relatório de Execução — Passo 1068 (Decisão Final e Desenho Arquitetural)

**Data**: 2026-08-17
**Passo**: 1068 — Expansão dos Módulos de Constantes por Domínio (`export/` e `stdlib/text/`)
**Gate**: `ADR-0127` (Auditoria e Desenho Arquitetural — Sem alterações de código neste passo)
**Status**: CONCLUÍDO (Decisão final de constantes nominais completas para o P1069, sem números soltos e sem alteração de comportamento numérico)

---

## 1. Parte 0 — Análise do Piloto de Referência (`layout/vanilla_defaults.rs`)

O módulo piloto [`01_core/src/compiler/layout/vanilla_defaults.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/vanilla_defaults.rs) estabeleceu 3 regras canônicas:
1. **Fan-in Real**: Consolidação de valores com múltiplos consumidores no mesmo domínio (8 arquivos em `layout/`).
2. **Prevenção de Hubs (ADR-0104)**: Módulo restrito à sua camada e domínio sem imports cruzados.
3. **Proveniência Obrigatória**: Cada constante possui citação de proveniência (`ref: ...`) com caminho relativo a `lab/typst-original/`.

---

## 2. Parte 1 — Levantamento e Auditoria dos Candidatos

### 2.1 Domínio `01_core/src/compiler/stdlib/text/` (Camada L1)
* **Auditoria**: **0 constantes numéricas dispersas encontradas**. Os estilos de texto são gerenciados estruturalmente via tipos do motor (`TextStyle` / `TextElem`).
* **Conclusão**: **Dispensado** da criação de módulo para evitar arquivos vazios e sem propósito técnico.

### 2.2 Domínio `03_infra/src/export/` (Camada L3) — Decisões de Design:

1. **`KAPPA = 0.552_284_749_831`**:
   * *Locais*: `stream.rs:1245` (`const KAPPA`), `stream.rs:1458` (`const K`), `stream.rs:1597` (`const KAPPA`).
   * *Decisão*: Consolidar como `pub const KAPPA: f64 = 0.552_284_749_831;` em `pdf_defaults.rs`.
2. **`FAUX_BOLD_K = 0.04`**:
   * *Locais*: `stream.rs:242` e `stream.rs:397`.
   * *Decisão*: Consolidar como `pub const FAUX_BOLD_K: f64 = 0.04;` em `pdf_defaults.rs`.
3. **Dimensões A4 Precisas vs Arredondadas (Sem Literais Mágicos Soltos)**:
   * *Decisão*: Criar duas duplas de constantes nominais em `pdf_defaults.rs` para preservar 100% da paridade numérica sem deixar literais soltos:
     - `pub const A4_DEFAULT_WIDTH: f64 = 595.28;` (dimensão exata de A4, usada em `builder.rs:658`).
     - `pub const A4_DEFAULT_HEIGHT: f64 = 841.89;` (dimensão exata de A4, usada em `builder.rs:659`).
     - `pub const A4_FALLBACK_WIDTH_ROUNDED: f64 = 595.0;` (fallback inteiro de emergência, usado em `builder.rs:1682`).
     - `pub const A4_FALLBACK_HEIGHT_ROUNDED: f64 = 842.0;` (fallback inteiro de emergência para conversão $Y$, usado em `builder.rs:1682, 2042, 2113`).
4. **`FontDescriptorMetrics` (Tipo Composto / Struct)**:
   * *Decisão*: Implementar `impl Default for FontDescriptorMetrics` em `03_infra/src/export/fonts.rs`, substituindo os blocos duplicados de `builder.rs:1090` e `builder.rs:1578` por `.unwrap_or_default()`.

---

## 3. Parte 2 — Desenho Arquitetural do Módulo `pdf_defaults.rs` (Passo 1069)

### 3.1 Nomenclatura e Localização
* **Arquivo**: `03_infra/src/export/pdf_defaults.rs`
* **Camada**: **L3** (`typst_infra::export`) — Estritamente restrito a `03_infra/src/export/`.
* **Prompt L0 Associado**: `00_nucleo/prompts/infra/export/pdf_defaults.md`.

### 3.2 Código Proposto para `pdf_defaults.rs`:
```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/pdf_defaults.md
//! @prompt-hash <hash>
//! @layer L3
//! @updated 2026-08-17
//!
//! Constantes canónicas de serialização PDF e padrões geométricos de exportação (Passo 1069).

/// Constante para aproximação de arcos circulares e elípticos por curvas Bézier cúbicas: 4 * (sqrt(2) - 1) / 3.
/// ref: ISO 32000-1:2008 (PDF 1.7 Spec) §4.4 / Adobe PostScript Language Reference Manual
pub const KAPPA: f64 = 0.552_284_749_831;

/// Proporção do tamanho da fonte utilizada para o offset de traço em negrito sintético (faux bold).
/// ref: lab/typst-original/crates/typst-pdf/src/text.rs
pub const FAUX_BOLD_K: f64 = 0.04;

/// Largura canónica precisa de página A4 em pontos tipográficos (72 pt/in, 210mm).
/// ref: ISO 216 / Adobe PostScript Paper Sizes
pub const A4_DEFAULT_WIDTH: f64 = 595.28;

/// Altura canónica precisa de página A4 em pontos tipográficos (72 pt/in, 297mm).
/// ref: ISO 216 / Adobe PostScript Paper Sizes
pub const A4_DEFAULT_HEIGHT: f64 = 841.89;

/// Largura arredondada de página A4 utilizada em rotas de fallback de metadados quando dimensões estão ausentes.
/// Preserva compatibilidade numérica exacta com os pontos de unwrap_or existentes.
pub const A4_FALLBACK_WIDTH_ROUNDED: f64 = 595.0;

/// Altura arredondada de página A4 utilizada em rotas de fallback de conversão de coordenadas Y quando páginas estão ausentes.
/// Preserva compatibilidade numérica exacta com os pontos de unwrap_or existentes.
pub const A4_FALLBACK_HEIGHT_ROUNDED: f64 = 842.0;
```

---

## 4. Conclusão e Próximos Passos (P1069)

1. Decisões de design 100% fechadas e sem pontas soltas.
2. Pronto para execução no Passo 1069 sob confirmação do dono.
