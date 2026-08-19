# Relatório de Auditoria e Diagnóstico — Passo 1091 (Dedução Numérica Definitiva)

**Data**: 2026-08-19  
**Status**: Causa Raiz Provada (Sem Termo Espúrio de SpaceAfterScript)  
**Gate**: Nenhum (Auditoria analítica sem alteração de código).

---

## 1. Resumo Executivo da Auditoria

A auditoria do Passo 1091 comprovou a causa exata do degrau de **`0.055457 pt`** no fechamento do parêntese `)` em termos com sub-anexo de segundo nível ($g_i(x^*)$, $h_j(x^*)$ e $w_k(x^*)$).

---

## 2. Prova Numérica e Física Exata

### 2.1 Avaliação de `space_after_script` no Vanilla e Crystalline
Auditado em `lab/typst-original/crates/typst-layout/src/math/scripts.rs:108,149`:
```rust
let (font, size) = base.font(ctx, styles);
...
let space_after_script = font.math().space_after_script.at(size);
```
- `space_after_script` é avaliado em `size`, que é o **tamanho da base** do anexo.
- Para o sub-anexo $x^*$, a base é $x$, que está em tamanho `Script` ($7.70\text{ pt}$) **em ambos os compiladores**.
- Portanto:
  $$\text{space\_after\_script} = 56\text{ du} \times \frac{7.70\text{ pt}}{1000} = 0.431200\text{ pt}\quad \text{(em ambos)}$$
  $$\Delta_{\text{space}} = \mathbf{0.000000\text{ pt}}$$

### 2.2 O Erro de Escala do Glifo $*$ (`asteriskmath`, GID 986) em `attach.rs:40-52`
No Crystalline, o estilo para o sobrescrito $*$ do nó $x^*$ foi multiplicado pelo fator de primeiro nível ($0.70$) sobre o estilo já em Script ($7.70\text{ pt}$):
- **Tamanho no Crystalline**: $7.70\text{ pt} \times 0.70 = \mathbf{5.390000\text{ pt}}$ ($0.70 \times 0.70 = 0.49$)
- **Tamanho no Vanilla**: $11.00\text{ pt} \times 0.50 = \mathbf{5.500000\text{ pt}}$ (`ScriptScriptPercentScaleDown` = 50%)

### 2.3 Decomposição Exata do Delta Horizontal Medido
Na tabela `hmtx` da fonte `NewCMMath-Regular.otf`, o glifo $*$ tem avanço de **`500 du`**:

1. **Diferença de avanço no glifo $*$**:
   $$w_*^{\text{Vanilla}} = 500\text{ du} \times \frac{5.50\text{ pt}}{1000} = \mathbf{2.750000\text{ pt}}$$
   $$w_*^{\text{Cryst}} = 500\text{ du} \times \frac{5.39\text{ pt}}{1000} = \mathbf{2.695000\text{ pt}}$$
   $$\Delta w_* = 2.750000\text{ pt} - 2.695000\text{ pt} = \mathbf{+0.055000\text{ pt}}$$

2. **Offset inicial da margem de página (1cm)**:
   $$\Delta x_0 = 28.346000\text{ pt} - 28.346457\text{ pt} = \mathbf{-0.000457\text{ pt}}$$

3. **Delta Total Medido na Coordenada Absoluta $X$ do Glifo $)$**:
   $$\Delta X_{)} = - \Delta w_* + \Delta x_0 = -0.055000\text{ pt} - 0.000457\text{ pt} = \mathbf{-0.055457\text{ pt}}$$

A conta fecha com **paridade analítica exata** até a 6ª casa decimal contra o valor medido no PDF ($X_C = 47.814900\text{ pt}$, $X_V = 47.870358\text{ pt}$, $\Delta = -0.055458\text{ pt}$).

---

## 3. Conclusão para o Passo 1092

A causa raiz é unicamente a escala do glifo no nível ScriptScript:
- Em `attach.rs`, o cálculo de `top_style` e `bottom_style` deve transicionar o `MathSize` discretamente:
  - `Display`/`Text` $\rightarrow$ `Script` (escala `script_percent_scale_down` = $0.70$)
  - `Script` $\rightarrow$ `ScriptScript` (escala $\frac{\text{script\_script\_percent\_scale\_down}}{\text{script\_percent\_scale\_down}} = \frac{0.50}{0.70} \approx 0.7142857$)
  - `ScriptScript` $\rightarrow$ `ScriptScript` (escala $1.0$)
- Esta correção restaura o tamanho de $*$ para $5.50\text{ pt}$, recupera os $0.0550\text{ pt}$ em cada ocorrência de $(x^*)$ e zera o resíduo da Equação 3 e da largura de página.
