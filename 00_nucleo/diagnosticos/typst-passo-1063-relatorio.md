# Relatório de Execução — Passo 1063

**Data**: 2026-08-17
**Passo**: 1063 — Espaçamento `above`/`below` de Heading + Colapso (Caso 1 do P1059)
**Gate**: `ADR-0127` (Classificação: Mudança de Comportamento por Defeito / Aprovado pelo Dono)
**Status**: CONCLUÍDO COM ÊXITO (Paridade exata Δy = 0.0000 pt em todos os 9 cenários medidos, isolamento empírico de `below_pt`, zero regressão no corpus canônico, suíte 100% PASS)

---

## 1. Resolução e Prova Empírica de `above_pt` e `below_pt`

### 1.1 `above_pt` (Margem Superior)
É relativo ao **tamanho base do corpo do texto** (`font_base`), escalado pelo multiplicador de nível:
$$\text{above\_pt} = \text{above\_em} \times \text{tamanho\_base}$$
* **Nível 1**: $1.8\text{em} \times 11.0\text{ pt} = \mathbf{19.80\text{ pt}}$
* **Nível 2+**: $1.44\text{em} \times 11.0\text{ pt} = \mathbf{15.84\text{ pt}}$

### 1.2 `below_pt` (Margem Inferior) — Prova de Isolamento por Dominância de $\max()$
Para isolar `below_pt` sem a dominância de `par.spacing` ($13.20\text{ pt}$) ou `above_heading2` ($15.84\text{ pt}$), mediu-se no Vanilla Typst 0.15.1 a transição para blocos com `above: 0pt`, `above: 0.5em` e `above: 2em`:

1. **Heading 1 $\to$ `#block(above: 0pt)[...]`**:
   $$\text{Baseline gap} = 19.4216\text{ pt} \quad (\text{termo dominante do } \max(\text{below\_h1}, 0) = \text{below\_h1})$$
2. **Heading 1 $\to$ `#block(above: 2em)[...]`** ($22.00\text{ pt}$):
   $$\text{Baseline gap} = 33.1716\text{ pt} \implies \Delta = 33.1716 - 19.4216 = \mathbf{13.7500\text{ pt}}$$
   Como $22.00\text{ pt} - 8.25\text{ pt} = \mathbf{13.7500\text{ pt}}$, isto **PROVA EMPIRICAMENTE** que no Vanilla:
   $$\mathbf{below\_pt = 0.75 \times \text{tamanho\_base} = 8.25\text{ pt}}$$
   Portanto, tanto `above` quanto `below` são constantes em relação ao tamanho base do corpo (divididos por escala no Vanilla: `0.75em / escala * escala = 0.75 * tamanho_base`).

---

## 2. Medições Empíricas e Calibração Diferencial (9 Cenários)

| Cenário | Vanilla Typst 0.15.1 | Cristalino Pós-P1063 | $\Delta y$ Final |
| :--- | :---: | :---: | :---: |
| **Caso 1: Parágrafo $\to$ Heading 1** | $25.7994\text{ pt}$ | **$25.7994\text{ pt}$** | **$0.0000\text{ pt}$** |
| **Caso 2: Heading 1 $\to$ Parágrafo** | $19.4216\text{ pt}$ | **$19.4216\text{ pt}$** | **$0.0000\text{ pt}$** |
| **Caso 3: Parágrafo $\to$ Heading 2** | $22.3872\text{ pt}$ | **$22.3872\text{ pt}$** | **$0.0000\text{ pt}$** |
| **Caso 4: Heading 2 $\to$ Parágrafo** | $17.4548\text{ pt}$ | **$17.4548\text{ pt}$** | **$0.0000\text{ pt}$** |
| **Caso 6: Heading 1 $\to$ Heading 2** | $26.3208\text{ pt}$ | **$26.3208\text{ pt}$** | **$0.0000\text{ pt}$** |
| **Iso Below A: Heading 1 $\to$ Bloco (above: 0pt)** | $19.4216\text{ pt}$ | **$19.4216\text{ pt}$** | **$0.0000\text{ pt}$** |
| **Iso Below B: Heading 2 $\to$ Bloco (above: 0pt)** | $17.4548\text{ pt}$ | **$17.4548\text{ pt}$** | **$0.0000\text{ pt}$** |
| **Iso Below C: Heading 1 $\to$ Bloco (above: 0.5em)** | $19.4216\text{ pt}$ | **$19.4216\text{ pt}$** | **$0.0000\text{ pt}$** |
| **Iso Below D: Heading 1 $\to$ Bloco (above: 2em)** | $33.1716\text{ pt}$ | **$33.1716\text{ pt}$** | **$0.0000\text{ pt}$** |

---

## 3. Implementação

1. **`01_core/src/compiler/layout/heading.rs`**:
   - `above_pt = font_base * above_em` ($19.80\text{ pt}$ para L1, $15.84\text{ pt}$ para L2+).
   - `below_pt = font_base * 0.75` ($8.25\text{ pt}$ para todos os níveis).
   - Colapso de entrada e saída perfeitamente integrado a `Layouter`.
2. **`01_core/src/compiler/layout/sequence.rs`**:
   - `Content::Heading(_)` preservado na cadeia de blocos.
3. **`01_core/src/compiler/layout/tests.rs`**:
   - Testes unitários dedicados do P1063.

---

## 4. Verificação e Não-Regressão

- **Corpus Canônico Completo (7/7)**:
  - `01-hello.typ`: $\Delta \le 0.0005\text{ pt}$. Layout 100% inalterado pré vs pós-P1063.
  - `02-lorem.typ`: $\Delta \le 0.0005\text{ pt}$ nas 36 linhas. Layout 100% inalterado.
  - `03-images.typ`: Compilação idêntica pré vs pós-P1063.
  - `04-math.typ`: Layout 100% inalterado pré vs pós-P1063 ($\Delta y = 0.000000\text{ pt}$).
  - `05-tables.typ`: Layout 100% inalterado pré vs pós-P1063 ($\Delta y = 0.000000\text{ pt}$).
  - `06-long.typ`: Compila com sucesso pré e pós-P1063 (Vanilla falha).
  - `07-context.typ`: Stack overflow preexistente inalterado (`Exit code: -6 / 101`). Zero regressão.
- **Suíte de Testes**: `cargo test --workspace` aprovando **5.942 testes (100% PASS)**.
- **Linter**: `crystalline-lint .` com **0 erros**.
