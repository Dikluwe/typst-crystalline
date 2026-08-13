# Prompt L0 — `compiler/stdlib/text/shift` — `sub`, `super`
Hash do Código: 905b66af

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/text/shift.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/text.md` — dono de `text/mod.rs`
e da história por marco (`f28fba77d`, `87bc1c64d`). Este L0 especifica **a superfície do
nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/text/shift.rs:16` (`SubElem`) e `:75` (`SuperElem`) — no
mesmo ficheiro. (O comentário no código cristalino cita `text/sub.rs` e
`text/superscript.rs`; esses caminhos **não existem** no vanilla em quarentena —
`ls lab/typst-original/crates/typst-library/src/text/` dá `shift.rs`. Corrigido aqui.)

**Fronteira medida**: par confirmado pelo critério 3 — `f28fba77d` (2026-06-24) muda o
corpo das duas nativas e **só** das duas, com a atribuição de fronteira corrigida. O
`smallcaps` que aparecia neste cluster era artefacto (corpo intacto; ver `smallcaps.md`).
O `highlight` que aparecia ligado a `super` em `fa5bda1d4` é o mesmo artefacto, na
direcção contrária.

---

## Contexto

`sub` e `super` deslocam a baseline de um run e reduzem-lhe o corpo. As duas nativas são
**gémeas mecânicas**: a mesma validação, a mesma superfície, o mesmo transporte —
divergem apenas no `Style` emitido e no sinal do deslocamento aplicado pelo layout. O
nome `superscript` no cristalino existe porque `super` é keyword de Rust; a superfície da
linguagem é `super(...)`, e é essa que conta para a paridade (ADR-0107).

Não emitem variant próprio: viajam como `Content::Styled` com `Style::Subscript` /
`Style::Superscript`, reaproveitando a cadeia de estilos.

## Instrução

`sub(body, size: ?)` e `super(body, size: ?)` — 1 posicional `Content | Str` obrigatório.

| Nomeado | Tipo aceite | Semântica |
|---|---|---|
| `size` | `Length` | tamanho explícito do script; sem ele, o layout usa o default (`0.6×`) — materializado em `87bc1c64d` (2026-06-27) |

- `Content` → usado directamente; `Str` → `Content::text(s)`.
- `sub` emite `Content::sub_with_size(body, size)`; `super` emite
  `Content::superscript_with_size(body, size)`.
- `size` com tipo diferente de `Length` → `{fn}(size:) espera length, recebeu {tipo}`.
  Qualquer outro nomeado → `{fn}() argumento nomeado inesperado '{nome}'`. As mensagens
  usam o nome da **linguagem** (`sub`, `super`), nunca o nome Rust.
- Erros posicionais como no resto do módulo: 0 → `{fn}() exige body como argumento
  posicional`; tipo errado → `{fn}() espera content ou string, recebeu {tipo}`; mais de
  1 → `{fn}() recebeu {n} argumentos posicionais (espera 1)`.

O consumer de layout (redução para `0.6×` do corpo, `baseline_offset` de `−0.2em` para
`sub` e `+0.3em` para `super`, aplicado em `cursor.rs`) é propriedade de
`compiler/layout/text.md`.

> **Correcção de deriva**: o L0 pai declarava `offset` **e** `size` como scope-out
> ADR-0054. `size` foi materializado em `87bc1c64d` e é aceite pelas duas nativas — o
> scope-out vale agora só para `offset` configurável.

## Restrições Estruturais

- L1 puro. `_ctx`/`_world`/`_current_file` são pass-through — nunca usados.
- As duas funções são gémeas por construção: qualquer passo que altere a superfície de
  uma altera a outra no mesmo commit, ou declara no L0 porque é que divergem. Esta é a
  razão de as manter num nó só, e é a invariante que o nó existe para tornar visível.
- Nenhum helper privado partilhado: a duplicação dos dois corpos é deliberada e
  preservada por esta fatia (corte e cola, não reescrita). Factorizar os dois numa
  função com discriminante — como `build_decoration` faz em `deco.rs` — é uma mudança de
  desenho, não parte deste fatiamento.

## Critérios de Verificação

```
#sub[2]                    → Styled(text("2"), [Subscript(true)])
#sub("2")                  → idem, body coagido de Str
#sub(size: 4pt)[2]         → Styled com size Some(4pt)
#sub(size: 4)[2]           → Err "sub(size:) espera length, recebeu integer"
#sub()                     → Err "sub() exige body como argumento posicional"
#sub[a][b]                 → Err "recebeu 2 argumentos posicionais (espera 1)"
#sub(foo: 1)[a]            → Err "sub() argumento nomeado inesperado 'foo'"
#super[2]                  → Styled(text("2"), [Superscript(true)])
#super(size: 4pt)[2]       → Styled com size Some(4pt)
#super(1)                  → Err "super() espera content ou string, recebeu integer"
```
