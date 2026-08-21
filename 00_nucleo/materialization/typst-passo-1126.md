# L0 — Passo 1126: Correção do Delimitador Vertical do Coeficiente Binomial `binom(n, k)` (Secção 7)

**Gate**: `cargo test --workspace` verde (5.958+ testes) e paridade estrita de altura em `sec_07.typ`.

**Base**: Achado visual do Passo 1125 (§1). A inspeção com `pdftoppm` revelou que o parêntese delimitador de `binom(n, k)` na Banda 6 de `sec_07` é renderizado no Cristalino com **56px (26.88 pt)** de altura contra **44px (21.12 pt)** no Vanilla a 150 dpi (sobre-esticamento de $+5.76\text{ pt}$ / $+12\text{ px}$). Este sobre-esticamento empurra todas as linhas subsequentes da secção para baixo.

---

## 1. Diagnóstico do Problema

1. **Sintoma**:
   - Vanilla: O coeficiente binomial $\binom{n}{k}$ utiliza parênteses de tamanho intermediário que cobrem estritamente o empilhamento vertical de $n$ e $k$ ($21.12\text{ pt}$).
   - Cristalino: O delimitador esticado seleciona uma variante maior de glifo (`assembly.rs` / `lr.rs` / `binom.rs`), resultando em $26.88\text{ pt}$, estendendo a caixa delimitadora do bloco.

2. **Causa a Investigar**:
   - `01_core/src/compiler/math/layout/lr.rs` / `assembly.rs`: Como a altura alvo (`target_height`) de delimitadores automáticos para `binom` é calculada.
   - `01_core/src/compiler/math/layout/mod.rs`: Seleção da variante de glifo (tamanhos discretos vs assembly contínuo) para delimitadores de `binom`.

---

## 2. Critérios de Aceitação

1. O delimitador vertical de `binom(n, k)` em `sec_07` deve coincidir com a altura do Vanilla ($21.12\text{ pt}$ / $44\text{ px}$ a 150 dpi).
2. A altura total da `sec_07_crystalline.pdf` deve atingir paridade com `sec_07_vanilla.pdf` ($294.163\text{ pt}$).
3. Nenhuma regressão na suíte de testes (`cargo test --workspace`).
