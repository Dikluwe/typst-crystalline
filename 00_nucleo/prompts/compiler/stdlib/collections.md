# Prompt L0 — `stdlib/collections` — superfícies de array, dict, str, bytes e arguments
Hash do Código: 3dad9a91

**Camada**: L1  
**Ficheiro alvo**: `01_core/src/compiler/stdlib/collections.rs`  
**Criado em**: 2026-06-25 (Passo P466)  
**Atualizado em**: 2026-08-30 (P1284 — inventário completo, `arguments` e projeções não ligadas; P1142 — formas estáticas `array.all`/`str.clusters`, predicado booleano estrito e grapheme clusters reais; P690 — `str.len/at/slice` em bytes; P691 — `str.find` devolve substring/`none`; P692 — `str.matches` e `str.normalize`; P693 — `str.match` aceita `str | regex`; P714 — `array.at(index, default:)`; P730 — `array.slice(start, end?, count:)`)
**ADRs**: ADR-0013 (`unicode-segmentation` permitido em L1), ADR-0037 (coesão por domínio), ADR-0107 (paridade com a linguagem), ADR-0108 (medir antes de decidir), ADR-0117 Cláusula 4 (métodos de tipos existentes; não propõe estrutura em elementos), ADR-0127 (correção de paridade em fluxo contínuo).

---

## 1. Visão geral

`collections.rs` implementa as superfícies dos tipos de coleção do Typst:
métodos de instância de `array`, `dict`, `str` e `bytes` e os membros estáticos
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

### 1.3 P1214 — métodos de `bytes`

Medição binária no vanilla ratificado `a51e02804`, congelada em
`00_nucleo/diagnosticos/p1214-bytes-oraculos.tsv`, confirma exatamente três
métodos no scope do valor: `len`, `at` e `slice`. `first`/`last` não existem.

| Método | Contrato |
|---|---|
| `len()` | número de bytes; `bytes("é")` tem comprimento 2 |
| `at(index, default:)` | índice positivo/negativo; byte como int 0–255; default somente fora de limites |
| `slice(start, end?, count:)` | novo bytes; fronteiras admitem len; negativos; end exclusivo; end<start vazio |

`slice` resolve `start` antes de calcular count. `end` explícito prevalece
sobre `count:` se ambos forem fornecidos. A soma `start_resolvido + count`
usa wrapping de `i64` porque o índice envolvido e a mensagem resultante são
observáveis no vanilla ratificado: os casos extremos P1214 reportam
`-9223372036854775808` e `-9223372036854775807`. Não substituir por panic,
saturação ou erro de overflow inventado.

Aridade, tipos e named desconhecidos preservam as mensagens do oráculo. O
dispatcher só reconhece os três nomes e retorna `None` para os demais.

Esta é correção de paridade em fluxo contínuo ADR-0127: nenhum tipo Rust
público, default geral ou fase do pipeline muda.

### 1.4 P1215 — âncoras diagnósticas de `bytes`

Medição vanilla/cristalina em compile e eval separou as regiões observáveis:

- ausência de argumento e índice fora de limites ancoram na chamada inteira;
- cast de positional ancora somente no argumento correspondente;
- positional excedente ancora somente no primeiro excedente;
- named desconhecido ancora no argumento named completo (`nome: valor`).

O dispatcher recebe metadados internos da chamada AST (span integral, spans
posicionais e named), sem alargar `entities::Args`. Chamadas sintéticas usam
`Args::span` como fallback. Mensagem, valor e superfície permanecem os de
P1214. A correção não autoriza especialização no renderer por texto/método.

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

---

## 8. P1284 — superfície completa e projeções não ligadas

### 8.1 Medição anterior à decisão

O contrato independente `C-P1284-v2`, selado antes de código candidato em
`00_nucleo/diagnosticos/p1284-contract-receipt.md`, e a fonte vanilla
ratificada `a51e02804` medem como ausentes no valor-tipo as projeções abaixo.
No vanilla, cada método de instância também é uma função no scope do tipo:
`type(array.len) == function` e `array.len((1, 2, 3)) == 3`. A forma ligada e
a não ligada compartilham nome, metadata, validação e semântica; a segunda
apenas recebe `self` como primeiro positional.

### 8.2 Decisão e fronteira de ownership

Este consumer é o único owner da semântica de coleção e de `arguments`. O
owner `bindings/field_access` descobre wrappers, e `call_dispatch` encaminha a
forma ligada; nenhum deles reimplementa operações. Não se adicionam variantes,
campos ou métodos Rust públicos. Helpers novos ficam privados ou `pub(crate)`.

As tabelas seguintes são normativas e substituem assinaturas históricas mais
estreitas deste documento quando houver conflito.

| Tipo | Superfície completa P1284 |
|---|---|
| `array` | `all(self,test)`; `any(self,test)`; `at(self,index,default:?)`; `chunks(self,chunk-size,exact:false)`; `contains(self,value)`; `dedup(self,key:?)`; `enumerate(self,start:0)`; `filter(self,test)`; `find(self,searcher)`; `first(self,default:?)`; `flatten(self)`; `fold(self,init,folder)`; `insert(self,index,value)`; `intersperse(self,separator)`; `join(self,separator:none,last:?,default:none)`; `last(self,default:?)`; `len(self)`; `map(self,mapper)`; `pop(self)`; `position(self,searcher)`; `product(self,default:?)`; `push(self,value)`; `reduce(self,reducer)`; `remove(self,index,default:?)`; `rev(self)`; `slice(self,start,end:none,count:?)`; `sorted(self,key:?,by:?)`; `split(self,at)`; `sum(self,default:?)`; `to-dict(self)`; `windows(self,window-size)`; `zip(self,exact:false,..others)`; estática/global `range(start:0,end,inclusive:false,step:1)` |
| `dictionary` | `at(self,key,default:?)`; `filter(self,test)`; `insert(self,key,value)`; `keys(self)`; `len(self)`; `map(self,mapper)`; `pairs(self)`; `remove(self,key,default:?)`; `values(self)` |
| `str` | `at(self,index,default:?)`; `clusters(self)`; `codepoints(self)`; `contains(self,pattern)`; `ends-with(self,pattern)`; `find(self,pattern)`; `first(self,default:?)`; `last(self,default:?)`; `len(self)`; `match(self,pattern)`; `matches(self,pattern)`; `normalize(self,form:"nfc")`; `position(self,pattern)`; `replace(self,pattern,replacement,count:?)`; `rev(self)`; `slice(self,start,end:none,count:?)`; `split(self,pattern:none)`; `starts-with(self,pattern)`; `trim(self,pattern:none,at:?,repeat:true)`; `to-unicode(self)` delegado a `foundations/str`; estática `from-unicode(value)` no mesmo owner irmão |
| `bytes` | `at(self,index,default:?)`; `len(self)`; `slice(self,start,end:none,count:?)` |
| `arguments` | `at(self,key,default:?)`; `filter(self,test)`; `len(self)`; `map(self,mapper)`; `named(self)`; `pos(self)` |

`?` significa named opcional sem valor materializado quando ausente. `none`,
`false`, `true`, `0`, `1` e `"nfc"` são defaults observáveis. Parâmetros após
`self` preservam exatamente a ordem acima; `..others` é variádico positional.
Named desconhecido, positional excedente, argumento ausente e tipo errado
falham, sem serem consumidos silenciosamente.

### 8.3 Semântica discriminatória obrigatória

- `all`, `any`, `find`, `position`, `filter` e `sorted(by:)` exigem retorno
  booleano; `all`/`any` fazem curto-circuito. `map`/`fold`/`reduce` preservam
  ordem. `dictionary.filter/map` e `arguments.filter/map` aplicam callback ao
  valor, preservando chaves/names e ordem.
- `arguments.len()` conta todos os argumentos, posicionais e named.
  A implementação calcula a superfície da linguagem a partir da representação
  existente; não altera silenciosamente o método Rust público `Args::len`.
- `flatten` é recursivo sobre arrays aninhados; `dedup` mantém a primeira
  ocorrência global e aceita `key:`; `sorted` é estável e aceita `key:` e
  `by:` simultaneamente.
- `chunks(exact:true)` descarta o resto incompleto; `windows` só produz janelas
  completas; tamanhos zero falham. `zip` sem outros arrays produz tuplas
  unitárias, trunca no menor por default e, com `exact:true`, rejeita qualquer
  comprimento divergente.
- `first`/`last`/`at`/`remove` usam `default:` apenas no caso vazio/fora de
  limites. Índices negativos contam do fim. `insert` admite a fronteira final.
- `push`, `pop`, `insert` e `remove` preservam a semântica mutante apenas para
  receiver ligado que seja l-value; a projeção não ligada não ganha mutação
  remota de bindings. Resultado e erro continuam os do método vanilla.
- `sum`/`product` usam as operações da linguagem e exigem `default:` no vazio;
  `reduce` no vazio devolve `none`. `range` rejeita step zero, respeita sinal,
  inclusive e overflow terminal sem loop infinito.
- `dictionary.keys/values/pairs` preservam ordem de inserção; chave repetida em
  `array.to-dict` escolhe o último valor; cada entrada deve ser par `(str, any)`.
- `str.len/at/slice/position/match/matches` usam índices UTF-8 em bytes;
  `clusters` usa graphemes estendidos e `codepoints` scalars Unicode.
  `first`/`last` operam em scalar; `at`/`slice` rejeitam fronteira inválida.
- `bytes.len` conta octetos, `bytes.at` devolve inteiro 0–255 e `bytes.slice`
  devolve `bytes`, nunca array nem string.

### 8.4 Critérios P1284

Para cada nome: existência nas formas ligada e não ligada, kind `function`,
metadata completa, chamada válida, defaults, erros de aridade/named/tipo e
igualdade semântica dos dois caminhos. Sentinelas mínimas:
`array.len((1,2,3)) == 3`, `bytes.len(bytes("é")) == 2` e
`arguments.len(arguments(1,x:2)) == 2`. Extensões cristalinas já documentadas
(`char-*`, `repeat`, `dict.update`) permanecem; não podem substituir nem
ocultar os nomes vanilla.

## P1307-R3 — transformação causal de arguments

**Estado**: `DRAFT_L0_AWAITING_ADR0127`. As decisões abaixo são alterações
explícitas de transporte e, nos casos de ordem/ocorrências, de comportamento;
não são uma extensão automática do fluxo contínuo histórico P1284.

### Medição anterior à decisão

No HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais P1306,
`01_core/src/compiler/stdlib/collections.rs:545-549` retira receiver ao
reconstruir Args; `:2402-2487` filter/map percorrem posicionais antes de
named e recriam views sem origens. No vanilla ratificado `a51e02804`,
`lab/typst-original/crates/typst-library/src/foundations/args.rs:414-445`
percorre cada ocorrência na ordem conjunta; filter conserva o Arg, map
conserva nome/arg-span e cria value-span detached. `:485-491` deixa o
span agregado dos dois resultados detached. Não inferir span de valor novo
da igualdade do resultado com o anterior.

As origens de Args/sink foram medidas na matriz independente
`00_nucleo/diagnosticos/p1307-r2-measurement.json`, SHA-256
`847faabad41df603a82f7fc5c5d0435180cdec66c33ae9b3a1fd55d7ee320fa2`.
Esta extensão de filter/map deriva da fonte acima; sua cobertura binária
adversarial adicional permanece obrigação futura, não PASS já medido.

### Decisão sujeita à confirmação humana

O owner usa `entities/args.md`; não redefine seu tipo. Na delegação estática,
retirar o receiver por `remove_positional(0)`, preservando a sequência dos
argumentos restantes. O Value::Args usado como receiver mantém sua sequência
independente da lista que o passa ao método.

`arguments.filter` percorre `occurrence_sequence` na ordem, aplica o
predicado uma vez por ocorrência e exige bool. Retém a ocorrência original
quando true. `arguments.map` percorre a mesma ordem e aplica mapper uma vez
por ocorrência; mantém name/arg-span, substitui value e destaca value-span.
Os dois resultados são `Args::from_occurrences(Span::detached(), sequence)`.
Callbacks continuam por apply_func, com argumentos de callback sintéticos;
não ganham a origem lexical do valor transportado por conveniência.

Isto preserva named repetidos e pode mudar quantidade/ordem de callbacks
relativamente às antigas views separadas. É mudança semântica declarada.
`arguments.len` conta a sequência quando Some (todas as ocorrências);
quando None, conta items+named. O método Rust Args::len não muda.
`pos` e `named` devolvem as views; `at` mantém índice positional ou lookup
do último named e os defaults/erros vigentes. Repr é de `compiler/eval/repr.md`.

Não invalidar carrier de usuário para manter um loop legado sobre views.
Outras coleções, mensagens de métodos (exceto revisão P1308 abaixo),
curto-circuito e mutação l-value ficam como antes. Spans privados P1215 não migram automaticamente para novo esquema;
as fronteiras de correção de cada método continuam nos seus contratos.

### Aceitação

Após gate, cobrir ordem mista, named repetido obtido legalmente por spread,
callbacks que distinguem ordem e falham na primeira ocorrência, filter que
remove um dos repetidos, map com retorno igual e com outro tipo, erros após
map com arg-span preservado e value-span detached, e resultado vazio.
Medir mensagens/hints/traces/âncoras sem confundir detached observado com
Unknown. As mudanças de ordem, contagem e span agregado acima não podem ser
ocultadas como ajustes mecânicos de struct literal.

## P1308 — cast do retorno de arguments.filter na origem da função

### Medição anterior à decisão

Baseline P1308 SHA-256
`62c53690cbe3dd36b5b79168ea59a32f6eb88cc52ea52a8ca97365c5a9459394`.
A matriz pública R6 final, pinada no relatório predecessor, mede
`expected boolean, found int` detached contra `integer` no parâmetro da
closure. Fonte ratificada `foundations/args.rs:408–412` faz cast bool e
ancora em `test.span()`; a origem da closure é params.span em
`typst-eval/src/call.rs:638`, não seu resultado nem o valor filtrado.

### Decisão autorizada

Somente no cast do retorno de `arguments.filter`, usar o nome longo
`vanilla_type_name` e `test.diagnostic_span()` do carrier privado de Func.
Não reancorar erro produzido dentro do callback. Não usar span do body,
argumento filtrado, última chamada, posição atual ou AST reconstruída.
Função sintética detached conserva detached. Aliases e With mantêm origem
pela entidade Func, sem lógica própria aqui. Callbacks continuam recebendo
Args sintéticos e são executados uma vez por ocorrência em ordem causal.
True retém a ocorrência completa; false a remove. Map permanece destacando
somente value_span do resultado, sem inventar origem por igualdade.

Panics nos callbacks seguem o contrato da nativa panic/call_dispatch: este
owner apenas propaga esses erros. Não corrigir outras coleções, aridade,
named, outros casts ou mensagens incidentais. Aceitação exige callback
direto/nomeado/alias/With, duplicatas, primeiro erro e controles de sucesso;
texto, hints, origem e traces são comparados integralmente.
