# Prompt L0 — `compiler/stdlib/structural/outline` — `outline`, `lof`, `lot`
Hash do Código: 01b891e9

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/outline.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada. Este L0 especifica **a superfície do nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/outline.rs` (`OutlineElem`). `lof`/`lot` **não têm
homólogo vanilla** — são extensão cristalina, aliases de `outline(target:)`.

**Fronteira medida — o único cluster real de seccionamento**: `2ca61c873` (2026-06-27) é
commit de corpo real das três. Não é inserção pura: o corpo de `native_outline` foi
**alterado** (`Ok(Value::Content(Content::outline_with(title, depth, indent)))` removido,
substituído pelo parser de `target:` + `OutlineElem::with_target(...)`), e `lof`/`lot`
nasceram no mesmo commit **como aliases desse `target:`**.

```
$ git show 2ca61c873 -U0 -- <path> | grep '^@@'
@@ -267 +269,61 @@ pub fn native_outline(      ← -267 = linha removida: alteração de corpo
```

Um commit só, portanto **evidência fina** — mas com **mecanismo**: as três partilham a
entidade (`OutlineElem` + `OutlineTarget`), e os aliases não podiam existir sem a mudança
em `outline`. Mecanismo + mudança de corpo real distingue este caso do padrão "duas
funções nascidas no mesmo lote sem se tocarem", que não é sinal de fronteira.

> Antes de 2026-08-13 este nó chamava-se `sectioning` e incluía `heading`, `title` e
> `divider`. Esses três estavam lá sustentados por clusters que não existiam (artefacto de
> atribuição de fronteira) e saíram para nós próprios; o núcleo medido é este.

---

## Contexto

`outline` produz um sumário navegável do documento. `lof` e `lot` são a mesma coisa com
`OutlineTarget` diferente — é isso, e só isso, que as mantém no mesmo nó: mudar o conjunto
de targets muda as três ao mesmo tempo.

O título por omissão (`"Índice"`) e a resolução das entradas são do layout; este nó só
constrói o elemento.

## Instrução

### `outline(title: ?, depth: ?, indent: ?, target: ?)`

Título por **named `title`** ou por **1.º posicional** — os dois ao mesmo tempo são erro
(`outline(): não pode usar título posicional e named 'title' simultaneamente`).

| Nomeado | Tipo aceite | Default | Nota |
|---|---|---|---|
| `title` | `Content \| Str \| none` | `None` | o layout renderiza "Índice" quando ausente |
| `depth` | `Int` ≥ 1 | `3` | `0` → `outline(depth:): depth deve ser >= 1` |
| `indent` | `Bool \| Length \| Func \| auto \| none` | `OutlineIndent::Auto` | `bool`, medida fixa, ou função de indentação |
| `target` | `"headings"\|"heading"` · `"figures"\|"figure"` · `"tables"\|"table"` | `Headings` | valor fora da lista → erro que enumera os três aceites |

Emite `Content::Outline(OutlineElem::with_target(title, depth, indent, target))`.

### `lof(title: ?)` / `lot(title: ?)`

Só `title` (`Content | Str | none`); **zero** posicionais. Emitem `Content::lof(title)` /
`Content::lot(title)` — equivalentes a `outline(target: "figures")` e
`outline(target: "tables")`. Tipo errado → `{fn}(title:): espera content ou string,
recebeu {tipo}`.

## Restrições Estruturais

- L1 puro. `_ctx`/`_world`/`_current_file` são pass-through nas três — nunca usados.
- `lof`/`lot` **não duplicam** a lógica de `outline`: delegam nos construtores de
  `Content` com o target fixo. Se um passo futuro lhes acrescentar `depth`/`indent`, faz-se
  por delegação em `outline`, não por cópia do parser.
- O default de `title` é `None`, não a string "Índice": quem materializa o texto é o
  layout. Assar o título aqui seria mudança de fase (ADR-0127).

## Critérios de Verificação

```
#outline()                          → Outline{title:None, depth:3, indent:Auto, target:Headings}
#outline[Sumário]                   → título posicional
#outline(title: "S")                → título named
#outline[S](title: "T")             → Err (posicional + named)
#outline(depth: 2)                  → depth 2
#outline(depth: 0)                  → Err "depth deve ser >= 1"
#outline(depth: "x")                → Err "espera int, recebeu string"
#outline(indent: false)             → OutlineIndent::Bool(false)
#outline(indent: 1em)               → OutlineIndent::Length
#outline(target: "figures")         → target Figures
#outline(target: "xpto")            → Err enumerando headings/figures/tables
#lof() / #lot()                     → Outline com target Figures / Tables
#lof(title: "Figuras")              → título named
#lof(title: 1)                      → Err "lof(title:): espera content ou string, recebeu integer"
```

## P1284 — gate `outline.entry`; este consumer permanece inalterado

### Medição antes da decisão

O vanilla expõe `outline.entry` como função-elemento com scope próprio e cinco
subfields (`body`, `inner`, `page`, `prefix`, `indented`). O cristalino possui
somente `OutlineElem`/`Content::Outline`; não possui `OutlineEntry` nem variante
`Content` equivalente. Além disso, os defaults P1284 medidos para `outline`
não coincidem com os defaults históricos acima.

### Decisão

`outline.entry` e seus cinco filhos são
`BLOCKED_ADR0127_PUBLIC_CONTRACT`. Este L0 não autoriza editar
`structural/outline.rs`, criar dict/module/stub, nem reutilizar `OutlineElem`
como entry. Filhos permanecem `Unknown(blocked_by_ancestor)`.

Alterar `outline` para `target:heading`, `depth:none` ou qualquer default
medido no residual também exige decisão humana por comportamento default e
compatibilidade. Até esse gate, P1284 preserva integralmente as assinaturas e
defaults vigentes deste owner.
