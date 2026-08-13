# Passo 1024 — Catálogo do P1021: sistémicos, bugs e início do Bloco 3

**Data**: 2026-08-13

## Proveniência e uma discrepância de base

Medições na árvore de trabalho, não commitada, sobre `HEAD = 1de103cef`.

**Os números do plano não reproduzem.** O plano declara como base "489 achados, 80 prompts,
134 graves". Existem duas fontes e nenhuma dá esses números:

| Fonte | Achados | Prompts | Graves |
|---|---:|---:|---:|
| Relatório P1021 commitado (`typst-passo-1021-relatorio.md`) | 36 | 5 | 24 |
| Catálogo completo (`temp/p1021/normalized_findings.md`) | 1085 | 296 | 203 |
| Plano do P1024 | 489 | 80 | 134 |

O relatório commitado cobre **uma partição** (o seu anexo lista as excluídas). O catálogo
completo é o ficheiro normalizado em `temp/`. Os 489/80/134 do plano não são derivável de
nenhum dos dois sem um filtro que não está registado — pela regra de proveniência do
projecto, não os uso para fechar nada. Trabalhei sobre o catálogo completo, com os números
que consigo medir e citar. Igualmente: o plano fala de "83 achados do Bloco 3"; a medição dá
**166** (90 graves + 58 leves + 18 sem severidade classificada).

## Parte 1 — as duas frases sistémicas (fechada)

**Fonte comum confirmada** (caso 1 do plano): todos os prompts de elemento já citavam
`entities/elements/_comum.md` na linha de cabeçalho. A correcção foi na fonte.

Alcance medido: **67 prompts** em `entities/elements/` usam pelo menos um dos termos — 54
usam "braço atual/actual", 56 usam "Não-locatável".

Escrita a secção **§A.0 — Glossário** em `_comum.md`, com as duas definições feitas por
medição, não por paráfrase:

- **§A.0.1 `Não-locatável`** — três condições simultâneas: estar no grupo `=> false` do match
  exaustivo de `is_locatable` (`compiler/introspect/locatable.rs:217`); `extract_payload(c)`
  devolver `None` (invariante declarada no próprio módulo, `locatable.rs:10-11`); e manter os
  defaults `None` de `element_kind()`/`to_payload()` no trait. Com a consequência mecânica
  medida em `compiler/layout/mod.rs:993-994` (`advance_locator_if_locatable`): não consome
  slot do `Locator`, não recebe `Location`, não entra em `runtime.positions` — logo não é
  alcançável por `query` nem serve de âncora a `counter`/`state`.
- **§A.0.2 `braço do hub`** — o hub é o `enum Content` e os seus matches exaustivos, listados
  com `file:line`: `elem_name:1258`, `is_empty:2566`, `plain_text:2671`,
  `get_field:3015` (`match (self, field)`), `map_content:3058`, `map_text:3308`. O sexto eixo,
  a extracção de payload, é o único **fora** de `content.rs`
  (`compiler/introspect/extract_payload.rs`) e é absorvido pelo trait via
  `element_kind()`/`to_payload()`.

Substituição da linha de referência em cada prompt afectado, para nomear o glossário:
42 na forma "Trait: ver …" → "Trait e glossário (§A.0): ver …"; 24 na forma "Trait e regras
partilhadas: ver …" → "Trait, regras partilhadas e glossário (§A.0): ver …"; `curve.md` não
tinha referência nenhuma e recebeu-a explicitamente (§A.0.1).

**Verificação, não presunção** (o plano exigia-a): 67/67 prompts que usam um dos termos têm
agora caminho para a definição; **zero** sem caminho. E a fonte contém, verificado
programaticamente, as duas secções, a invariante do `extract_payload`, a citação de
`locatable.rs:217` e a do gate do layout.

**Rendimento no catálogo**: 27 achados de Bloco 2 em 21 prompts distintos citam estes termos
(3 graves, 24 leves) — de 182 achados de Bloco 2 no total. Uma correcção na fonte, 27
achados endereçados.

## Parte 2 — os dois achados tipo-bug (fechada)

Nos dois casos o lado errado era o **L0**, e o código estava correcto. Medido antes de
decidir, como o plano exige.

### `pagebreak.md` — contradição interna sobre `Hash`

- Código: `entities/elements/pagebreak.rs:20` é `#[derive(Debug, Clone, PartialEq, Hash)]`;
  **não existe** `impl Hash` manual no ficheiro. `entities/parity.rs:26` deriva `Hash`
  também (`Copy + Eq` sem floats) — que é a dependência de lote já registada na nota do
  próprio L0.
- O L0 dizia as duas coisas: a secção `Struct` mostrava o derive (correcto), a secção
  `Critério` dizia "`Hash` manual via Debug" (errado). A intenção original está escrita na
  nota da secção `Struct` e bate com o código; o Debug-hash é o precedente de
  `HSpace`/`VSpace`, que carregam `Length`/`f64`.
- Corrigida a linha do `Critério` para "`Hash` **por derive** (não Debug-hash)", com a
  medição citada. **Zero alteração de código.**

### `outline.md` — exemplo inválido

O achado apontava `indent: true` para um campo `OutlineIndent`. A medição encontrou **dois**
erros, não um (`entities/elements/outline.rs:87-94`):

1. a assinatura documentada tinha **2** parâmetros (`title`, `target`) com `depth: 3,
   indent: true` fixos no corpo; a real tem **4** — `depth` e `indent` deixaram de ser fixos;
2. `indent: true` **não compila**: o campo é `OutlineIndent`
   (`Auto | Bool(bool) | Length | Function`), logo o valor válido é `OutlineIndent::Bool(true)`,
   não o `bool` cru.

Caso 2 do plano (o tipo não aceita `bool` cru) → corrigido o exemplo. O bloco novo é
**transcrição literal** das linhas 87-94 do ficheiro, verificado programaticamente
(`bloco do código presente no L0 verbatim: True`) — é a forma mais forte de "compilar o
exemplo": o bloco *é* código que compila como parte do crate. Documentados também os
defaults reais de `lof`/`lot` (`depth = 1`, `indent = OutlineIndent::Bool(false)`,
`entities/content.rs:2153-2161`).

## Parte 3 — Bloco 3 (aberta, com o primeiro cluster fechado)

Medição do catálogo: **90 achados graves de Bloco 3, em 64 prompts** (22 da família math, 68
restantes). Lista ordenada e separada por fonte de evidência em
`temp/p1024/bloco3_graves.md`, para o passo seguinte não a ter de rederivar.

Fontes de evidência disponíveis, verificadas: o corpus `00_nucleo/corpus-docs/math/` (20
ficheiros `.typ`) e a fonte vanilla em quarentena (`lab/typst-original`).

> **Correcção de identificação do alvo** (feita no passo seguinte, 2026-08-13). A versão
> original desta secção chamava `lab/typst-original` "0.15.0 — a referência de paridade
> declarada" e tratava `/usr/local/bin/typst` (0.15.1) como "versão diferente do alvo".
> **Ambas as afirmações estavam erradas**: o alvo ratificado (2026-08-11) é upstream/main
> `a51e02804`, e os dois binários de referência — `lab/typst-original/target/release/typst`
> e `/usr/local/bin/typst` — são esse mesmo build (reportam `typst 0.15.1 (e0e8ca4d)`, hash
> do nosso repo porque o lab não tem `.git`). O que li como "0.15.0" foi
> `./target/release/typst`, que é **o cristalino**. Ver `CLAUDE.md` §"Referência de
> paridade".
>
> **As medições deste passo não mudam**: foram feitas contra a fonte vendorizada em
> `lab/typst-original`, que é o baseline ratificado — só o rótulo estava errado.

### Cluster fechado — a "margem de 10% do vanilla" não é do vanilla

Dois achados (`cases.md`, `matrix.md`) afirmavam que a folga de altura dos delimitadores de
grelha "aplica a margem de 10% **do vanilla**". Medição dos dois lados:

| | cristalino | vanilla 0.15.0 |
|---|---|---|
| operação | **multiplica** a altura da grelha | **subtrai** do alvo de esticamento |
| fórmula | `grid_height_pt = (ascent + descent) * 1.1` (`compiler/math/layout/mod.rs:390`) | `let short_target = target - short_fall;` (`typst-layout/src/math/fragment/glyph.rs:271`) |
| grandeza | 10% proporcional à altura | `DELIM_SHORT_FALL = Em::new(0.1)` — absoluto ao corpo (`typst-library/src/math/lr.rs:17`) |
| sentido | alvo **maior** que a tinta | alvo **menor** que o pedido |

É o padrão do P996 (afirmação além do que a fonte sustenta), agravado: não é imprecisão de
grau, é **sentido oposto**. A 11pt o vanilla encurta o alvo ~1,1pt independentemente da
altura; o cristalino aumenta-o em 10% — ~1,4pt numa grelha de 14pt, ~5pt numa de 50pt. **A
divergência cresce com a altura da matriz.**

Correcção (caso 3 do plano): a afirmação foi corrigida para o que a fonte sustenta, na fonte
partilhada — `compiler/math/layout/_comum.md` §P912-folga, com a tabela acima —, e os dois
nós passam a remeter para lá. **Zero alteração de código**: mudar a fórmula altera output de
produção, logo é passo próprio com gate (ADR-0127). Fica **aberto com dono**: o passo que
revisitar o dimensionamento de delimitadores de grelha decide entre portar o short fall em em
(paridade) ou manter a folga proporcional (divergência declarada com medição de output que a
justifique).

### O que fica aberto, e porquê

**88 dos 90 graves** de Bloco 3 continuam abertos (20 da família math, 68 fora). Não é falta
de método — é que cada um exige a sua própria caça à evidência (citação de docs ou medição
directa do vanilla) e o valor do passo está em fechar cada um **com prova**, não em marcar
90 como tratados. Fechar um cluster de dois com medição dos dois lados custou mais do que
parecia justamente porque a resposta contrariou a afirmação.

Os leves de Bloco 3 (58) e o Bloco 4 (394 achados, referências a passo) ficam fora por
decisão já registada no plano.

## Validação

```
crystalline-lint .         → 0 erros; 3 avisos V7 pré-existentes
crystalline-lint --fix-hashes → 60 ficheiros resselados (só cabeçalhos @prompt-hash)
cargo test --workspace     → 5842 passed; 0 failed
```

Zero alteração de código de produção neste passo — só L0s e cabeçalhos de linhagem.
