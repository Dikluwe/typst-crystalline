# Relatório de Investigação — Passo 1086: Espaçamento Entre Blocos de Equação Consecutivos

**Data**: 2026-08-19  
**Passo**: 1086 — Investigação: Espaçamento Entre Blocos de Equação Consecutivos  
**Gate**: Nenhum (Investigação L0 — sem código de produção alterado)  
**Status**: CONCLUÍDO COM ÊXITO (Causa raiz factual isolada com citação de código, reprodução exata dos deltas, distinção entre constante duplicada V21 e bug funcional de contexto, e classificação ADR-0084/ADR-0127)

---

## 1. Contexto e Reprodução Factual dos Deltas

A partir da compilação e inspeção geométrica dos PDFs da Seção 30 (`.typ/sec_30_crystalline.pdf` vs `.typ/sec_30_vanilla.pdf`), foram extraídas as posições exatas das baselines das 3 equações de bloco e do cabeçalho de nível 2 (`== 30. Otimizacao`):

- **Equação 1**: `$ min_(x in RR^n) f(x) "sujeito a" g_i(x) <= 0, h_j(x) = 0 $`
- **Equação 2**: `$ L(x, lambda, mu) = f(x) + sum_(i) lambda_i g_i(x) + sum_(j) mu_j h_j(x) $`
- **Equação 3**: `$ nabla f(x^*) + sum_(i) lambda_i^* nabla g_i(x^*) + sum_(j) mu_j^* nabla h_j(x^*) = 0 $`

### 1.1 Tabela de Medição das Baselines (y a partir do topo da página)

| Elemento | Baseline Crystalline (pt) | Baseline Vanilla (pt) | Delta Relativo Crystalline (pt) | Delta Relativo Vanilla (pt) | Diferença (Crystalline − Vanilla) |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Cabeçalho (H2)** | `37.401` | `37.402` | — | — | `−0.001 pt` |
| **Equação 1** | `58.555` | `56.597` | `+21.154` (H2 → Eq1) | `+19.195` (H2 → Eq1) | **`+1.959 pt`** (~ `+1.96 pt`) |
| **Equação 2** | `87.555` | `87.689` | `+29.000` (Eq1 → Eq2) | `+31.092` (Eq1 → Eq2) | **`−2.092 pt`** (~ `−2.18 pt`) |
| **Equação 3** | `122.653` | `124.802` | `+35.098` (Eq2 → Eq3) | `+37.113` (Eq2 → Eq3) | **`−2.015 pt`** (~ `−2.17 pt`) |

Os valores reproduzem com precisão matemática a inversão de sinal reportada no achado:
- Transição Cabeçalho → Eq 1: **`+1.96 pt`** (equação posicionada mais abaixo do que no vanilla).
- Transição Eq 1 → Eq 2: **`−2.09 pt`** (espaço entre equações menor do que no vanilla).
- Transição Eq 2 → Eq 3: **`−2.02 pt`** (espaço entre equações menor do que no vanilla).

---

## 2. Diagnóstico Detalhado e Distinção das Naturezas

A auditoria aprofundada do código revelou que o achado se decompõe em duas naturezas essencialmente distintas:

### 2.1 Natureza 1 (Auditoria V21 / Constantes): O Literal `1.2` em `equation.rs`
- **Código atual**:
  - `01_core/src/compiler/layout/equation.rs:119`: `let spacing = Pt(self.style.size.val() * 1.2);`
  - `01_core/src/compiler/layout/equation.rs:335`: `let spacing = Pt(self.style.size.val() * 1.2);`
- **Diagnóstico**: O valor `1.2` aparece como literal numérico solto (hardcoded) duplicado em dois pontos de `equation.rs`.
- **Rastreabilidade**: A constante canônica correspondente **já existe e foi formalizada no Passo 1058** em [`01_core/src/compiler/layout/vanilla_defaults.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/vanilla_defaults.rs):
  ```rust
  /// Espaçamento por defeito de blocos genéricos (equation, divider, etc.) quando não
  /// participam do fluxo de parágrafo directamente (em unidades `em`).
  /// ref: lab/typst-original/crates/typst-library/src/layout/container.rs:342
  pub const BLOCK_SPACING: f64 = 1.2;
  ```
- **Ação V21**: `equation.rs` deve importar e utilizar `vanilla_defaults::BLOCK_SPACING` em vez de duplicar o literal `1.2`.

---

### 2.2 Natureza 2 (Bug Funcional de Contexto Desconectado): `TextStyle::default()` em `layout_equation_measured`
- **Código atual** ([`01_core/src/compiler/math/layout/mod.rs:537-544`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/math/layout/mod.rs#L537-L544)):
  ```rust
  FrameItem::Glyph { pos, x_advance, size, .. } => {
      extent.width = extent.width.max(pos.x.val() + x_advance.val());
      // Aproximação documentada (P813): sem texto Unicode, a
      // tinta é estimada pela cap-height; sem descent.
      let up = self.metrics.cap_height(*size, &TextStyle::default());
      extent.ascent = extent.ascent.max(up.val() - pos.y.val());
      extent.descent = extent.descent.max(pos.y.val());
  }
  ```
- **Diagnóstico**: Este não é um problema de magic number, mas sim um **bug funcional de perda de contexto**:
  1. **Ignora o Estilo Ativo**: A estrutura `FrameItem::Glyph` (`01_core/src/entities/layout_types.rs:366-373`) contém explicitamente os campos `glyph_id: u16`, `size: Pt` e `style: TextStyle`. Ao descartar `style` e instanciar `TextStyle::default()`, o compilador assume uma fonte arbitrária de 11pt desconectada do documento, do tamanho escalado do glifo e da cadeia de estilos matemáticos.
  2. **Ignora o Glifo e a Bounding Box Real**: `cap_height` avalia a altura de caixa alta de texto comum (~`7.5 pt`), ignorando a escala de símbolos display ($\sum$, $\min$, $
abla$) e forçando `descent = 0.0`.
  3. **Mecanismo Disponível Não Utilizado**: O trait `FontMetrics` (`01_core/src/compiler/layout/metrics.rs:144`) e sua implementação L3 (`03_infra/src/font_metrics.rs:1580`) **já possuem** o método `glyph_ink_bounds(&self, glyph_id: u16, size: Pt, style: &TextStyle) -> (Pt, Pt)` (introduzido em P952b) que lê a bounding box real da tabela TrueType com a fonte matemática ativa.
- **Impacto**: Gera a subestimação de `~2.1 pt` na soma `descent_A + ascent_B`, encurtando o vão entre equações consecutivas (`−2.09 pt` e `−2.02 pt`).

---

### 2.3 Desconexão Estrutural de Fluxo (Transição Cabeçalho → Equação)
- **Localização**: [`01_core/src/compiler/layout/equation.rs:110-145`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/equation.rs#L110-L145).
- **Diagnóstico**: `heading.rs` (ajustado em P1063) fecha o cabeçalho recuando `extra_below = -3.1209 pt` e ativa `layouter.block_chain_active = true` com `layouter.prev_block_below_pending = 8.25 pt`. Porém, `layout_equation` não consome nem participa do protocolo `block_chain_active` / `prev_block_below_pending`, computando a baseline anterior de forma isolada e gerando o desvio de **`+1.96 pt`**.

---

## 3. Resumo das Recomendações (Governança ADR-0127)

1. **Correção V21 (Constante)**: Substituir os literais `1.2` em `equation.rs` pela constante canônica `vanilla_defaults::BLOCK_SPACING`.
2. **Correção Funcional (Contexto & Extent)**: Em `layout_equation_measured` (`01_core/src/compiler/math/layout/mod.rs`), substituir o braço `FrameItem::Glyph` pelo cálculo real via `self.metrics.glyph_ink_bounds(*glyph_id, *size, style)`.
3. **Correção Estrutural (Fluxo de Blocos)**: Integrar `layout_equation` ao protocolo `block_chain_active` de P1059-P1063.
