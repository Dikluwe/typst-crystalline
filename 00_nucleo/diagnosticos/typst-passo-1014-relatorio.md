# Passo 1014 — Relatório: `stdlib::structural` fatiado em hub + 9 nós

**Resultado**: fatiado. 9 nós por domínio, código preservado item a item, zero regressão,
4 órfãos do Passo 1001 absorvidos.
**Proveniência**: medições a partir de `HEAD = d4145093c` (Passo 1013), 2026-08-12
~18:40 −03. `git status` limpo antes de começar (só o ficheiro de passo untracked).

---

## Fase A — Inventário: 50 funções, não a amostra do P1004

O Passo 1004 caracterizou `structural.rs` por amostra (`strong`, `emph`, `raw`,
`heading`, `par`, table/grid, bibliografia/cite). O ficheiro tem **50 funções** em 4116
linhas — e os domínios que a amostra não nomeou são substanciais:

| Visibilidade | Contagem |
|---|---:|
| `pub fn` | 37 |
| `fn` (privada) | 13 |

Domínios ausentes da caracterização do P1004: **matemática** (`accent`, `cancel`,
`class`, `underover`, `op`, `make_math_module` — 465 linhas), **listas** (`list`, `enum`,
`terms` — 302), **sumários** (`outline`, `title`, `lof`, `lot` — parte das 422 de
seccionamento), **metadados** (`document`, `asset` — 135). Juntos, 900+ linhas que a
proposta de 3 nós do passo (`markup_elements`, `table_grid`, `bibliography`) não cobria.

Nenhuma visibilidade restrita escondida desta vez (`pub(super)`/`pub(crate)`: zero) — o
padrão genérico adoptado no P1013 foi aplicado à partida.

---

## Fase B — Os 4 critérios

### Critério 3 — co-mudança (o que decidiu as fronteiras)

`python3 tools/analysis/cochange_metrics.py 01_core/src/compiler/stdlib/structural.rs '(rules|engine|compiler)/stdlib/structural\.rs$'`

Descontado o ruído estrutural (os dois renames `rules→engine` e `engine→compiler`, e o
`cargo fmt` global de P797, que tocam tudo), os agrupamentos são nítidos:

| Cluster | Commits que o isolam |
|---|---|
| markup inline | P101 (`native_emph`+`native_strong`), P109/P96.5/P97-99 (+`raw`, `heading`) |
| listas | P494-P495 e P505 — `native_list` e `native_enum` **sempre juntos**, com os seus testes |
| células de tabela/grelha | P229-230, P233-238, P327 — `native_grid_cell`+`native_table_cell` |
| cabeçalhos/rodapés | P772f-j, P320 — os quatro header/footer juntos |
| **linhas** (hline/vline) | P512+P513, P739 — as quatro nativas de linha **como bloco próprio, nunca com as células** |
| bibliografia | P159a/c/d/e/g — `extract_bib_entries`+`native_bibliography`+`native_cite` |
| matemática | P290-301, P317 (`accent`,`cancel`,`op`,`underover`,`op_value`,`make_math_module`); P813-827 (`math_class`+`vanilla_type_name_class`) |
| seccionamento | P763a-P765a — `native_outline`+`native_title` |
| fluxo | P806 — `native_par`+`native_quote` |

**A separação `table_grid` / `table_lines` é o achado.** A proposta do passo juntava-as
num só nó, e o vanilla também as junta (as linhas fazem parte do resolve do grid). A
história diz o contrário: em 60 commits, nenhum move uma nativa de linha junto com uma
célula. Aqui o **critério 3 corrigiu o critério 4** — e na direcção oposta à do P1013.

### Critério 4 — vanilla

O vanilla é **mais** fragmentado do que qualquer proposta em cima da mesa:
`typst-library/src/model/` tem **um ficheiro por elemento** — `strong.rs`, `emph.rs`,
`heading.rs`, `outline.rs`, `title.rs`, `divider.rs`, `list.rs`, `enum.rs`, `terms.rs`,
`par.rs`, `quote.rs`, `footnote.rs`, `link.rs`, `table.rs`, `bibliography.rs`, `cite.rs`,
`document.rs`, `asset.rs` — mais `math/{accent,cancel,op,underover,style}.rs` e
`layout/grid/{mod,resolve}.rs`. São ~21 ficheiros contra os nossos 9.

**Não seguimos o vanilla até ao fim, e a razão é o critério 3**: `list` e `enum` nunca
mudaram um sem o outro; os quatro header/footer também não; as seis nativas de math
movem-se em bloco. Separá-los por elemento produziria ficheiros que só mudam em conjunto
— fragmentação sem coesão. Os 9 nós são a granularidade que a história sustenta.

### Critério 2 — pureza vs estado

Vácuo, como no P1013: **nenhuma** das 50 funções recebe `Engine`. Todas são
`(&EvalContext, &Args, &World, FileId) → SourceResult<Value>` ou puras. Confirmado por
`file:line`, não presumido: mesmo `native_bibliography` e `native_cite` — que o passo
suspeitava precisarem de `World` para ler `.bib` — **não lêem ficheiros**; recebem as
entradas já parseadas (`Value::Array<Value::Dict>`), com o parsing em
`compiler/eval/bibtex.rs` e o carregamento em L3. A suspeita do passo estava errada, e é
essa fronteira que mantém o nó em L1.

### Critério 1 — isolamento de teste

Vácuo pela quarta vez, e desta vez **contra o fatiamento**: os 56 testes partilham um
único `TestWorld` e um conjunto de helpers (`named_args`, `test_file_id`, `call_*`).
Distribuí-los pelos 9 nós exigiria replicar o harness nove vezes. Ficaram no hub — ver
abaixo.

---

## Fase C — Órfãos absorvidos (o que o passo pedia)

Os quatro prompts órfãos do Passo 512 correspondem **exactamente** ao cluster de
co-mudança das linhas — convergência independente entre um plano escrito em Junho e o
que a história mostra:

| Órfão | Destino |
|---|---|
| `stdlib/grid_hline.md` (40 l.) | absorvido em `structural/table_lines.md` |
| `stdlib/grid_vline.md` (40 l.) | idem |
| `stdlib/table_hline.md` (40 l.) | idem |
| `stdlib/table_vline.md` (40 l.) | idem |

Confrontados com o código actual antes de absorver: as assinaturas, os defaults, a
regra de `position: auto` e a nota **P739A** (`stroke: none` aceite; zero-thickness
proibido) continuam correctos e foram transcritos para o L0 do nó. Os quatro ficheiros
foram removidos e as suas quatro excepções de órfão retiradas do `crystalline.toml`.

**Trabalho poupado, medido**: 160 linhas de L0 já escritas e válidas, de um total de 74
no `table_lines.md` final — a absorção condensou-as (as quatro eram quase idênticas duas
a duas), mas a semântica não teve de ser redescoberta.

**Correcção de dívida do P1002**: `decimal-arithmetic.md` tinha sido absorvido e apagado
no Passo 1002, mas a sua excepção de órfão ficou no `crystalline.toml` a apontar para um
ficheiro inexistente. Removida agora — o mesmo erro que este passo estava prestes a
repetir com os quatro do P512.

---

## Fase D — Materialização

`structural.rs` (4116 linhas) → directório `structural/` com hub + 9 nós:

| Ficheiro | Linhas | Nativas |
|---|---:|---|
| `structural/mod.rs` (hub + testes) | 944 | — |
| `structural/markup.rs` | 182 | `strong`, `emph`, `raw`, `link` |
| `structural/sectioning.rs` | 454 | `heading`, `outline`, `title`, `lof`, `lot`, `divider` |
| `structural/lists.rs` | 326 | `list`, `enum`, `terms` |
| `structural/flow.rs` | 258 | `par`, `quote`, `footnote` |
| `structural/table_grid.rs` | 803 | `table`/`grid` + células, cabeçalhos, rodapés |
| `structural/table_lines.rs` | 242 | `grid.hline`/`vline`, `table.hline`/`vline` |
| `structural/bibliography.rs` | 495 | `bibliography`, `cite` |
| `structural/math.rs` | 490 | `accent`, `cancel`, `class`, `underover`, `op`, `make_math_module` |
| `structural/document.rs` | 156 | `document`, `asset` |

Uma única aresta entre nós: `table_grid` importa `default_hline_stroke` de
`table_lines` (passou a `pub(super)`).

### Os testes ficam no hub — e porquê

Ao contrário dos nós, os 56 testes não se deixam particionar: partilham um `TestWorld`,
`named_args`, `test_file_id` e os wrappers `call_*`. Distribuí-los exigiria nove cópias
do harness. Ficaram em `structural/mod.rs`, onde `use super::*` os liga às
re-exportações do hub. É a única razão pela qual o hub tem 944 linhas em vez de ~40.

Consequência registada, para não ser lida como descuido: **o hub deste nó não é
puramente uma fronteira de re-exportação** como o de `eval/bindings` — aloja a suite. O
L0 do hub di-lo explicitamente.

### Prova de preservação

```
nós:    original 50 itens | novos 50 | em falta: nenhuma | corpos diferentes: NENHUM
testes: bloco idêntico ignorando linhas `use` e vazias | #[test]: 56 → 56
```

Alterações reais no bloco de testes, todas de imports:
- **acrescentados** 4 (`Content`, `SourceResult`, `Span`, `EcoString`) — vinham antes por
  `use super::*` a partir dos imports de topo do ficheiro, que agora vivem nos nós;
- **removido** 1 (`ListItemElem`) — passou a chegar por outra via; confirmado pelo rustc
  como não usado.

Os 64+12 `use` retirados dos nós foram removidos com as sugestões do próprio rustc
(`--message-format=json`, `unused_imports`), aplicadas só a ficheiros sob
`stdlib/structural/`, iteradas até estabilizar. 39 linhas em branco residuais dessa
remoção foram colapsadas. Zero warnings novos.

---

## Fase E — Validação

| Verificação | Resultado |
|---|---|
| `cargo build --workspace` | ✅ ok, zero warnings nos ficheiros novos |
| `crystalline-lint --fix-hashes .` | `Nothing to fix` (após resselo) |
| `crystalline-lint .` | **2 warnings**, ambos V7 órfãos pré-existentes |
| `cargo test --workspace` | **5832 passed, 0 failed, 3 ignored** |
| `#[test]` em `HEAD` vs working tree | 5832 = 5832 |

Os V7 caíram de 6 para 2 potenciais: os quatro órfãos do P512 deixaram de existir. Os 2
restantes (`auditar-spec.md`, `infra/package_version_resolution.md`) são pré-existentes e
alheios.

---

## Fase F — Avaliação do método

| Critério | P1002 | P1006 | P1013 | P1014 |
|---|---|---|---|---|
| 1 — isolamento de teste | vácuo | vácuo | vácuo | **vácuo, e contra-indicativo** |
| 2 — pureza vs estado | vácuo | vácuo | discriminou (3 classes) | vácuo (zero `Engine`) |
| 3 — co-mudança | corrigiu a inspecção | inaplicável (trait) | **confirmou** o vanilla | **corrigiu** o vanilla |
| 4 — vanilla | útil | enganador | confirmou, mas só 3/5 nós | mais fino que o útil |

### O que este passo acrescenta ao método

1. **O critério 4 pode errar por excesso, não só por defeito.** No P1006 o mapeamento
   vanilla apontava para o ficheiro errado; aqui aponta para o ficheiro certo mas com
   granularidade fina demais (21 ficheiros para 50 funções). "Seguir o vanilla" não é uma
   regra — é uma hipótese que o critério 3 aceita ou corrige.
2. **O critério 1 deve ser retirado ou reformulado** — quatro aplicações, quatro vácuos,
   e nesta chegou a apontar no sentido contrário (a partilha de harness é razão para
   *não* dividir os testes). A proposta do P1013 (substituir por "quem chama") mantém-se.
3. **Os testes são uma unidade à parte da lógica.** O método não tem nada a dizer sobre
   onde eles ficam quando um agregado é fatiado. Aqui a resposta foi "no hub, porque o
   harness é único", e é uma decisão que qualquer fatiamento futuro de ficheiro com suite
   partilhada vai ter de tomar. Devia estar no método.
4. **Os órfãos valem a leitura.** Segunda vez que um prompt órfão descreve com precisão o
   que a co-mudança identifica como nó (P1002 com `decimal-arithmetic.md`, P1014 com os
   quatro do P512). Vale como sinal barato antes de medir: um órfão existente é uma
   hipótese de fronteira já escrita por alguém.

---

## Resultado

A leva de candidatos do Passo 1008 fica completa: `rules` (P1009/P1011), `closures`
(P1012), `bindings` (P1013), `structural` (P1014). Falta `stdlib::text`, deixado fora
desta leva por decisão do dono.
