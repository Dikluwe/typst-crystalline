# Auditoria retroactiva — fronteiras decididas antes da correcção do artefacto

**Data**: 2026-08-13
**Motivo**: `tools/analysis/cochange_metrics.py` foi corrigido em 2026-08-13 (artefacto de
atribuição de fronteira). A mesma ferramenta, **antes** da correcção, decidiu as fronteiras
de três fatiamentos já materializados em código. Esta auditoria mede se alguma dessas
fronteiras foi decidida por ruído.

## Proveniência

Árvore de trabalho não commitada sobre `HEAD = fffcd9060`, 2026-08-13. Cada família foi
medida sobre o histórico do monólito **antes** do fatiamento (os ficheiros já não existem;
`git log --follow` atravessa os renames `rules/`→`engine/`→`compiler/`):

| Família | Caminho medido | Fatiada em |
|---|---|---|
| operators | `01_core/src/engine/eval/operators.rs` | `302b1de85` (2026-08-12) |
| bindings | `01_core/src/compiler/eval/bindings.rs` | `d4145093c` (2026-08-12) |
| structural | `01_core/src/compiler/stdlib/structural.rs` | `0ddd054c1` (2026-08-12) |

Duas perguntas, medidas em separado para não se confundirem:

- **(A) Artefacto que *sustentava* uma fronteira** — cluster intra-nó que a versão antiga
  via e a corrigida não vê. É evidência a favor de manter dois itens juntos que, na
  verdade, nunca co-mudaram.
- **(B) Co-mudança real que *atravessa* uma fronteira** — cluster corrigido com itens de
  nós diferentes. É evidência contra a separação. Pares hub↔nó contam à parte: tocar o
  dispatcher ao acrescentar um braço é estrutural, não sinal de fronteira.

## Veredicto

| Família | Itens | Nós | Artefactos que inflavam clusters | Fronteiras decididas por artefacto | Clusters citados no relatório que sobrevivem |
|---|---:|---:|---:|---:|---|
| operators | 13 | 5 | **0** | **0** | 3/3 |
| bindings | 43 | 5 | 10 | **0** | 4/4 (3 membros individuais caem) |
| structural | 50 | 9 | 15 | **2** | 7/9 |

**Duas fronteiras em `structural` foram decididas por ruído.** As restantes sete, e as
famílias `operators` e `bindings`, sobrevivem à correcção.

## operators — intacto

Zero artefactos: nenhuma linha de banner foi atribuída a função anterior neste ficheiro
(as funções novas nasceram quase todas dentro do `match` de `eval_binary_op`, não como
itens de topo novos). As três afirmações do relatório verificam-se:

| Afirmação | Corrigida |
|---|---|
| `value_eq`+`values_eq` juntas no lote P818, nunca com `sanitize_length_nan` | confirma (`af4887e7e`) |
| `binary_mismatch`+`vanilla_type_name` criados juntos, nunca separados | confirma (`6740c2729`) |
| ordenação e igualdade nasceram no mesmo lote e nunca co-mudaram depois | confirma |

Observação lateral, não regressão: 7 dos 8 clusters reais são **hub↔nó**
(`eval_binary_op` + um nó), o que é o padrão dispatcher↔braço. A coesão *interna* dos nós
tem pouca evidência de co-mudança própria (só `error_formatting`) — as fronteiras de
`operators` sustentam-se nos critérios 2 e 4, e isso já estava escrito assim no relatório
("nós declarativos").

## bindings — intacto, com composição de clusters corrigida

Dez artefactos, todos a **inflar** clusters que existem de verdade. Nenhum criou um
cluster de nada. Confronto com a tabela do relatório:

| Cluster citado | Commit | Corrigido |
|---|---|---|
| `binding` — os 8 e só os 8 | `7a901edd4` | **8/8 confirmados** |
| `method_dispatch` — 5 funções | `c570d53e5` | 4/5 (`call_method_access` era artefacto) |
| `access` — 3 funções | `2b981e7e0` | 2/3 (`missing_key` era artefacto) |
| `value_methods` — 5 commits | vários | 4/5 (o par color+state de `6b321acc1` era artefacto) |
| `field_access` — sem commit isolador, cruza com selector/counter | P417/P493/P504 | **confirmado como cruzamento real** |

Os três membros que caem (`call_method_access`, `missing_key`, `eval_state_method`) têm,
cada um, evidência real independente no mesmo nó (`b806f562d` para os dois primeiros;
`969087ecf` e `43c252f2b` para o terceiro). Nenhuma colocação muda.

O cruzamento `field_access`↔`value_methods` (3 commits independentes: `ee0833edb`,
`048076f3e`, `21a98c64d`) **confirma-se como real** — e o relatório já o tinha declarado
como tal, em vez de o esconder. É o padrão dispatcher↔braço outra vez, com o dispatcher
dentro de um nó em vez do hub. A separação continua defensável; agora está medida com a
ferramenta certa.

## structural — duas fronteiras decididas por artefacto

Sete dos nove clusters citados no relatório sobrevivem: markup inline, listas, células,
cabeçalhos/rodapés, linhas, bibliografia, matemática. Dois não:

### 1. `flow` — o cluster citado não existe

O relatório cita `| fluxo | P806 — native_par+native_quote |`. Medição directa:

```
$ git show c98ffc8ac -U0 -- 01_core/src/compiler/stdlib/structural.rs | grep '^@@'
@@ -27 +27 @@ use crate::entities::geometry::Stroke;
@@ -655,0 +656,79 @@ pub fn native_quote(
@@ -3509,0 +3589,93 @@ mod tests {
```

`-655,0` = **zero linhas removidas**: é inserção pura de `native_par` (79 linhas) depois de
`native_quote`. O corpo de `native_quote` não foi tocado. O contexto do hunk mostra a
função *anterior*, e era esse contexto que a ferramenta lia como dono.

Com a atribuição corrigida, `flow` tem **zero** clusters internos. E o vanilla separa as
três nativas (`model/par.rs`, `model/quote.rs`, `model/footnote.rs`). Ou seja: o nó não
tem suporte do critério 3 **nem** do 4 — é agrupamento por inspecção ("blocos no fluxo
vertical").

### 2. `sectioning` — o cluster citado não existe; o núcleo real é outro

O relatório cita `| seccionamento | P763a-P765a — native_outline+native_title |`. Medição:

```
$ git show f36ca1abe -U0 -- 01_core/src/compiler/stdlib/structural.rs | grep '^@@'
@@ -374,0 +375,64 @@ pub fn native_outline(
```

Mesma assinatura: inserção pura de `native_title` depois de `native_outline`, que não
mudou.

O que a co-mudança sustenta, corrigida, é `native_lof`+`native_lot`+`native_outline`
(`2ca61c873`) — o núcleo `OutlineElem`. `heading`, `title` e `divider` estão no nó sem
suporte medido, e o vanilla também os separa. Os outros dois clusters aparentes que os
ligavam (`3197135b0` heading+outline, `fbc806c15` divider+heading) são o mesmo artefacto.

### O que foi feito com isto

Corrigi as **afirmações falsas nos L0s** — que era onde o dano estava, porque um L0 é o que
legitima o código:

- `structural/flow.md`: a linha "`native_par`/`native_quote` co-mudam (P806)" foi
  substituída pela medição real, pela nota de que o vanilla separa as três, e pelo registo
  explícito de que o agrupamento é por inspecção.
- `structural/sectioning.md`: a afirmação de co-mudança confirmada passou a citar o
  cluster que existe (`lof`+`lot`+`outline`) e a declarar que `heading`, `title` e
  `divider` estão lá sem suporte medido.

**Não re-fatiei código.** Dividir `flow` em três nós e separar `heading`/`title`/`divider`
de `sectioning` é uma decisão nova, com custo, e o vanilla apoia-a — mas é do dono do
projecto, não desta auditoria. O que a auditoria devia entregar é a medição e o registo, e
ambos os L0s ficam agora a dizer a verdade sobre o que os sustenta.

## Achado de segunda ordem — o mesmo bug, uma camada acima

O primeiro script desta auditoria reproduziu a mesma classe de erro: excluí as funções de
teste dos itens, mas **não limitei o alcance do último item de topo**, e as linhas
acrescentadas dentro de `mod tests` passaram a ser atribuídas à última função antes da
suite. Resultado: commits sobre `heading` apareciam a co-mudar com `native_table_vline`
(a última função de topo de `structural.rs`). Só notei porque o par não tinha explicação
plausível.

Duas consequências:

1. O script da auditoria passou a parar no início de `mod tests`.
2. A ferramenta passou a **rotular** os itens de teste (`test:<nome>`) em vez de os deixar
   confundir-se com unidades de fronteira. Rotular, não excluir: uma linha de teste tem de
   continuar a ter dono, senão volta a ser atribuída ao último item de topo — que é
   exactamente o artefacto original.

A lição de método: **um par de co-mudança sem mecanismo plausível é para verificar, não
para explicar**. Nos três casos falsos de `stdlib/text.rs`, nos dois de `structural.rs` e
neste, a assinatura foi sempre a mesma — e é reconhecível à mão:

> `git show <commit> -U0 -- <path> | grep '^@@'` → um hunk `-N,0 +M,K` (**zero linhas
> removidas**) cujo contexto é o nome da função *anterior* significa inserção pura: a
> função do contexto não mudou.

## Resumo

- **operators**: nada a fazer.
- **bindings**: nada a fazer; três membros de cluster caem sem mudar colocação, e o
  cruzamento `field_access`↔`value_methods` confirma-se como real e já estava declarado.
- **structural**: dois L0s corrigidos (`flow`, `sectioning`); o código fica como está, com
  a fronteira registada como não medida e a decisão de dividir em aberto, com dono.
- **método**: regra de reconhecimento do artefacto (assinatura da inserção pura) e
  rotulagem de itens de teste, ambas em `auditar-fatiamento.md` e na ferramenta.
