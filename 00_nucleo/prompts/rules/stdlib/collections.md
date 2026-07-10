# Prompt L0 — `stdlib/collections` — métodos de instância de array, dict e str

**Camada**: L1  
**Ficheiro alvo**: `01_core/src/rules/stdlib/collections.rs`  
**Criado em**: 2026-06-25 (Passo P466)  
**Atualizado em**: 2026-07-10 (P690 — `str.len/at/slice` em bytes; P691 — `str.find` devolve substring/`none`; P692 — `str.matches` e `str.normalize` implementados; `unicode-normalization` autorizada em L1)  
**ADRs**: ADR-0037 (coesão por domínio), ADR-0107 (paridade com a linguagem — aqui a linguagem **é** bytes), ADR-0108 (medir antes de decidir), ADR-0117 Cláusula 4 (métodos de tipos existentes; não propõe estrutura em elementos).

---

## 1. Visão geral

`collections.rs` implementa métodos de instância para os tipos de coleção do Typst: `array`, `dict` e `str`. Os métodos são despachados pela sintaxe de chamada de método (ex.: `(1, 2, 3).first()`, `"ab".repeat(3)`) em `rules/eval/closures.rs::eval_func_call`, antes do dispatch genérico de funções.

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

---

## 2. Métodos de `array`

| Método | Assinatura | Semântica |
|--------|-----------|-----------|
| `first` | `array.first() -> any` | Primeiro elemento ou `none`. |
| `last` | `array.last() -> any` | Último elemento ou `none`. |
| `rev` | `array.rev() -> array` | Array invertido. |
| `sum` | `array.sum() -> int \| float` | Soma numérica; rejeita tipos não-numéricos. Mistura int/float produz float. |
| `sorted` | `array.sorted(key: function?) -> array` | Ordenação; `key` opcional. Compara `int`, `float` e `str`. |
| `filter` | `array.filter(pred: function) -> array` | Filtra por predicado. |
| `map` | `array.map(func: function) -> array` | Mapeia por função. |
| `find` | `array.find(pred: function) -> any` | Primeiro elemento que satisfaz ou `none`. |
| `any` | `array.any(pred: function) -> bool` | Algum satisfaz? |
| `all` | `array.all(pred: function) -> bool` | Todos satisfazem? |
| `zip` | `array.zip(other: array) -> array` | Pares `(a_i, b_i)`; trunca no menor tamanho. |
| `enumerate` | `array.enumerate() -> array` | Pares `(index, value)`. |
| `dedup` | `array.dedup() -> array` | Remove duplicados adjacentes. |
| `chunks` | `array.chunks(n: int) -> array` | Divide em sub-arrays de tamanho `n`; último pode ser menor. |
| `windows` | `array.windows(n: int) -> array` | Janelas deslizantes de tamanho `n`. |
| `flatten` | `array.flatten() -> array` | Acha um nível de arrays aninhados. Elementos não-array são incluídos tal como estão. |
| `fold` | `array.fold(start: any, reducer: function) -> any` | Reduz o array a um único valor aplicando `reducer(acc, item)` em cada elemento. |

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
| `clusters` | `str.clusters() -> array` | Array de strings com cada char (clusters simplificados). |
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
| `match` | `str.match(pattern: regex) -> dict \| none` | Primeiro match: dict `{start, end, text, captures}` com índices em **bytes**, ou `none`. Capturas em ordem posicional (grupos nomeados inclusive). (**P689**) ⚠️ **Débito (P692)**: o vanilla aceita `str \| regex`; o cristalino (P689) só aceita `regex`. Variação de assinatura, fora do alcance deste passo. |
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
- `Value::truthy()` (definido em `entities/value.rs`) é usado para avaliar o resultado dos predicados.
- `array.dedup`, `array.chunks`, `array.windows` são métodos puramente estruturais (sem closures).

---

## 6. Scope-outs

- `array.min()`, `array.max()`.
- `str.replace` com regex.
- Métodos com argumento `default` (exceto `dict.at(default:)`, implementado).
- Unicode avançado (`str.clusters()` devolve chars, não grapheme clusters reais).
- ~~`str.find` devolve `int` no cristalino vs substring no vanilla~~ — **resolvido em P691**: `find` agora devolve `str | none` (paridade vanilla); o índice fica disponível via `position`.
- Mutação do dict original em `.remove()` / `.insert()` (dispatch por valor devolve novo dict).

---

## 7. Testes canônicos

Ver `01_core/src/rules/eval/tests.rs`, secção P466 e P493, com testes E2E por método.
