# Passo 1473 — fechar as oito rotas públicas não matemáticas restantes

**Estado:** ESCRITO — AGUARDA MEDIÇÕES POR OWNER E GATE ADR-0127

**Data da auditoria:** 2026-09-20T17:40:24-03:00

**Baseline de redação:** `306ca50022104619447fae2625090b8d9c7b0a35`,
tree `0f7f307e8ffda460270714cab0ae525237dc3433`, com o P1470 fora do escopo.

**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`

## 1. Objetivo

Fechar as oito rotas restantes após P1472:

```text
color.spot
color.spot.tint
enum.item
figure.caption
footnote.entry
list.item
polygon.regular
terms.item
```

`color.spot.tint` aparece como bloqueado pelo ancestral no inventário, mas é
um membro público real do valor `SpotColorant`; seu fechamento depende de
`color.spot`.

## 2. Por que um passo, mas vários owners

O passo coordena um único gate humano e uma única remedição final, mas não cria
um mega-owner. Cada família mantém L0/consumer exclusivo:

| Família | Evidência pinada | Natureza |
|---|---|---|
| `color.spot`/`.tint` | `visualize/color.rs:2260-2420` | tipo/variante pública nova + export |
| `list.item` | `model/list.rs:165-176` | elemento público filho |
| `enum.item` | `model/enum.rs:228-241` | elemento público filho |
| `terms.item` | `model/terms.rs:113-126` | elemento público filho |
| `figure.caption` | `model/figure.rs:493-598` | elemento público sintetizado |
| `footnote.entry` | `model/footnote.rs:198-331` | elemento público de realização |
| `polygon.regular` | `visualize/polygon.rs:53-88` | função qualificada |

Se a medição revelar dependência de layout/export não coberta pelo owner
atual, individualizar seu L0 antes de escrever código. Não colocar sete
consumers sob um prompt genérico.

## 3. Fase A — `color.spot` e `color.spot.tint`

O L0 vigente já classifica esta rota como
`BLOCKED_ADR0127_PUBLIC_CONTRACT`: não há variante de cor spot no cristalino.
Logo a implementação deve começar por sondas de:

- assinatura `color.spot`, domínio do nome e fallback;
- `type`, `repr`, igualdade e hashing do colorant;
- assinatura de `.tint`, limites e defaults;
- métodos de instância da cor resultante;
- serialização/repr e comportamento fora de PDF;
- emissão PDF Separation/alternate color space, recursos e tint transform.

Só depois criar a entidade pública e a variante canônica de `Color`. É
proibido aproximar por sRGB, dict, module, string ou `none`. A fallback color
não substitui a identidade spot.

## 4. Fase B — seis elementos filhos

Medir e publicar cada filho como função/elemento no namespace do constructor
pai, preservando identidade e capacidade de `set`/`show`/seleção:

- `list.item(body)`;
- `enum.item(number, body)`;
- `terms.item(term, description)`;
- `figure.caption(...)` com fields públicos e síntese;
- `footnote.entry(...)` com os fields e restrições da realização;
- `polygon.regular(...)` com argumentos geométricos medidos.

Não basta retornar o mesmo payload com uma função anônima. `repr`, `.func()`,
fields, selector, set/show e diagnósticos fazem parte da linguagem. Reutilizar
o constructor/carrier existente quando ele já é o owner; criar entidade nova
somente quando a superfície pinada é distinta.

`footnote.entry` é realizado pelo pipeline de footnotes, mas seu constructor e
estilos não podem mover numeração/layout para o stdlib. Preservar a fronteira
de fase do L0 vigente.

## 5. Gates independentes dentro do passo

Antes de qualquer código de uma família:

1. medir source + probes bilaterais;
2. atualizar o Prompt L0 owner e, se necessário, os owners de entidade/layout;
3. ressellar hashes e validar V5/V15/V26;
4. incluir a família no commit causal de L0;
5. ao concluir todos os L0s, **parar para confirmação humana única**, citando
   explicitamente as oito rotas.

A confirmação de uma fase isolada não deve ser inferida para as demais.

## 6. RED e critérios de fechamento

Cada rota precisa de casos positivos e de missing/extra/named/cast, além dos
observáveis específicos:

- spot: repr, tint, métodos, hashing e PDF estrutural;
- items/caption/entry: `.func()`, fields, set/show, selector e layout;
- polygon: vertices/geometry e defaults sem comparar pixels apenas.

Depois do GREEN, regenerar duas vezes os oito catálogos. O esperado é zero das
17 rotas ausentes originais, assumindo P1472 aplicado; `color.spot.tint` deve
ser observado diretamente, não apenas desaparecer da classe “blocked by
ancestor”.

Executar suíte completa, all-features, Clippy, build, fmt, lint total,
V5/V15/V26 fatal e `git diff --check`. Registrar contagens e proveniência no
próprio passo.

## 7. Auditoria L0 e retificação por família (2026-09-21)

### 7.0 Recalibração de escopo (registada com o dono antes de escrever L0)

Medição inicial (bilateral + leitura das fontes pinadas locais em
`lab/typst-original/`) revelou pesos muito diferentes entre as 8 rotas —
mais heterogéneo do que a primeira leitura do passo sugeria. Apresentado
ao dono em duas rodadas (`AskUserQuestion`); decisão final: tentar as 8
nesta sessão, aceitando profundidade de medição menor nas mais pesadas.
Classificação medida:

| Família | Peso real medido |
|---|---|
| `list.item`/`enum.item` | pequeno — entidades `ListItemElem`/`EnumItemElem` já existem com os campos certos; só falta o constructor público |
| `polygon.regular` | pequeno/auto-contido — fórmula geométrica exata obtida da fonte pinada, delega para `native_polygon` já existente |
| `terms.item` | médio, com achado real: `terms()` cristalino usa uma convenção de chamada (`chave: descrição`) divergente da vanilla (posicional `TermItem`) — fechar `.item` exige estender `terms()`, não só registar a rota |
| `figure.caption` | grande — elemento novo do zero, com síntese (`kind`/`supplement`/`numbering`/`counter`) que a fatia não replica |
| `footnote.entry` | grande — elemento novo do zero, ligado à `Location` de um `FootnoteElem` real; `.realize()`/`ShowSet` inteiramente scope-out |
| `color.spot`/`.tint` | maior — tipo novo na enumeração fechada `Type`; `.tint()` exigiria estender o `enum Color` fechado (dezenas de consumers exaustivos) + emissão PDF `/Separation`; escopo reduzido a `color.spot` sem `.tint()` |

### 7.1 `list.item`/`enum.item`/`terms.item` — `compiler/stdlib/structural/lists.md`

Medido: `ListItem`/`EnumItem`/`TermItem` vanilla (`model/{list,enum,terms}.rs`)
têm exatamente os campos que `ListItemElem`/`EnumItemElem`/`TermItemElem`
cristalinos já possuem. `EnumItem`'s dois positionals (`number: Smart<u64>`,
`body: Content`) são casados por **tipo**, não ordem (medido:
`enum.item(3,[a])` == `enum.item([a],3)`). Achado: `terms()` cristalino
(`native_terms`, `structural/lists.rs:27-58`, documentado no próprio L0
como `terms(named: descrição, …)`) rejeita qualquer positional; vanilla
usa exclusivamente variádica posicional de `TermItem`. Decisão: `terms()`
ganha uma segunda forma de chamada (positional `Content::TermItem`), a
forma nomeada não é removida (extensão cristalina aditiva preservada).

### 7.2 `polygon.regular` — `compiler/stdlib/shapes.md`

Fonte pinada transcrita integralmente (`visualize/polygon.rs:53-88`) —
algoritmo de vértices exato, `size` default `1em`, `vertices` default `3`
(`u64`, `0..=vertices` inclusive fecha a forma). Delega para o mesmo
`Content::Shape(ShapeElem{kind:Path(...)})` que `native_polygon` já
constrói, reutilizando `PathItem`/`parse_paint`/`parse_shape_stroke`/
`path_bbox`/fallback P732 — zero duplicação.

### 7.3 `figure.caption` — `compiler/stdlib/figure_image.md` + nova entidade `entities/elements/figure_caption.md`

Escopo reduzido explícito: constrói `FigureCaptionElem{position,separator,
body}` (settable `position`/`separator` aceites e preservados, sem efeito
de layout); `native_figure` passa a aceitar `Content::FigureCaption` em
`caption:`, extraindo só o `.body`. **Não implementa**: síntese
(`kind`/`supplement`/`numbering`/`counter`/`figure_location`), `.realize()`,
resolução de `separator` por idioma. Medido e registado: acessar
`.position`/`.separator` numa caption não sintetizada é, no vanilla, o erro
especial `field "X" in caption is not known at this point` — não
reproduzido nesta fatia (divergência mecânica aceite, ADR-0107).

### 7.4 `footnote.entry` — `compiler/stdlib/structural/footnote.md` + nova entidade `entities/elements/footnote_entry.md`

Escopo reduzido explícito: constrói `FootnoteEntryElem{note,separator,
clearance,gap,indent}`, os quatro últimos aceites e preservados sem efeito
de layout (mesmo padrão já vigente para `ListItemElem`). **Não implementa**:
`.realize()` (link/superscript/número via `Counter::of(FootnoteElem)`,
depende de `note.location()` real), `ShowSet`. O renderer atual de
`compiler/layout/footnote.rs` não muda — continua a gerar a lista de notas
exatamente como hoje, sem consultar este constructor.

### 7.5 `color.spot`/`.tint` — `compiler/stdlib/color.md` + `entities/color.md` + `entities/value.md`

Escopo reduzido explícito, o mais restrito dos oito. Fecha: `Type` ganha
`SpotColorant`; nova entidade `SpotColorant{name,fallback}` com
constructor/repr/eq medidos (`color.spot(none, ..)` aceite; `fallback`
deve ser `Color` de processo, spot-de-spot rejeitado com `expected color,
found spot colorant`). **Não fecha `color.spot.tint`** — motivo medido e
registado: `.tint()` no vanilla produz um valor `type()=="color"`, o que
exigiria estender o `enum Color` fechado (7+ variantes, casado
exaustivamente por dezenas de consumers — operadores, `to-hex`, emissão
PDF, gradientes) mais a emissão PDF `/Separation` real (sem equivalente no
exportador cristalino hand-rolled). `color.spot.tint` fica scope-out
explícito, não esquecido, a reabrir com gate ADR-0127 próprio.

**Achado técnico durante a escrita do L0 (autocorreção, ADR-0108):**
`entities/color.rs::Color` deriva só `Copy, Clone` (`PartialEq` manual,
**sem** `Hash` — `f32` cru não é `Hash` no Rust std). A primeira redação
de `entities/color.md`/`entities/elements/footnote_entry.md` afirmava
incorretamente que `SpotColorant`/`FootnoteEntryElem` podiam derivar
`Hash` direto; corrigido antes do commit para `Hash` manual via
`format!("{self:?}")`, mesmo padrão já em `ListItemElem`/`EnumItemElem`
(`entities/elements/list_item.rs`) — `entities/elements/figure_caption.md`
não precisou de correção (sem campo `Length`/`f64`, deriva `Hash` direto).

### 7.6 Seis registos ganham namespace — `compiler/eval.md`

`list`/`enum`/`terms`/`polygon`/`figure`/`footnote` já eram `Value::Func`
bare (ao contrário de `pdf`/`html` em P1471, que eram `Value::Module`) —
retrofit mais simples: `Func::native` → `Func::native_with_namespace`,
sem mudança de `type()`.

### 7.7 Gate ADR-0127

10 L0s tocados/criados (`structural/lists.md`, `shapes.md`,
`figure_image.md`, `structural/footnote.md`, `eval.md`, `compiler/stdlib/
color.md`, `entities/color.md`, `entities/value.md`, mais os dois novos
`entities/elements/{figure_caption,footnote_entry}.md`). Hashes
ressealados só nos L0s (`crystalline-lint --fix-hashes .`); os 8
`@prompt-hash` dos `.rs` com owner pré-existente revertidos (`git checkout
--`) — reseal do código só junto da implementação. `crystalline-lint
--checks v15,v26 .`: zero violações. `crystalline-lint --checks v5 .`
mostra o drift esperado (8 ficheiros, L0 mudou/código ainda não). Commit
causal somente-L0 a seguir; **confirmação humana explícita obrigatória**,
citando as 8 rotas e os três escopos reduzidos (`figure.caption`,
`footnote.entry`, `color.spot` sem `.tint`) explicitamente. A confirmação
de P1467-P1472 não cobre esta mudança de contrato público.

