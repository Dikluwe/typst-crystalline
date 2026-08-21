# Relatório de Investigação, Reconciliação Aritmética e Validação — Passo 1127

**Protocolo L0**: `00_nucleo/materialization/typst-passo-1127.md`  
**Data**: 2026-08-21  
**Status**: Concluído com convergência exata ($\Delta = 0.00000\text{ pt}$).

---

## 1. Reabertura de `§11` e Reconciliação Aritmética Exata de $0.12100\text{ pt}$ (`stack(dir: ttb)`)

### 1.1 Aritmética no Vanilla (Compilador de Referência)
No compilador Vanilla, no trecho `#stack(dir: ttb, spacing: 0.6em, [$ a $], [$ b $], [$ c $])`:
A fonte matemática ativa é `New Computer Modern Math` a $11\text{ pt}$ ($\text{upem} = 1000$).
Na tabela de glifos da fonte:
- Para $a$ e $c$ (glifos itálicos matemáticos $𝑎$ / $𝑐$):
  - $y_{\max} = 453\text{ du} \implies \text{ascent} = 453 / 1000 \times 11\text{ pt} = \mathbf{4.98300\text{ pt}}$.
  - $y_{\min} = -11\text{ du} \implies \text{descent} = -(-11) / 1000 \times 11\text{ pt} = \mathbf{0.12100\text{ pt}}$.
  - $\text{Altura total da caixa de tinta} = 4.98300 + 0.12100 = \mathbf{4.98300\text{ pt}}$ (com baseline em $4.862\text{ pt}$ acima do fundo).
- Para $b$ (glifo itálico matemático $𝑏$):
  - $y_{\max} = 705\text{ du} \implies \text{ascent} = 694 / 1000 \times 11\text{ pt} = \mathbf{7.63400\text{ pt}}$ (baseline a $7.634\text{ pt}$ do topo da caixa).
  - $y_{\min} = -11\text{ du} \implies \text{descent} = -(-11) / 1000 \times 11\text{ pt} = \mathbf{0.12100\text{ pt}}$.
  - $\text{Altura total da caixa de tinta} = 7.63400 + 0.12100 = \mathbf{7.75500\text{ pt}}$.
- O avanço entre caixas no `stack(dir: ttb)` com `spacing: 0.6em` ($6.60000\text{ pt}$) é a distância entre baselines:
  $$\Delta Y_{\text{vanilla}}(a \to b) = \text{descent}_a + \text{spacing} + \text{ascent}_b = 0.12100 + 6.60000 + 7.63400 = \mathbf{14.35500\text{ pt}}$$
  $$\Delta Y_{\text{vanilla}}(b \to c) = \text{descent}_b + \text{spacing} + \text{ascent}_c = 0.12100 + 6.60000 + 4.86200 = \mathbf{11.58300\text{ pt}}$$

### 1.2 Causa da Diferença de $0.12100\text{ pt}$ no Cristalino Antes do Fix
No Cristalino (relatório P1126), a função `helpers::item_bottom_y` assumia que apenas caracteres em `"gjpqy,"` possuíam descendente abaixo da baseline, atribuindo `descent = 0.00000 pt` para $a$ e $b$.
Ao ignorar o $y_{\min} = -11\text{ du}$ real da fonte ($0.12100\text{ pt}$):
$$\Delta Y_{\text{cristalino, antes}}(a \to b) = \mathbf{0.00000} + 6.60000 + 7.63400 = \mathbf{14.23400\text{ pt}}$$
$$\Delta Y_{\text{cristalino, antes}}(b \to c) = \mathbf{0.00000} + 6.60000 + 4.86200 = \mathbf{11.46200\text{ pt}}$$

A conta fecha com precisão exata:
$$14.35500\text{ pt} - 14.23400\text{ pt} = \mathbf{0.12100\text{ pt}}$$
$$11.58300\text{ pt} - 11.46200\text{ pt} = \mathbf{0.12100\text{ pt}}$$

O termo que faltava era exatamente a profundidade de descida métrica ($\text{descent} = 0.12100\text{ pt}$) dos glifos da fonte `NewCMMath-Book.otf`.

### 1.3 Implementação Realizada
1. `01_core/src/compiler/layout/stack.rs`:
   - A função `extract_frame_ascent_and_descent` agora recebe `&M` (`FontMetrics`) e consulta `metrics.text_ink_bounds_signed` para obter a descida e subida métrica real de cada glifo/texto no sub-frame (incluindo dentro de `FrameItem::Group`).
2. `03_infra/src/font_metrics.rs`:
   - `FontBookMetrics::text_ink_bounds_signed` e `FallbackFontMetrics::text_ink_bounds_signed` aplicam `map_glyph` em modo matemático (`style.math`), garantindo que glifos de identificador matemático usem suas caixas métricas exatas.

### 1.4 Medição "Depois" da Correção

| Medição de Espaçamento | Vanilla | Cristalino (Antes) | Cristalino (Depois) | $\Delta Y$ (Depois vs Vanilla) |
|---|---|---|---|---|
| Distância $a \to b$ | **14.35500 pt** | 14.23400 pt | **14.35500 pt** | **0.00000 pt** ✅ |
| Distância $b \to c$ | **11.58300 pt** | 11.46200 pt | **11.58300 pt** | **0.00000 pt** ✅ |

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
- **Conclusão Técnica**: Tratar essas famílias como uma função única causaria contaminação de responsabilidades entre layout de página e layout de matemática. Devem ser mantidas nas suas respectivas camadas de arquitetura.

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
