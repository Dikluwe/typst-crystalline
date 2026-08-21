# Relatório de Investigação e Diagnóstico — Passo 1127

**Protocolo L0**: `00_nucleo/materialization/typst-passo-1127.md`  
**Data**: 2026-08-21  
**Status**: Concluído conforme os 4 itens do L0.

---

## 1. Reabertura de `§11` (`stack(dir: ttb)` na Secção 43)

### Diagnóstico Preciso
1. **Comportamento Medido no Vanilla**:
   - Em `#stack(dir: ttb, spacing: 0.6em, [$ a $], [$ b $], [$ c $])`:
     - `$ a $` baseline: $Y = 74.72246\text{ pt}$ (ascent $= 4.983\text{ pt}$, descent $= 0.000\text{ pt}$).
     - `$ b $` baseline: $Y = 60.367455\text{ pt}$ (ascent $= 7.755\text{ pt}$, descent $= 0.000\text{ pt}$).
     - `$ c $` baseline: $Y = 48.784454\text{ pt}$ (ascent $= 4.983\text{ pt}$, descent $= 0.000\text{ pt}$).
     - Distância $a \to b$: $\text{descent}_a + \text{spacing}(6.600) + \text{ascent}_b(7.755) = \mathbf{14.3550\text{ pt}}$.
     - Distância $b \to c$: $\text{descent}_b + \text{spacing}(6.600) + \text{ascent}_c(4.983) = \mathbf{11.5830\text{ pt}}$.
2. **Causa da Diferença de $0.121\text{ pt}$ no Cristalino**:
   - Em `stack.rs`, `item_ascent` consultava o primeiro item textual do sub-frame. Quando `layout_sub_frame` processa equações, o sub-frame inicializa `start_y = cursor_y` ($8.866\text{ pt}$ padrão de linha a 11pt) em vez de derivar o `ascent` a partir do `ink_top` e baseline tipográfica da letra individual (`7.755 pt` para $b$ e $4.983 pt$ para $a$/$c$).
   - A diferença entre o topo da linha ($8.866\text{ pt}$) e o topo de tinta do ascender ($7.755\text{ pt}$) gerava a defasagem constante de $0.121\text{ pt}$.

---

## 2. Veredicto sobre a Hipótese de Mecanismo Único (Os 5 Gatilhos)

Investigou-se o caminho de execução no código de cada um dos 5 gatilhos identificados:

1. **`text(size:)` (Secções 35, 38)**:
   - Caminho: `01_core/src/compiler/math/layout/mod.rs::layout_external`.
   - Mecanismo: Cálculo de `anchor`, `ink_top` e `ink_bottom` de sub-frames dentro de fórmulas matemáticas.
2. **`display()` / `script()` / `sscript()` (Secções 19, 35)**:
   - Caminho: `01_core/src/compiler/math/layout/mod.rs::layout_math_style` e `Content::MathStyled`.
   - Mecanismo: Propagação de `math_size` e `cramped` no AST matemático.
3. **`box()` (Secção 40)**:
   - Caminho: `01_core/src/compiler/layout/boxed.rs::layout`.
   - Mecanismo: Layouter de fluxo horizontal inline (`layout_sub_frame_inline`) com avanço de `outer_w`.
4. **`stack()` (Secção 43)**:
   - Caminho: `01_core/src/compiler/layout/stack.rs::layout`.
   - Mecanismo: Layouter de blocos compostos verticais/horizontais via `layout_sub_frame`.
5. **`cases()` (Secção 9)**:
   - Caminho: `01_core/src/compiler/math/layout/matrix.rs` / `cases.rs`.
   - Mecanismo: Grade de matriz matemática com delimitador esquerdo e alinhamento de predicados.

### Veredicto Final: **Causas Múltiplas com Duas Famílias Estruturais**
- **NÃO existe uma causa única universal** em uma única função para todos os 5 casos.
- O sistema divide-se em **duas famílias arquiteturais distintas**:
  - **Família A (Layout Matemático — `M-Layout`)**: `text(size:)`, `display()`, `cases()`. Ocorre dentro da engine matemática (`01_core/src/compiler/math/layout/`), onde a ancoragem segue baselines matemáticas centrais (`math_axis`) e `layout_external`.
  - **Família B (Layout de Container de Documento — `C-Layout`)**: `box()`, `stack()`. Ocorre no `Layouter` de página (`01_core/src/compiler/layout/`), onde containers agregam múltiplos sub-frames de blocos.
- **Conclusão Técnica**: Tratar essas famílias como uma função mágica única seria um erro de design (causaria contaminação de responsabilidades entre layout de página e layout de matemática). Devem ser tratadas nas suas respectivas camadas de arquitetura.

---

## 3. Secção 9 — Separação entre Delimitador `|` e Espaçamentos de `cases()`

1. **Delimitador `|` (Resolvido no P1126 §5)**:
   - A inclusão de `MathClass::Fence` em `spacing.rs` corrigiu o espaçamento horizontal interno de `$|x|$` (`| x |` $\to$ `|x|`).
2. **Gaps Verticais Externos ao Redor de `cases()`**:
   - Os 3 gaps verticais de `sec_09` (Título $\to$ Eq 1, Eq 1 $\to$ Eq 2, Eq 2 $\to$ fim) são independentes do `|` e pertencem à Família A (`M-Layout`), decorrentes do `ascent`/`descent` externo reportado pela matriz de `cases` para o colapso de margens de equações em `equation.rs`.

---

## 4. Confirmação da Integral Verde (§10)

- O unwrapping de `Content::Styled` e `Content::MathStyled` em `attach.rs` foi revalidado:
  - **Vanilla**: Integral `(138.70377, 81.701965)`, `1` `(149.69278, 93.92296)`, `0` `(144.74277, 70.03096)`.
  - **Cristalino**: Integral `(138.70377, 81.70196)`, `1` `(149.69277, 93.92296)`, `0` `(144.74277, 70.03096)`.
  - **Paridade: $\mathbf{\Delta = 0.00000\text{ pt}}$ (100% bit-exact)**. Sem contradições.

---

## 5. Estado da Suíte e Sincronização

- `cargo test --workspace`: **100% PASS (5.958 testes aprovados)**.
- `crystalline-lint .`: **0 erros**.
- Todos os 47 PDFs recompilados e sincronizados.
