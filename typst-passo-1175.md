# P1175 — auditar quarto lote global-only de tags HTML tipadas

**Data:** 2026-08-25
**Estado:** `EXECUTADO — PARADO NO GATE ADR-0127`
**Baseline cristalina:** working tree P1174.1 GREEN, ainda não staged
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Gate:** ADR-0127 obrigatório antes de acrescentar bindings públicos

## Objetivo

Auditar e nuclear o quarto lote candidato:

```text
nav picture pre s samp search section small sub sup u var
```

Confirmar contra fonte e binário pinados que cada candidata aceita exatamente
os 76 atributos globais P1168 e body content opcional, sem específicos,
void/raw ou constructor especial. Atualizar apenas os L0s que a medição exigir
e parar no gate ADR-0127 antes de resselo, testes ou código público.

## 1. Proveniência

Registrar no início:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
git diff --cached --stat
git diff --cached --quiet
sha256sum /usr/local/bin/typst ./target/debug/typst
/usr/local/bin/typst --version
./target/debug/typst --version
```

Declarar a working tree herdada de P1173.1–P1174.1, enumerar todos os paths e
confirmar o índice vazio. Não usar `--version` sozinho como prova do pin.

## 2. Fontes obrigatórias

Ler integralmente, sem acessar nem listar `00_nucleo/context/` ou
`00_nucleo/materialization/`:

- `AGENTS.md`, `01_core/CLAUDE.md`;
- ADR-0107, ADR-0108, ADR-0109, ADR-0127 e ADR-0128;
- P1167–P1174.1 e diagnósticos HTML relevantes;
- L0s `compiler/stdlib/html.md`, `entities/html.md` e
  `infra/export/html.md`, conferindo hashes vigentes;
- L1 `compiler/stdlib/html.rs` e L3 `export/html.rs`;
- vanilla pinado `typst-html/src/typed.rs`, `tag.rs`, `property.rs` e dados das
  12 entradas;
- inventário `superficie-linguagem-p1140.26.json` somente como índice.

Se a fonte gerada de assets não estiver disponível, registrar a limitação e
triangular lockfile, inventário e binário; não inventar `file:line`.

## 3. Composição do lote

Comparar mecanicamente cada lista de parâmetros com `html.div`, incluindo
nomes, ordem e metadados, e produzir:

```text
tag | total | globais | específicos | body | void/raw |
display | phrasing/flow/sectioning | regra contextual | decisão
```

Critério cumulativo:

- os mesmos 76 globais, sem extra ou ausência;
- body content posicional opcional;
- zero específicos;
- não void, raw nem escapable-raw;
- nenhuma realização especial no constructor.

Uma divergência retira somente a tag afetada. Categoria de conteúdo, display
e assinatura são eixos distintos.

## 4. Sondas vanilla

Para cada candidata, medir com `/usr/local/bin/typst eval --features html`:

```typst
repr(type(html.TAG))
repr(html.TAG())
repr(html.TAG[X])
repr(html.TAG(id: "k", class: "c", hidden: true)[X])
```

Confirmar function, `body: none`, Content, casts globais e ordem. Tentar em
cada tag `href`, `value` e `start`; medir também unknown named, segundo body e
body named em representantes block e phrasing. Não extrapolar diagnóstico
não medido.

## 5. Morfologia e exporter

Comparar fixtures tipadas vanilla com fixtures cristalinas pré-código feitas
por `html.elem`:

1. cada candidata vazia e com texto;
2. `nav[section[...]]`, `picture[...]`, `pre[...]` e `search[...]`;
3. as oito phrasing candidatas no topo e dentro de block;
4. mistura com tags P1168–P1174.1;
5. whitespace formatado entre block siblings e phrasing siblings;
6. atributos vazios e escaping.

Medir especificamente se `picture` vazio/com texto tem realização distinta e
se `pre` preserva whitespace textual além da serialização genérica. Confirmar
`BLOCK_TAGS` para `nav`, `pre`, `search`, `section` e
`GROUPABLE_PHRASING_TAGS` para `picture`, `s`, `samp`, `small`, `sub`, `sup`,
`u`, `var`. Qualquer diferença nova de export exige L0 L3 antes de correção.

## 6. Baseline cristalina

Com feature off, preservar o diagnóstico gated. Com feature on:

- provar as 12 candidatas ABSENT antes do código;
- provar os 40 bindings fechados até P1174.1 presentes;
- confirmar que `html.elem` aproxima o DOM, sem substituir binding tipado;
- classificar semântica, sintaxe e morfologia por tag como MATCH, PARTIAL ou
  ABSENT.

Bindings, assinaturas, casts, erros, repr e DOM são língua. Wrappers Rust,
macro e tabela estática são mecânica.

## 7. Decisão posterior à medição

Hipótese inicial: as sobreviventes requerem apenas wrappers para
`native_typed_html(tag, &[], args)` e entradas em `TYPED_TAGS`. Refutadores:
atributo específico, body distinto, raw/void, comportamento próprio de
constructor, estado novo de entidade ou regra L3 ainda ausente.

As 16 globais remanescentes — `body`, `caption`, `datalist`, `head`, `html`,
`noscript`, `rp`, `rt`, `ruby`, `summary`, `table`, `tbody`, `tfoot`, `thead`,
`title`, `tr` — ficam explicitamente fora. Documento, tabela, ruby,
escapable-raw e `noscript` exigem auditorias próprias; não puxá-los para o lote.

## 8. Diagnóstico, L0 e gate

Criar:

```text
00_nucleo/diagnosticos/typst-p1175-auditoria-html-tags-global-only-lote4.md
```

Somente depois da evidência:

1. atualizar `compiler/stdlib/html.md` com a lista exata e assinatura das
   sobreviventes;
2. atualizar `infra/export/html.md` somente se houver novo observável;
3. atualizar `entities/html.md` somente se a representação for insuficiente;
4. não ressellar e não escrever testes/código L1–L4.

Parar e pedir aprovação ADR-0127 para exatamente os bindings sobreviventes.
A aprovação não cobre tags retiradas ou fora do lote, específicos, novo
`AttrKind`, entidade, exporter, default ou pipeline.

## Critérios de aceitação

- composição e categorias medidas no pin `a51e02804`;
- sondas de repr/erros por tag e por classe;
- particularidades de `picture` e `pre` verificadas, não presumidas;
- estado cristalino pré-código e 40 bindings anteriores conferidos;
- L0 escrito somente após medição;
- nenhum resselo ou código;
- `git diff --check` limpo e índice vazio;
- parada efetiva no gate ADR-0127.

## Próximo passo após aprovação

Escrever P1175.1 para resselo, RED e materialização GREEN somente das tags
confirmadas.

## Resultado da execução

As 12 candidatas sobreviveram. A auditoria também encontrou uma correção L3
transversal: espaço entre inline siblings vazios precisa de proteção
`white-space: pre-wrap`, já observável com `html.span`. Os dois L0s foram
atualizados, mas nenhum hash foi ressellado e nenhum código/teste L1–L4 foi
escrito. O passo está parado antes dos bindings públicos no gate ADR-0127.
