# Passo 1106 — Relatório: Investigação do Deslocamento Vertical entre Heading e Primeiro Bloco

## 1. Re-execução dos Casos Iso A-D e Casos 1-4 do P1063

As medições empíricas comparativas entre o Crystalline e o Vanilla Typst 0.15.1 revelaram com precisão os seguintes números:

| Caso de Teste | Gap Baseline Crystalline | Gap Baseline Vanilla | $\mathbf{\Delta Gap}$ | Status |
|---|---|---|---|---|
| **Caso 2: Heading 1 $\to$ Parágrafo** | $15.7630\text{ pt}$ | $15.7630\text{ pt}$ | **`0.0000 pt`** | **PARIDADE EXACTA** |
| **Caso 4: Heading 2 $\to$ Parágrafo** | $15.7630\text{ pt}$ | $15.7630\text{ pt}$ | **`0.0000 pt`** | **PARIDADE EXACTA** |
| **Iso A: Heading 1 $\to$ Bloco (above: 0pt)** | $20.5744\text{ pt}$ | $15.7630\text{ pt}$ | **`+4.8114 pt`** | **DISCREPÂNCIA** |
| **Iso B: Heading 2 $\to$ Bloco (above: 0pt)** | $17.6352\text{ pt}$ | $15.7630\text{ pt}$ | **`+1.8722 pt`** | **DISCREPÂNCIA** |
| **Iso C: Heading 1 $\to$ Bloco (above: 0.5em)** | $20.5744\text{ pt}$ | $15.7630\text{ pt}$ | **`+4.8114 pt`** | **DISCREPÂNCIA** |
| **Iso D: Heading 1 $\to$ Bloco (above: 2em)** | $20.5744\text{ pt}$ | $29.5130\text{ pt}$ | **`-8.9386 pt`** | **DISCREPÂNCIA** |
| **Caso 1: Parágrafo $\to$ Heading 1** | $30.0080\text{ pt}$ | $30.3644\text{ pt}$ | **`-0.3564 pt`** | **DISCREPÂNCIA** |
| **Caso 3: Parágrafo $\to$ Heading 2** | $24.6290\text{ pt}$ | $24.8952\text{ pt}$ | **`-0.2662 pt`** | **DISCREPÂNCIA** |
| **Secção 32: Heading 2 $\to$ Equação 1** | $26.7487\text{ pt}$ | $21.7987\text{ pt}$ | **`+4.9500 pt`** | **DISCREPÂNCIA** |

---

## 2. Diagnóstico da Causa Raiz

### 2.1. O que aconteceu na remoção do `extra_below` no P1104
* No P1104, ao remover o ajuste empírico `extra_below` do `heading.rs`, unificou-se o colapso para parágrafos (`Heading → Parágrafo`), atingindo $\Delta Gap = 0.0000\text{ pt}$ perfeito.
* Contudo, quando o elemento seguinte é um **`Block`** (`block.rs`) ou uma **`Equation` de bloco** (`equation.rs`), o protocolo de colapso trata o avanço de linha do `flush_line()` do heading como um frame de bloco fechado.
* Em `equation.rs:159`, o cursor é posicionado através de:
  $$\text{cursor\_y} = \text{prev\_baseline} + \text{gap} + \text{ext.ascent}$$
  onde `prev_baseline` retém a baseline do heading ($37.40\text{ pt}$), `gap` assume $8.25\text{ pt}$ (weakness 3 do heading), mas `ext.ascent` do `MathLayouter` inclui indevidamente a altura do limite superior `alpha` sem deduzir a `baseline_frame` do bloco, empurrando a baseline da equação $+4.9500\text{ pt}$ abaixo do ponto exato do Vanilla.

### 2.2. Separação de Responsabilidades Arquiteturais
1. **Protocolo de Colapso de Heading (`heading.rs`)**:
   - `below_pt = font_base * 0.75` ($8.25\text{ pt}$) é a margem de colapso correta.
   - O `flush_line()` do heading deve registrar a aresta inferior real da caixa (`heading_baseline + heading_descent`) para transições de bloco, ou a baseline para transições de parágrafo.
2. **Posicionamento de Equação de Bloco (`equation.rs`)**:
   - O topo do frame da equação de bloco deve ser alinhado a $\text{fundo\_anterior} + \text{gap}$, e a baseline da equação posicionada a $\text{topo} + \text{math\_box.ascent}$.

---

## 3. Próximos Passos
- Implementar a correção desacoplada no alinhamento de arestas de blocos e equações após heading sem reintroduzir `extra_below` arbitrário.
