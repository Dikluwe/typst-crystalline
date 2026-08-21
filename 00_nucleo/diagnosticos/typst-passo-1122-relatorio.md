# Relatório de Investigação — Passo 1122

## Secções 18, 19, 20, 23: Generalização do Espaçamento Entre Blocos e Diagnóstico de Causas Raiz

---

### 1. Secção 19 — `display()` / `inline()` / `script()` / `sscript()` vs `#h()`

- **Achado Central**: `#h()` manual possui paridade estrita de 100% ($\Delta X = 0.00000\text{ pt}, \Delta Y \le 0.00002\text{ pt}$ nas 5 primeiras linhas). O desvio acumulado ocorre exclusivamente a partir da linha 7 (`inline(1/2)`), crescendo nas linhas 8 e 9 (`script(1/2)`, `sscript(1/2)`).
- **Causa Raiz Identificada no Código Real**:
  - Em `01_core/src/compiler/math/layout/mod.rs` (linhas 621–640), o braço `Content::MathStyled(m)` aplicava `size_factor` sobre `style.size`, mas **NÃO** atualizava `math_style.math_size` nem `math_style.cramped`.
  - Como consequência, uma equação de bloco que contenha `$ inline(1/2) $` mantinha `style.math_size == MathSize::Display`.
  - Quando `layout_frac` (`frac.rs`) era invocado, ele avaliava `style.math_size == MathSize::Display`, disparando a descida para `MathSize::Text` com factor `1.0` (em vez de descer de `Text` para `Script` com factor `0.7` — $7.7\text{ pt}$).
  - Adicionalmente, `layout_frac` usava as constantes de shift e gap de `Display` em vez de `Text`/`Script`/`ScriptScript`.

---

### 2. Secção 23 — Sintoma de Oscilação Horizontal e Acúmulo Vertical

- **Achado Central 1 (Oscilação Horizontal $\pm 1.83\text{ pt}$ na Equação 1)**:
  - Na Equação 1 ($Res(f(z), z=z_0) = \dots$), a equação como um todo estava com largura de $177.57434\text{ pt}$ no Cristalino vs $173.92234\text{ pt}$ no Vanilla ($\Delta W = +3.65200\text{ pt}$).
  - Como equações de bloco são centradas na página via `offset_x = (page_width - eq_width) / 2`, o excesso de $3.652\text{ pt}$ na largura deslocou o ponto inicial exatamente $\Delta X = -1.82600\text{ pt}$.
  - **Causa Raiz no Código Real**:
    - Em `01_core/src/compiler/eval/math.rs` (linhas 1360–1375), ao avaliar chamadas em modo math cujo callee é string (`#let Res = "Res"`), o fallback para múltiplos argumentos posicionais injetava `Content::MathText(", ".into())`.
    - O literal `", "` contém um espaço de texto ASCII embutido que furava o motor de classes matemáticas (`MathClass::Punctuation`), quebrando o kerning e espaçamento de relação matemática ao redor de `=`.
- **Achado Central 2 (Acúmulo Vertical entre Equações 2 e 3)**:
  - As equações 2 e 3 sofrem do avanço vertical nominal entre equações de bloco consecutivas quando a medição de ascent/descent de integrais e subscritos em display difere das caixas mínimas do vanilla.

---

### 3. Secção 20 — Transição Heading $\to$ Equação Numerada e Colapso de Margem

- **Achado Central**: A discrepância de $+4.95000\text{ pt}$ no topo da primeira equação após o Heading 2 foi diagnosticada com precisão.
- **Causa Raiz no Código Real**:
  - Em `.typ/sec_20.typ`, a regra `#set math.equation(numbering: "(1)")` está posicionada imediatamente entre o `== 20.` e a primeira equação `$ E = m c^2 $`.
  - Na árvore de conteúdo, o `#set` encapsula a sequência restante em `Content::Styled`.
  - No fluxo de layout, os nós de espaçamento ao redor de `#set` acionavam `push_space` $\to$ `ensure_initial_baseline()`, que consumia a margem pendente do cabeçalho (`prev_block_below_pending = 0.0`) e desativava a cadeia (`block_chain_active = false`).
  - Ao entrar em `layout_equation`, a condição `is_heading_preceding` falhava, fazendo o layouter usar o espaçamento cheio por defeito (`spacing = 13.2pt`) em vez do gap colapsado com o Heading (`8.25pt`), gerando o salto residual de $+4.95\text{ pt}$ ($13.20 - 8.25 = 4.95\text{ pt}$).

---

### 4. Secção 18 — Catálogo de Resíduos Menores

- **Dimensões Globais**: $\Delta W = 0.00001\text{ pt}, \Delta H = 0.07372\text{ pt}$ (praticamente nulo).
- **Catálogo de Resíduos**:
  1. *Fração contínua*: ligeira acumulação de descida de `MathSize` por nível de profundidade ($\le 0.03\text{ pt}$).
  2. *Radicais múltiplos (`√a + √b + √c`)*: pequeno deslocamento vertical do alinhamento da barra superior.
  3. *`underbrace(overbrace(...))`*: resíduo de extensão do brace horizontal.

---

### 5. Estado dos Testes e Validação

- `cargo test --workspace`: **100% PASS (5.958 aprovados, 0 falhas, 3 doc-tests ignorados)**.
- `crystalline-lint .`: **0 violações**.
