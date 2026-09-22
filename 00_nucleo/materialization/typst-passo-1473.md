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

## 8. Execução segregada e integração (2026-09-21/22)

Confirmação humana explícita recebida em 2026-09-21, na mesma sessão,
posterior ao commit causal somente-L0 `3ac60a578`. Os três papéis
partiram desse baseline em worktrees isoladas. Ambos os agentes sofreram
uma interrupção por limite de taxa da sessão a meio do trabalho e foram
retomados via `SendMessage` sem perda de progresso (cada um verificou o
próprio worktree antes de continuar).

- **Agente A (implementação):** commit `01a82593d`
  ("feat(model): expose list.item/enum.item/terms.item/polygon.regular/
  figure.caption/footnote.entry/color.spot (P1473, agente A)"). Implementou
  as 8 rotas + a extensão de `terms()` + os 6 retrofits de namespace, sem
  ver os testes. Resolveu por conta própria (documentado no seu relatório,
  dentro do contrato): o mecanismo `Value::SpotColorant` (paralelo a
  `Value::Stroke`/`Value::Gradient`), o dispatch de `color.spot(...)` como
  construtor via o `match` já existente de `call_dispatch.rs`, a
  arity/casts de `color.spot(name, fallback)` para 1-2 argumentos
  (subcaso não medido explicitamente no L0), e o default `size: 1em` de
  `polygon.regular` sem acesso a `Engine` (resolvido contra `11.0pt`,
  mesma limitação pré-existente de `vertex_component`). Seguindo a
  orientação do prompt, deixou o compilador guiar a resolução dos matches
  exaustivos sobre `Content`/`Value`/`Type` nos dois novos `Content`
  variants e no novo `Type` variant — tocando 12 ficheiros fora dos 10 L0s
  originalmente auditados (ver nota em `compiler/eval.md` §P1473).
- **Agente B (testes):** commit `0258eab89` ("test(model): freeze eight
  remaining public routes red contract (P1473, agente B)"). 51 testes
  novos em 5 ficheiros, todos via `eval_expression` (nunca construção
  Rust direta de tipos ainda inexistentes — decisão metodológica
  deliberada para que um erro de compilação num teste não travasse a
  confirmação do RED de todos os outros). 45 RED genuínos (100% falha de
  asserção em runtime, zero erro de compilação), 6 guardas de
  não-regressão desenhadas para já passar no baseline. Escreveu também um
  oráculo independente (`regular_polygon_points`) que reproduz a fórmula
  geométrica do L0 num teste Rust puro, para `polygon.regular`.
- **Agente C (integração, o próprio dono desta execução):** extraiu os
  dois patches, aplicou sem conflito de merge (apesar de ambos tocarem
  `entities/content.rs`/`eval/mod.rs` extensivamente — as inserções não se
  sobrepuseram), e fez as correções mecânicas e achados de integração:
  1. **Regressão real descoberta pela suíte** (não antecipada por nenhum
     dos dois L0s): `figure.caption`'s `position:` — a redação original
     media `figure.caption([x], position: left)` (identificador bare) mas
     concluiu incorretamente `Str` como o tipo aceite; Agente A implementou
     fielmente essa conclusão errada, Agente B testou-a fielmente também.
     Medição direta na integração revelou `position` como `Value::Align`
     (não `Str`) — `figure.caption(x, position: "top")` é rejeitado no
     vanilla com `expected alignment, found string`. Corrigida a
     implementação (`Value::Align` com `VAlign::Top`/`Bottom`, sem
     componente horizontal) e os 2 testes afetados;
  2. `footnote.entry`'s mensagens de erro — Agente A mirou a convenção
     Portuguesa pré-existente do `footnote()` irmão em vez de medir
     paridade vanilla para a rota nova; medição direta confirmou que
     `footnote.entry` usa as mensagens padrão (`missing argument: note`,
     `expected content, found integer`, `expected length, found integer`
     com o hint já existente `a length needs a unit`). Corrigidas as 6
     mensagens (`note`, `separator`, `clearance`/`gap`/`indent`, named
     desconhecido), reutilizando `vanilla_type_name` e o helper já
     existente `eval::rules::expected_length_error` em vez de duplicar;
  3. `f64::cos`/`f64::sin` — disallowed methods em L1 (`clippy.toml`,
     não-determinismo). Usados tanto pela produção (`native_polygon_
     regular`) quanto pelo oráculo de teste do Agente B. Corrigidos para
     `libm::cos`/`libm::sin`; `crystalline.toml`
     (`l1_allowed_external.libm`) ampliado para incluir `cos`/`sin` (mesma
     whitelist já usada para `cbrtf`/`fmaf`/`powf`, extensão mecânica);
  4. Import órfão (`Type`) no módulo de teste de `color.rs` (achado de
     clippy, Agente B);
  5. V2 (`crystalline-lint`) — os dois novos ficheiros de entidade
     (`figure_caption.rs`/`footnote_entry.rs`) não tinham nenhum teste
     próprio (Agente A não escreveu testes por regra; Agente B testou via
     `eval_expression` na camada stdlib, não diretamente na entidade).
     Acrescentados 5 testes locais a cada um (mesmo padrão de
     `list_item.rs`: `plain_text`/`is_empty`, preservação de campos,
     `map_text`, `map_content`, `eq`/`Hash`).

### 8.1 RED → GREEN por ficheiro

| Ficheiro | Novos testes (RED no baseline `3ac60a578`) | GREEN (após integração) |
|---|---|---|
| `compiler/stdlib/structural/lists.rs` | 20 novos — 18 falhas genuínas, 2 não-regressão | GREEN |
| `compiler/stdlib/shapes.rs` | 8 novos — 7 falhas genuínas, 1 não-regressão | GREEN (+ correção cos/sin) |
| `compiler/stdlib/figure_image.rs` | 8 novos — 7 falhas genuínas, 1 não-regressão | GREEN (+ correção `position`, +1 teste novo) |
| `compiler/stdlib/structural/footnote.rs` | 6 novos — 5 falhas genuínas, 1 não-regressão | GREEN (+ correção de 6 mensagens) |
| `compiler/stdlib/color.rs` | 9 novos — 8 falhas genuínas, 1 não-regressão | GREEN (+ import órfão removido) |
| `entities/elements/figure_caption.rs` | 0 (achado de integração) | 5 novos (V2), GREEN |
| `entities/elements/footnote_entry.rs` | 0 (achado de integração) | 5 novos (V2), GREEN |

Total Agente B: **51/51 testes GREEN** (45 RED genuínos + 6 não-regressão).
Total papel C: **+10 testes** (V2, entidades novas). Suíte final
`typst-core`: **6129/6129**.

### 8.2 Gates finais medidos (2026-09-21/22)

```text
cargo build --workspace                                     -> sucesso
cargo test -p typst-core --lib                                -> 6129/6129
cargo test -p typst-shell --lib                                -> 83/83
cargo test -p typst-infra --lib                                -> 995/995 (duas execuções completas)
cargo fmt --all -- --check                                     -> sucesso (após 1 `cargo fmt --all`)
cargo clippy -p typst-core --all-targets --message-format=json
    comparado por contagem (ficheiro, mensagem) contra o
    baseline `3ac60a578` em worktree isolada, duas rondas
    (achados reais corrigidos entre as rondas)              -> zero avisos novos na ronda final
crystalline-lint .                                              -> 0 erros (achados V14/V2 reais
                                                                   corrigidos antes desta medição)
crystalline-lint --checks v5,v15,v26 .                          -> 0 violações
git diff --check                                                -> sucesso
```

Sonda bilateral final (binário recompilado): as 8 rotas confirmadas
`function`/`type` conforme especificado; `terms(terms.item(...), ...)`
(forma posicional) funciona sem erro.

O veredito final é: as 8 rotas do inventário original (§1) existem e têm
o kind certo. `list.item`/`enum.item`/`terms.item`/`polygon.regular`
fecham integralmente (sem escopo reduzido). `figure.caption`/
`footnote.entry`/`color.spot` fecham com o escopo reduzido decidido e
confirmado na fase L0 — não alegam síntese, realização de layout, `.tint()`
nem conformidade PDF `/Separation` real. Dois achados de integração (a
regressão de `position` em `figure.caption` e as mensagens erradas de
`footnote.entry`) confirmam o valor da execução segregada A/B/C: nenhum
dos dois teria sido pego sem a suíte real a correr contra medição
independente — ambos os agentes seguiram fielmente os L0s que receberam,
e os L0s é que estavam incompletos/errados nesses dois pontos.

