# Diagnóstico P1140.15 — origem e âncora RTL nas regiões L1

**Data:** 2026-08-24  
**Passo:** `00_nucleo/materialization/typst-passo-1140.15.md`  
**Estado:** concluído

## Proveniência

Validação final em `45b547073d7686cdd5d3e3030c82de3e22ec395f`, working tree não
commitado, em `2026-08-24T13:09:37-03:00`. Estado acumulado no momento da
medição: `73 files changed, 548 insertions(+), 486 deletions(-)` por
`git diff HEAD --stat`. O conjunto inclui as alterações ainda não commitadas
de P1140.14 e P1140.15.

Os números geométricos abaixo são oracles de teste derivados da fonte da
sonda; não foram introduzidos como constantes de produção.

## Matriz antes/depois

| Fatia | Antes | Causa | Depois |
|---|---|---|---|
| Margens assimétricas | direita reutilizava a margem esquerda; `SetPage` colapsava quatro lados num `f64` | perda de informação entre avaliação, entidade e `PageConfig` | `PageMarginSpec` preserva auto/valor por lado; `PageMargins` resolve os quatro lados; consumidores usam o lado físico correspondente |
| `#columns()` RTL | direção externa não era vista e a primeira coluna lógica ficava à esquerda | `body_dir` só inspecionava estilos embutidos no body | fallback para `StyleChain::custom("text.dir")`; offsets físicos permanecem estáticos e só a ordem lógica é invertida |
| bloco RTL explícito | shape de 60pt começava em `x=12pt` | `block.rs` fixava a origem em `saved_line_start` independentemente da direção | sem alinhamento físico externo, a origem lógica usa `180 - 38 - 60 = 82pt` |

## Contratos e implementação

O gate ADR-0127 foi acionado porque a margem de página alterou contrato
público e comportamento padrão. Após confirmação do dono foram atualizados e
resselados os L0s de tipos de layout, conteúdo, avaliação, layout, colunas e
atomização.

A implementação ficou atomizada nos donos existentes:

- `entities/layout_types.rs`: `PageMarginSpec` e `PageMargins`;
- `eval/rules.rs`: precedência `left/right/top/bottom > x/y > rest`;
- consumidores de layout: lado físico correspondente;
- `layout/columns.rs`: seleção da ordem lógica;
- `layout/block.rs`: origem lógica do bloco.

Não foi criado helper global de “correção RTL”, não houve reflow em L3 e
nenhum valor empírico entrou na fórmula de produção.

## RED → GREEN

O teste de bloco registrou RED com origem `x=12pt`, contra `x=82pt` esperado.
Após a mudança em `block.rs`, ficou GREEN. Os testes de margens assimétricas e
herança RTL de `#columns()` também ficaram GREEN:

- `p1140_15_margens_assimetricas_chegam_ao_layout`;
- `p1140_15_columns_funcao_herda_direcao_rtl_externa`;
- `p1140_15_bloco_rtl_ancora_na_aresta_logica_inicial`.

## Validação final

- `cargo test -p typst-core p1140_15_ -- --nocapture`: 3 passed;
- `cargo check --workspace --all-targets`: passou sem erros;
- `cargo test -p typst-core --lib`: 5159 passed, 0 failed;
- `crystalline-lint --fix-hashes .`: 0 drift warnings após resselo;
- `crystalline-lint .`: exit 0, sem violações bloqueantes.

## Remanescentes

As três divergências que motivaram P1140.15 estão fechadas. Permanecem fora
do escopo deste passo bidi Unicode intralinha, escrita vertical e igualdade
mecânica de PDF. A próxima frente deve voltar à matriz diferencial de
P1140.x e escolher a primeira divergência de linguagem ainda reproduzível,
sem reabrir o reflow removido em P1140.14.
