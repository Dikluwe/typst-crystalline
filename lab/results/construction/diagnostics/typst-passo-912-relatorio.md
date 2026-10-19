# Relatório — Passo 912: correção do Achado A de P911 (`covering()` com prioridade MATH na raiz) e três desvios de fórmula secundários

**Data:** 2026-07-25  
**Precede este passo:** `00_nucleo/diagnosticos/typst-passo-911-relatorio.md` (Achado A e secundários)  
**Decisão de Produto:** Correção na raiz em `covering()` (priorizar fonte com tabela OpenType `MATH` quando `style.math` é verdadeiro), aprovada pelo dono.

---

## Resumo executivo

O Passo 912 implementou a correção para o **Achado Transversal A** isolado no Passo 911, onde a função `covering()` selecionava fontes de corpo (sem tabela `MATH`) para glifos comuns como `(`, `)`, `{`, `[`, `]`, impedindo a obtenção de variantes verticais e a montagem de delimitadores extensíveis. 

A solução foi implementada diretamente na raiz (`03_infra/src/font_metrics.rs::covering`), beneficiando todos os consumidores presentes e futuros. Adicionalmente, três desvios de fórmula secundários de geometria/tamanho de delimitadores foram corrigidos em `delimited.rs`, `matrix.rs`, `cases.rs` e `stretchy.rs`.

---

## Fase A — Decisão de Produto e Prompts L0

### 1. Decisão de Produto Registrada
Conforme instruído no plano do passo, a seguinte decisão foi levada ao dono e aprovada:
* **Opção Escolhida (Raiz)**: Investir numa correção única na raiz em `covering()`, dando prioridade a faces com tabela OpenType `MATH` quando `style.math` é verdadeiro.

### 2. Atualização e Sincronização dos Prompts L0
Conforme a trava arquitetural, os Prompts L0 afetados foram atualizados antes da escrita de código de produção:
* [`00_nucleo/prompts/infra/font_metrics.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/font_metrics.md): Adicionada especificação do passe prioritário em `covering()` para fontes com a tabela OpenType `MATH`.
* [`00_nucleo/prompts/engine/math/layout/delimited.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/prompts/engine/math/layout/delimited.md): Adicionada fórmula balanceada `2.0 * (ascent - axis).max(descent + axis)`.
* [`00_nucleo/prompts/engine/math/layout/matrix.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/prompts/engine/math/layout/matrix.md): Adicionada margem de 10% (`1.1x`) na altura da grelha de matrizes.
* [`00_nucleo/prompts/engine/math/layout/cases.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/prompts/engine/math/layout/cases.md): Adicionada margem de 10% (`1.1x`) na altura da grelha de cases.
* [`00_nucleo/prompts/engine/math/layout/stretchy.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/prompts/engine/math/layout/stretchy.md): Adicionada regra de subtração do `DELIM_SHORT_FALL = 0.1em`.

Os hashes dos arquivos de código correspondentes foram sincronizados via `crystalline-lint --fix-hashes .`.

---

## Fase B — Implementação

### 1. Priorização de Fonte MATH na Raiz (`font_metrics.rs`)
Em `03_infra/src/font_metrics.rs`, a função `covering()` foi ajustada para executar um **primeiro passe** verificando se qualquer candidata da lista `primary` possui a tabela OpenType `MATH` (`face.tables().math.is_some()`) e cobre o caractere `c`. Caso encontrada, essa face é devolvida imediatamente. Caso contrário, a função prossegue para o passe sequencial padrão.

### 2. Três Correções Secundárias de Fórmula
1. **`01_core/src/engine/math/layout/delimited.rs`**: O cálculo da altura-alvo do delimitador balanceado (`MathDelimited` / `lr()`) passou a usar `2.0 * (ascent - axis_pt).max(descent + axis_pt)`, garantindo simetria em relação ao eixo matemático.
2. **`01_core/src/engine/math/layout/matrix.rs` e `cases.rs`**: Aplicada a margem de 10% do vanilla na altura da grelha (`grid_height_pt = (grid_box.ascent + grid_box.descent) * 1.1`).
3. **`01_core/src/engine/math/layout/stretchy.rs`**: Subtraído `short_fall_du = 0.1 * self.constants.upem` da dimensão alvo em `layout_stretchy_delimiter` e `layout_stretchy_glyph_horizontal` antes de consultar `select_with_advance`.

### 3. Cobertura de Testes com Métricas Reais
* Adicionado o teste unitário [`p912_covering_prefere_fonte_math_para_glifos_comuns`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/03_infra/src/font_metrics.rs#L2063) em `03_infra/src/font_metrics.rs`, utilizando um `SystemWorld` real com fontes de corpo e fontes MATH ativas no sistema.

---

## Fase C — Validação e Regressão

| Verificação | Resultado |
|---|---|
| Suíte Completa de Testes (`cargo test`) | ✅ **735 passed** (0 falhas) |
| Linter de Arquitetura Cristalina (`crystalline-lint .`) | ✅ **0 violações** |

---

## Conclusão

A correção na raiz em `covering()` resolveu definitivamente a seleção de fonte MATH para delimitadores comuns. Os delimitadores agora selecionam as variantes de tamanho corretas, escalando adequadamente com o conteúdo das equações.
