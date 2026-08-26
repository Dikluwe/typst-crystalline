# Prompt L0 — `compiler/stdlib/text` — hub do módulo `text`
Hash do Código: a8732f7f

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/text/mod.rs`
**Criado em**: 2026-04-23 — extracção de `stdlib.rs` conforme ADR-0037
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**ADRs**: ADR-0037 (coesão por domínio), ADR-0054 (perfil graded / scope-outs),
ADR-0107 (paridade com a linguagem), ADR-0108 (medir antes de decidir),
ADR-0109 (atomização — lógica no ficheiro da unidade), ADR-0127 (gate de L0).

---

## Contexto

O módulo `text` reúne as nativas globais que produzem, transformam ou decoram texto. Era
um ficheiro único de 1212 linhas com 23 funções livres e um enum privado; está fatiado em
**hub + 8 nós**, um ficheiro por unidade, conforme ADR-0109.

**Este hub não tem lógica e não tem tabela de despacho.** `text.rs` era um agregado plano:
as nativas são chamadas por nome a partir de `stdlib/mod.rs`, sem dispatcher intermédio.
O hub correcto para um agregado plano é **só fronteira de reexportação** — `mod` + `pub
use`, zero `fn`. Isto não é "hub por preencher": é a forma que o módulo deve ter, e está
registado aqui para que não seja lido como lacuna.

A suite de testes destas nativas **não** vive aqui: está em `stdlib/mod.rs`, que partilha
um único harness (`NullWorld`/`TestWorld`) com o resto do stdlib e chama as nativas
através das reexportações do hub. Dividir a suite pelos nós criaria oito cópias do
harness — decisão tomada no fatiamento de `structural` e mantida aqui.

## Instrução

### Índice de nós

| Nó | Ficheiro | Superfície da linguagem | Vanilla homólogo |
|---|---|---|---|
| `constructor` | `text/constructor.rs` | `text(...)` + 6 validadores privados | `text/mod.rs::TextElem` |
| `case` | `text/case.rs` | `upper`, `lower`, `replace` | `text/case.rs` (+ `foundations/str.rs`) |
| `deco` | `text/deco.rs` | `underline`, `strike`, `overline`, `highlight` | `text/deco.rs` |
| `smallcaps` | `text/smallcaps.rs` | `smallcaps` | `text/smallcaps.rs` |
| `shift` | `text/shift.rs` | `sub`, `super` | `text/shift.rs` |
| `smartquote` | `text/smartquote.rs` | `smartquote` | `text/smartquote.rs` |
| `lorem` | `text/lorem.rs` | `lorem` | `text/lorem.rs` |
| `regex` | `text/regex.rs` | `regex` | — (`foundations/str.rs`; ver o L0 do nó) |

Cada nó tem L0 próprio em `00_nucleo/prompts/compiler/stdlib/text/<nó>.md`, que é o dono
da sua superfície. Este hub é o dono da **fronteira** e da **história**.

### Superfície reexportada

O hub reexporta exactamente as 14 nativas — nada mais. Helpers privados dos nós
(`build_decoration`, `DecoKind`, `default_highlight_color`, `lorem_impl`, os seis
validadores de `text(...)`) **não** atravessam a fronteira:

```
constructor  → native_text
case         → native_upper, native_lower, native_replace
deco         → native_underline, native_strike, native_overline, native_highlight
smallcaps    → native_smallcaps
shift        → native_subscript, native_superscript
smartquote   → native_smartquote
lorem        → native_lorem
regex        → native_regex
```

`stdlib/mod.rs` continua a reexportar este conjunto para o resto do compilador; a lista de
lá e a de cá têm de coincidir.

### Regra para nativas novas

Uma nativa nova neste domínio entra **num nó**, nunca no hub. Se não couber em nenhum dos
oito, o L0 do nó novo escreve-se antes do código, e a fronteira medida (co-mudança +
homólogo vanilla) fica registada nele — como nos oito existentes.

### Fronteiras com outros módulos

O vanilla agrupa em `text/` matérias que no cristalino vivem noutro sítio. Não é deriva; é
fronteira conhecida, e o L0 de destino é o dono:

| Vanilla | Cristalino | Dono do L0 |
|---|---|---|
| `text/raw.rs` | `stdlib/structural/markup.rs` | `stdlib/structural/markup.md` |
| `text/linebreak.rs`, `text/space.rs` | markup e layout de parágrafo | L0 de eval/layout |
| `text/lang.rs`, `text/item.rs`, `text/font/` | entidades L1 (`entities/lang.rs`, `entities/font_*.rs`) | L0s de `entities/` |
| set rule `#set text(...)` | `compiler/eval/rules.rs` | `compiler/eval.md` |
| `dir: rtl` (chain + bidi) | `compiler/eval.rs` + `infra/layout_bidi.rs` | `compiler/eval.md`, `infra/layout_bidi.md`, `entities/dir.md` |
| consumers de layout (escala de versaletes, `baseline_offset`, glifo lang-aware, `FrameItem::Line`/`Shape`) | `compiler/layout/text.rs`, `cursor.rs` | L0s do layout |

### História por marco

Proveniência por commit (a regra vigente proíbe referência a número de passo fora de
`Criado em`/`Histórico de Revisões`; o hash do commit é o identificador estável):

| Commit | Data | Marco |
|---|---|---|
| `d2faea55d` | 2026-04-23 | extracção de `stdlib.rs`; `upper`/`lower`/`replace` |
| `fdd39b892`, `1aea00338` | 2026-04-23 | consolidação de `upper`/`lower`/`replace` |
| `147605058` | 2026-05-19 | `underline`/`strike`/`overline` + `DecoKind`/`build_decoration` |
| `a1fe997d2` | 2026-05-19 | `smartquote` |
| `077792dfa` | 2026-06-11 | largura de `SmartQuote` no layout |
| `e1f09cc24` | 2026-06-18 | de-bake da figura (fonte única) — lote transversal |
| `6e29fbaba` | 2026-06-22 | `lorem` e `regex` (`#show regex(...)`) |
| `0b97d2d78` | 2026-06-22 | `smallcaps` (variant + nativa) |
| `e7ae938f6` | 2026-06-24 | consumer real de `smallcaps` (fallback por escala) |
| `f28fba77d` | 2026-06-24 | `sub` / `super` |
| `fa5bda1d4` | 2026-06-24 | `highlight` |
| `87bc1c64d` | 2026-06-27 | `size` em `sub`/`super`; `radius`/`extent` em `highlight` — lote transversal |
| `7e5c65041` | 2026-06-29 | `text(...)` global com `fill` posicional |
| `76f73904c` | 2026-07-05 | `dir: rtl` (chain + alinhamento) |
| `04eda8179` | 2026-07-17 | span de `Args` — lote transversal |
| `c98ffc8ac` | 2026-07-21 | `lorem` com byte-parity vanilla (crate `lipsum`) |
| `2ae3ff53f` | 2026-07-22 | `variations:` em `text(...)` |
| `3c8839e72` | 2026-07-23 | os seis validadores de `text(...)` |
| `0661aef91` | 2026-07-19 | `cargo fmt` global — ruído, sem mudança de comportamento |
| `6636c5ea6`, `0f5575cd0` | — | renames `rules`→`engine`→`compiler` — ruído |

Os quatro commits marcados como lote/ruído são os que a análise de co-mudança tem de
descontar; estão listados para que o desconto seja reproduzível, não conjectural.

### Nota — duas mensagens de erro citam número de passo

As mensagens de scope-out de `underline`/`strike`/`overline` (`evade`, `background`) e de
`smartquote` (`alternative`, `quotes`) contêm literalmente `P284 §A.1` e `P287 §A.2`. Os
L0s dos nós citam-nas porque **a mensagem é o observável** (ADR-0108) — a citação não é
referência de legitimação.

O texto em si é, no entanto, um defeito de produto: quem escreve o documento não sabe o
que `P284 §A.1` significa, e o vanilla não emite nada parecido. Corrigi-lo muda texto
visível ao utilizador, logo é mudança de comportamento por defeito — **gate ADR-0127, fora
deste fatiamento**. Fica registado aqui com dono: o passo que revisitar os scope-outs
graded de `text`.

## Restrições Estruturais

- L1 puro em todos os nós: zero I/O, zero estado global (V13), zero crate externa não
  declarada (V14). A única externa deste módulo é `lipsum`, usada só pelo nó `lorem`.
- **Zero `fn` em `text/mod.rs`** — a fronteira é `mod` + `pub use` e mais nada. Uma função
  no hub é sinal de que um nó está a faltar.
- V15: um `@prompt` por ficheiro `.rs`. Os nove ficheiros (hub + 8 nós) apontam para os
  nove L0s correspondentes.
- Visibilidade preservada no fatiamento: as nativas eram `pub fn` num módulo privado
  (`mod text;`), e continuam `pub fn` nos nós, reexportadas por `pub use` no hub. Os
  helpers eram privados ao ficheiro e passam a privados ao nó — nenhum deles sobe para o
  hub.
- Helpers partilhados (`err`, `expect_no_named` de `stdlib/mod.rs`, `parse_color` de
  `stdlib/shapes.rs`, as mensagens de erro de `eval/rules.rs`) continuam a ser importados
  do sítio onde estão; o fatiamento não os duplica nem os move.
- A reexportação do nó `regex` **tem de ser** `pub use self::regex::native_regex;`. O nome
  do nó sombreia a crate externa `regex`, e sem o `self::` o classificador de imports do
  linter lê `regex::native_regex` como tipo externo não declarado (V14). O `self::` é aqui
  necessário por colisão de nome — não é o padrão geral, que continua a ser reexportar sem
  prefixo.

## Critérios de Verificação

O fatiamento é **corte e cola, não reescrita** — a prova é de preservação, item a item:

```
Contagem de itens: 23 fn + 1 enum privado, antes e depois — 24 = 7 (constructor)
  + 3 (case) + 7 (deco) + 1 (smallcaps) + 2 (shift) + 1 (smartquote) + 2 (lorem)
  + 1 (regex)
grep -c 'fn ' em text/mod.rs                    → 0
Reexportações do hub                            → exactamente 14 nativas
Lista de `stdlib/mod.rs` vs lista do hub        → idênticas
cargo test --workspace                          → contagem de #[test] idêntica à de antes
crystalline-lint .                              → zero violations (V3, V5, V14, V15)
```

## Resultado Esperado

- `01_core/src/compiler/stdlib/text/mod.rs` — hub: `mod` × 8, `pub use` × 8 grupos,
  cabeçalho de linhagem a apontar para este L0. Zero lógica.
- `01_core/src/compiler/stdlib/text/{constructor,case,deco,smallcaps,shift,smartquote,
  lorem,regex}.rs` — um por nó, cada um com cabeçalho de linhagem para o seu L0.
- `01_core/src/compiler/stdlib/text.rs` deixa de existir; `mod text;` em `stdlib/mod.rs`
  passa a resolver para o directório.
- Nenhuma alteração em `stdlib/mod.rs` além do que a resolução de módulo exigir: a lista
  de reexportação de `text::{…}` fica igual, porque o hub reexporta os mesmos nomes.

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-23 | Criação inicial (extracção de `stdlib.rs`) | `stdlib/text.rs` |
| 2026-08-13 | Fatiado em hub + 8 nós; o L0 passa a especificar a fronteira e a história, a superfície muda para os L0s dos nós; fecha a lacuna "deriva (F4)" (`upper`/`lower`/`replace` nunca especificados) e três derivas de scope-out (`size` em `sub`/`super`, `radius`/`extent` em `highlight`, consumer de `smallcaps`) | `stdlib/text/*.rs`, `stdlib/text/*.md` |
