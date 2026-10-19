# P1172 — auditar realização phrasing e agrupamento em parágrafos HTML

**Data:** 2026-08-25  
**Estado:** `EXECUTADO — FLUXO CONTÍNUO ADR-0127`  
**Baseline cristalina:** working tree P1171.1 GREEN, ainda não staged  
**Baseline de linguagem:** vanilla ratificado `a51e02804`  
**Gate:** classificar pelo ADR-0127 após medir; parar somente se a solução mudar
fase, contrato público ou comportamento além da correção de paridade

## Objetivo

Auditar e nuclear o gap remanescente de realização HTML: conteúdo phrasing no
topo — por exemplo `html.a` — deve ser agrupado em `<p>`, enquanto elementos
block permanecem siblings diretos. Determinar a tabela/categoria mínima, os
boundaries de parágrafo e a camada dona antes de propor implementação.

Este passo não adiciona constructors tipados, tags, CSS, MathML, raw, frame ou
pretty printing. Não altera código nem hashes. Se a medição localizar a
correção inteiramente no exporter L3 existente, ela é paridade interna e pode
seguir fluxo contínuo em P1172.1; se exigir mover realização entre fases,
alterar entidade ou API, parar no gate ADR-0127.

## 1. Proveniência

Registrar:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
git diff --cached --stat
sha256sum /usr/local/bin/typst ./target/debug/typst
/usr/local/bin/typst --version
./target/debug/typst --version
```

Declarar working tree não commitado P1168–P1171.1, todos os paths alterados e
índice vazio. Não usar `--version` isolado como prova do vanilla.

## 2. Fontes obrigatórias

Ler integralmente, sem acessar/listar `00_nucleo/context/` ou
`00_nucleo/materialization/`:

- `AGENTS.md`, `01_core/CLAUDE.md`;
- ADR-0107, ADR-0108, ADR-0109, ADR-0127 e ADR-0128;
- P1170–P1171.1 e seus diagnósticos;
- L0s `compiler/stdlib/html.md`, `entities/html.md`, `entities/content.md` e
  `infra/export/html.md`;
- `03_infra/src/export/html.rs`;
- vanilla `typst-html/src/tag.rs:185-317` e `:495-546`;
- vanilla `fragment.rs`, `convert.rs`, `rules.rs` e pontos de
  `RealizationKind::Fragment` que criam parágrafos.

Medir antes de copiar tabelas. `tag.rs:504-546` declara intenção explícita:
agrupar tags phrasing visíveis quando parágrafos são forçados; API HTML
low-level isolada não força parágrafos por si só.

## 3. Reconstruir as categorias da fonte

Extrair com `file:line`, sem transcrever de memória:

1. conjunto `is_phrasing_content`;
2. exceções `display:none` excluídas por `should_group_into_pars`;
3. tags transparentes (`a`, `ins`, `del`, `noscript`, `map`) e a decisão
   vanilla atual de agrupá-las incondicionalmente;
4. tags block/default display usadas como boundaries;
5. `br` como whitespace-collapsing, já materializado em P1171.1;
6. condições que “forçam” parágrafos: topo, coexistência com block Typst e
   `parbreak`;
7. distinção entre fragmento block, inline e math.

Produzir inventário estático mínimo necessário ao subconjunto cristalino já
representável. Não importar a tabela inteira se apenas tags hoje construíveis
podem alcançar o exporter; declarar expansão futura.

## 4. Matriz vanilla de agrupamento

Medir `repr` e HTML compacto, variando um fator por fixture:

### Topo

- texto puro;
- `span`, `strong`, `em`, `a`, `br` isolados;
- dois phrasing adjacentes, na mesma linha e formatados;
- texto + phrasing + texto;
- `div`, `p`, headings, `ul`, `ol` isolados;
- phrasing antes/depois de block;
- phrasing separado por `parbreak`;
- body vazio e somente whitespace.

### Dentro de elemento

- phrasing dentro de `div` e `li` — não criar `<p>` sem prova;
- phrasing dentro de `p` — nunca aninhar `<p>`;
- `a` com body phrasing e com body block;
- `br` entre texto;
- sequência com `Content::Linebreak` Typst e `html.br`.

### API genérica versus tipada

Comparar `html.a` com `html.elem("a")`, `html.div` com
`html.elem("div")` e `html.br` com `html.elem("br")`. Determinar se o
agrupamento depende apenas da tag/morfologia ou da origem do constructor.

## 5. Boundaries e estado de parágrafo

Medir sequências canónicas:

```text
phrasing phrasing                 -> um <p>
phrasing block phrasing           -> <p>…</p><block>…</block><p>…</p>
block whitespace block            -> dois blocks, sem <p> vazio
phrasing parbreak phrasing        -> dois <p>
brasing/void dentro de phrasing   -> mesmo <p>
somente whitespace                -> nenhum <p>
```

Registrar tratamento de espaços nas bordas, entre siblings e junto aos
boundaries. Revalidar P1171.1; não duplicar normalização em dois passes se um
único walker puder ser dono das duas decisões.

## 6. Baseline cristalina e localização causal

Repetir a matriz com feature HTML ligada. Classificar:

```text
fixture | Content/repr vanilla | Content/repr cristalino |
DOM vanilla | DOM cristalino | origem causal | MATCH/PARTIAL/ABSENT
```

Hipótese inicial: Content coincide e `export_html` já possui um agrupador
rudimentar (`block_sequence`), mas trata todo `HtmlElem` como boundary block.
A correção mínima seria classificar `HtmlElem` phrasing versus block dentro de
L3 e alimentar phrasing no buffer de parágrafo.

Refutadores:

- repr/Content diverge antes do exporter;
- grouping depende de estilos/show rules não transportados;
- body de tag transparente exige realização recursiva não representável;
- correção exige mudar a ordem eval → realização → export.

## 7. Desenho a decidir somente após medir

Comparar pelo menos:

- tabela estática L3 de tags groupable/block;
- funções puras `is_phrasing`/`should_group` no owner do exporter;
- classificação compartilhada L1 somente se houver consumidor L1 real;
- walker de sequência com buffer de parágrafo e boundaries explícitos.

Rejeitar:

- booleano público em `HtmlElem`;
- `match` por tag espalhado;
- inferência por “tag não conhecida = inline”;
- wrapper `<p>` incondicional para qualquer `HtmlElem`;
- dependência de `lab` ou crate de assets em produção;
- mudança em parser/eval sem refutador comprovado.

## 8. L0 e diagnóstico

Criar
`00_nucleo/diagnosticos/typst-p1172-auditoria-html-agrupamento-phrasing.md`
com proveniência, inventário, matriz, boundaries, classificação e decisão.

Somente após medir:

1. atualizar `00_nucleo/prompts/infra/export/html.md` com a regra completa;
2. atualizar `entities/html.md` somente se a representação atual for
   comprovadamente insuficiente;
3. atualizar L0 de pipeline/realização somente se houver mudança de fase;
4. não alterar `compiler/stdlib/html.md` sem contrato de constructor;
5. declarar tags ainda ausentes e casos não representáveis fora.

Não ressellar nem alterar L1–L4 neste passo.

## 9. Decisão ADR-0127

Se a solução for apenas correção interna no `export_html` existente, registrar
“fluxo contínuo ADR-0127” e escrever P1172.1 para RED→GREEN sem pedir novo
consentimento de contrato. Se houver campo público, API, default ou mudança de
fase, **PARAR** e pedir aprovação exata antes de P1172.1.

## Critérios de aceitação

- categorias vêm da fonte com `file:line`;
- matriz cobre phrasing, block, parbreak, void e whitespace;
- `a` top-level e aninhado são distinguidos corretamente;
- API genérica/tipada é comparada;
- camada causal é provada por repr + DOM;
- solução proposta preserva enum fechado e topologia de imports;
- L0 precede código e não é ressellado na auditoria;
- `git diff --check` limpo e índice vazio;
- decisão explícita entre fluxo contínuo e gate.

## Próximo passo

P1172.1 materializa somente o agrupamento aprovado/classificado. Novos
constructors tipados retomam em passo posterior, sem serem acoplados a esta
correção de realização.
