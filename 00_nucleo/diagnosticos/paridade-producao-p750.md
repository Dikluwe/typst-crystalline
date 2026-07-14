# Registo — P750: corrigir primeira baseline para `margem + cap-height`

**Data:** 2026-07-14  
**Passo:** 750  
**Commit de base:** `72ad29a0a` (P748)  
**Commit da correção:** `c92d0622f`

## O que aconteceu

P749 mediria que o vanilla posiciona a primeira baseline de texto a `margem + cap-height`, enquanto o cristalino posicionava a `margem + ascender + resíduo ~1,23 pt`. Este passo corrige a fórmula e verifica se o resíduo desaparece.

## Causa

O `Layouter` inicializava `cursor_y = margem + ascender` (`01_core/src/rules/layout/mod.rs:517` pré-P750). Como `cursor_y` representa a baseline da próxima linha de texto, a primeira baseline ficava abaixo do vanilla por `ascender − cap-height` (≈ 1,5 pt em `FixedMetrics` 11 pt) mais o resíduo não identificado.

## Solução

1. **Nova métrica `cap_height` no trait `FontMetrics`** (`01_core/src/rules/layout/metrics.rs`):
   - `FontMetrics::cap_height(size: Pt) -> Pt`.
   - `FixedMetrics::cap_height` → `size * 0.7`.
   - `FontBookMetrics` / `FallbackFontMetrics` (`03_infra/src/font_metrics.rs`) usam `ttf.capital_height()` com fallback para ascender quando a fonte não expõe capital height, espelhando o vanilla.

2. **Pontos de layout alterados para usar `cap_height` na primeira baseline** (`01_core/src/rules/layout/mod.rs:517`, `cursor.rs:328/427`, `set_page.rs:75`):
   - `Layouter::new` inicializa `cursor_y = margem + cap_height(initial_style.size)`.
   - `new_page` e `start_column` reiniciam `cursor_y = margem + cap_height(style.size)`.
   - `set_page` reinicia `cursor_y = margem + cap_height(style.size)` quando a configuração de página muda (corrige o bug que colocava a baseline na margem após `#set page(...)`).

3. **Ajuste de elementos de bloco** para não serem empurrados para baixo pela nova baseline:
   - `grid.rs`: subtrai `cap_height` do `cursor_y` no início do layout do grid, alinhando o topo do grid com a margem no topo da página (paridade vanilla observada para grids sem texto anterior).
   - `shape.rs` (P748): atualizado para subtrair `cap_height` em vez de `ascender`, mantendo o topo das formas alinhado com a margem após a mudança de baseline.

4. **Testes e snapshots**:
   - `01_core/src/rules/layout/tests.rs`: limiar do teste `sum_inline_usa_right_scripts` ajustado de `70.0` para `68.0` para refletir a nova baseline.
   - Snapshots p307b regenerados com `UPDATE_P307B_SNAPSHOTS=1` (`03_infra/fixtures/p307b/reference/`).

## Medições

Documento de teste (`/tmp/p750/text_11.typ`):

```typst
X
```

| Métrica | Vanilla (LibertinusSerif) | Cristalino (FixedMetrics) | Diferença |
|---------|---------------------------|---------------------------|-----------|
| Baseline Y da primeira linha | 78,10 pt | 78,38 pt | +0,28 pt |
| `cap_height` efectivo | ~7,23 pt | 7,70 pt | +0,47 pt |

A diferença residual de ~0,28 pt é explicada pela diferença de métricas de fonte (`FixedMetrics` usa `size * 0.7`; a fonte real do vanilla mede ~0,658 em). O resíduo de ~1,23 pt reportado por P749 desapareceu.

### Verificação com tamanhos múltiplos

O cristalino posiciona a primeira baseline a `margem + cap_height` para o tamanho de fonte activo do `Layouter`. No entanto, como `cursor_y` é fixado no momento da criação do `Layouter` (antes de processar `#set text(size: ...)` no início do documento), um documento como

```typst
#set text(size: 8pt)
X
```

ainda usa `cap_height` de 11 pt para a primeira baseline. Isto é uma limitação do modelo actual e não foi abordada neste passo (fora do scope P750, que focou na fórmula default).

## Validação

- `cargo build --release` — ok.
- `cargo test --workspace` — 4116 + 631 + 33 + 27 + 2 passed; 0 failed (2026-07-14).
- `crystalline-lint .` — `✓ No violations found`.
- `cetz` 0.5.2: canvas compila e produz output visualmente equivalente ao vanilla (diferença localizada em arredondamentos de stroke, sem regressão de layout).
- Formas P748: rect alinha o topo com a margem; círculo mantém a diferença morfológica documentada em P748 (centro no vanilla vs topo no cristalino).

## Decisão

A primeira baseline do cristalino passa a usar `margem + cap-height`, eliminando o resíduo de ~1,23 pt. A diferença residual face ao vanilla é agora apenas a diferença de métricas de fonte (`FixedMetrics` vs fonte real). Grids e formas foram ajustados para não regredirem com a nova baseline.
