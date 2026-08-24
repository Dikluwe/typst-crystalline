# Prompt L0 — `stdlib/collections` — superfícies de array, dict e str

**Camada**: L1  
**Ficheiro alvo**: `01_core/src/compiler/stdlib/collections.rs`  
**Criado em**: 2026-06-25 (Passo P466)  
**Atualizado em**: 2026-08-24 (P1142 — formas estáticas `array.all`/`str.clusters`, predicado booleano estrito e grapheme clusters reais; P690 — `str.len/at/slice` em bytes; P691 — `str.find` devolve substring/`none`; P692 — `str.matches` e `str.normalize`; P693 — `str.match` aceita `str | regex`; P714 — `array.at(index, default:)`; P730 — `array.slice(start, end?, count:)`)
**ADRs**: ADR-0013 (`unicode-segmentation` permitido em L1), ADR-0037 (coesão por domínio), ADR-0107 (paridade com a linguagem), ADR-0108 (medir antes de decidir), ADR-0117 Cláusula 4 (métodos de tipos existentes; não propõe estrutura em elementos), ADR-0127 (correção de paridade em fluxo contínuo).

---

## 1. Visão geral

`collections.rs` implementa as superfícies dos tipos de coleção do Typst:
métodos de instância de `array`, `dict` e `str` e os membros estáticos
explicitamente especificados neste L0. Os métodos são despachados pela sintaxe
de chamada de método (ex.: `(1, 2, 3).first()`, `"ab".repeat(3)`) antes do
dispatch genérico; os membros estáticos resolvem por field access no valor-tipo
e delegam ao mesmo owner sem duplicar semântica.

A assinatura do dispatcher é:

```rust
pub(crate) fn try_dispatch_collection_method(
    target: Value,
    method: &str,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> Option<SourceResult<Value>>
```

Retorna `Some(Result)` se o método for reconhecido; `None` caso contrário, permitindo fallback para o dispatch genérico de funções.

### 1.1 Medição P1142 — vanilla ratificado `a51e02804`

Fonte conferida no objeto Git ratificado:

- `foundations/array.rs:698-714`: `all(self, test)` chama o predicado em ordem,
  exige retorno `bool`, para no primeiro `false` e devolve `true` no vazio;
- `foundations/str.rs:275-280`: `clusters(self)` usa extended grapheme clusters,
  não scalars Unicode isolados.

As duas instalações do binário ratificado produziram a mesma sonda:

```text
type(array.all) = function; repr(array.all) = "all"
array.all((1, 2, 3, 4), x => x < 3) = false
array.all((), x => false) = true
type(str.clusters) = function; repr(str.clusters) = "clusters"
str.clusters("á👍🏽👩‍💻🇵🇹") = ("á", "👍🏽", "👩‍💻", "🇵🇹")
```

Um predicado que devolve inteiro falha com `expected boolean, found integer`;
argumentos `self`/`test` são posicionais. A sonda com `panic` depois do primeiro
`false` confirma curto-circuito observável. Medição executada em P1142, HEAD
cristalino `a3e72f35b3fc0f5cd765d0dc0ca89990ff1314a5`, working tree não
commitado, em 2026-08-24.

### 1.2 Decisão P1142 produzida pela medição

- `array.all` e `str.clusters` existem simultaneamente como forma de instância
  e função não ligada no valor-tipo; ambas as formas usam a mesma semântica.
- `array.all` aceita exatamente `(self: array, test: function)` em posição e o
  resultado de cada chamada do predicado tem de ser `bool`; truthiness não é
  substituto de linguagem.
- `str.clusters` aceita exatamente `(self: str)` em posição e segmenta extended
  grapheme clusters via `unicode_segmentation::UnicodeSegmentation`, dependência
  L1 já autorizada pela ADR-0013.
- A mudança é correção de paridade dentro do owner existente, sem assinatura
  Rust pública nova, campo público, trait, default ou mudança de fase. Portanto
  segue o fluxo contínuo do ADR-0127: L0 primeiro, RED→GREEN e revalidação.

---

## 2. Métodos de `array`

| Método | Assinatura | Semântica |
|--------|-----------|-----------|
| `first` | `array.first() -> any` | Primeiro elemento ou `none`. |
| `last` | `array.last() -> any` | Último elemento ou `none`. |
| `at` | `array.at(index: int, default: any?) -> any` | Elemento no índice; negativo conta a partir do fim (`len + index`). Fora de limites usa `default` se fornecido, senão erro. (**P714**) |
| `slice` | `array.slice(start: int, end: int?, count: int?) -> array` | Sub-array de `start` (inclusivo) a `end` (exclusivo); negativos contam a partir do fim. `count:` é alternativa a `end` (mutuamente exclusivos). Fora de limites → erro; `end < start` → vazio. (**P730**) |
| `rev` | `array.rev() -> array` | Array invertido. |
| `sum` | `array.sum() -> int \| float` | Soma numérica; rejeita tipos não-numéricos. Mistura int/float produz float. |
| `sorted` | `array.sorted(key: function?) -> array` | Ordenação; `key` opcional. Compara `int`, `float` e `str`. |
| `filter` | `array.filter(pred: function) -> array` | Filtra por predicado. |
| `map` | `array.map(func: function) -> array` | Mapeia por função. |
| `find` | `array.find(pred: function) -> any` | Primeiro elemento que satisfaz ou `none`. |
| `any` | `array.any(pred: function) -> bool` | Algum satisfaz? |
| `all` | `array.all(pred: function) -> bool` | Todos satisfazem; exige retorno booleano, faz curto-circuito e devolve `true` no vazio. A forma não ligada é `array.all(self: array, pred: function)`. (**P1142**) |
| `zip` | `array.zip(other: array) -> array` | Pares `(a_i, b_i)`; trunca no menor tamanho. |
| `enumerate` | `array.enumerate() -> array` | Pares `(index, value)`. |
| `dedup` | `array.dedup() -> array` | Remove duplicados adjacentes. |
| `chunks` | `array.chunks(n: int) -> array` | Divide em sub-arrays de tamanho `n`; último pode ser menor. |
| `windows` | `array.windows(n: int) -> array` | Janelas deslizantes de tamanho `n`. |
| `flatten` | `array.flatten() -> array` | Acha um nível de arrays aninhados. Elementos não-array são incluídos tal como estão. |
| `fold` | `array.fold(start: any, reducer: function) -> any` | Reduz o array a um único valor aplicando `reducer(acc, item)` em cada elemento. |
| `join` | `array.join(separator: any?, last: any?, default: any?) -> any` | Combina todos os itens num só valor via a op `join` da linguagem. Separador posicional opcional (default `none`); `last:` separador alternativo antes do último elemento; `default:` devolvido para array vazio (vazio sem default → `none`). (**P843, #60**) |

### Semântica de `join` (P843, #60 — medida em `temp/p843/join*.typ`)

Paridade vanilla `Array::join` (`foundations/array.rs:754-786`):

```
("a", "b").join("-")               → "a-b"
("a", "b").join()                  → "ab"
("a",).join("-")                   → "a"
().join("-")                       → none
().join("-", default: "x")         → "x"
("a", "b", "c").join("-", last: " and ") → "a-b and c"
("a", "b").join("-", last: " and ")      → "a and b"
(1, 2).join("-")                   → Err "cannot join integer with string"
```

A concatenação usa `operators::join` (a mesma op dos code blocks, P728):
`none` é identidade; strings/content/bytes/arrays/dicts/args combinam;
tipos incompatíveis dão o erro `cannot join {a} with {b}` com nomes
longos (paridade vanilla).

### Semântica de `at` (P714)

Paridade com o vanilla (`foundations/array.rs:207-221`, helper interno
`locate_opt`): índice negativo conta a partir do fim
(`resolved = len + index`, via `checked_add` — sem overflow para
índices patológicos). Índice fora de `0 <= resolved < len` usa
`default:` se fornecido; sem `default`, erro com a mesma mensagem do
vanilla (`out_of_bounds_no_default`):

```
(10, 20, 30).at(1)                    → 20
(10, 20, 30).at(-1)                   → 30           (último elemento)
(10, 20).at(5, default: 0)            → 0
(10, 20).at(5)                        → Err "array index out of bounds
                                          (index: 5, len: 2) and no
                                          default value was specified"
(10, 20).at(1, foo: 1)                → Err "argumento nomeado
                                          desconhecido"
```

Reproduz o uso real medido em `cetz` (`aabb.typ:43,45,75-77`):
`bounds.high.at(2, default: 0)`, `padding.at("left", default: 0)` —
este segundo caso é `dict.at`, já implementado desde P495; `array.at`
fechava a lacuna para `Array` especificamente.

### Semântica de `slice` (P730)

Paridade com o vanilla (`foundations/array.rs:279-300`, sobre o helper
`locate(index, end_ok: true)` de `array.rs:122-137`): índices negativos
contam a partir do fim (`len + index`, `checked_add`); `start` e `end`
efetivos têm de estar em `0 <= i <= len` (`end_ok` admite `i == len`);
`end` omitido → `len`; `count:` equivale a `end = start_resolvido + count`;
`end` final é clampado a `>= start` (`end < start` → sub-array vazio, sem
erro). Erros com as mensagens exactas do vanilla (observável — ADR-0107):

```
(1,2,3,4).slice(1, 3)            → (2, 3)
(1,2,3,4).slice(-2)              → (3, 4)
(1,2,3,4).slice(1)               → (2, 3, 4)
(1,2,3,4).slice(0, count: 2)     → (1, 2)
(1,2,3,4).slice(-3, -1)          → (2, 3)
(1,2,3,4).slice(1, count: -1)    → ()          (end efetivo 0 → max(start) → vazio)
().slice(0)                      → ()
(1,2,3,4).slice(1, 2, count: 2)  → Err "`end` and `count` are mutually exclusive"
(1,2,3,4).slice(10)              → Err "array index out of bounds (index: 10, len: 4)"
(1,2,3,4).slice(0, count: 99)    → Err "array index out of bounds (index: 99, len: 4)"
```

Consumidor real, pesado: `cetz` usa `array.slice` em ≥10 sítios
(`draw/shapes.typ:620,624,630,971,973,1949,2160`, `anchor.typ:218`,
`coordinate.typ:203`, `drawable.typ:145`) — era o bloqueio da reprodução
completa de `cetz` após P728/P729. Dispatcher:
`try_dispatch_collection_method` (mesmo mecanismo de `array.at`, P714).

### Semântica de `dedup`

```rust
fn array_dedup(arr: Vec<Value>) -> Vec<Value> {
    let mut result = Vec::new();
    for item in arr {
        if result.last() != Some(&item) {
            result.push(item);
        }
    }
    result
}
```

### Semântica de `chunks`

- `n <= 0` → erro eval.
- Array vazio → array vazio.
- Último chunk pode ter tamanho `< n`.

```rust
(1, 2, 3, 4, 5).chunks(2) → ((1, 2), (3, 4), (5))
```

### Semântica de `windows`

- `n <= 0` → erro eval.
- `n > len` → array vazio.
- Array vazio → array vazio.

```rust
(1, 2, 3).windows(2) → ((1, 2), (2, 3))
```

### Semântica de `flatten`

Acha um nível de arrays aninhados. Elementos que não são arrays são preservados na posição original.

```rust
(1, (2, 3), 4).flatten() → (1, 2, 3, 4)
((1, 2), (3, 4)).flatten() → (1, 2, 3, 4)
(1, 2, 3).flatten() → (1, 2, 3)
```

### Semântica de `fold`

Reduz o array a um único valor, aplicando a função `reducer(start, item)` e acumulando o resultado.

```rust
(1, 2, 3).fold(0, (acc, x) => acc + x) → 6
("a", "b", "c").fold("", (acc, x) => acc + x) → "abc"
```

---

## 3. Métodos de `dict`

| Método | Assinatura | Semântica |
|--------|-----------|-----------|
| `at` | `dict.at(key: str, default: any?) -> any` | Valor associado a `key`, ou `default` se a chave não existir. Sem `default` → erro. |
| `pairs` | `dict.pairs() -> array` | Array de pares `(key, value)`. |
| `remove` | `dict.remove(key: str) -> any` | Valor removido ou `none`. **Nota**: no cristalino o dict original não é mutado (dispatch por valor). |
| `update` | `dict.update(other: dict) -> dict` | Mescla com outro dict. |
| `len` | `dict.len() -> int` | Número de entradas. |
| `insert` | `dict.insert(key: str, value: any) -> dict` | Devolve novo dict com a entrada adicionada/alterada. |

---

## 4. Métodos de `str`

| Método | Assinatura | Semântica |
|--------|-----------|-----------|
| `len` | `str.len() -> int` | Número de **bytes** (UTF-8). (**P690**: era chars; paridade vanilla.) |
| `first` | `str.first() -> str \| none` | Primeiro char como string, ou `none` se vazia. |
| `last` | `str.last() -> str \| none` | Último char como string, ou `none` se vazia. |
| `at` | `str.at(index: int) -> str` | Char no índice em **bytes** (negativo conta bytes do fim). Erro se fora de limites ou se o índice não for uma fronteira de carácter. (**P690**: era chars.) |
| `slice` | `str.slice(start: int, end: int?, count: int?) -> str` | Substring em **bytes** de `start` até `end` ou `count` bytes. Erro se ambos `end` e `count`, se algum índice estiver fora de limites, ou se não for fronteira de carácter; `start > end` → `""`. (**P690**: era chars, com clamp.) |
| `char-len` | `str.char-len() -> int` | Número de **chars** (codepoints). **Extensão cristalina**, não-portável. (**P690**) |
| `char-at` | `str.char-at(index: int) -> str` | Char no índice em **chars** (negativo conta do fim). Erro se fora de limites. **Extensão cristalina**, não-portável. (**P690**) |
| `char-slice` | `str.char-slice(start: int, end: int?, count: int?) -> str` | Substring em **chars** de `start` até `end` ou `count` chars (clamp aos limites). Erro se ambos `end` e `count`. **Extensão cristalina**, não-portável. (**P690**) |
| `clusters` | `str.clusters() -> array` | Array de extended grapheme clusters Unicode. A forma não ligada é `str.clusters(self: str)`. (**P1142**) |
| `contains` | `str.contains(substr: str) -> bool` | Contém substring? |
| `starts-with` | `str.starts-with(prefix: str) -> bool` | Começa com prefixo? |
| `ends-with` | `str.ends-with(suffix: str) -> bool` | Termina com sufixo? |
| `find` | `str.find(pattern: str \| regex) -> str \| none` | A **substring** encontrada (texto do primeiro match), ou `none` se não houver. Aceita `str` (substring literal) ou `regex` (texto do match). (**P691**: era `int` com o índice; corrigido para paridade vanilla. O índice continua disponível via `position`.) |
| `replace` | `str.replace(old: str, new: str) -> str` | Substitui substring literal. |
| `trim` | `str.trim() -> str` | Remove whitespace dos extremos. |
| `split` | `str.split(sep: str) -> array` | Divide por separador. |
| `repeat` | `str.repeat(n: int) -> str` | Repete `n` vezes; `n >= 0`. **Extensão cristalina** (não existe no vanilla 0.15). |
| `codepoints` | `str.codepoints() -> array` | Array de strings, um por char (scalar value). (**P689**) |
| `normalize` | `str.normalize(form: str = "nfc") -> str` | Normalização Unicode. `form` (named) ∈ `nfc`, `nfd`, `nfkc`, `nfkd`; default `nfc`. (**P692**) |
| `position` | `str.position(hay: str \| regex) -> int \| none` | Índice em **bytes** da primeira ocorrência, ou `none`. Aceita `str` ou `regex`. (**P689**) |
| `match` | `str.match(pattern: str \| regex) -> dict \| none` | Primeiro match: dict `{start, end, text, captures}` com índices em **bytes**, ou `none`. Aceita `str` (literal; `captures` vazio) ou `regex` (capturas em ordem posicional, grupos nomeados inclusive). (**P689**; **P693** alargou de `regex`-only para `str \| regex`.) |
| `matches` | `str.matches(pattern: str \| regex) -> array` | Array de dicionários `{start, end, text, captures}` (índices em **bytes**), um por ocorrência não sobreposta; `()` se nenhum. Aceita `str` (literal) ou `regex`. (**P692**) |

**Nota P690 (indexação unificada em bytes — ADR-0107):** `len`, `at` e `slice` passam a
indexar por **byte**, como o vanilla (medido: `"café".len() == 5`, `"éabc".at(2) == "a"`,
`"éabc".slice(2, 4) == "ab"`, `"café mais texto".at(s.position("m")) == "m"`). Isto é
**semântica da linguagem** (não mecânica — ADR-0107) e remove a inconsistência interna em
que `position`/`match` (bytes, P689) não combinavam com `len`/`at`/`slice` (chars).
`at`/`slice` rejeitam índices que não são fronteira de carácter ou que estão fora de
limites, como o vanilla (medido: `"éabc".at(1)` → erro "not a character boundary";
`"éabc".slice(0, 100)` → erro "out of bounds"). A indexação por carácter, genuinamente
útil para texto humano, é preservada sob `char-len`/`char-at`/`char-slice` — extensão
cristalina **não-portável**, registada com razão (padrão `table.numbering` de P459;
regra P662-P664: diferença de linguagem só com nome distinto e decisão consciente).
`first`/`last`/`clusters`/`codepoints`/`contains`/`starts-with`/`ends-with`/`replace`/
`trim`/`split`/`rev` não indexam (devolvem strings/arrays/bool), pelo que a questão
byte/char não se lhes aplica e permanecem inalterados; `position`/`match` já eram bytes
(P689).

**Nota P692 (`matches` e `normalize`):** `matches(pattern)` devolve um array de dicionários
`{start, end, text, captures}` (índices em bytes), um por ocorrência não sobreposta —
`str` usa `str::match_indices` (literal), `regex` usa `Regex::captures_all` (novo,
generalização de `captures_first` de P689). `normalize(form:)` usa a crate
`unicode-normalization` (NFC/NFD/NFKC/NFKD; default NFC), computação pura sobre strings,
sem I/O — admissível em L1 e declarada em `[l1_allowed_external.unicode_normalization]`
(`crystalline.toml`). A crate já era dependência transitiva (via `hayagriva → biblatex`);
P692 torna-a dependência **directa** de `typst-core` e regista-a na whitelist. Com estes
dois métodos, o cristalino cobre **todos** os métodos de instância de `str` da
documentação oficial do Typst 0.15 (`len, first, last, at, slice, clusters, codepoints,
to-unicode, normalize, contains, starts-with, ends-with, find, position, match, matches,
replace, trim, split, rev`) — os restantes símbolos do cristalino (`char-*`, `repeat`,
`to-upper`, `to-lower`) são extensões não-portáveis, não ausências.

---

## 5. Convenções

- Usar `SourceDiagnostic::error(Span::detached(), ...)` para erros de tipo ou aridade.
- Métodos que recebem closures (`filter`, `map`, `find`, `any`, `all`, `sorted` com `key`) aplicam a função via `apply_func` existente em `rules/eval/closures.rs`.
- **P881 — tipos chamáveis como closures:** nomes que no escopo global denotam tanto um tipo quanto um construtor (`str`, `int`, `float`, `type`, `counter`, `state`, `symbol`, `bytes`, `datetime`) podem ser passados como valor de primeira classe para métodos de ordem superior (ex.: `(0, 1).map(str)`). O dispatcher aceita `Value::Type(callable)` e converte internamente para a `Func` nativa correspondente, replicando o comportamento do vanilla. Tipos não chamáveis (`bool`, `array`, `length`, …) continuam a ser rejeitados com a mensagem de tipo apropriada.
- `Value::truthy()` pode continuar nos métodos cujo contrato vigente o admite;
  `array.all` exige `Value::Bool` tanto na forma estática quanto na de instância
  e emite erro de tipo para qualquer outro retorno. (**P1142**)
- `array.dedup`, `array.chunks`, `array.windows` são métodos puramente estruturais (sem closures).

---

## 6. Scope-outs

- `array.min()`, `array.max()`.
- `str.replace` com regex.
- Métodos com argumento `default` (exceto `dict.at(default:)` e
  `array.at(default:)`, P714, implementados).
- ~~Unicode avançado em `str.clusters`~~ — **resolvido em P1142** com extended
  grapheme clusters; `str.codepoints` continua deliberadamente separado por
  scalar Unicode.
- ~~`str.find` devolve `int` no cristalino vs substring no vanilla~~ — **resolvido em P691**: `find` agora devolve `str | none` (paridade vanilla); o índice fica disponível via `position`.
- Mutação do dict original em `.remove()` / `.insert()` (dispatch por valor devolve novo dict).

---

## 7. Testes canônicos

Ver `01_core/src/compiler/eval/tests.rs`, secção P466 e P493, com testes E2E por método.
