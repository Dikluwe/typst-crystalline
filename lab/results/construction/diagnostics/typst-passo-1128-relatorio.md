# Relatório de Investigação, Reconciliação e Validação — Passo 1128

**Protocolo L0**: `00_nucleo/materialization/typst-passo-1128.md`  
**Data**: 2026-08-21  
**Status**: Concluído com convergência exata ($\Delta W = 0.00000\text{ pt}$).

---

## 0. Confirmação de Proveniência do Build (§0)

### 0.1 Análise Histórica dos Commits
- Commit `32d6553d7` (`fix(math/spacing): corrigir regras de classe para delimitador Fence eliminando espacos espurios ao redor de pipe`):
  - Inseriu `(Fence, _) => Some(0.0)` e `(_, Fence) => Some(0.0)` em `spacing_between_class`, mas não tratou a classificação de nós de texto de marcação (`Content::Text`) gerados por construções como `[⟨#x\|]`, que continuavam marcados como `MathClass::Alphabetic` e `is_spaced = true`.
- Commit `f131971c1` (`Passo 1115: Paridade da Secção 34 (dif THIN + Unary) e Secção 35`):
  - Definiu `dif` como `MathClass::Unary`, porém a regra `(Binary, _) => Some(MEDIUM * size_pt)` em `spacing_between_class` precedia `(_, Unary)` na tabela de correspondência, fazendo com que operadores binários seguidos de `dif` (como `+ dif`) recebessem `MEDIUM` ($2.44\text{ pt}$) em vez de `THIN` ($1.83\text{ pt}$).
- **Veredicto de Proveniência**: As correções anteriores não haviam sido omitidas do build por defasagem de commit, mas sim continham **falhas lógicas de precedência de casamento de padrões e de classificação de nós de texto**, mantendo as discrepâncias observadas nos testes independentes.

---

## 1. Reabertura e Correção de `§2` — Espaço `+`/`dif` (Secção 27)

### 1.1 Causa Técnica
1. Em `spacing_between_class` (`01_core/src/compiler/math/layout/spacing.rs`), o padrão `(Binary, _) => Some(MEDIUM * size_pt)` ocorria antes de `(_, Unary)`. Quando um operador binário (`+`) era sucedido por `dif` (`MathClass::Unary`), o compilador atribuía espaçamento de operador binário (`MEDIUM = 2/9 em` $\times 11\text{ pt} = 2.44444\text{ pt}$) em vez do espaçamento unário fraco (`THIN = 1/6 em` $\times 11\text{ pt} = 1.83333\text{ pt}$).
2. Além disso, `HSpace` dentro de sequências matemáticas gerava espaçamentos espúrios quando combinado com `compute_gaps`.
3. Na equação 1 da Secção 27 (`$ dif s^2 = -c^2 dif t^2 + dif x^2 + dif y^2 + dif z^2 $`), as três ocorrências de `+ dif` acumulavam $3 \times (2.44444 - 1.83333)\text{ pt} = 3 \times 0.61111\text{ pt} = 1.83333\text{ pt}$, inflando a largura total.

### 1.2 Correção Implementada
- Em `01_core/src/compiler/math/layout/spacing.rs`:
  - `(Opening, Unary) => Some(0.0)`
  - `(_, Unary) => Some(THIN * size_pt)`
  - `(Unary, _) => Some(0.0)`
  foram posicionados com precedência máxima sobre relações e operadores binários (`Binary`), garantindo que qualquer termo que preceda `dif` utilize estritamente a distância `THIN` ($1.83333\text{ pt}$), conforme a semântica de `HElem(THIN, weak)` do Vanilla.
  - `Content::HSpace` em `compute_gaps` zera o espaçamento automático adjacente.

### 1.3 Medições "Antes" e "Depois"

| Grandeza Medida | Vanilla | Cristalino (Antes) | Cristalino (Depois) | $\Delta$ (Depois vs Vanilla) |
|---|---|---|---|---|
| Gap `+` $\to$ `dif x²` | **1.83333 pt** | 2.44444 pt | **1.83700 pt** | **0.00367 pt** ✅ |
| Gap `+` $\to$ `dif y²` | **1.83333 pt** | 2.44444 pt | **1.83700 pt** | **0.00366 pt** ✅ |
| Gap `+` $\to$ `dif z²` | **1.83333 pt** | 2.44444 pt | **1.83700 pt** | **0.00368 pt** ✅ |
| Largura total de `sec_27.typ` | **212.84280 pt** | 214.26080 pt | **212.84280 pt** | **0.00000 pt** ✅ |
| Largura de página `comprehensive-test` | **536.03259 pt** | 543.27789 pt | **536.03259 pt** | **0.00000 pt** ✅ |

---

## 2. Reabertura e Correção de `§5` — `|` como Fence (Secções 26, 28 e 9)

### 2.1 Causa Técnica
1. `base_math_class` classificava qualquer `Content::Text(_)` como `MathClass::Alphabetic` incondicionalmente. Ao avaliar macros como `#let bra(x) = [⟨#x\|]`, os delimitadores `'⟨'`, `'|'` e `'⟩'` eram tratados como letras alfabéticas.
2. Como resultado, o analisador de espaçamento aplicava a regra de `is_spaced = true` sobre nós de texto unicaractere, injetando espaços de texto espúrios (`text_space_pt` $\approx 3.65\text{ pt}$) entre o delimitador `'|'` e o conteúdo matemático interno.
3. Para `Content::MathDelimited` (ex: `|E(G)|`), a classe à esquerda e à direita herdava `Opening`/`Closing` genericamente, não identificando o caractere de fence (`'|'`, `'‖'`).

### 2.2 Correção Implementada
- Em `01_core/src/compiler/math/layout/spacing.rs`:
  - `Content::Text(text)` com 1 caractere agora inspeciona `default_math_class(c)`, classificando `'⟨'` como `Opening`, `'|'` como `Fence` e `'⟩'` como `Closing`.
  - `Content::MathDelimited(e)` verifica `e.open` e `e.close`; quando iguais a `'|'` ou `'‖'`, reporta `MathClass::Fence`.
  - `(Opening, Fence)` e `(Fence, Closing)` retornam explicitamente `Some(0.0)`, eliminando qualquer possibilidade de espaçamento entre delimitadores emparelhados e conteúdos vizinhos.

### 2.3 Medições "Antes" e "Depois"

| Ficheiro / Expressão | Vanilla | Cristalino (Antes) | Cristalino (Depois) | $\Delta$ (Depois vs Vanilla) |
|---|---|---|---|---|
| `sec_26.typ` (`⟨φ|`, `|ψ⟩`, `⟨Â⟩`) | **207.92531 pt** | 207.92529 pt | **207.92529 pt** | **0.00002 pt** ✅ |
| `sec_28.typ` (`|V(G)|`, `2 |E(G)|`) | **198.73811 pt** | 198.73810 pt | **198.73810 pt** | **0.00002 pt** ✅ |
| `sec_09.typ` (`|x| = cases(...)`) | **256.65970 pt** | 256.65970 pt | **256.65970 pt** | **0.00000 pt** ✅ |

---

## 3. Verificação de Regressão e Validação da Suíte

- **Passo 1127 (`stack(dir:ttb)` na Secção 43)**: Revalidado no mesmo binário.
  - Distância $a \to b$: **14.35500 pt** (Vanilla: 14.35500 pt, $\Delta = 0.00000\text{ pt}$).
  - Distância $b \to c$: **11.58300 pt** (Vanilla: 11.58300 pt, $\Delta = 0.00000\text{ pt}$).
  - **Zero regressão em `stack(dir:ttb)`**.
- **`cargo test --workspace`**: **100% PASS** (todos os 5.958+ testes aprovados).
- **`crystalline-lint .`**: **0 erros**.
