# P1165 — auditoria da superfície pública `html` feature-gated

**Data da medição:** 2026-08-25T12:48:57-03:00 a 2026-08-25T12:51:48-03:00  
**Baseline cristalina:** `30a6f11bcb8344f083a88edc8de89b3a1b5d6d07`  
**Baseline de linguagem:** vanilla ratificado `a51e02804`  
**Estado:** L0 redigido; aguarda gate ADR-0127

## 1. Proveniência

No início, `git status --short` continha apenas `?? typst-passo-1165.md` e
`git diff HEAD --stat` era vazio. Nenhum ficheiro de código estava alterado.

| artefato | SHA-256 | versão impressa |
|---|---|---|
| `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` | `typst 0.15.1 (e0e8ca4d)` |
| `./target/debug/typst` | `4213b506b9d23d2310850d52e35370cfef4178530295f5b0405f1614f0d3e759` | `typst 0.15.1 (4ed7f6a8)` |

A versão impressa não identifica o baseline. A ratificação do primeiro
binário continua sendo o pin `a51e02804`; a descrição do segundo está obsoleta
em relação ao HEAD e não foi usada para decidir.

## 2. Matriz medida

`typst eval --help` vanilla põe `--features <FEATURES>` nos argumentos comuns,
com valores `html`, `bundle` e `a11y-extras`. O cristalino não apresenta essa
flag nem `--target` no `eval`.

| sonda | vanilla | cristalino | classe |
|---|---|---|---|
| `eval 'repr(type(html))' --format raw` | exit 1; acesso recusado porque a feature não está habilitada, com hints para `--features html` | exit 1; `unknown variable html` | PARTIAL |
| mesma + `--features html` | exit 0; `module` | exit 2; argumento inesperado | ABSENT |
| `eval 'repr(html)' --features html --format raw` | exit 0; `<module html>` | flag ausente | ABSENT |
| `compile ... --format html`, feature off | exit 1; exige explicitamente `--features html` | exit 0 + warning experimental | EXTRA/default divergente |
| mesma + `--features html` | exit 0 + warning experimental | exit 2; flag ausente | ABSENT |
| `eval 'repr(target())' --target paged --features html` | exit 0; `"paged"` | `--target` ausente | ABSENT |
| mesma com `--target html` | exit 0; warning HTML; nesta sonda sem contexto o valor continuou `"paged"` | `--target` ausente | divergência a isolar |
| `info --format json` | `features.html: false` | `features.html: true` | default divergente |
| `info --format json --features html` | a flag não pertence ao subcomando `info`; exit 2 | idem | MATCH sintático |

Inferência: formato/target e feature são eixos distintos, pois o vanilla
recusa `--format html` sem a feature. É refutada se uma sonda ratificada de
compile HTML sem feature produzir sucesso. O resultado de `target()` em
`eval --target html` não autoriza concluir que o target seja ignorado em
documento contextual; a sonda sem contexto mede apenas o observável acima.

O documento mínimo `Hello *HTML*.` produziu nos dois exporters, quando
autorizados, o mesmo DOM semântico:
`html > head(meta charset, meta viewport) + body > p > text,strong,text`.
Os dois artefatos tinham 189 bytes sob a proveniência acima; isso é somente
reprodução desta fixture, não prova de paridade global.

A sentinela obrigatória mais rica produziu 258 bytes no vanilla e 259 no
cristalino. Há duas divergências semânticas observáveis: `= Heading` tornou-se
`<h2>` no vanilla e `<h1>` no cristalino; antes de `#linebreak()`, o vanilla
emitiu `.<br>` e o cristalino `. <br>`. Strong/emphasis e a estrutura de
parágrafos coincidiram. Esses achados são dívida do exporter/eval já existente
e não foram puxados para a implementação do gate.

## 3. Namespace vanilla completo

Fonte ratificada: `typst-library/src/lib.rs:369` registra o binding com
`Feature::Html`; `typst-html/src/lib.rs:34-41` cria o módulo; `typed.rs:31-43`
registra a tabela gerada de tags. A tabela `typst-assets` pinada em `94dcb99`
contém 112 tags. O namespace possui 114 bindings públicos:

- `html.elem(tag, attrs: (:), body: none)`: elemento `elem`; `tag` obrigatório,
  `attrs` named dobrável, body posicional opcional. `repr` medido:
  `elem(tag: "article", attrs: (lang: "pt"), body: [Olá])`. Tag com espaço
  erra `the character " " is not valid in a tag name`.
- `html.frame(body)`: elemento `frame`, body posicional obrigatório; `repr`
  medido `frame(body: [Olá])`.
- 112 construtores tipados, todos funções nativas geradas, com atributos named
  específicos da tag e body posicional opcional salvo tags void/raw:
  `a, abbr, address, area, article, aside, audio, b, base, bdi, bdo,
  blockquote, body, br, button, canvas, caption, cite, code, col, colgroup,
  data, datalist, dd, del, details, dfn, dialog, div, dl, dt, em, embed,
  fieldset, figcaption, figure, footer, form, h1, h2, h3, h4, h5, h6, head,
  header, hgroup, hr, html, i, iframe, img, input, ins, kbd, label, legend,
  li, link, main, map, mark, menu, meta, meter, nav, noscript, object, ol,
  optgroup, option, output, p, picture, pre, progress, q, rp, rt, ruby, s,
  samp, script, search, section, select, slot, small, source, span, strong,
  style, sub, summary, sup, table, tbody, td, template, textarea, tfoot, th,
  thead, time, title, tr, track, u, ul, var, video, wbr`.

Exemplo medido: `html.div(class: "x")[Olá]` representa
`elem(tag: "div", attrs: (class: "x"), body: [Olá])`; atributo desconhecido
erra `unexpected argument`. `typed.rs:49-82` mostra que as assinaturas e casts
são derivados da tabela, tags raw recebem string e tags void não recebem body.
Logo, nomes conhecidos de HTML não bastam para reconstruir a assinatura.

## 4. Estado cristalino e gaps

| superfície vanilla | estado cristalino | classe | owner/L0 |
|---|---|---|---|
| feature `html`, default off | não há modelo; `info` fixa true em `04_wiring/src/main.rs:161` | ABSENT/EXTRA | L2/L4; `shell/cli.md`, `wiring.md` |
| binding gated `html` | `make_stdlib` em `01_core/src/compiler/eval/mod.rs:1391` não o registra | ABSENT | L1 eval/stdlib; novo `stdlib/html.md` |
| `html.elem`/`html.frame`/tags | nenhuma nativa | ABSENT | L1 stdlib |
| nó HTML com tag/attrs/body | `Content` em `entities/content.rs:135` não o representa | ABSENT | L1 entidade; novo `entities/html.md` |
| target eval | `EvalTarget { Paged, Html }` em `eval/mod.rs:266`; `native_target` em `stdlib/foundations/query.rs:115` | PARTIAL | `compiler/eval.md` |
| exporter semântico | `03_infra/src/export/html.rs:12`; subconjunto explícito | PARTIAL | `infra/export/html.md` |
| pipeline HTML | `03_infra/src/pipeline.rs:130` avalia target HTML e exporta `Content` | MATCH no corte atual | `infra/pipeline.md` |
| output/dispatch HTML | formato em `02_shell/src/cli.rs:785`; dispatch em `04_wiring/src/main.rs:371` | EXTRA sem gate | `shell/cli.md`, `wiring.md` |
| profundidade HTML | helper puro em `entities/world_types.rs:405`, ainda sem fio público | PARTIAL | futuro |
| DOM/CSS/MathML/positions | ausentes | ABSENT | cortes futuros |

O exporter existente não legitima o módulo: serialização L3 e superfície de
linguagem L1 são contratos separados.

## 5. Classificação ADR-0107/0108

São linguagem/observáveis: existência condicional do binding, default off,
diagnóstico, nomes e assinaturas, `repr`/fields, casts de atributos, estrutura
DOM e HTML emitido. São mecânica: crate `typst-html`, geração da tabela,
representações Rust, algoritmo de expansão e divisão interna entre módulos.
Inferência arquitetural: uma variante dedicada de `Content` é o menor
transporte cristalino estático; seria refutada por uma representação L1 já
existente capaz de preservar tag, attrs e body sem perda semântica — ela não
foi encontrada.

## 6. Cortes verticais propostos

1. **P1166 — gate e `html.elem`:** `Feature::{Html}`, coleção pública de
   features default vazia, `--features html`, propagação L2→L4→L3→L1,
   diagnóstico do binding desligado, `HtmlElem` L1 e serialização. Mantém tags
   tipadas, frame, expansão rica, CSS/MathML/introspecção fora.
2. **P1167 — tags tipadas fundamentais:** tabela declarativa inicial para
   estrutura/texto (`div`, `span`, `p`, headings, listas, links), com L0
   explicitamente incompleto até os lotes seguintes.
3. **P1168+ — completar as 112 assinaturas por famílias:** tabelas, forms,
   mídia e metadata/raw/void, medindo casts e diagnósticos por família.
4. **Posteriores:** `html.frame`, whitespace/expansão, MathML/CSS,
   introspecção, anchors e positions.

O Corte A isolado foi rejeitado como materialização útil: criaria uma feature
sem binding. O primeiro corte combina gate + representação mínima + `elem`,
mas não declara paridade do namespace.

## 7. ADR-0128 e contratos no gate

ADR-0128 deve ser ratificada e receber adendo: o pipeline semântico continua
válido, mas formato HTML não habilita a feature; default é off; módulo público
e expansão são incrementais. A alteração de estado/adendo aguarda o dono.

Contratos sujeitos ao gate ADR-0127: enum/coleção pública de features; flag
`--features`; default HTML off; propagação pelos entrypoints; binding global
`html`; entidade/variante pública HTML; `html.elem`; diagnóstico feature-off;
e mudança de `info.features.html` de true fixo para estado efetivo.

**PARAGEM:** nenhum hash foi ressellado, nenhum teste RED ou código L1–L4 foi
escrito e nada foi staged.
