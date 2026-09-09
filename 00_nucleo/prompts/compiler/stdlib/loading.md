# Prompt L0 — `stdlib/loading` — módulo de carregamento de dados
Hash do Código: c0f17d3b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24

## P1307-R5 — snapshot de conteúdo consultado (proposta; gate ADR-0127 pendente)

### Medição anterior à decisão

Baseline R5 `00_nucleo/diagnosticos/p1307-r5-baseline.json`, SHA-256
`32bae26c9d5175cb4567a6c0b4c1cbae17e8bb0818466879182936fd473d5d7a`:
HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working tree não
commitado com diff/stat integral. A medição independente R5, SHA-256
`82b2de8863ae5cd4b706eb9d5a6b285e1ed31dce3c126c8e5383a5af59ab46c8`,
preserva fontes, horários e executáveis; referência upstream `a51e02804`.

`compiler/stdlib/loading.rs:240–263` tem fallback CBOR por repr; R4 mostrou
que o encoder novo não pode recuperar chain perdida pelo walk. Essa perda
refuta expressamente a inferência R3 de suficiência sem entidade além de Args.

### Decisão proprietária

A partição total de Value dos três encoders permanece a de R3/R4. No braço
Content, LocatedContent Some produz func e mapa autoritativo, recursivamente,
sem defaults adicionados por campo, transformação em repr ou realização
runtime. LocatedContent None usa a projeção pública legada de sua árvore,
assim como Content raw usa campos de presença — não os defaults hardcoded
da CLI. O alcance de Content não fica restrito a Heading: todas as classes
obrigatórias R4 continuam classificadas/testadas pelo braço pertinente.

As dívidas do produtor de Heading (named tolerados mas descartados, callbacks
e região não modelada) não são corrigidas nem mascaradas no encoder. O novo
carrier resolve transporte, não justifica alegar paridade desses inputs.
Defaults completos do Heading padrão consultado continuam obrigatórios nos
três formatos, inclusive label causal e corpo estruturado. Testes de tipos
compostos devem preservar um snapshot aninhado e sua ordem.

No CBOR, manter Content/LocatedContent como fallback textual existente. Para
LocatedHeading Some, a repr realizada de
`00_nucleo/prompts/compiler/eval/repr.md` R5 muda deliberadamente a string
CBOR: registrar o delta, não trocar pela saída vanilla estruturada. Raw
Content/Symbol mantêm suas dívidas anteriores e controles. Esta exceção
estreita soma-se às exceções R3 de State/Counter/Location/With/Args; não é
promessa de preservação byte-a-byte dos snapshots nem correção geral CBOR.

Validação, spans, Args, formatos, escalar/None/pretty e dependências R3/R4
não mudam. É inferência que o snapshot resolve o bloqueio de dados do domínio
modelado; uma rota que descarte o carrier ou um campo obrigatório ainda
irrecuperável a refuta. Não publicar GREEN enquanto essa inferência não for
atacada com testes causais independentes após o gate do carrier.

---


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
| `csv(path\|bytes, delimiter:, row-type:)` | `Path\|Str\|Bytes` + named | `Array` de linhas | `decode_csv` |
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
  do `Position` do erro nesta revisão histórica. **P1315 abaixo substitui
  somente essa origem de `line` pelo ordinal do registro.**
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

Na decisão histórica P1141, `read` e `csv` aceitam `Value::Path | Value::Str`;
P1313 abaixo amplia somente CSV para incluir Bytes. `json`, `yaml`, `toml`,
`cbor` e `xml` aceitam `Value::Path | Value::Str | Value::Bytes`. Um helper
único converte `PathOrStr`: path já enraizado segue a `World::read_path`; string
passa uma vez por `World::resolve_path(current_file, s)` e depois por
`read_path`. Decoders puros e semântica de `Bytes` não mudam. Erros que incluem
o path usam a vpath portátil, não `PathBuf` físico.

As assinaturas/casts públicos mudam; implementar somente após o gate P1141.

## 5. Estratificação de erro (critério de aceitação 4)

### Validação causal de argumentos CSV — P1321

#### Medição anterior à decisão

Working tree P1319/P1320 sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, identificado com fonte/L0,
diff/stat, horários e executáveis em `p1321-baseline.json` e
`00_nucleo/diagnosticos/p1321-measurement.json`, SHA-256
`f13c4ea33fe8d8f91a5ef443d3183d1cbb3f77d9ac6726c0e4c5e23c54fbef05`.
Em loading.rs:1381–1389 o unknown precede fonte/opções; excedentes não são
rejeitados antes do parser. No vanilla ratificado `a51e02804`, fonte inválida
vence unknown, delimiter inválido vence unknown, e remanescente vence tanto
arquivo inexistente quanto CSV malformado. Duas ordens de unknown/excesso
mostram que a primeira ocorrência restante decide o diagnóstico.

A intenção explícita está em `typst-macros/src/func.rs:375–425`: handlers
dos parâmetros, finish, chamada; `typst-library/src/loading/csv.rs:27–46`
declara source, delimiter, row-type nessa ordem. `foundations/args.rs:152–183`
declara missing/positional e hint, :218–235 converte todas as ocorrências
named, :259–266 rejeita o primeiro remanescente em seu span completo.
A checagem extra `csv(source: ...)` versus `csv(bytes(...), source: ...)`
refuta tratar o nome source sempre como missing: havendo positional válido,
ele é um unknown remanescente. Mensagens/âncoras são comportamento medido;
a política de rejeição está documentada na fonte, não inferida de um acidente.

#### Obrigação proprietária

Somente a validação nativa CSV passa a seguir esta ordem, antes de obter dados:

1. Primeiro positional obrigatório source e seu cast vigente Path/Str/Bytes.
   Ausente: se houver named source, erro `the argument `source` is positional`
   no span completo da primeira ocorrência source, hint `try removing `source:``.
   Sem named source: `missing argument: source` no span agregado de Args.
   Havendo positional, seu cast precede opções e qualquer remanescente;
   preservar mensagem/cast e value_span P1313, inclusive rejeição de Symbol.
2. Todas as ocorrências delimiter e depois todas row-type, com as conversões,
   ordem interna, último válido e value_span P1314. Falha interrompe antes
   da validação de remanescentes. Defaults/casts não mudam.
3. Ignorar o primeiro positional consumido e todas as ocorrências dos dois
   nomes de opção; rejeitar a primeira ocorrência restante na ordem causal.
   Positional excedente: `unexpected argument`; named: `unexpected argument: N`.
   Usar span completo, não value_span nem chamada, com erro único, severidade
   Error e sem hints adicionais. Named source com positional é remanescente
   comum, não ganha conselho de argumento ausente.
4. Somente argumentos integralmente válidos chegam à resolução/leitura e
   ao parser. Não há I/O por falha de validação nem parsing que a antecipe.

With/Args/spread/sink conservam ocorrência e origem transportadas; detached
explícito continua detached, sem inventar range. Args sintético sem occurrences
segue a política existente de occurrence_sequence: items na ordem, depois
named na ordem do mapa, com spans individuais detached. Missing sem named usa
args.span também no sintético. Args não muda. A reabertura R2 abaixo autoriza
somente o transporte agregado CSV no owner de dispatch; demais owners intactos.

Esta seção substitui expressamente as preservações de missing/unknown/excesso
e da ordem externa de validação em P1313–P1319. Excesso com CSV inválido deixa
de atingir o parser; isso é o delta pretendido, não regressão a esconder.
Testes anteriores que misturavam parsing com excesso devem conservar ambos
os controles: parsing sem excesso e rejeição sem leitura com excesso.

#### Fronteiras e aceitação

Preservar parser único, decode_csv puro, valores e diagnósticos após validação
bem-sucedida, origem/sufixo P1319 binário, texto UTF-8 válido e erros de I/O
legados, resolução current_file/Path e demais loaders/encoders. Coerção Symbol
continua fora: casos mistos podem ganhar a precedência contratada sobre unknown,
mas não receberão alegação indevida de paridade Symbol. Não criar csv.encode,
API pública Rust, campo/trait, crate, flag, fase, contrato de I/O ou helper público.

Aceitação no nível da língua e do diagnóstico integral: RED→GREEN local,
origens distintas de span/value_span/args.span, fallback sintético, ausência
de I/O, colisões de erros, named source com/sem positional, ordem causal e
transporte With/Args, controles de opções/parser/outros loaders. A/B separado
congela antes do patch as correções vanilla e preservações/deltas normativos
explicitamente delimitados, repete/reordena; Unknown obrigatório bloqueia.
É inferência que os carriers atuais bastam no owner: origem irrecuperável,
contrato público novo ou perda de precedência a refuta e exige nova decisão.
Correção de paridade interna em fluxo contínuo ADR-0127, L0 primeiro, sem
mudança arquitetural de fase/pipeline e sem promessa de paridade geral CSV.

#### P1321-R2 — agregado recebido pelo validador

Medição anterior à decisão: o candidato R1, binário SHA-256
`deb6aed85d133665b00d416fabee92aeda04196ff76b6a734ef0451bbdeaea3c`,
falhou no A/B imutável `p1321-ab2-candidate.json`, SHA-256
`d1699b5ae42327c7ba7254f5f1f2830453d0cd225fce8f79807cf329aa2a1a25`:
as oito fontes missing sem named source marcam a lista, não a chamada inteira.
Estado, diff/stat e UTC anteriores à reabertura constam de
`00_nucleo/diagnosticos/p1321-r2-baseline.json`, SHA-256
`8ca670bb4ab656a29667abd67fc06bc4d227a3dcc0afa9330a5e9ad49537f065`.
Em call_dispatch.rs:398, Args recebe args_node.span(); :455–487 transporta
a chamada só para encoders e panic. A perda de origem refutou a suficiência
do owner único, não o resultado esperado. A fonte ratificada
`typst-eval/src/call.rs:56–78` entrega a chamada ao agregado e
`foundations/args.rs:160–173` usa esse agregado no missing.

Loading continua dono exclusivo da validação e usa o args.span recebido,
sem procurar AST/Source, comparar mensagens ou fabricar ranges. O L0 de
call_dispatch passa a legitimar separadamente o transporte de call.span()
para CSV por identidade nativa e através de With, antes da aplicação. Esta
é a única exceção R2 à proibição inicial de alterar dispatch. Aplicação
sintética sem AST conserva o span recebido. Mensagens, hints, ocorrências,
value_spans, demais validações e todos os oráculos R1 permanecem inalterados.
Correção interna de paridade/entrada em mapeamento existente: fluxo contínuo
ADR-0127, dois owners com L0s 1:1, sem novo campo, assinatura ou fase.

### Diagnóstico CSV de arquivo com buffer UTF-8 inválido — P1319

#### Medição anterior à decisão

Baseline commit `d31047d7b8af7837c84adae4ded3d2ff50c62093`, que integra
P1315–P1318. `00_nucleo/diagnosticos/p1319-measurement.json`, SHA-256
`4396b098d5f5db30bfd832d210c1027eb9529d8120c18c5a63fd9bc16d8ae3bd`,
preserva fonte/L0, estado, UTC, bytes das fixtures reais, argv/cwd e saídas.
Em `loading.rs:1375-1376`, CSV Path/Str descarta o caminho resolvido e retorna
o decoder detached. A referência ratificada `a51e02804` indica
`found 1 instead of 2 fields in line 2 in invalid-after-earlier-ls.csv:1:1`
e sublinha o argumento; o cristalino omite caminho/posição e origem.

`lab/typst-original/crates/typst-library/src/diag.rs:851-855,895-923`
seleciona apresentação binária pela validade do buffer inteiro, não pelo
tipo de erro. O byte inválido posterior ao registro curto comprova que
UnequalLengths continua vencedor. O caminho de Project não tem slash inicial;
Package usa a spec seguida da vpath com slash. A intenção explícita é não
apresentar conteúdo ilegível como fonte externa; a posição peculiar é
comportamento medido, não promessa de localizar o byte inválido.

A checagem extra com texto válido encontra range no arquivo externo
(`diag.rs:858-873`), não sufixo binário. O contrato `World::read_path`
(`contracts/world.rs:58-60`) fornece bytes, não FileId; RootedPath conserva
root/vpath, sem identidade numérica. `include_path` é contrato de inclusão
de Source, não atalho para fabricar identidade diagnóstica. Esse outro
estrato não cabe nesta correção local e permanece explicitamente aberto.
Detached Str e excesso também refutam paridade geral: vanilla pode falhar
na resolução/validação antes do parsing, enquanto o baseline chega ao CSV.

#### Obrigação proprietária e fronteiras

Somente parsing nativo `csv(Path|Str)` cujo buffer inteiro não seja UTF-8
válido recebe, antes do parêntese final da causa, ` in P:L:C`, com P o
caminho virtual resolvido. Project: vpath sem slash inicial. Package:
representação canônica da PackageSpec concatenada à vpath com slash.
Não usar caminho físico, string original não normalizada, nome de fixture,
basename isolado ou nova resolução de um Path já enraizado.

A posição continua sendo o offset byte do parser saturado a u32, convertido
pela regra binária P1318 (LF no prefixo; chars com substituição após o último
LF; sem LF, coluna 1), com fallback ordinal/1 se não houver posição.
Offset impossível não deve causar panic ou posição inventada: preservar
o caminho sem o componente L:C. A validade do buffer e conversão só são
consultadas após o parser escolher a falha. Não validar UTF-8 antecipadamente,
não reparsear e não reconstruir dados estruturados pelo texto do erro.

O span primário passa a ser o value_span da primeira ocorrência posicional
de Args, ignorando named e posicionais posteriores. With/Args/spread/sink
conservam a origem causal fornecida; detached explícito ou Args sintético
sem occurrences continua detached, mas não suprime o caminho/posição.
Preservar causa Utf8/UnequalLengths, ordinal, hints, severidade e erro único;
dispatch conserva seu papel nos traces. Não apontar para bytes do arquivo
inválido. Parsing com excesso e leitura Str detached mantêm as precedências
legadas: seus deltas são normativos separados, não paridade com vanilla.

Reusar resolução/leitura existentes uma única vez e reter RootedPath até o
diagnóstico. Str continua resolvido no current_file vigente; não corrigir
incidentalmente a dívida de resolução causal entre arquivos. Path preserva
root/base já capturados. Erros de resolução/leitura permanecem literalmente
os anteriores, assim como ordem unknown/cast/opções/dados/parsing. Nenhum
erro de opção deve realizar leitura. Não usar include_path/source nem
adicionar consulta World para obter FileId nesta etapa.

O decoder público puro, CSV Bytes P1318, arquivos UTF-8 válidos (valores e
erros), outros loaders/encoders, casts/opções/unknown/missing/excesso e
csv.encode permanecem inalterados. Esta seção substitui somente a preservação
de mensagens/spans Path/Str dos passos anteriores para buffers inválidos.
É permitida representação privada do contexto diagnóstico no mesmo owner,
mantendo um único parser; não criar API pública, entidade, trait, crate,
flag, namespace, fase ou I/O em L1. Não é paridade geral CSV/Path.

#### Aceitação e classe

Atualizar explicitamente antes do candidato as expectativas Path inválido
dos testes anteriores; conservar decoder puro, controles UTF-8 válido,
causas e asserções restantes. Exigir RED→GREEN, teste de identidade Project/
Package e caminho normalizado, byte inválido após erro anterior, CR/LF/CRLF,
Unicode, ambos row-types, origem distinta e detached, validação sem I/O,
resolução/leitura única e preservação Bytes. A/B independente congela
diagnósticos integrais vanilla nos casos compatíveis e expectativas normativas
separadas para precedências/resolução divergentes; repete e reordena.

Diagnóstico é observável de linguagem (exceção ADR-0108); a forma Rust é
livre. É inferência que RootedPath e Args atuais bastam neste ramo binário;
necessidade de FileId/novo contrato ou perda de informação a refuta e exige
nova decisão. Correção de paridade interna: fluxo contínuo ADR-0127 com L0
primeiro e revalidação. O ramo de texto válido é dívida separada, não um
constructor parcialmente materializado nem fechamento implícito futuro.

### Posição textual no parsing CSV Bytes — P1318

#### Medição anterior à decisão

Working tree P1315/P1316/P1317 não commitado sobre HEAD
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, fonte/L0 completos e estado em
`00_nucleo/diagnosticos/p1318-measurement.json`, SHA-256
`c19f749ad22b06e07d3078d6d9a72017e3c2e1f2b2546f738042f34362302378`.
O mapper em `loading.rs:944-1006` descarta a posição do parser; a composição
Bytes em `loading.rs:1304-1319` só ajusta o span do argumento. O vanilla
ratificado `a51e02804` acrescenta `at 2:1` ao erro de `a,b\n1` e `at 3:1`
ao de `"a\nb",c\n1`, conservando `line 2` como ordinal em ambos.

Na fonte ratificada, `loading/csv.rs:138-157` declara posição pelo offset
byte do erro, com fallback ordinal/coluna 1 quando ausente. `diag.rs:845-925`
acrescenta posição apenas para Bytes e separa texto válido de binário inválido;
`diag.rs:1025-1035`, `typst-syntax/src/lines.rs:88-95,252-274` e
`typst-syntax/src/lexer.rs:1144-1152` definem as duas conversões.
A intenção explícita é informar a posição do parser; os detalhes abaixo são
comportamento diagnóstico medido, não promessa de posição intuitiva do byte ruim.

A checagem extra refuta usar Position.line, ordinal, LF-only ou valid_up_to
indiscriminadamente: `a,b\r\n1` dá `at 1:5` (offset entre CR/LF), enquanto
o mesmo cabeçalho seguido de dados UTF-8 inválidos dá `at 1:1`. Byte inválido
após um registro anterior de largura errada também seleciona a conversão
binária, mas preserva UnequalLengths como erro vencedor. Separadores Unicode
contam como linhas no texto válido, não na conversão binária. A sonda Path
absoluta mede I/O por resolução virtual, não parsing nem nova obrigação Path.

#### Obrigação proprietária e fronteiras

Somente os erros de parsing da composição nativa CSV Bytes acrescentam
` at L:C` antes do parêntese final da causa. L/C são 1-based e derivados do
offset byte fornecido pelo parser (saturado ao range u32 de referência),
nunca de Position.line, ordinal, valid_up_to ou primeiro byte não UTF-8.
Sem offset, usar ordinal do registro e coluna 1, como o fallback explícito
da fonte. Offset impossível não pode causar panic ou origem fabricada.

Após o parser escolher o erro, a validade UTF-8 do buffer inteiro seleciona
a conversão. Texto válido: reconhecer LF/VT/FF/CR/NEL/LS/PS, CRLF como uma
quebra cujo início seguinte vem após LF; contar colunas em chars, não bytes
ou UTF-16. Um offset entre CR e LF ainda pertence à linha anterior. Buffer
inválido: contar LF no prefixo até o offset; contar chars com substituição
UTF-8 desde o último LF; sem LF no prefixo, coluna 1. A escolha pelo buffer
inteiro vale mesmo para UnequalLengths anterior ao byte inválido. A análise
de posição só sucede a falha; não antecipar nem trocar a precedência CSV.

Preservar causa P1317, ordinal P1315, hints, severidade, erro único e origem
P1316. With/Args/spread/map e Args sintético sem occurrences recebem o sufixo
independentemente de span; detached permanece detached, não é motivo para
omitir a posição textual. Dispatch/traces continuam vigentes. Parsing com
excesso mantém a precedência legada e recebe o mesmo sufixo como efeito
normativo, não como paridade com o erro vanilla de excesso.

decode_csv público mantém assinatura, valores e mensagens sem sufixo/spans
detached. Path/Str mantêm sua composição, mensagens e origem legadas. É
permitida delegação privada dentro deste owner para preservar o contexto
de posição antes de formatar o erro, com um único parser e sem reparse.
Não construir posição a partir da mensagem formatada. Nenhuma API, entidade,
trait, crate, flag, namespace, fase ou I/O novo; read e demais loaders intactos.

Esta seção substitui somente a ausência de sufixo na rota nativa Bytes das
preservações P1313/P1314/P1315/P1316/P1317. Não corrige mensagens de Path/Str,
casts, opções, unknown/missing/excesso ou csv.encode. Não é paridade geral CSV.
Expectativas locais anteriores que exigiam mensagem Bytes sem sufixo devem
ser atualizadas antes do candidato, com delta explícito, sem retirar casos,
asserções de origem/causa ou preservações dos decoders/caminhos.

Aceitação: RED→GREEN e A/B congelado com LF/CRLF/CR, campos multilinha,
Unicode e separadores, BOM, Utf8 em header/dados, erro anterior a byte inválido,
origens distintas/detached e controles Path/Str/opções/valores/outros loaders.
Comparar diagnóstico completo ao baseline acrescentando somente o sufixo
pré-declarado; nos casos bilaterais compatíveis exigir também vanilla integral.
Colisões de excesso usam expectativa normativa separada. Repetir/reordenar.
É inferência que a posição ainda disponível no owner basta sem contrato novo;
necessidade de outro owner, perda de origem ou posição divergente refuta o
fechamento. Mensagem é observável da linguagem (ADR-0108), não identidade Rust.
Fluxo contínuo ADR-0127: correção interna de paridade, L0 antes do código.

### Causa textual de UTF-8 inválido em CSV — P1317

#### Medição anterior à decisão

Working tree P1315/P1316 não commitado sobre HEAD
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`; fontes, hashes e estado em
`00_nucleo/diagnosticos/p1317-measurement.json`, SHA-256
`92b04d68105d573523c673c1bcd0fb64ec2ffe57d17788c79742ce0450ee2fdf`.
`loading.rs:955-965` trata Utf8 pelo fallback Display do parser. Bytes 255
produz `CSV parse error: record 0 ... invalid utf-8 ...`; o vanilla ratificado
`a51e02804` produz `file is not valid UTF-8 at 1:1` dentro do mesmo envelope.
Em `lab/typst-original/crates/typst-library/src/loading/csv.rs:138-157`,
o formatter declara explicitamente a causa `file is not valid UTF-8` para
ErrorKind::Utf8; `diag.rs:845-925` acrescenta localização por outra camada.
Mensagem diagnóstica é observável da linguagem (exceção ADR-0108).

A checagem extra de precedência em `p1317-measurement-focal.json`, SHA-256
`2a995791a497c9579fa8854d0531eebc74428b71afe92d59f249ec6143fa4fe0`,
mostra que linha curta com byte inválido gera UnequalLengths, não Utf8, nos
dois produtos. Logo validar UTF-8 antecipadamente mudaria o erro vencedor.
Arrays literais corrigem as sondas iniciais de concatenação Bytes, que o
baseline não suporta; aquelas sondas não provam parsing. Sondas de arquivo
inexistente medem somente I/O. A intenção é a causa explícita no formatter,
não copiar a mecânica de leitura/posição do vanilla.

#### Obrigação proprietária e aceitação

Quando o parser CSV retornar ErrorKind::Utf8, decode_csv deve emitir um
único erro com mensagem exata `failed to parse CSV (file is not valid UTF-8)`.
Aplicar tanto ao cabeçalho dictionary quanto aos registros de dados em ambos
os modos. Selecionar pela variante do erro, nunca por Display, mensagem,
fixture ou varredura UTF-8 anterior ao parser. Preservar todos os demais
erros literalmente, incluindo ordinal P1315 e precedência UnequalLengths.

O decoder puro permanece detached. A composição Bytes conserva a origem
P1316, inclusive With/Args e detached legítimo; Path/Str continuam detached
como antes. A mensagem nova propaga pelo decoder às rotas Path/Str sem
alterar leitura/resolução/ordem. Não acrescentar `at l:c`, `in path`, spans
externos, hints ou traces. Esta seção substitui somente a preservação da
mensagem Utf8 em P1313/P1314/P1315/P1316; demais fronteiras ficam íntegras.

Preservar assinaturas, entidades, opções, delimiters, casts, valores,
header/quoting, ordem unknown-named→cast→opções→dados→decode, I/O, outros
loaders e encoders. Não adicionar crate, fase, API, flag ou csv.encode.
CSV malformado com excesso positional continua chegando ao parsing legado;
se esse erro for Utf8, sua mensagem muda como efeito normativo, não como
paridade com o erro vanilla de excesso. Não alegar paridade geral CSV.

Aceitação: RED→GREEN local com cabeçalho/dados/Unicode válido, precedência
de UnequalLengths, origens distintas e caminho via World simulado; A/B
independente congelado preserva diagnóstico integral baseline mudando
somente a primeira linha dos casos Utf8 declarados. A causa deve coincidir
com a causa vanilla medida, sem remover posições para alegar igualdade total.
Replays, controles de opções/outros loaders e normal/repeat/reverse exigidos.
É inferência que o mapper atual basta; perda de precedência/origem ou
necessidade de outro owner refuta o recorte e exige nova decisão.
Correção interna de paridade: fluxo contínuo ADR-0127, L0 antes do código.

### Origem de parsing CSV em Bytes — P1316

#### Medição anterior à decisão

Working tree P1315 não commitado sobre HEAD
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`; estado, fontes integrais e hashes
anteriores em `00_nucleo/diagnosticos/p1316-measurement.json`, SHA-256
`b5d25936d78b8584cec7469fdde2a2cd5130da2f52936bcdd9f103efc28154d7`.
`loading.rs:1301-1302` retorna decode_csv(Bytes) diretamente, perdendo
contexto do argumento em erros de parsing. `csv(bytes("a,b\n1"))` produz
erro detached no baseline, mas aponta para `bytes(...)` no vanilla; With e
spread de Args conservam a origem pré-ligada e o trace no vanilla.

Na fonte ratificada `a51e02804`, `loading/mod.rs:79-110` transporta o span
da fonte e `diag.rs:845-925` distingue explicitamente Bytes de Path: Bytes
usa loaded.source.span, inclusive em erro UTF-8. Path com texto UTF-8 usa
range no arquivo externo, não o argumento. Essa checagem extra impede
generalizar incorretamente a correção a todos os sources. O comportamento
medido com Args.map tem origem detached legítima, que deve permanecer assim.

#### Obrigação proprietária e aceitação

Somente falhas de parsing na rota nativa `csv(Bytes)` recebem como span a
origem do valor da primeira ocorrência posicional de Args. Ignorar named
anteriores; não usar span do named, da chamada, dos bytes internos ou de
outra ocorrência. Origem explicitamente detached e Args sintético sem
occurrences continuam detached, sem range fabricado. Usar o carrier causal
existente. With/spread/sink preservam a origem que o carrier fornece.

Preservar exatamente message, hints e demais campos dos erros produzidos
pelo decoder; o dispatch vigente continua responsável por traces causais.
A apresentação dos traces pode mudar como consequência da origem correta,
de acordo com os casos congelados. Não escolher origem a partir do texto
do erro, nome de fixture ou conteúdo CSV. Não adicionar sufixo `at l:c`,
normalizar UTF-8 ou reescrever mensagens neste lote.

decode_csv puro mantém sua assinatura, comportamento e spans detached;
csv(Path/Str), I/O e diagnósticos de arquivo permanecem integrais. O parser,
os valores, o ordinal P1315 e a ordem unknown-named→cast→opções→dados→decode
não mudam. Nenhum campo público, trait, dependência, flag ou fase nova.
Bytes permanece sem chamadas ao World. Outros loaders/encoders ficam
protegidos. Esta cláusula substitui somente a preservação de origem detached
de parsing CSV Bytes em P1313/P1314/P1315, não as demais fronteiras.

#### Fronteira de excesso medida antes do candidato

A medição independente `p1316-ab-baseline-runs.json` em diagnósticos mostra
que Bytes malformado com positional excedente atinge parsing no baseline,
mas excesso no vanilla. Não alterar essa precedência neste lote. Nesse caso,
o erro de parsing legado recebe a origem do primeiro Bytes como nos demais;
é efeito normativo explícito, não paridade com o diagnóstico vanilla de excesso.
Exigir controle local com primeiro/segundo positional de origens distintas.
Preservar a medição bilateral e a dívida, sem forçar oráculo de span a partir
de erro vanilla de outro estrato. No A/B, excesso com fonte válida é controle
de preservação; as colisões parsing+excesso não provam paridade bilateral.

Aceitação: RED→GREEN local e A/B independente congelado, diagnóstico
integral com o texto baseline e apresentação da origem/traces vanilla.
Mensagens distintas continuam declaradas como dívida, não são removidas para
alegar paridade total. Cobrir UnequalLengths e UTF-8, array/dictionary,
With/Args/spread/map, origem detached e controles Path/Str, valores e opções.
É inferência que o carrier existente basta; perda de origem conservada pelo
vanilla refuta o fechamento e exige diagnóstico, não fallback inventado.
Origem diagnóstica é observável de linguagem (ADR-0108), não identidade Rust.
Correção interna de paridade: fluxo contínuo ADR-0127 com L0 primeiro.

### Ordinal no erro de campos CSV — P1315

#### Medição anterior à decisão

Baseline commit `bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, sem diff
produtivo; medição `00_nucleo/diagnosticos/p1315-measurement.json`, SHA-256
`ee6535435927f5ea6bbe8dadcd05663636b8cb41324151bd3d59a2a0c4019fe0`,
registra estado, horários, argv e binários. Em `loading.rs:956-963`, o
diagnóstico UnequalLengths usa Position.line. Para CSV com primeiro registro
`"a\nb",c` seguido de `1`, informa `line 3`; o vanilla ratificado
`a51e02804` informa `line 2 at 3:1`, separando ordinal e posição física.
Linhas vazias iniciais também refutam usar a linha física como ordinal.

A intenção explícita está em
`lab/typst-original/crates/typst-library/src/loading/csv.rs:54-77,150-153`:
o comentário rejeita Position.line devido ao comportamento do parser, e o
diagnóstico recebe o ordinal enumerado, com cabeçalho contado. A checagem
extra com CRLF confirma que reconstruir linhas físicas não é substituto.
UTF-8 inválido, sufixo de posição e span continuam divergentes e não autorizam
declarar paridade completa do diagnóstico.

#### Obrigação proprietária e aceitação

Somente o número `N` no fragmento `found X instead of Y fields in line N`
de UnequalLengths passa a ser o ordinal 1-based do registro CSV rejeitado.
Contar todos os registros retornados pelo parser, incluindo o cabeçalho em
dictionary; não contar linhas vazias ignoradas nem quebras internas de campos
citados como registros adicionais. Array e dictionary devem concordar no
ordinal para os mesmos dados. Não derivar N de Position.line/byte nem contar
newlines manualmente. X/Y continuam vindo de UnequalLengths.

Preservar texto circundante, erro único, hints, spans detached e traces
vigentes (origem de parsing na rota Bytes substituída explicitamente por
P1316 acima). Não acrescentar `at linha:coluna` ou caminho ao erro. Outros erros,
inclusive UTF-8 e header inválido, permanecem literais. Preservar opções,
ordem de validação, casts, I/O, valores, quoting, delimiters, rigidez de
campos e nomes/ordem dos dicionários. Nenhuma API, entidade, trait, crate ou
fase nova. A assinatura pública de decode_csv permanece idêntica.

Esta obrigação substitui a origem histórica de `line` em P787 e somente a
preservação desse número em P1313/P1314. Não é um novo parser nem constructor
dividido: a correção do ordinal é completa; o formato geral dos diagnósticos
continua dívida separada. Mensagem é observável da linguagem (exceção
ADR-0108). Fluxo contínuo ADR-0127, L0 primeiro, RED→GREEN e revalidação.
É inferência que enumerar os registros já consumidos basta; divergência
de ordinal em qualquer modo/caso obrigatório refuta o fechamento.

Exigir testes locais e A/B congelado com registros multilinha, vazios,
CRLF, cabeçalho, erro posterior e excesso/falta de campos; valores válidos
e demais erros são controles. No A/B, comparar o fragmento com vanilla e o
diagnóstico completo com o baseline alterando apenas N explicitamente.
Nunca remover sufixos/spans para alegar igualdade total dos produtos.

### Opções CSV — P1314

#### Medição anterior à decisão

Baseline P1313 no HEAD `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, working
tree não commitado, em `00_nucleo/diagnosticos/p1314-baseline.json` (SHA-256
`75cccd6e392df84001640f244facde73883fd414de045253e97e6a12bfad70c6`).
Medição bilateral `p1314-measurement.json` em diagnósticos (SHA-256
`660fd77c95fe243e22590036b03dc5e364b0e3a107888592b35c4d166d557626`)
registra argv, UTC, fontes e estado integral. `loading.rs:1233-1273` usa
somente Args.named e err detached para delimiter/row-type. Mensagens das
opções inválidas ordinárias coincidem com vanilla, mas não suas origens.
Uma opção delimiter inválida pré-ligada por With, sobreposta por outra válida,
é aceita no cristalino e rejeitada pelo vanilla na origem da primeira.

Na fonte ratificada `a51e02804`, `loading/csv.rs:32-45,103-135` declara os
casts e a ordem delimiter antes de row-type; `foundations/args.rs:218-235`
converte cada ocorrência e conserva a última somente após todas passarem,
atribuindo erro ao span do valor. Essa é a intenção explícita, não inferida
do resultado. Duplicatas sintáticas diretas são rejeitadas antes da função,
nos dois produtos, e não provam o comportamento dos argumentos transportados.

A checagem de fronteira refuta paridade geral: unknown-named vence o cast
no cristalino; delimiter Symbol é convertido a string no vanilla, mas
rejeitado no cristalino. Erros de parsing têm mensagem/âncora distintas.
Essas dívidas são separadas do consumidor de opções e permanecem explícitas.

#### Obrigação proprietária e aceitação

Somente as opções CSV delimiter e row-type passam a consumir a sequência
causal de Args. Validar todas as ocorrências delimiter em ordem, depois todas
row-type em ordem, independentemente da ordem relativa entre os dois nomes.
A primeira conversão inválida interrompe; a última válida vence se todas
forem válidas. Nenhuma ocorrência inválida pré-ligada pode desaparecer por
sobrescrita no mapa. Reusar o carrier existente de Args; não mudar entidade,
trait, assinatura pública Rust, dispatch, fase ou dependência.

O erro usa o value_span da ocorrência que falhou, não o named completo,
última ocorrência, fonte CSV ou chamada agregada. Args sintético sem sequência
usa seus named vigentes e origem detached. Origem explicitamente detached
permanece detached; não inventar ranges. Mensagem, hints e propagação de traces
seguem os casos medidos; não escolher origem por mensagem ou fixture.

Delimiter continua aceitando somente Str de um caractere ASCII; defaults e
mensagens permanecem (`expected exactly one character`, `delimiter must be
an ASCII character`, `expected string, found <tipo longo>`). Row-type aceita
somente Type::Array ou Type::Dictionary, com mensagens existentes. Os valores
decodificados de todas as opções válidas mantêm semântica e morfologia.
Delimiter Symbol continua rejeitado com `expected string, found symbol`
na origem do valor: esse efeito normativo não é paridade da coerção Symbol.
Row-type Symbol recebe `expected type, found symbol` na origem medida.

Preservar a ordem externa: unknown-named, fonte/cast, delimiter, row-type,
obtenção de dados, decode. Nenhuma falha de opção provoca I/O antecipado.
Preservar rejeição sintática de duplicatas diretas, missing, excesso, I/O,
parsing, cast da fonte/Bytes P1313, read P1312, outros decoders P1310 e
encoders/campos. Não corrigir incidentalmente essas superfícies. Esta cláusula
substitui a preservação de opções/duplicatas P1313 somente para origem dos
erros e conversão de todas as ocorrências dos dois nomes aqui delimitados.

Diagnósticos completos e valores CSV são observáveis de linguagem
(ADR-0107/0108), não igualdade de estruturas Rust. Correção interna de
paridade, sem nova superfície ou assinatura: fluxo contínuo ADR-0127,
L0 primeiro, RED→GREEN, freeze A/B e revalidação. É inferência que o carrier
existente basta: perda de uma ocorrência/origem conservada pelo vanilla antes
do consumer refuta o fechamento local e exige diagnóstico, não fallback
inventado. Não há promessa de paridade geral CSV.

### CSV/DataSource — P1313 (aprovado pelo dono em 2026-09-08)

#### Medição anterior à decisão

Baseline P1312 não commitado no HEAD
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, registrado em
`00_nucleo/diagnosticos/p1313-baseline.json`, SHA-256
`0ad67b8f92c25160f044c352241572916716e2489ad105a6314608d51f59de92`.
Medição bilateral `p1313-measurement.json` em diagnósticos, SHA-256
`f07efa356c554a4b80a5702c516c4e7f7bee51f4b70fc8cc419ea6cf8754cb43`,
registra fontes, argv, UTC, saídas e estado. No vanilla ratificado `a51e02804`,
`csv(bytes("a,b\n1,2"))` retorna duas linhas de strings; bytes vazios retornam
array vazio; delimiter `;` e row-type dictionary funcionam com Bytes, inclusive
via With. O cristalino rejeita todos esses casos no cast path-only.

A intenção vem de `lab/typst-original/crates/typst-library/src/loading/csv.rs:27-46`
(source: Spanned<DataSource>) e `loading/mod.rs:46-63,79-110` (PathOrStr ou
Bytes, ramo Bytes sem leitura). No cristalino, `loading.rs:944-1006` já decodifica
bytes puros; `loading.rs:1013-1023,1202-1262` restringe a entrada nativa a
Path/Str. `csv(42)` deve dar `expected path, string, or bytes, found integer`
na origem de 42, mas atualmente dá mensagem portuguesa, tipo curto e detached.

A mesma medição revela limites: erro de CSV malformado do vanilla inclui
posição textual e span da fonte; o mapper existente `loading.rs:956-970`
produz erro detached e texto parcialmente distinto. Named desconhecido também
vence o cast no cristalino, ao contrário do vanilla. Não confundir suporte a
Bytes e cast correto com paridade integral de diagnóstico/validação CSV.

#### Contrato aprovado

CSV aceita `Value::Path | Value::Str | Value::Bytes` como primeiro positional,
com named delimiter/row-type e defaults vigentes. Path/Str mantêm resolução e
leitura existentes; Bytes é conteúdo já carregado, nunca nome de arquivo,
Unicode a decodificar como caminho, nem motivo de acesso a World. Os mesmos
bytes, opções e modo de linhas devem produzir o mesmo valor de linguagem
quando fornecidos em RAM ou pelo caminho legado. Preservar strings, Unicode,
quoting CSV, linhas, ordem de campos e representação array/dictionary.

O cast inválido usa `expected path, string, or bytes, found <tipo público longo>`
e value_span da primeira ocorrência posicional. Named anteriores não são a
origem; With/Args devem preservar o carrier causal. Args sintético ou origem
explicitamente detached continua detached, sem range inventado. O ramo de cast
não lê arquivos e não altera valores. Não criar tipo DataSource público, trait,
assinatura Rust nova, modo CLI, membro csv.encode ou mudança de fase.

Preservar a ordem existente: rejeição de named desconhecido, validação da fonte,
opções, obtenção de dados e decode. Opções inválidas não devem provocar leitura
antecipada. Reusar o decoder puro existente; sua API e política de parsing não
mudam neste recorte. Para Bytes malformados ou opções inválidas agora atingíveis,
aplicar as mensagens/âncoras legadas do decoder/validador, com deltas explicitados
nos testes antes do candidato. Não mascarar divergências para declarar paridade:
o fechamento é admissão de Bytes e valores decodificados, mais o cast inválido,
não o formato completo dos erros de parsing/opções.

Symbol permanece rejeitado; recebe `expected path, string, or bytes, found symbol`
na origem do valor. O vanilla converte Symbol para Str e tenta leitura; o efeito
do formatter é normativo e separado de paridade. Não implementar essa coerção.
Argumento ausente, named/excesso, opção duplicada, I/O e demais validações mantêm
o comportamento anterior. Read/P1312, decoders P1310, campos P1311 e encoders
ficam protegidos. Esta cláusula aprovada substitui somente a restrição
CSV Path/Str em §2/P1141 e a preservação de rejeição Bytes/cast CSV de P1312.

#### Gate e aceitação

Houve paragem por dúvida ADR-0127 entre paridade contínua e ampliação do cast
público Typst/precedente P1141. O dono respondeu especificamente «Autorizo» em
2026-09-08 à proposta de ampliação CSV Bytes conforme P1313. O recibo
`00_nucleo/diagnosticos/p1313-implementation-baseline.json` identifica a proposta
aprovada e o estado anterior ao código. A aprovação não amplia o recorte acima.

Congelar expectativas independentes
para valores/Bytes, tipos inválidos, origem, Symbol e preservações; revisar
essas políticas antes do patch. Exigir RED→GREEN, ausência de I/O no caminho
Bytes, replays e lint. Semântica de valores e diagnóstico são observáveis de
linguagem (ADR-0107/0108), não identidade de Vec/Arc ou algoritmo do parser.
É inferência que o owner existente basta; perda de origem, necessidade de
outro owner ou alteração de contrato público Rust refuta esse escopo e exige
nova decisão antes de implementar. Nenhum selo ou paridade geral é prometido.

### Cast do caminho em `read` — P1312

#### Medição anterior à decisão

Baseline P1311 não commitado sobre HEAD
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, registrado em
`00_nucleo/diagnosticos/p1312-baseline.json`, SHA-256
`0597c75b13da999990886b32577b563bab330d8d1be4e5bf85e97ff0f03588c5`.
Medição bilateral `p1312-measurement.json` em diagnósticos, SHA-256
`d986f593b9d8ca8358d3bf8fc5303eade65bd47382dec0424c3cd3d9d54e07c1`,
conserva fontes, argv, UTC, saídas e diff/stat. `read(42)` no vanilla ratificado
`a51e02804` produz `expected path or string, found integer` em `42`; o
cristalino dá mensagem portuguesa, tipo curto e origem detached. Named antes
do positional não altera a âncora; With preserva a origem pré-ligada e o trace.

`lab/typst-original/crates/typst-library/src/loading/read.rs:24-29` declara
Spanned<PathOrStr>; `foundations/path.rs:216-224` define seu cast. Na mesma
fonte, `loading/csv.rs:27-31` recebe Spanned<DataSource>: `csv(bytes("a,b"))`
funciona no vanilla, enquanto o cristalino rejeita. Portanto a coorte histórica
path-only read/csv não é semanticamente homogênea. No cristalino,
`loading.rs:1013-1023,1085-1099,1191-1204` compartilha arg_path nos dois.
Uma mudança indiscriminada nesse helper alteraria CSV sem corrigir seu contrato.

#### Obrigação proprietária e fronteiras

Somente `read` corrige o diagnóstico do primeiro positional de tipo inválido:
`expected path or string, found <tipo público longo>`, com nomenclatura canônica
vanilla_type_name. Bytes continuam inválidos em read. O span é o value_span
da primeira ocorrência posicional de Args, ignorando named anteriores, não
o span agregado nem o argumento inteiro. Origem explicitamente detached ou
Args sintético sem ocorrências permanece detached; não inventar ranges.
Preservar erro único, hints e propagação causal de traces pelo dispatch existente.

Path e Str continuam válidos e seguem a resolução/leitura existente; o cast
não realiza I/O nem transforma valores. É permitido helper privado próprio
de read neste owner, separado do helper CSV; não discriminar o diagnóstico
por comparação de fname, nome lexical, fixture ou texto de erro. As assinaturas
públicas, entidades e pipeline permanecem inalterados.

Symbol no vanilla converte a Str e tenta abrir seu Unicode, como confirma
`foundations/value.rs:632-637` e a medição de `read(sym.alpha)`. A coerção está
fora deste lote. A rejeição cristalina permanece, com mensagem explícita
`expected path or string, found symbol` e origem do valor; esse efeito do
formatter não é paridade Symbol. Deve ser testado por expectativa normativa
separada, congelada antes do patch, sem rotulá-lo como igualdade vanilla.

Preservar CSV integralmente, inclusive sua rejeição atual de Bytes/Symbol e
mensagem antiga. O contrato CSV/DataSource exige revisão própria futura; este
passo não o materializa nem quita. Preservar os cinco decoders P1310, encoders,
missing positional, named/excesso, encoding e sua ordem de validação, erros de
leitura, campos de funções P1311 e demais owners. Esta obrigação substitui
o scope-out de read de P1310 somente para o cast inválido aqui delimitado.

Intenção de rejeitar tipos incompatíveis vem do cast declarado; mensagem e
origem são comportamento medido. Diagnósticos são observáveis de linguagem
(exceção ADR-0108), não igualdade estrutural Rust. É inferência que os carriers
existentes bastam; origem conservada pelo vanilla mas irrecuperável no consumer
refuta o escopo local. Gate ADR-0127: correção interna de paridade L0-first,
RED→GREEN e revalidação; sem nova API ou mudança de fase. Testes devem distinguir
mensagem, tipo longo e origem em chamada direta, alias/With e transporte Args,
com controles positivos e negativos dos casos explicitamente preservados.

### Cast da fonte nos cinco decoders — P1310

#### Medição anterior à decisão

No baseline `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`,
`01_core/src/compiler/stdlib/loading.rs:1047-1064` aceita Path/Str/Bytes,
mas o braço de tipo inválido produz mensagem portuguesa com `type_name()`
curto e `Span::detached()`. O vanilla ratificado `a51e02804`,
`lab/typst-original/crates/typst-library/src/loading/mod.rs:46-63`, declara
o cast DataSource como PathOrStr ou Bytes; os decoders recebem a fonte
Spanned. A medição bilateral P1309 de `json(42)` registra
`expected path, string, or bytes, found integer` com origem em `42`.
Recibo `00_nucleo/diagnosticos/p1309-sentinels-p1308-vanilla.json`, SHA-256
`8f29121b65d5b5e7e98c9c146c4750d4b0a6d08566028f97dc08bc8691848df5`.
O baseline P1310 preserva fonte, binários e diff/stat em
`00_nucleo/diagnosticos/p1310-baseline.json`, SHA-256
`3a19c3b842c5bdcc2d4e6df3ea8778a407a9843b2fc5ec183f3b07620c25d65f`.

#### Decisão proprietária e aceitação

Em `cbor/json/toml/xml/yaml`, tipo inválido no primeiro posicional produz
exatamente `expected path, string, or bytes, found <tipo público longo>`.
Reusar a nomenclatura pública canônica, inclusive integer, float, boolean,
dictionary, function, none e os demais tipos do Value vigente. O span primário
é o `value_span` da primeira ocorrência posicional, não o argumento completo,
um named anterior ou a chamada agregada. A origem causal já transportada por
With/Args/spread deve ser conservada; origem explicitamente detached e Args
sintéticos sem ocorrências permanecem detached, sem range inventado.

Os tipos admitidos Path/Str/Bytes, ordem das demais validações, parsing,
leitura e decoders/encoders não mudam. Argumento ausente, named/excesso,
read/csv e transporte de chamada pertencem a outros lotes; não corrigir esses
casos incidentalmente nem alegar paridade deles. Não adicionar campo, trait,
assinatura, dependência ou lógica de I/O ao core.

Mensagem completa, hints, span primário e traces observados pertencem à
linguagem diagnóstica (exceção ADR-0108), não à igualdade estrutural Rust.
A intenção normativa é rejeitar tipos fora do cast declarado com sua origem;
os textos/âncoras são comportamento medido. É inferência que o carrier Args
vigente basta: uma rota que conserve origem no vanilla e a perca antes deste
consumer refuta o fechamento local e exige diagnóstico, não fallback inventado.
Testes independentes devem distinguir mensagem correta com span errado,
nomes curtos/longos e preservação das entradas válidas. Correção de paridade
interna: fluxo contínuo ADR-0127 com RED→GREEN e revalidação, sem novo gate
público ou promessa de equivalência geral dos decoders.

#### Fronteira Symbol medida antes do candidato

A medição independente `00_nucleo/diagnosticos/p1310-ab-freeze-measurement.json`
revela que `json(sym.alpha)` no vanilla tenta ler o arquivo `α`, em vez de
rejeitar o cast. Na fonte ratificada, `foundations/path.rs:216-224` compõe
PathOrStr com Str; `foundations/value.rs:632-637` converte Symbol em Str.
Isso refuta a hipótese de que toda variante fora de Path/Str/Bytes também seja
rejeitada pelo vanilla. Não implementar a coerção Symbol→Str neste lote.
A rejeição cristalina já existente permanece, e o ramo comum passa a emitir
`expected path, string, or bytes, found symbol` na origem do valor. Esse delta
diagnóstico é efeito explícito do formatter comum, não fechamento de paridade
Symbol nem preservação literal da mensagem anterior. Registrar a comparação
bilateral e testar separadamente o efeito normativo; não classificar essa
testemunha como igualdade vanilla.

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

## P1307-R3 — encoders textuais (contrato proposto, gate público pendente)

### Medição anterior à classificação

O recibo `00_nucleo/diagnosticos/p1307-contract-measurement.json`, SHA-256
`f1191d3de029dabf6f66f87872bee8146aa43106ae54e84facc22952afd238d0`,
e seu sucessor `p1307-r2-measurement.json`, SHA-256
`847faabad41df603a82f7fc5c5d0435180cdec66c33ae9b3a1fd55d7ee320fa2`,
registram fontes literais, strings e diagnósticos integrais, argv, horários,
estado e perfis default/html/a11y/html+a11y. HEAD
`b303f1f15b610e09872b567027e0d806387fde8c`, working tree P1306 não commitado
com diff/stat completos nos baselines P1307/R2; vanilla ratificado
`a51e02804`, executável SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
As tentativas contextuais antigas inválidas não são RED: a rota R2 compile
documental tornou Location/LocatedContent observáveis bilateralmente.

Na fonte ratificada sob `lab/typst-original/crates/typst-library/src/`,
`loading/json.rs:131-151`, `loading/toml.rs:104-126` e
`loading/yaml.rs:108-123` declaram as assinaturas e a intenção de encode.
`foundations/value.rs:343-364`, `foundations/symbol.rs:394-401`,
`foundations/bytes.rs:362-374` e `foundations/content/mod.rs:709-719`
determinam a partição de valores; emissão exata
foi medida, não inferida da intenção. `foundations/args.rs:218-235,259-266`
valida cada named antes de reter o último e rejeita o primeiro remanescente.
`01_core/src/compiler/stdlib/loading.rs:240-263` já tem encoder CBOR com
fallback textual distinto para Symbol/Content; os três encoders faltam.

### Superfície e ownership

O único consumer é `01_core/src/compiler/stdlib/loading.rs`. Deve implementar
as nativas `native_json_encode`, `native_toml_encode`, `native_yaml_encode`,
usando a assinatura vigente de função nativa, sem API pública auxiliar de
serializer ou novo `Serialize` global em Value. A superfície Typst é:

```typst
json.encode(value, pretty: true) -> str
toml.encode(value, pretty: true) -> str // value: dictionary
yaml.encode(value) -> str
```

Exatamente um positional obrigatório; JSON/TOML aceitam somente o named
booleano `pretty`; YAML não aceita named. Defaults equivalem a `pretty:true`.
Não há novo modo CLI, feature, I/O, relógio ou fase. Decoder, namespace e
registro têm seus owners; estes encoders não abrem arquivos nem usam World
para serializar. Usar dependências já autorizadas; ADR-0111 mantém saphyr,
não adicionar serde_yaml/serde_yml ou outra crate para resolver emissão.

### Valores e emissão

None/Bool/Int/Float/Str/Array/Dict são estruturados recursivamente. Bytes nos
formatos human-readable produz a string `bytes(N)`, nunca lista de bytes.
Symbol produz Unicode do símbolo, não sua repr. Content, inclusive carrier
LocatedContent, produz mapa com `func` primeiro e campos públicos na ordem
da linguagem, recursivamente; não emitir a repr textual do conteúdo. Os
demais Values usam a fachada canônica de repr de eval. Não há branch por
nome de fixture, Debug, reconhecimento de origem ou execução de contexto
nova dentro do serializer. A classificação é total sobre o enum vigente.

JSON: pretty com dois espaços, compacto quando false, sem newline final;
não finitos viram null e `-0.0` mantém sinal/ponto. TOML: somente dict no topo,
omissão de campo none, none em array é erro; arrays pretty não vazios usam
quatro espaços e forma multilinha medida. Dict vazio gera string vazia;
documento não vazio termina com newline. Escalares precedem tables,
preservando ordem relativa dentro de cada classe. YAML: newline final,
null, `.nan`, `.inf`, `-.inf`; quoting, block scalar, indentação e chomping
devem reproduzir o valor integral congelado. JSON/YAML preservam inserção.
As três rotas preservam dados integrais, nesting e escaping de Unicode,
controles, aspas e quebras. Não normalizar strings para declarar paridade;
round-trip é controle adicional, não substituto da string pública exata.

### Validação e origem diagnóstica

As nativas validam a sequência causal fornecida por Args; o dispatch somente
transporta e não escolhe âncora pelo texto do erro. Cada ocorrência de pretty
é convertida a bool em ordem, mesmo quando uma posterior a sobrepõe; a última
válida só vence se nenhuma conversão falhou. Named inválido pré-ligado não
pode desaparecer. Missing value produz `missing argument: value` na chamada;
`value:` nominal produz ``the argument `value` is positional``, com hint
``try removing `value:` `` e âncora no named completo. Cast do valor e de pretty
aponta para value_span. Primeiro excesso positional dá `unexpected argument`;
primeiro named desconhecido dá `unexpected argument: <nome>`, no argumento
completo. TOML de tipo errado dá `expected dictionary, found <tipo>`; falha
de none em array dá `failed to encode value as TOML (unsupported None value)`,
no positional de entrada. Ordem de validação, mensagem completa, hints,
laterais, exit e spans seguem os casos congelados, não uma seleção genérica
pela prioridade conveniente de erros.

Origem detached legítima de um valor transformado por arguments.map não
recebe range inventado. Em contrapartida, perda de origem que vanilla conserva
em spread de Args, sink, With ou filter é falha; não normalizar para detached.
Args sintético `None` usa a política explicitamente definida por seu owner.

### Preservação e fronteiras

Os decoders continuam chamáveis, inclusive path/Bytes; não criar csv.encode,
xml.encode ou read.encode. CBOR mantém seu formato e a dívida já medida de
Symbol/Content; não reutilizar cegamente a classificação dos novos formatos
para "corrigi-lo". Há exceção explícita à preservação: o fallback CBOR chama
repr canônica, logo as correções contratadas de State/Counter/Location/With
e Args causal propagam à string codificada. Isso exige casos independentes
congelados antes do candidato e aprovação humana deste escopo, não autoriza
mudanças em outras classes ou alegação de paridade geral de CBOR.

É inferência, refutável por emissor incompatível, payload sem dados públicos
ou perda de origem, que esta divisão basta sem nova entidade além do carrier
Args. Caso de filho repr fora do escopo não pode ser omitido do oracle para
declarar sucesso total. RED→GREEN, controles e ataques independentes são
obrigatórios depois do gate; Unknown obrigatório bloqueia. Nenhum código
fica autorizado pela simples redação deste L0.
