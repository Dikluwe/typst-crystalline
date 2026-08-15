# Relatório de Execução — Passo 1051

**Data**: 2026-08-14
**Passo**: 1051 — Corrigir `Ascent`/`Descent` divergente no `FontDescriptor` exportado
**Gate**: `ADR-0127` (Classificação: Metadado Estrutural de PDF / Fluxo Contínuo de Paridade — sem alteração de posição visual)
**Status**: CONCLUÍDO COM ÊXITO (5.930/5.930 testes aprovados, paridade exata de `FontDescriptor` contra Vanilla Typst)

---

## 1. Sumário Executivo e Diagnóstico de Causa-Raiz

Durante o Passo 1050, a inspeção direta dos streams de PDF revelou que o `FontDescriptor` do Crystalline emitia valores hardcoded genéricos (`/Ascent 800 /Descent -200 /CapHeight 700 /FontBBox [-1000 -200 2000 900]`), enquanto o Vanilla Typst emitia as métricas tipográficas reais da fonte (`/Ascent 894 /Descent -246 /CapHeight 658 / 645`).

A investigação isolou a causa-raiz em `03_infra/src/export/builder.rs`:
- No caminho de exportação multi-font (`build_paged_multi_font`, linha 1580), o dicionário `/FontDescriptor` era emitido com uma string formatada contendo valores fixos hardcoded, ignorando a função existente `font_descriptor_metrics(face)`.
- No caminho single-font (`build_paged_single_font`), `subset_face` era parseado a partir dos bytes já recortados pelo subsetter, onde a tabela `OS/2` podia estar ausente ou truncada, fazendo com que `cap_height` recaísse para fallback.

---

## 2. Modificações Implementadas

1. **`03_infra/src/export/builder.rs`**:
   - Em `build_paged_multi_font`, substituído o bloco hardcoded pela extração das métricas da face original (`faces.get(fi)`):
     - `let fd = face_to_use.map(font_descriptor_metrics).unwrap_or_else(...)`.
     - `/FontBBox [{:.5} {:.5} {:.5} {:.5}] /ItalicAngle {:.5} /Ascent {:.5} /Descent {:.5} /CapHeight {:.5}`.
   - Em `build_paged_single_font`, garantido que `font_descriptor_metrics` consulta primeiro a face completa (`ttf_parser::Face::parse(font_data, 0)`), preservando a leitura correta das tabelas `OS/2` e `hhea`.

2. **Snapshot de Referência PDF**:
   - Atualizado o snapshot binário de `03_infra/fixtures/p307b/reference/09-cidfont.pdf` para refletir o novo dicionário `/FontDescriptor` com métricas tipográficas reais.

---

## 3. Verificação Empírica Direta (Vanilla Typst vs Crystalline)

Documento: `sample4_grid.typ` (Libertinus Serif Bold e Regular):

### Dicionários `/FontDescriptor` Extraídos do PDF:

- **Fonte Regular (`LibertinusSerif-Regular`)**:
  - **Vanilla**: `<</Type/FontDescriptor/FontName/.../Ascent 894/Descent -246/CapHeight 658/ItalicAngle 0...>>`
  - **Crystalline**: `<< /Type /FontDescriptor /FontName /... /Ascent 894.00000 /Descent -246.00000 /CapHeight 658.00000 /ItalicAngle 0.00000 ... >>`
  - **Paridade**: $\mathbf{\Delta Ascent = 0.00}$, $\mathbf{\Delta Descent = 0.00}$, $\mathbf{\Delta CapHeight = 0.00}$.

- **Fonte Bold (`LibertinusSerif-Bold`)**:
  - **Vanilla**: `<</Type/FontDescriptor/FontName/.../Ascent 894/Descent -246/CapHeight 645/ItalicAngle 0...>>`
  - **Crystalline**: `<< /Type /FontDescriptor /FontName /... /Ascent 894.00000 /Descent -246.00000 /CapHeight 645.00000 /ItalicAngle 0.00000 ... >>`
  - **Paridade**: $\mathbf{\Delta Ascent = 0.00}$, $\mathbf{\Delta Descent = 0.00}$, $\mathbf{\Delta CapHeight = 0.00}$.

### Bounding Boxes Reconstruídos (`pdftotext -bbox-layout`):

| Métrica | Vanilla | Crystalline | Delta |
| :--- | :---: | :---: | :---: |
| **$\Delta h$ (altura de célula em todas as 9 células)** | 12.54 pt | 12.54 pt | **`0.0000 pt`** |
| **$\Delta w$ (largura de célula em todas as 9 células)** | — | — | **`0.0000 pt`** |
| **$\Delta x$ (posição horizontal em todas as 9 células)** | — | — | **`0.00 pt`** |

---

## 4. Testes e Não-Regressão

- `cargo test --workspace`: **5.930 testes executados, 5.930 aprovados (100% PASS)**.
- `crystalline-lint .`: **0 erros**.
