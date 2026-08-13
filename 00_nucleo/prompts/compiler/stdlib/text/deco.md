# Prompt L0 — `compiler/stdlib/text/deco` — `underline`, `strike`, `overline`, `highlight`
Hash do Código: 963b3c89

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/text/deco.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/text.md` — dono de `text/mod.rs`
e da história por marco (`147605058`, `fa5bda1d4`, `87bc1c64d`). Este L0 especifica **a
superfície do nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/text/deco.rs` — `UnderlineElem`, `OverlineElem`,
`StrikeElem`, `HighlightElem` no mesmo ficheiro.

---

## Contexto

Marcas gráficas aplicadas *por detrás ou por cima* de um run de texto, sem alterar o
texto. As três linhas (`underline`/`strike`/`overline`) partilham a mesma superfície de
argumentos e um único parser interno, `build_decoration`, discriminado pelo enum privado
`DecoKind`; emitem variants próprios de `Content` que o layout resolve como
`FrameItem::Line`.

`highlight` acompanha-as por correspondência vanilla (mesmo ficheiro `deco.rs`), mas é
mecanicamente distinto: **não** usa `build_decoration` e emite `Content::Styled` com
`Style::Highlight`, resolvido no layout como `FrameItem::Shape` rectangular por detrás
do texto. A co-mudança cristalina é vácuo para o par `highlight`↔linhas: depois de
descontar os lotes transversais (`87bc1c64d`, `04eda8179`), o `cargo fmt` global
(`0661aef91`) e os renames de módulo (`6636c5ea6`, `0f5575cd0`), nenhum commit os liga —
logo a fronteira vanilla fica. O único cluster que restava, `default_highlight_color`+
`highlight`↔`super` em `fa5bda1d4` (2026-06-24), é artefacto de atribuição de fronteira:
o corpo de `super` ficou intacto nesse commit.

## Instrução

### `underline` / `strike` / `overline`

`fn(body, stroke: ?, offset: ?, extent: ?)` — 1 posicional `Content | Str` obrigatório.

| Nomeado | Tipo aceite | Semântica |
|---|---|---|
| `stroke` | `Color \| none` | paint da linha; `none` desactiva o default. Objecto `Stroke` rico do vanilla é scope-out |
| `offset` | `Length \| Int \| Float \| none` | override do deslocamento Y (Int/Float em pt) |
| `extent` | `Length \| Int \| Float \| none` | extensão horizontal para lá do texto |

Offsets Y por defeito (em-units, aplicados pelo layout): `underline` `+0.10` (abaixo do
baseline) · `strike` `−0.25` (atravessa o x-height) · `overline` `−0.80` (acima do
cap-height).

**Scope-out com erro educacional** (ADR-0054 graded): `evade` e `background` respondem
com mensagem que cita o scope-out, não com "argumento inesperado" genérico. Qualquer
outro nomeado → `{fn}(): argumento nomeado inesperado '{nome}'`.

Emitem `Content::underline` / `Content::strike` / `Content::overline`
`(body, stroke, offset, extent)`.

### `highlight`

`highlight(body, fill: ?, radius: ?, extent: ?)` — 1 posicional `Content | Str`.

| Nomeado | Tipo aceite | Semântica |
|---|---|---|
| `fill` | `Color \| none` | cor do fundo; default amarelo vanilla `rgba(255, 242, 54, 255)`; `none` desactiva |
| `radius` | `Length \| none` | cantos arredondados |
| `extent` | `Length \| none` | extensão horizontal |

Emite `Content::highlight_full(body, fill, radius, extent)`.

> **Correcção de deriva**: `radius` e `extent` foram materializados em `87bc1c64d`
> (2026-06-27); o L0 pai continuava a declará-los scope-out. Estão implementados — a
> cláusula de scope-out vale agora só para gradient fill e math mode.

## Restrições Estruturais

- L1 puro. `_ctx`/`_world`/`_current_file` são pass-through — nunca usados.
- `DecoKind` e `build_decoration` são privados ao nó: são um eixo de variação interno,
  não superfície de linguagem. Só as cinco nativas são reexportadas pelo hub.
- `parse_color` vem de `compiler/stdlib/shapes.rs` (ponto único de coerção de cor);
  não reimplementar aqui.
- `default_highlight_color` é função, não `static` — L1 proíbe estado global (V13).

## Critérios de Verificação

```
#underline[a]                     → Content::Underline { stroke:None, offset:None, extent:None }
#underline("a")                   → body coagido de Str
#underline(stroke: red)[a]        → stroke Some(red)
#underline(stroke: none)[a]       → stroke None
#underline(offset: 2pt)[a]        → offset Some(2pt)
#underline(offset: 2)[a]          → offset Some(2pt) (Int em pt)
#underline()                      → Err "underline() exige body como argumento posicional"
#underline[a][b]                  → Err (2 posicionais)
#underline(evade: true)[a]        → Err com a mensagem literal do código:
                                    "…não suportado neste passo (P284 §A.1 scope-out /
                                    ADR-0054 graded)" — citação da mensagem visível ao
                                    autor do documento, não referência de legitimação
                                    (ver nota do hub sobre estas duas mensagens)
#underline(xpto: 1)[a]            → Err "argumento nomeado inesperado 'xpto'"
#strike[a] / #overline[a]         → idem, variants próprios
#highlight[a]                     → Styled([Highlight(Some(#fff236))])
#highlight(fill: red)[a]          → Highlight(Some(red))
#highlight(fill: none)[a]         → Highlight(None)
#highlight(radius: 2pt)[a]        → + Style::highlight_radius
#highlight(extent: 2pt)[a]        → + Style::highlight_extent
#highlight()                      → Err
```
