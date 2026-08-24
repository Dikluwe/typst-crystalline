# Prompt L0 — `stdlib/loading` — módulo de carregamento de dados
Hash do Código: 27a4efb8

**Camada**: L1 (decode puro) + composição com L3 já existente.
**Ficheiro alvo**: `01_core/src/compiler/stdlib/loading.rs`
**Origem**: achado da Lista B do Passo 386 (`typst-falta-migrar-lista-B-passo-386.md` §3) — cluster `loading` ausente em L1, não catalogado, bloqueante de `bibliography`. Convenção de assinatura e helpers: ver `stdlib/_comum.md`.
**Passo de origem**: P387 (materialização).
**ADRs**: ADR-0029 (pureza L1 / zero I/O — a razão da separação decode/leitura), ADR-0107 (paridade é com a língua — paridade = `Value` de saída, não a árvore interna do parser), ADR-0054 (perfil graded — subset por formato documentado), ADR-0111 (autorização de crates de parsing, PROPOSTO neste passo), ADR-0017 (não adicionar variant `Value` sem ADR + tipo migrado — base do scope-out de `Value::Bytes`).

---

## 1. Arquitetura — decode puro (L1) compõe com leitura (L3 já existente)

O vanilla acopla, em cada função, a leitura de disco ao parsing. O perfil L1 (ADR-0029, zero I/O) **proíbe** o acoplamento e isso é a engenharia melhor: a função parte em dois estratos.

- **L3 — leitura (JÁ EXISTE, reusar).** `World::read_bytes(current_file: FileId, path: &str) -> Result<Arc<Vec<u8>>, String>` já está no contrato (`01_core/src/contracts/world.rs:41`, usado por `native_image`). O `current_file` resolve o path relativo; obtém-se do `ctx` (ficheiro em avaliação). **Não** se cria L3 novo. É o único sítio que toca disco.
- **L1 — decode puro (NOVO).** `decode_{csv,json,yaml,toml,cbor,xml}(bytes: &[u8], opts) -> SourceResult<Value>`. Entrada: bytes já em memória. Zero I/O. Determinístico. Testável com bytes literais. Aqui vive a paridade de língua (ADR-0107).
- **Native funcs (registo).** `native_{read,csv,json,yaml,toml,cbor,xml}(ctx, args)` compõem: `bytes = ctx.world.read_bytes(ctx_current_file, path)?` (L3) → `decode_X(&bytes, opts)` (L1) → `Value`. A composição é o único sítio que conhece os dois estratos.

Consequência testável: o decode L1 prova-se com bytes literais (sem fixture de disco); o adapter L3 já tem cobertura. A paridade do `Value` de saída é toda L1, pura.

## 2. Funções nativas (registo em `stdlib/mod.rs`)

Assinatura `fn native_X(ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value>` (ver `_comum.md`). Posicional obrigatório: **caminho (`Str`) ou dados crus (`Bytes`)** — P701 estende `json`/`yaml`/`toml`/`cbor`/`xml` (a família que partilha a macro `native_loader!`) para aceitar `Value::Bytes` directamente, sem I/O, além do caminho já suportado. Paridade vanilla: estas 5 funções usam `DataSource` (`Str | Bytes`) no vanilla (`loading/{json,yaml,toml,cbor,xml}.rs`, todas `source: Spanned<DataSource>`) — a mesma dualidade, não uma extensão inventada. `read`/`csv` **não** foram tocadas por P701 (não partilham a macro; fora do âmbito medido).

| Função | Args | Devolve | Decode L1 |
|--------|------|---------|-----------|
| `read(path, encoding:)` | path + named `encoding:` (`"utf8"` default \| `none`) — **ver §4** | `Str` (utf8) ou `Bytes` (`encoding: none`) | (sem decode; bytes→Str utf8 ou Bytes) |
| `csv(path, delimiter:, row-type:)` | path + named | `Array` de linhas | `decode_csv` |
| `json(path \| bytes)` | path ou bytes | árvore `Value` | `decode_json` |
| `yaml(path \| bytes)` | path ou bytes | árvore `Value` | `decode_yaml` |
| `toml(path \| bytes)` | path ou bytes | árvore `Value` | `decode_toml` |
| `cbor(path \| bytes)` | path ou bytes | árvore `Value` | `decode_cbor` |
| `xml(path \| bytes)` | path ou bytes | `Array` de nós | `decode_xml` |
| `cbor.encode(value)` | 1 posicional (`any`) | `Bytes` | `value_to_cbor` — **ver §3.4** |

### 2.1 `cbor.encode` — valor com namespace, não função plana (P701)

`cbor` deixa de ser `Value::Func(Func::native("cbor", native_cbor))` plano e
passa a `Func::native_with_namespace("cbor", native_cbor, ns)`, com
`ns.define("encode", Value::Func(Func::native("cbor.encode", native_cbor_encode)))`.
Precedente idêntico já em uso: `curve`/`grid`/`table` (P512/P513/P493b,
`rules/eval/mod.rs`). `cbor(...)` continua chamável directamente (decode);
`cbor.encode(...)` é acedido por field access, resolvido pelo braço
`Value::Func(f) => f.namespace()` já existente em
`rules/eval/bindings.rs:618` — nenhuma mudança no dispatch de field access,
só no registo do valor.

## 3. Decode L1 — paridade de saída por formato (ADR-0107)

Mapeamento do modelo de dados de cada formato → `Value`. **Paridade é este mapeamento**, não a mecânica do parser. Crates per ADR-0111 (matcham o vanilla → saída paritária).

### 3.1. `decode_json` / `decode_yaml` / `decode_toml` / `decode_cbor` — árvore

Mapa canónico documento→`Value`:

| Tipo no formato | `Value` cristalino |
|-----------------|---------------------|
| null / ~ | `Value::None` |
| bool | `Value::Bool` |
| inteiro | `Value::Int` (i64; fora de i64 → `Float` ou Err — ver nota) |
| real | `Value::Float` |
| string | `Value::Str` |
| array/seq | `Value::Array` |
| objeto/mapa/table | `Value::Dict` (ordem de inserção, IndexMap — ADR-0023) |
| datetime (toml) | `Value::Str` (RFC 3339) — **graded**: mapa rico para `Value::Datetime` deferido (sub-caso raro; ADR-0054) |
| byte string (cbor) | `Value::Bytes` (P398) |

- Inteiro fora de i64 → seguir o vanilla (provável `Float`); documentar no fecho com o caso de teste.
- Chave de mapa não-string (yaml/cbor) → `Err` (paridade vanilla: chaves de dict são string).

### 3.2. `decode_csv` — Array 2D

- Sem header parsing (paridade vanilla: "Header rows will not be treated specially").
- `row-type: array` (default) → cada linha = `Array` de `Str`.
- `row-type: dictionary` → cada linha = `Dict` (1ª linha como chaves; per vanilla).
- `delimiter:` → `Str` de 1 char (default `","`); inválido (multi-char) → `Err`.
- Todas as células são `Str` (CSV não tipa).

**§P787 — rigor de parsing e API de `row-type` (mensagens medidas no vanilla
0.15.0 por execução, 2026-07-20):**

- **Linhas com nº de campos divergente são rejeitadas** (o modo `flexible`
  fica desligado — antes aceitava em silêncio, bug P786 B1). Mensagem exacta:
  `failed to parse CSV (found {len} instead of {expected_len} fields in line
  {line})` — `len`/`expected_len` do `csv::ErrorKind::UnequalLengths`, `line`
  do `Position` do erro (não inventada).
- **`delimiter:`** — 1 char ≠ → `expected exactly one character`; char
  não-ASCII → `delimiter must be an ASCII character` (a mensagem anterior
  "deve ser um único carácter" era enganadora — P786 D2).
- **`row-type:`** aceita o **tipo** (`dictionary`/`array` como `Value::Type`),
  não a string — API anterior divergente nos dois sentidos (P786 D4).
  Tipo errado (ex.: `str`) → `` expected `array` or `dictionary` ``; valor
  que não é tipo → `expected type, found {string|integer|...}` (nomes longos
  do vanilla: `str`→`string`, `int`→`integer`; fallback `type_name()`).


### 3.3. `decode_xml` — Array de nós

- Cada elemento → `Dict { tag: Str, attrs: Dict, children: Array }` (forma vanilla `convert_xml`).
- Nós de texto → `Str`. Documento → `Array` de nós de topo.

### 3.4. `value_to_cbor` — `Value` → CBOR (P701, `cbor.encode`)

Direcção inversa de §3.1 (`Value` cristalino → bytes CBOR), confirmada
contra `impl Serialize for Value` do vanilla
(`foundations/value.rs:345-366`) e `impl Serialize for Bytes`
(`foundations/bytes.rs:364-374`):

| `Value` cristalino | CBOR |
|---------------------|------|
| `None` | `Null` |
| `Bool` | `Bool` |
| `Int` | `Integer` |
| `Float` | `Float` |
| `Str` | `Text` |
| `Bytes` | `Bytes` (byte-string; CBOR não é human-readable, logo vanilla usa `serialize_bytes`, não a forma texto de `repr()` — `bytes.rs:370`) |
| `Array` | `Array` (recursivo) |
| `Dict` | `Map` (chave sempre `Text`; `IndexMap` preserva ordem — ADR-0023) |
| tudo o resto (Symbol, Content, Length, Color, Func, Module, Type, Label, Duration, Datetime, Version, Decimal, …) | `Text(repr_value(v))` |

**Divergência documentada (não é lacuna):** o vanilla dá a `Symbol` e
`Content` `Serialize` **dedicados** (`value.rs:357-358` — texto estruturado
para symbol, mapa descrevendo o elemento para content); tudo o resto cai no
fallback genérico `serializer.serialize_str(&other.repr())` (`value.rs:363`).
O cristalino **não** implementa os dois casos dedicados — Symbol e Content
caem no mesmo fallback `Text(repr_value(v))` que os restantes tipos opacos.
Scope-out explícito: nenhum consumidor real medido (`cetz`, único motivador
de P700/P701) usa `cbor.encode` com Symbol ou Content — os payloads são
dicts/arrays de `Int`/`Float`/`Str`/`None`. Revisitar se um consumidor real
precisar da forma estruturada.

## 4. `read` binário e byte-strings CBOR — P398 (comportamento de `read` corrigido em P824)

Com `Value::Bytes` materializado (Passo 398), o graded de P387 é levantado:

- `read(path)` usa heurística vanilla: tenta UTF-8 → `Value::Str`; se falhar, retorna `Value::Bytes`. **[REVOGADO em P824 — ver abaixo]**
- `decode_cbor` com byte-strings → `Value::Bytes`.

**Correcção P824 (medição refutou a heurística acima):** o comportamento real medido do vanilla 0.15.0 (`read.rs:24-47`, `diag.rs:797-807`) é:

- `read(path)` e `read(path, encoding: "utf8")` → `Value::Str`; ficheiro não-UTF8 → **erro** `failed to convert to string (file is not valid UTF-8 in {ficheiro}:{linha}:{col})` (posição via `LineCol::try_from_byte_pos`, `diag.rs:1027-1039`).
- `read(path, encoding: none)` → `Value::Bytes` (bytes crus, mesmo em ficheiro UTF-8 válido).
- Named `encoding:` aceita apenas `"utf8"` ou `none` (enum `Encoding` do vanilla só tem `Utf8`): outra string → erro `expected "utf8" or none`; outro tipo → erro `expected "utf8" or none, found {tipo}`.

A "heurística UTF-8 com fallback silencioso para Bytes" que este L0 descrevia **não corresponde ao vanilla medido** (refutada em P810 §11, reconfirmada em P824) e foi removida do código e deste L0. Encoding detection sofisticado (BOM, ISO, etc.) permanece scope-out ADR-0054 graded — e é também scope-out do próprio vanilla (que só conhece `utf8`).

**Histórico**: em P387, `Value::Bytes` estava ausente (ADR-0017); `read` binário e byte-strings cbor eram graded e registados como DEBT-62. P398 fecha DEBT-62.
- **yaml usa `saphyr`** (parser mantido; mapa `Yaml → Value` manual), após `serde_yaml`/`serde_yml` se confirmarem não-mantidas (ADR-0111). A paridade de saída é independente da crate, provada pelos testes de bytes literais (§7).

---

## P418 (XL) — loading bibliográfico

**Extensão**: o módulo `loading` é reutilizado por `bibliography()` para ler bytes do ficheiro `.bib`/`.yaml`/`.json`. O decode propriamente dito (BibTeX/BibLaTeX/CSL-JSON) é delegado ao `hayagriva` crate (ADR-0111), não a um novo decode L1.

**Fronteira**: `native_bibliography` recebe o path, chama `ctx.world.read_bytes(...)` (L3) e entrega os bytes ao hayagriva. Não adiciona `decode_bib` L1.

## Histórico de revisões

| Data | Motivo | Arquivos |
|------|--------|----------|
| 2026-06-23 | P418 (XL): documentar reutilização do loading para bibliografia. | `loading.md`, `loading.rs`, `bibliography.md`, `bibliography.rs` |
| 2026-07-11 | P701: `cbor` ganha namespace (`cbor.encode`); `json`/`yaml`/`toml`/`cbor`/`xml` aceitam `Bytes` além de path (desbloqueia `cetz`, P700). | `loading.md`, `loading.rs`, `rules/eval/mod.rs` |
| 2026-07-22 | P823: erro CBOR no formato do vanilla (`format_cbor_error` + sufixo ` in {ficheiro}`); P824: `read` ganha named `encoding:` (`"utf8"`/`none`), não-UTF8 sem `encoding: none` passa a erro (revoga a "heurística" de §4, refutada por medição). | `loading.md`, `loading.rs` |

## P1141 — consumers `path | str` (gate ADR-0127)

### Medição antes da decisão

Vanilla `loading/mod.rs:46-110` usa `DataSource::Path(PathOrStr)` e resolve a
string no span, mas preserva `RootedPath`. No cristalino,
`loading.rs:405-455` aceita apenas `Value::Str` para path. A sonda cross-file
P1141 prova que re-resolver um `Value::Path` no consumer mudaria sua base.

### Decisão

`read` e `csv` aceitam `Value::Path | Value::Str`; `json`, `yaml`, `toml`,
`cbor` e `xml` aceitam `Value::Path | Value::Str | Value::Bytes`. Um helper
único converte `PathOrStr`: path já enraizado segue a `World::read_path`; string
passa uma vez por `World::resolve_path(current_file, s)` e depois por
`read_path`. Decoders puros e semântica de `Bytes` não mudam. Erros que incluem
o path usam a vpath portátil, não `PathBuf` físico.

As assinaturas/casts públicos mudam; implementar somente após o gate P1141.

## 5. Estratificação de erro (critério de aceitação 4)

Mensagens distintas por estrato — o utilizador distingue I/O de malformado:

- **L3 (leitura)**: ficheiro inexistente/ilegível → erro de I/O (vem de `world.read_bytes`, propagado).
- **L1 (decode)**: bytes malformados (json inválido, csv com delimiter mau, etc.) → erro de parsing com o formato e, quando o parser oferecer, linha/coluna. Não inventar offset que o parser não dê.

## 6. Pureza (critério de aceitação 2) — V4/V13

O módulo de decode L1 **não importa nada de disco/rede/relógio**. Só os parsers (ADR-0111) e tipos L1 (`Value`, `Args`, `SourceResult`). As crates de parser entram em `[l1_allowed_external]` por ADR-0111; nenhum tipo do parser (serde_json::Value, etc.) aparece em contrato público L1 (V14): as funções devolvem `Value` cristalino, a conversão é interna.

## 7. Critérios de Verificação (decode L1 — bytes literais, sem disco)

```
// json
decode_json(b"{\"a\":1,\"b\":[true,null]}") → Ok(Dict{a:Int(1), b:Array[Bool(true),None]})
decode_json(b"3.5") → Ok(Float(3.5));  decode_json(b"\"x\"") → Ok(Str("x"));  decode_json(b"{bad") → Err
// yaml
decode_yaml(b"a: 1\nb: hello") → Ok(Dict{a:Int(1), b:Str("hello")})
decode_yaml(b"- 1\n- 2") → Ok(Array[Int(1),Int(2)])
// toml
decode_toml(b"x = 1\ny = \"s\"") → Ok(Dict{x:Int(1), y:Str("s")})
decode_toml(b"d = 1979-05-27") → Ok(Dict{d:Str("1979-05-27")})  // graded: datetime→Str
// cbor (tree); byte-string → Bytes
decode_cbor(<cbor de {"a":1}>) → Ok(Dict{a:Int(1)});  decode_cbor(<cbor byte-string 0xDE 0xAD>) → Ok(Bytes([0xDE, 0xAD]))
// csv
decode_csv(b"a,b\n1,2", delim=',', row=array) → Ok(Array[Array[Str("a"),Str("b")], Array[Str("1"),Str("2")]])
decode_csv(b"a,b\n1,2", row=dictionary) → Ok(Array[Dict{a:Str("1"), b:Str("2")}])
decode_csv(b"x;y", delim=';') → Ok(Array[Array[Str("x"),Str("y")]]);  decode_csv(b"x", delim="ab") → Err
// xml
decode_xml(b"<r><c>t</c></r>") → Ok(Array[Dict{tag:Str("r"), attrs:Dict{}, children:Array[Dict{tag:Str("c"),attrs:Dict{},children:Array[Str("t")]}]}])
// read (texto ou binário) — P824
read(utf8 bytes) → Ok(Str);  read(bytes inválidos utf8) → Err("failed to convert to string (file is not valid UTF-8 in {ficheiro}:{l}:{c})")
read(utf8 bytes, encoding: "utf8") → Ok(Str);  read(bytes, encoding: none) → Ok(Bytes)
read(path, encoding: "latin1") → Err("expected \"utf8\" or none");  read(path, encoding: 5) → Err("expected \"utf8\" or none, found integer")
// estratificação
native_csv(path inexistente) → Err (I/O, L3);  native_json(path com bytes malformados) → Err (parsing, L1)
// P701 — cbor.encode (Value → CBOR) e cbor(bytes)/json(bytes)/etc. (aceitar Bytes, não só path)
value_to_cbor(Dict{a: Int(1), b: Str("texto"), c: Array[Int(1),Int(2),Int(3)]}) → CBOR map equivalente
cbor.encode((a: 1, b: "texto", c: (1, 2, 3))) → Bytes;  type(cbor.encode(1)) == bytes
cbor(cbor.encode((a: 1))) → Dict{a: Int(1)}  // ida e volta
native_json(bytes de `{"a":1}`) → Ok(Dict{a:Int(1)})  // aceita Bytes, não só path
```

> Cada caso de teste do decode usa **bytes literais** — prova a paridade de língua sem montar disco. Os bytes de cbor são produzidos por fixture pequena no teste (helper), não lidos de ficheiro.
