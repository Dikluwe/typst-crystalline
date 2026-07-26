# Relatório — Passo 913: repetição de peças extensoras (`is_extender`) em `assembly.rs` (Achado B de P911)

**Data:** 2026-07-25  
**Precede este passo:** `00_nucleo/diagnosticos/typst-passo-911-relatorio.md` (Achado B) e `00_nucleo/diagnosticos/typst-passo-912-relatorio.md`  

---

## Resumo executivo

O Passo 913 corrigiu o **Achado Transversal B** identificado no Passo 911: o algoritmo de montagem por partes de delimitadores (`layout_assembly` e `layout_assembly_horizontal` em `01_core/src/engine/math/layout/assembly.rs`) iterava as peças exatamente uma única vez, sem nunca repetir peças extensoras (`is_extender == true`) e mantendo o parâmetro `_target_advance` morto.

A solução implementou a paridade com o algoritmo `assemble` do Typst vanilla, utilizando um laço para calcular o número necessário de repetições das peças extensoras e o fator de espalhamento `ratio` entre sobreposição máxima e mínima dos conectores.

---

## Fase A — Análise do Vanilla e Prompts L0

### 1. Análise da Função `assemble` do Vanilla
A leitura de `lab/typst-original/crates/typst-layout/src/math/fragment/glyph.rs:567-660` confirmou os seguintes pontos do algoritmo:
* Um laço incrementa o contador `repeat` até `MAX_REPEATS = 1024` (ou até que a dimensão total `full` atinja `target`).
* Peças não-extensoras aparecem exatamente 1 vez; peças com `is_extender = true` aparecem `repeat` vezes.
* Quando `full < target` e há sobreposição ajustável nos conectores, calcula-se `ratio = ((target - full) / growable).min(1.0)`.
* A sobreposição final entre peças consecutivas passa a ser `max_overlap - ratio * max_overlap`.

### 2. Sincronização do Prompt L0
O Prompt L0 [`00_nucleo/prompts/engine/math/layout/assembly.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/prompts/engine/math/layout/assembly.md) foi atualizado com a especificação da repetição de extensores e sincronizado com o arquivo `01_core/src/engine/math/layout/assembly.rs` via `crystalline-lint --fix-hashes .`.

---

## Fase B — Implementação

### 1. Repetição de Extensores em Y e X (`assembly.rs`)
Em `01_core/src/engine/math/layout/assembly.rs`, `layout_assembly` (empilhamento no eixo Y) e `layout_assembly_horizontal` (empilhamento no eixo X) foram atualizados:
* O parâmetro `_target_advance` passou a ser consumido como `target_advance_du` (convertido para `target_pt`).
* O laço de repetição e o cálculo de `ratio` foram aplicados para determinar a lista final de glifos empilhados.
* A sobreposição ajustada foi incorporada no cálculo das posições acumuladas de cada glifo.

### 2. Teste Unitário Sintético
Adicionado o teste unitário [`p913_layout_assembly_repete_extensores_para_alvo_grande`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/engine/math/layout/tests.rs#L2259) em `01_core/src/engine/math/layout/tests.rs`. O teste constrói uma `GlyphAssembly` sintética com peças extensoras e assere que:
1. Para alvos médios (1200du), utiliza exatamente 3 peças (1 extensor).
2. Para alvos grandes (3000du), multiplica as peças extensoras (> 3 peças) e atinge a altura pedida de ~30pt.

---

## Fase C — Validação e Regressão

| Verificação | Resultado |
|---|---|
| Suíte Completa de Testes (`cargo test`) | ✅ **735 passed** (0 falhas) |
| Linter de Arquitetura Cristalina (`crystalline-lint .`) | ✅ **0 violações** |

---

## Conclusão

Com a conclusão dos Passos 912 e 913, tanto a seleção da fonte MATH na raiz quanto a montagem por partes com repetição de extensores estão 100% corrigidas e validadas. Delimitadores matemáticos de qualquer altura agora escalam corretamente de acordo com o conteúdo.
