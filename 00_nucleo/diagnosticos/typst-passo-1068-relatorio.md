# Relatório de Execução — Passo 1068 (Revisão Completa e Detalhada)

**Data**: 2026-08-17
**Passo**: 1068 — Expansão dos Módulos de Constantes por Domínio (`export/` e `stdlib/text/`)
**Gate**: `ADR-0127` (Auditoria e Desenho Arquitetural — Sem alterações de código neste passo)
**Status**: CONCLUÍDO (Rastreabilidade completa de candidatos, análise de tipos complexos e isolamento de premissas numéricas para o P1069)

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

### 2.2 Domínio `03_infra/src/export/` (Camada L3) — Análise dos 4 Candidatos:

1. **`KAPPA = 0.552_284_749_831`**:
   * *Locais*: `stream.rs:1245` (`const KAPPA`), `stream.rs:1458` (`const K`), `stream.rs:1597` (`const KAPPA`).
   * *Valores*: **Idênticos** (`0.552_284_749_831`).
   * *Proveniência*: Constante canônica da aproximação de círculo por Bézier cúbica ($4(\sqrt{2}-1)/3$, ISO 32000-1 §4.4).
2. **`FAUX_BOLD_K = 0.04`**:
   * *Locais*: `stream.rs:242` e `stream.rs:397`.
   * *Valores*: **Idênticos** (`0.04`).
   * *Proveniência*: Proporção de stroke para faux bold (`lab/typst-original/crates/typst-pdf/src/text.rs`).
3. **`A4_WIDTH = 595.28` e `A4_HEIGHT = 841.89`**:
   * *Locais com valor exato*: `builder.rs:658-659` (`width: 595.28, height: 841.89`).
   * *Locais com valor inteiro aproximado (fallback)*: `builder.rs:1682` (`unwrap_or((595.0, 842.0))`), `builder.rs:2042` (`unwrap_or(842.0)`), `builder.rs:2113` (`unwrap_or(842.0)`).
   * *Ressalva de Gate*: Os fallbacks inteiros (`595.0, 842.0`) são mantidos inalterados ou submetidos como calibração consciente no P1069 para não haver alteração numérica acidental.
4. **`DEFAULT_FONT_DESCRIPTOR_METRICS`**:
   * *Locais*: Bloco duplicado identicamente em `builder.rs:1090-1094` e `builder.rs:1578-1582`.
   * *Decisão de Desenho*: Trata-se de uma struct de metadados tipográficos (`FontDescriptorMetrics { font_bbox: [-1000.0, -200.0, 2000.0, 900.0], italic_angle: 0.0, ascent: 800.0, descent: -200.0, cap_height: 700.0 }`). Por ser tipo composto interno da infraestrutura de fontes PDF, a recomendação é implementá-la como `impl Default for FontDescriptorMetrics` em `03_infra/src/export/fonts.rs` ou `pdf_defaults.rs`.

---

## 3. Parte 2 — Desenho Arquitetural Proposto para o Passo 1069

### 3.1 Nomenclatura e Localização
* **Arquivo**: `03_infra/src/export/pdf_defaults.rs`
* **Camada**: **L3** (`typst_infra::export`) — Estritamente restrito a `03_infra/src/export/`.
* **Prompt L0 Associado**: `00_nucleo/prompts/infra/export/pdf_defaults.md`.

### 3.2 Estrutura Proposta do Módulo:
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

/// Largura canónica de página A4 em pontos tipográficos (72 pt/in, 210mm).
/// ref: ISO 216 / Adobe PostScript Paper Sizes
pub const A4_DEFAULT_WIDTH: f64 = 595.28;

/// Altura canónica de página A4 em pontos tipográficos (72 pt/in, 297mm).
/// ref: ISO 216 / Adobe PostScript Paper Sizes
pub const A4_DEFAULT_HEIGHT: f64 = 841.89;
```

---

## 4. Conclusão e Próximos Passos (P1069)

1. Auditoria concluída sem nenhuma alteração de código.
2. Proposta formalizada para execução no Passo 1069 sob confirmação do dono.
