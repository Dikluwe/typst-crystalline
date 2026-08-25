# P1167 — auditar e nuclear o primeiro lote de tags HTML tipadas

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GATE ADR-0127 APROVADO EM 2026-08-25`
**Baseline cristalina:** working tree pós-P1166, ainda não commitada
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Gate:** ADR-0127 obrigatório antes de acrescentar bindings públicos ao módulo `html`

## Objetivo

Medir integralmente e nuclear o primeiro lote coerente de construtores HTML
tipados sobre o corte P1166. O lote candidato é a família estrutural/textual
fundamental:

```text
div, span, p, h1, h2, h3, h4, h5, h6,
strong, em, ul, ol, li, a, br
```

A lista é hipótese de trabalho, não decisão. O passo deve confrontá-la com a
tabela ratificada, assinaturas geradas e comportamento do exporter antes de
fixar o lote. Se alguma tag depender de casts ou semântica ainda sem owner,
separá-la para um passo posterior em vez de implementar parcialmente a
assinatura.

Este passo audita, atualiza L0 e para no gate. Não escreve código, testes RED,
hashes, staging ou commit.

## Estado herdado que deve ser revalidado

P1166 materializou:

- `Feature::Html` e `Features`, default vazio;
- `--features html` em compile/watch/eval;
- binding gated `html` com diagnóstico feature-off;
- módulo que expõe somente `html.elem`;
- `HtmlElem { tag, attrs, body }` e `Content::HtmlElem`;
- exporter de nós explícitos com attrs ordenados/escapados;
- `info.features.html = false` por default.

O exporter continua incompleto; não possui tabela void/raw, casts tipados,
CSS, MathML, `html.frame`, positions nem expansão HTML rica. Permanecem também
os achados P1165 de heading `<h1>/<h2>` e espaço antes de `<br>`.

## 1. Proveniência obrigatória

Antes de qualquer número ou classificação, registrar:

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

Como P1165/P1166 ainda podem estar não commitidos, declarar a lista exata de
ficheiros alterados. Não usar `--version` como prova do pin vanilla.

## 2. Fontes obrigatórias

Ler integralmente, sem listar/varrer `00_nucleo/context/` ou
`00_nucleo/materialization/`:

- `AGENTS.md` e `01_core/CLAUDE.md`;
- `00_nucleo/diagnosticos/typst-p1165-auditoria-html-feature-gated.md`;
- `00_nucleo/diagnosticos/typst-p1166-materializacao-html-elem.md`;
- `00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md`;
- `00_nucleo/adr/typst-adr-0108-disciplina-anti-deriva.md`;
- `00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md`;
- `00_nucleo/adr/typst-adr-0128-html-target-semantico.md`;
- `00_nucleo/prompts/compiler/stdlib/html.md`;
- `00_nucleo/prompts/entities/html.md`;
- `00_nucleo/prompts/entities/content.md`;
- `00_nucleo/prompts/infra/export/html.md`;
- `lab/typst-original/crates/typst-html/src/lib.rs`;
- `lab/typst-original/crates/typst-html/src/typed.rs`;
- o ficheiro pinado `typst-assets` `94dcb99/files/html/data.rs` já localizado
  por P1165, somente nas entradas do lote candidato e tabelas referenciadas.

Ler os L0s apontados pelos headers de qualquer owner adicional encontrado.

## 3. Medir o namespace antes de escolher o lote

Com `--features html`, medir cada candidato no vanilla ratificado:

```typst
repr(type(html.TAG))
repr(html.TAG)
repr(html.TAG())
repr(html.TAG[body])
```

Para cada tag, extrair da fonte gerada e confirmar por sondas:

```text
nome
tipo público
lista completa de atributos named
tipo/cast de cada atributo
default/unset
body ausente ou presente
tipo do body (content ou string raw)
tag void ou normal
repr observável
diagnósticos de named desconhecido, cast inválido e body inválido
HTML emitido em posição top-level e aninhada
```

Não inferir assinatura pela especificação HTML geral. A tabela Typst pinada é
a fonte da assinatura pública.

## 4. Sondas mínimas por família

### 4.1 Contentores genéricos

Medir `div`, `span` e `p`:

- sem args/body;
- body markup simples e body contendo `html.elem`;
- atributos globais realmente oferecidos (`id`, `class`, `lang`, `dir`,
  `title`, `hidden`, `style`, `role`, `data-*` somente se encontrados);
- atributo conhecido com tipo válido/inválido;
- atributo desconhecido.

### 4.2 Headings e ênfase

Medir `h1..h6`, `strong` e `em` com/sem body e attrs. Comparar explicitamente:

- `html.h1[H]` versus markup `= H` sob target HTML;
- `html.strong[X]` versus markup `*X*`;
- `html.em[X]` versus markup `_X_`;
- `repr`, DOM e show/set behavior observável.

Não usar a divergência do heading automático para alterar a tag explícita.

### 4.3 Listas

Medir `ul`, `ol` e `li`, incluindo os attrs específicos encontrados
(`start`, `reversed`, `type`, `value`, se e somente se constarem da tabela),
casts, nesting e body vazio. Comparar com listas Typst automáticas apenas para
classificar interação; não puxar realização de listas para o lote sem L0.

### 4.4 Link e void

Medir `a` com todos os attrs realmente declarados, em especial `href`,
`target`, `rel`, `download`, `hreflang`, `type` e `referrerpolicy` se
confirmados. Para `br`, confirmar:

- ausência de body na assinatura;
- repr;
- erro com body posicional;
- serialização `<br>` versus `<br></br>`;
- interação com whitespace e texto adjacente.

`br` é o canário da futura tabela void. Se suportá-lo corretamente exigir uma
classificação geral já medida, criar o tipo/tabela mínima declarativa; não
codificar `if tag == "br"` ad hoc.

## 5. Repetir no cristalino P1166

Para cada sonda vanilla, executar o equivalente no cristalino com feature
off/on. Esperado antes da implementação:

- feature off continua diagnóstico gated;
- feature on: `html.TAG` é campo ausente;
- `html.elem("tag", ...)` pode produzir DOM equivalente somente para attrs
  string genéricos, mas não prova a assinatura tipada;
- exporter atual fecha tags void incorretamente e isso deve ser medido, não
  presumido.

Registrar comando, stdout, stderr e exit code. Separar ausência do binding de
capacidade equivalente via `html.elem`.

## 6. Tabela de decisão por binding

Produzir diagnóstico P1167 com:

```text
binding | assinatura vanilla | casts/void | cristalino P1166 |
MATCH/PARTIAL/ABSENT | dependências | entra no lote?
```

Critério de entrada:

- assinatura e casts totalmente medidos;
- representação existente `HtmlElem` preserva todos os observáveis do corte;
- exporter consegue serializar sem ad hoc por binding;
- não exige CSS interno, context, location, frame, MathML ou introspecção;
- testes de língua podem ser escritos antes do código.

Uma tag é removida do lote se qualquer condição falhar. Registrar qual passo
futuro completa a dependência.

## 7. Classificação ADR-0107/0108

São observáveis de linguagem: presença do binding sob feature, tipo função,
assinatura, casts, unset/default, aceitação de body, repr, diagnóstico e DOM
emitido. São mecânica: geração da tabela, `LazyLock`, `NativeFuncData`, bump
allocator, enums Rust de attrs e organização do crate vanilla.

Marcar inferências e refutadores. Em particular:

- `html.elem` conseguir produzir uma tag não torna o binding tipado MATCH;
- atributo que serializa como string não implica que aceite `str` na API;
- tag void conhecida por HTML não implica assinatura sem body no Typst;
- igualdade de bytes numa fixture não prova completude do constructor.

## 8. Atualizar L0 e parar

Somente após a medição:

1. atualizar `00_nucleo/prompts/compiler/stdlib/html.md` com a lista aprovada,
   assinaturas completas, casts, repr, erros e divisão explícita entre passos;
2. atualizar `00_nucleo/prompts/entities/html.md` se forem necessários tipos
   declarativos de tag/atributo/void;
3. atualizar `00_nucleo/prompts/infra/export/html.md` com serialização normal,
   void e nesting do lote;
4. atualizar `00_nucleo/prompts/entities/content.md` somente se o contrato de
   `Content::HtmlElem` mudar;
5. atualizar eval/pipeline/wiring apenas se a medição exigir comportamento
   além do módulo/exporter já aprovado.

Os L0s devem declarar explicitamente o que permanece incompleto e nomear os
passos futuros. Não ressellar hashes neste passo.

Então **PARAR NO GATE ADR-0127** e pedir aprovação objetiva do conjunto exato
de bindings e assinaturas, pois cada função adicionada amplia contrato público.

## Entrega obrigatória

1. proveniência reproduzível;
2. inventário completo das assinaturas do lote candidato;
3. sondas vanilla e cristalinas feature off/on;
4. classificação normal/void/raw e casts por atributo;
5. tabela de decisão com lote final menor ou igual ao candidato;
6. gaps de representação/exporter;
7. diagnóstico `00_nucleo/diagnosticos/typst-p1167-auditoria-tags-html-lote1.md`;
8. L0s atualizados sem hash;
9. lista exata de contratos públicos no gate;
10. nenhuma alteração L1–L4, teste, staging ou commit.

## Critérios de aceitação

- todas as assinaturas escolhidas vêm da fonte/sonda ratificada;
- nenhum atributo é implementado parcialmente ou convertido permissivamente;
- `html.elem` e tags tipadas permanecem conceitos distintos;
- `br` não recebe tratamento ad hoc sem tabela declarada;
- feature off não regride;
- scope-outs e passo de conclusão de cada família ficam explícitos;
- `git diff --check` limpo;
- parada efetiva no gate ADR-0127.

## Próximo passo após aprovação

Escrever P1168 para ressellar os L0s, produzir testes RED do lote exato
aprovado e materializar os constructors por tabela estática, com exporter e
casts globais completos. O lote proposto pelo diagnóstico é `div`, `span`,
`p`, `h1..h6`, `strong`, `em` e `ul`; `ol/li`, `a` e `br` foram adiados para
P1169, P1170 e P1171, respetivamente.
