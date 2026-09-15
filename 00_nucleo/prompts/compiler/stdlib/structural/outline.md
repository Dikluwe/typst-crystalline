# Prompt L0 — `compiler/stdlib/structural/outline` — `outline`, `lof`, `lot`
Hash do Código: a5fe5133

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

## P1402 — proposta pública `outline.entry` e `body()` (gate ADR-0127 pendente)

### Medição antes da decisão

No baseline `be15d0ab73af089860d03db390508edf21a4bdac`, working tree limpa,
medido em `2026-09-14T22:02:33-03:00`, a sonda limitada descrita no Passo 1402
removeu a antiga incerteza de construção de P1400/P1284. Com uma fixture que
contém `#outline()` e um heading, o vanila ratificado `586e1bd43` expõe:

- `repr(outline.entry)` → `entry`;
- `repr(outline.entry.body)` → `body`;
- `query(outline.entry).first()` → uma ocorrência `entry` com `level`,
  `element` e `fill` realizados;
- `query(outline.entry).first().body()` → o Content do heading;
- `outline.entry(1, heading[H])` → construção pública válida.

Sem outline a consulta retorna zero; dois outlines sobre o mesmo heading geram
duas ocorrências. `outline.entry(1, strong[H]).body()` falha com `cannot outline
strong`. A fonte ratificada declara o scope em `model/outline.rs:246-250`, os
campos em `:468-522` e o método em `:668-674`. No consumer cristalino não existe
scope, construtor ou entry. Esta medição refuta `Unknown(blocked_by_ancestor)` e
classifica a mudança como contrato público novo.

### Contrato público proposto

O scope público de `outline` passa a conter a função-elemento `entry`. Ela é
selecionável por `query(outline.entry)`, tem nome/repr público `entry` e aceita:

```typ
outline.entry(level, element, fill: repeat(body: [.], gap: 0.15em))
```

- `level` é inteiro posicional obrigatório, 1-based e maior que zero;
- `element` é Content posicional obrigatório;
- `fill` é Content opcional ou `none`; quando omitido, usa o preenchimento
  linguístico equivalente a `repeat(body: [.], gap: 0.15em)`;
- os campos públicos observáveis da ocorrência são `level`, `element` e `fill`;
- a função pública `outline.entry.body` é chamável como método `entry.body()` e
  devolve o corpo semântico do elemento outlinable;
- elemento que não implementa a capacidade linguística outlinable produz erro
  explícito `cannot outline {nome}`, sem coerção para texto ou vazio.

Cada realização de `outline()` com target headings produz uma ocorrência por
elemento incluído, depois dos filtros `depth` e `outlined`, na ordem documental.
Sem outline não há ocorrência; dois outlines podem produzir duas entries para o
mesmo elemento fonte. O `element` preserva sua localização e campos realizados.

Esta cláusula não autoriza `indented`, `inner`, `page` ou `prefix`, nem altera os
defaults ou assinaturas existentes de `outline`, `lof` e `lot`. Também não
autoriza mover a geração para outra fase do pipeline: a materialização deve usar
o ponto de realização já vigente do outline e ser nucleada nos owners internos
afetados antes de código.

### Gate

Esta proposta substitui o bloqueio de P1284 somente após confirmação humana
explícita, conforme ADR-0127. Até a confirmação, o consumer permanece inalterado
e este texto é especificação proposta, não autorização para L1–L4.

## P1403 — proposta pública `prefix()` e `page()`

### Medição antes da decisão

No baseline cristalino limpo
`b7804e4daa9d3dd823dba5d1b828e60c845cc515`, medido em
`2026-09-14T22:50:19-03:00`, o vanila ratificado expõe as identidades
`outline.entry.prefix` e `outline.entry.page`. Sobre a entry consultada do
documento P1402 numerado, elas devolvem respectivamente `[1.]` e `[1]`. Um
heading sem numbering devolve `none` em `prefix`; page numbering `"I"` produz
`[I]`. Fonte: `model/outline.rs:629-667,676-693`. O cristalino não contém as
duas identidades. A rejeição atual de `#show outline.entry` acontece antes da
chamada e permanece lacuna separada.

### Contrato público proposto

O scope de `outline.entry` ganha as funções contextuais estáticas `prefix` e
`page`, ambas sem argumentos:

- `prefix()` resolve a capacidade outlinable do `element`; sem numbering
  retorna `none`; com numbering e Location preservada, formata o contador do
  elemento nessa Location e devolve Content, mantendo o prefixo específico do
  tipo;
- `page()` exige a Location do `element`, consulta a página e seu numbering no
  introspector contextual, usa padrão decimal `"1"` quando não há numbering e
  devolve Content formatado;
- Location ausente produz erro explícito e dica de customização por show rule;
- nenhum caminho usa página corrente, contador flat, texto pré-assado ou
  coerção de Content para string.

Esta materialização continua incompleta de forma declarada: P1404 completa
`inner`, P1405 completa `indented` e P1406 faz `#show outline.entry` alcançar
as entries geradas. P1403 não autoriza essas três parcelas nem muda a fase de
realização do outline.

### Gate

As duas funções ampliam o contrato público. Confirmação humana explícita
recebida em 2026-09-14; o contrato acima está autorizado para L1–L4 conforme
ADR-0127.

## P1404 — proposta pública `inner()`

### Medição antes da decisão

No baseline cristalino limpo
`efd82c1eb08bf40c4849df53bdca8c763cecdaa4`, medido em
`2026-09-14T23:18:11-03:00`, o vanila ratificado expõe
`outline.entry.inner` e, para a entry numerada P1402, devolve Content com a
morfologia `body, space, box(width: 1fr, fill), space, U+2060, page`. Com
`fill:none`, o trecho central é `h(1fr)` sem espaços laterais. Body forte e
fill customizado permanecem Content. Fonte:
`model/outline.rs:629-746`. O cristalino rejeita `inner` como método ausente.

### Contrato público proposto

O scope de `outline.entry` ganha a função contextual estática `inner`, sem
argumentos e chamável como `entry.inner()`:

- resolve `body()` e `page()` pelos helpers já proprietários; erros de elemento
  não outlinable e Location ausente são preservados;
- com fill, devolve sequência de body, espaço, caixa inline `1fr` contendo o
  fill, espaço, word joiner U+2060 e página;
- sem fill, devolve body, espaço horizontal `1fr`, U+2060 e página;
- em direção RTL efetiva, isola somente o body com U+202B/U+202C;
- body, fill e página permanecem Content, nunca string pré-assada;
- o suporte de largura fracionária necessário à caixa é carrier interno desta
  construção e não autoriza aceitar `box(width: 1fr)` na função pública geral.

P1404 permanece incompleto por desenho: P1405 completa `indented` e P1406 faz
`#show outline.entry` alcançar as entries geradas. Não há alegação sobre essas
parcelas nem sobre paridade geral.

### Gate

`inner` amplia o contrato público. Confirmação humana explícita recebida em
2026-09-14; o contrato acima está autorizado para L1–L4 conforme ADR-0127.
