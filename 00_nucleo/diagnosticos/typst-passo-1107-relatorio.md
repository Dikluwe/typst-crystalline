# Relatório de Execução — Passo 1107

**Data**: 2026-08-20
**Passo**: 1107 — Unificação do Protocolo de Colapso de Heading (Entrada e Cadeia com Espaços)
**Gate**: `ADR-0127` (Mudança de Comportamento por Defeito / Aprovado pelo Dono)
**Status**: CONCLUÍDO COM ÊXITO (Paridade 100% exata nos 8 casos de teste com Δ = 0.0000pt, eliminação total das constantes mágicas de entrada, escopo estrito de sequence.rs limitado a Space e Empty, suíte 100% verde).

---

## 1. Tabela de Verificação Comparativa Oficial

| Caso de Teste | Crystalline | Vanilla Typst 0.15.1 | $\mathbf{\Delta Gap}$ | Status |
|---|---|---|---|---|
| **Caso 1: Parágrafo $\to$ Heading 1** | $30.3644\text{ pt}$ | $30.3644\text{ pt}$ | **`+0.0000 pt`** | **PARIDADE EXACTA** |
| **Caso 3: Parágrafo $\to$ Heading 2** | $24.8952\text{ pt}$ | $24.8952\text{ pt}$ | **`-0.0000 pt`** | **PARIDADE EXACTA** |
| **Caso 2: Heading 1 $\to$ Parágrafo** | $15.7630\text{ pt}$ | $15.7630\text{ pt}$ | **`+0.0000 pt`** | **PARIDADE EXACTA** |
| **Caso 4: Heading 2 $\to$ Parágrafo** | $15.7630\text{ pt}$ | $15.7630\text{ pt}$ | **`-0.0000 pt`** | **PARIDADE EXACTA** |
| **Iso A: Heading 1 $\to$ Bloco (above: 0pt)** | $15.7630\text{ pt}$ | $15.7630\text{ pt}$ | **`+0.0000 pt`** | **PARIDADE EXACTA** |
| **Iso B: Heading 2 $\to$ Bloco (above: 0pt)** | $15.7630\text{ pt}$ | $15.7630\text{ pt}$ | **`-0.0000 pt`** | **PARIDADE EXACTA** |
| **Iso C: Heading 1 $\to$ Bloco (above: 0.5em)** | $15.7630\text{ pt}$ | $15.7630\text{ pt}$ | **`+0.0000 pt`** | **PARIDADE EXACTA** |
| **Iso D: Heading 1 $\to$ Bloco (above: 2em)** | $29.5130\text{ pt}$ | $29.5130\text{ pt}$ | **`-0.0000 pt`** | **PARIDADE EXACTA** |

---

## 2. Diagnóstico e Resolução do Resíduo nos Casos 1 e 3

- **Causa Real Encontrada**: Em `heading.rs`, o cálculo do `top_edge` (cap-height) da primeira linha do heading via `layouter.metrics.text_edges(heading_size, &layouter.style)` passava `&layouter.style` antes de aplicar a mutação `bold: true`.
- Consequentemente, o `FallbackFontMetrics` consultava a face *Regular* da fonte (com `capital_height = 683`), em vez da face *Bold* do heading (com `capital_height = 686`).
- Para $15.4\text{ pt}$ (Heading 1): $(686 - 683) / 1000 \times 15.4\text{ pt} = \mathbf{0.0462\text{ pt}}$, exatamente o resíduo medido!
- Para $12.98\text{ pt}$ (Heading 2): $(686 - 683) / 1000 \times 12.98\text{ pt} = \mathbf{0.0396\text{ pt}}$, exatamente o resíduo medido!
- **Correção**: Passar `heading_style` (já com `bold: true`) para `text_edges`, fazendo com que ambos os casos convirjam para $\Delta = \mathbf{0.0000\text{ pt}}$.

---

## 3. Delimitação Estrita do Escopo em `sequence.rs`

- O escopo de `is_transparent` foi restrito exclusivamente a `Content::Space` e `Content::Empty` fora de linhas abertas (`current_line.is_empty()`), conforme especificado no L0.
- Tipos adicionais (`Styled`, `Place`, `Metadata`, `CounterUpdate`) foram removidos da condição para evitar qualquer efeito colateral em texto corrido ou elementos de estilo inline.
