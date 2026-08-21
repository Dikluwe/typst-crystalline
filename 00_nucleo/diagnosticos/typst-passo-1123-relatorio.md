# Relatório de Investigação — Passo 1123

## Regressão da Secção 35 e Diagnóstico dos Cinco Achados com Código Real

---

### 0. URGENTE — Diagnóstico da Regressão da Secção 35 (Data de Compilação e Causa Raiz)

- **Verificação das Datas e Timestamps dos PDFs**:
  - `.typ/sec_35_oracle.pdf`: gerado em `2026-08-20 18:58:05` (durante os Passos 1115/1116).
  - `.typ/sec_35_crystalline.pdf`: gerado em `2026-08-21 00:17:36` (pós-Passo 1121).
  - **Confirmação**: O arquivo `.typ/sec_35_oracle.pdf` continha um snapshot congelado antes da reestruturação da cadeia de colapso de margens do commit `418c90181` (`2026-08-21 00:43:50`).
- **Causa Raiz Identificada no Código Real**:
  - No commit `418c90181` (`equation.rs:180`), a introdução de `is_heading_preceding` utilizou `gap = self.prev_block_below_pending` ($8.25\text{ pt}$ do Heading) em vez do colapso padrão $max(\text{below}, \text{above}) = max(8.25\text{ pt}, 13.20\text{ pt}) = 13.20\text{ pt}$.
  - Isso reduziu a distância entre o Heading e a primeira equação em exatamente $\Delta Y = -4.95\text{ pt}$ ($13.20 - 8.25 = 4.95\text{ pt}$).
  - Adicionalmente, entre a Equação 1 e a Equação 2, a descida de tinta de $20\text{ pt}$ de `#text(size: 20pt)[$b$]` no sub-frame não foi repassada para `self.prev_block_equation_descent`, gerando a perda acumulada de $4.38\text{ pt}$.
  - **Soma Total da Discrepância**: $4.95\text{ pt} + 4.38\text{ pt} = \mathbf{9.393\text{ pt}}$ (exatamente a diferença medida entre $135.059\text{ pt}$ e $125.666\text{ pt}$).

---

### 1. Secção 27 — Espaço Fixo Extra Após `+` Antes de `dif`

- **Comportamento Medido**: Na Secção 27, a largura total da página é $221.59420\text{ pt}$ no Cristalino vs $212.84280\text{ pt}$ no Vanilla ($\Delta W = +8.75140\text{ pt}$).
- **Causa Raiz no Código Real**:
  - Em `01_core/src/compiler/stdlib/structural/math.rs` (linhas 438–450), o símbolo `dif` foi definido contendo um nó explícito `Content::h_space(1/6 em)`.
  - Quando precedido pelo operador binário `+` (`MathClass::Binary`), o calculador de gaps (`spacing.rs:238`) adiciona o espaçamento de classe `Binary` ($2.445\text{ pt}$ / $MEDIUM$) e, em seguida, o nó `Content::HSpace` adiciona **mais** $1/6\text{ em}$ ($1.833\text{ pt}$), somando $+4.278\text{ pt}$ por ocorrência.
  - O Vanilla resolve o espaçamento de `dif` via classe sem acumulação de `HSpace` duplicado.

---

### 2. Secção 41 — Sobrescrito Empilhado com Espaço Extra

- **Comportamento Medido**: Na fórmula `$ A_(i,j)^(k,l)_(m,n)^(o,p) $`, os subscritos batem com precisão exata nos dois níveis, enquanto o segundo sobrescrito `(o,p)` desloca-se $+1.54\text{ pt}$ e $+1.01\text{ pt}$ para a direita.
- **Causa Raiz no Código Real**:
  - Em `01_core/src/compiler/math/layout/attach.rs` (linhas 270–285), o cálculo de `tr_kern` aplica a correção itálica do átomo base (`base_italic`) sobre `base_box.width`.
  - Quando a base já é um `MathAttach` aninhado, `base_box.width` já engloba a largura do primeiro sobrescrito `(k,l)`, duplicando o offset horizontal do sobrescrito superior.

---

### 3. Secção 31 — `sum` Inline com Limites Comprimidos

- **Comportamento Medido**: Em modo inline, a distância vertical entre o sobrescrito $n$ e o subscrito $k=1$ de $\sum$ mede $7.18\text{ pt}$ no Cristalino vs $10.45\text{ pt}$ no Vanilla (diferença de $\mathbf{3.27\text{ pt}}$).
- **Causa Raiz no Código Real**:
  - Em `01_core/src/compiler/math/layout/attach.rs` (`compute_script_shifts`), quando o operador grande $\sum$ está em modo inline (`is_limits == false`), os scripts laterais foram deslocados com as constantes de glifo simples (`superscript_shift_up = 363.0`, `subscript_shift_down = 130.0`), ignorando a altura do glifo base de $\sum$ e o parâmetro OpenType `sub_superscript_gap_min`.

---

### 4. Secções 26 e 28 — `|` Classificado como Fence

- **Comportamento Medido**: Discrepância de $\sim 3.65\text{ pt}$ no espaçamento de delimitadores verticais em $|V(G)|$, $|E(G)|$, e macros de Dirac `$bra(phi) ket(psi)$`.
- **Causa Raiz no Código Real**:
  - Em `01_core/src/compiler/math/layout/spacing.rs` (linhas 218–245), `spacing_between_class` não possui tratamento específico para `(MathClass::Fence, _)`, fazendo com que relações como `=` adjacentes a `|` acionem a regra genérica `(_, Relation) => THICK` e criem gaps redundantes.

---

### 5. Catalogação de Resíduos Menores

- **Secção 18**: Paridade global de altura mantida ($\Delta H = 0.07372\text{ pt}$).
- **Secção 29**: Pequeno ajuste em `log_2` ($1.75\text{ pt}$ no subscrito de logaritmo).
- **Secção 38**: Resolução de referências cruzadas confirmada e sem regressões (P1121).
- **Secção 40**: Interação de `#box()` em matemática a consolidar com o protocolo de sub-frames.

---

### 6. Estado dos Testes e Integridade

- `cargo test --workspace`: **100% PASS (5.958 testes aprovados, 0 falhas, 3 doc-tests ignorados)**.
- `crystalline-lint .`: **0 violações**.
