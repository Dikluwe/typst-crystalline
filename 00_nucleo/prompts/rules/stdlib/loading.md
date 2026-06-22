# Prompt L0 — `stdlib/loading` — módulo de carregamento de dados
Hash do Código: e447fbcb

**Camada**: L1 (decode puro) + composição com L3 já existente.
**Ficheiro alvo**: `01_core/src/rules/stdlib/loading.rs`
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

Assinatura `fn native_X(ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value>` (ver `_comum.md`). Path posicional obrigatório (`args.items[0]` → `Str`).

| Função | Args | Devolve | Decode L1 |
|--------|------|---------|-----------|
| `read(path)` | path | `Str` (utf8) ou `Bytes` (binário) — **ver §4** | (sem decode; bytes→Str utf8 ou Bytes) |
| `csv(path, delimiter:, row-type:)` | path + named | `Array` de linhas | `decode_csv` |
| `json(path)` | path | árvore `Value` | `decode_json` |
| `yaml(path)` | path | árvore `Value` | `decode_yaml` |
| `toml(path)` | path | árvore `Value` | `decode_toml` |
| `cbor(path)` | path | árvore `Value` | `decode_cbor` |
| `xml(path)` | path | `Array` de nós | `decode_xml` |

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

### 3.3. `decode_xml` — Array de nós

- Cada elemento → `Dict { tag: Str, attrs: Dict, children: Array }` (forma vanilla `convert_xml`).
- Nós de texto → `Str`. Documento → `Array` de nós de topo.

## 4. `read` binário e byte-strings CBOR — P398

Com `Value::Bytes` materializado (Passo 398), o graded de P387 é levantado:

- `read(path)` usa heurística vanilla: tenta UTF-8 → `Value::Str`; se falhar, retorna `Value::Bytes`.
- `decode_cbor` com byte-strings → `Value::Bytes`.

A heurística UTF-8 é suficiente para paridade linguagem (ADR-0107); encoding detection sofisticado (BOM, ISO, etc.) permanece scope-out ADR-0054 graded.

**Histórico**: em P387, `Value::Bytes` estava ausente (ADR-0017); `read` binário e byte-strings cbor eram graded e registados como DEBT-62. P398 fecha DEBT-62.
- **yaml usa `saphyr`** (parser mantido; mapa `Yaml → Value` manual), após `serde_yaml`/`serde_yml` se confirmarem não-mantidas (ADR-0111). A paridade de saída é independente da crate, provada pelos testes de bytes literais (§7).

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
// read (texto ou binário)
read(utf8 bytes) → Ok(Str);  read(bytes inválidos utf8) → Ok(Bytes)
// estratificação
native_csv(path inexistente) → Err (I/O, L3);  native_json(path com bytes malformados) → Err (parsing, L1)
```

> Cada caso de teste do decode usa **bytes literais** — prova a paridade de língua sem montar disco. Os bytes de cbor são produzidos por fixture pequena no teste (helper), não lidos de ficheiro.
