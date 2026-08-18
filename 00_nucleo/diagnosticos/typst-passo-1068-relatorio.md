# Relatório de Execução — Passo 1068

**Data**: 2026-08-17
**Passo**: 1068 — Expansão dos Módulos de Constantes por Domínio (`export/` e `stdlib/text/`)
**Gate**: `ADR-0127` (Auditoria e Desenho Arquitetural — Sem alterações de código neste passo)
**Status**: CONCLUÍDO (Auditoria do piloto concluída na Parte 0; levantamento empírico na Parte 1; proposta de desenho formalizada na Parte 2)

---

## 1. Parte 0 — Análise do Piloto de Referência (`layout/vanilla_defaults.rs`)

O módulo piloto [`01_core/src/compiler/layout/vanilla_defaults.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/vanilla_defaults.rs), criado no Passo 1058 sob o prompt L0 `00_nucleo/prompts/compiler/layout/vanilla_defaults.md`, estabeleceu os seguintes critérios canônicos:
1. **Fan-in e Coesão**: Consolida constantes que possuem **múltiplos consumidores dentro do mesmo domínio** (no piloto, 8 arquivos de `layout/` consumiam `PAR_LEADING`, `PAR_SPACING` e `BLOCK_SPACING`), eliminando literais mágicos espalhados.
2. **Isolamento de Domínio (Prevenção de Hubs / ADR-0104)**: O módulo não é um repositório global; é 100% circunscrito ao seu domínio (`layout/`), sem imports cruzados em outras camadas.
3. **Padrão de Rastreabilidade e Documentação**: Cada constante possui citação de proveniência (`ref: ...`) com caminho relativo a `lab/typst-original/` e documentação de seu significado tipográfico/geométrico.

---

## 2. Parte 1 — Levantamento de Constantes nos Dois Domínios

### 2.1 Domínio 1: `01_core/src/compiler/stdlib/text/` (Camada L1)
* **Arquivos Inspecionados**: `case.rs`, `constructor.rs`, `deco.rs`, `lorem.rs`, `shift.rs`, `smallcaps.rs`, `smartquote.rs`, `mod.rs`.
* **Constantes Numéricas de Design Hardcoded**: **0 encontradas**.
* **Diagnóstico**: As definições de texto (`size: 11pt`, `font: "Linux Libertine"`, `dir: ltr`, `weight: 400`, etc.) são estruturadas via `TextStyle` / `TextElem` e resolvidas dinamicamente via engine/estilos (`entities/layout_types.rs`). Não há escalares dispersos em arquivos individuais de `stdlib/text/`.
* **Veredicto para `stdlib/text/`**: **Não aplicável**. A criação de um `stdlib/text/vanilla_defaults.rs` resultaria em um módulo vazio ou artificial, violando o princípio de necessidade técnica.

### 2.2 Domínio 2: `03_infra/src/export/` (Camada L3)
* **Arquivos Inspecionados**: `builder.rs`, `stream.rs`, `svg.rs`, `images.rs`, `fonts.rs`, `gradients/`.
* **Candidatos Identificados com Múltiplos Consumidores (Fan-in Real)**:
  1. **`KAPPA = 0.552_284_749_831`**: Constante de aproximação de círculo/elipse por Bézier cúbica ($4(\sqrt{2}-1)/3$).
     - *Locais atuais*: Duplicada em `stream.rs:1245` (`const KAPPA`), `stream.rs:1458` (`const K`) e `stream.rs:1597` (`const KAPPA`).
     - *Proveniência*: Padrão da Computação Gráfica / ISO 32000-1 (PDF Reference §4.4).
  2. **`FAUX_BOLD_OFFSET_RATIO = 0.04`**: Fator de deslocamento de stroke para emulação de negrito sintético (4% do `font_size`).
     - *Locais atuais*: Duplicada em `stream.rs:242` e `stream.rs:397` (`const FAUX_BOLD_K: f64 = 0.04`).
     - *Proveniência*: `lab/typst-original/crates/typst-pdf/src/text.rs` / especificação de renderização do Typst.
  3. **`A4_WIDTH = 595.28` e `A4_HEIGHT = 841.89`**: Dimensões de fallback de página A4 em pontos tipográficos ($72\text{ DPI}$).
     - *Locais atuais*: `builder.rs:658-659` (`595.28, 841.89`) e aproximações truncadas em `builder.rs:1682, 2042, 2113` (`595.0, 842.0`).
     - *Proveniência*: Norma ISO 216 / Adobe PostScript & PDF Paper Sizes.
  4. **`DEFAULT_FONT_DESCRIPTOR_METRICS`**: Metadados de fallback de `/FontDescriptor` para PDF/A (`bbox: [-1000.0, -200.0, 2000.0, 900.0]`, `ascent: 800.0`, `descent: -200.0`, `cap_height: 700.0`).
     - *Locais atuais*: Bloco duplicado identicamente em `builder.rs:1090-1094` e `builder.rs:1578-1582`.
     - *Proveniência*: ISO 19005-1 / PDF Reference §5.7.

---

## 3. Parte 2 — Desenho Proposto para o Domínio `export/` (Passo 1069)

### 3.1 Localização e Nomenclatura
- **Novo Arquivo**: `03_infra/src/export/pdf_defaults.rs` (ou `vanilla_defaults.rs`)
- **Camada**: **L3** (`typst_infra::export`) — Estritamente restrito a `03_infra/src/export/`, sem vazamento para L1/L2 (em total conformidade com `ADR-0004` e `ADR-0104`).
- **Prompt L0 Associado**: `00_nucleo/prompts/infra/export/pdf_defaults.md`

### 3.2 Estrutura do Módulo `pdf_defaults.rs` Proposta:
```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/pdf_defaults.md
//! @prompt-hash <hash>
//! @layer L3
//! @updated 2026-08-17
//!
//! Constantes canónicas de serialização PDF e padrões geométricos de exportação (Passo 1069).

/// Constante mágica para aproximação de arcos circulares e elípticos por curvas Bézier cúbicas: 4 * (sqrt(2) - 1) / 3.
/// ref: ISO 32000-1:2008 (PDF 1.7 Spec) §4.4 / Adobe PostScript Language Reference Manual
pub const BEZIER_CIRCLE_KAPPA: f64 = 0.552_284_749_831;

/// Proporção do tamanho da fonte utilizada para o offset de traço em negrito sintético (faux bold).
/// ref: lab/typst-original/crates/typst-pdf/src/text.rs
pub const FAUX_BOLD_OFFSET_RATIO: f64 = 0.04;

/// Largura canónica de página A4 em pontos tipográficos (72 pt/in, 210mm).
/// ref: ISO 216 / Adobe PostScript Paper Sizes
pub const A4_DEFAULT_WIDTH: f64 = 595.28;

/// Altura canónica de página A4 em pontos tipográficos (72 pt/in, 297mm).
/// ref: ISO 216 / Adobe PostScript Paper Sizes
pub const A4_DEFAULT_HEIGHT: f64 = 841.89;
```

---

## 4. Conclusão e Recomendação para o Passo 1069

1. **`stdlib/text/`**: Dispensado de módulo de constantes por inexistência de literais hardcoded dispersos (zero overhead desnecessário).
2. **`export/`**: Recomendada a criação de `03_infra/src/export/pdf_defaults.rs` no Passo 1069, migrando as duplicações de `KAPPA`, `FAUX_BOLD_K` e dimensões de fallback de A4 em `stream.rs` e `builder.rs`.
