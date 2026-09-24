# Prompt L0 — `compiler/stdlib/structural/document` — metadados do documento
Hash do Código: 093b7ea0

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/document.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada destas nativas (marcos P69…P962). Este L0
especifica **a superfície do nó**; o detalhe por marco vive no pai.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla ratificado**: `586e1bd43`; `document` é independente, `asset` pertence a `typst-bundle/src/format.rs`.

---

## Contexto

Nativas que declaram metadados em vez de produzir conteúdo visível.


## P1484 — retificação do placeholder (CONFIRMADO; ADR-0127)

### Medição anterior à decisão

No HEAD `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`,
`01_core/src/compiler/stdlib/structural/document.rs:76-110` recebe
`asset(path, kind:?)` e infere kind da extensão. O vanilla pinado
`586e1bd43`, `lab/typst-original/crates/typst-bundle/src/format.rs:162-240`,
recebe `asset(path, data)` com dados `Bytes` ou `Str` e exige target bundle.
`repr(asset("x", "data"))` sob a feature devolve um asset com `data` no
vanilla, mas o cristalino pré-P1484 devolve apenas `asset("x")`.

### Decisão e divisão explícita

O placeholder P397 não é paridade vanilla e não deve ser publicado pelo
gate `bundle`. P1484 somente o retira da superfície global/`std` em eval;
P1485 substitui a assinatura, o carrier e o target. A função legada pode
permanecer fisicamente neste arquivo até P1485, mas não é critério de MATCH.

## Instrução

| Nativa | Assinatura |
|---|---|
| `document` | `document(title: ?, author: ?, date: ?, keywords: ?)` — metadata pura |
| `asset` | legado P397 `asset(path, kind:?)`, não publicável em P1484; P1485 define `asset(path, data)` |

- `author` e `keywords` aceitam `Str` **ou** `Array<Str>` — normalizados por
  `extract_string_list`.
- `title` aceita content.
- A inferência de `kind` e o `path` nomeado do placeholder são história P397,
  não contrato de linguagem nem aceitação P1484.
- Nomeado desconhecido em `document` é erro.

## Restrições Estruturais

- L1 puro: o placeholder legado não resolve nem lê ficheiros; P1484 não o
  expõe, e a resolução do bundle real pertence a P1485/L3.

## Critérios de Verificação

```
#document(title: [T])                 → title content
#document(author: "a")                → lista de 1
#document(author: ("a", "b"))         → lista de 2
#document(author: 1)                  → Err (tipo)
#document(keywords: "k")              → lista de 1
#document(date: …)                    → data aceite
#document()                           → sem argumentos, ok
#document(zz: 1)                      → Err (nomeado desconhecido)
```

## P1485 — constructors reais `asset` e `document` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No HEAD `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`, P1484
deixou `native_asset(path, kind:?)` fisicamente neste consumer, mas
retirado do scope público. `native_document` ainda recebe só
metadados e `document()` direto devolve content no cristalino.
No vanilla pinado, `typst-bundle/src/format.rs:194-240` define
`asset(path: BundlePath, data: AssetData)`;
`typst-library/src/model/document.rs:147-239` define
`document(path, body, format:?, metadata...)`. Sondas:
`repr(asset("a.txt","hello"))` é
`asset(path: "/a.txt", data: bytes(5))`;
`document()` ou `document(title:[T])` diretos falham
`missing argument: path`; `document("a.pdf", [T])` produz
`document(path: "/a.pdf", body: [T])`. O antigo contrato de
`document`/asset deste prompt e de `00_nucleo/prompts/compiler/stdlib/structural.md`
é histórico e fica explicitamente sucedido nesta superfície.

### Decisão

`asset` exige `path` e `data` posicionais, sem `kind:`.
Path é string não vazia com pelo menos um componente, normalizada
lexicalmente à raiz virtual do bundle; `.`, `..` e barras repetidas
seguem a normalização medida. Escape acima da raiz e backslash são
erros, com hints medidos. `data` aceita `Str` codificada em UTF-8
ou `Bytes` preservados integralmente. Outro tipo, argumento ausente,
terceiro posicional, `path:`, `data:` e `kind:` devolvem os
diagnósticos vanilla medidos em
`00_nucleo/diagnosticos/typst-p1485-bundle-sondas.md`.
O constructor é puro: não lê path, não escreve arquivo e não infere
categoria pela extensão. Produz `Content::Asset` com path normalizado
e bytes, conforme `00_nucleo/prompts/entities/content.md`.

`document` mantém propriedades de metadata para `#set document(...)`,
mas sua chamada direta constrói um arquivo de bundle somente com
path e body obrigatórios, metadata/format explícitos opcionais.
Fora do target bundle, usar o elemento em markup falha
`constructing a document is only supported in the bundle target`
com hints para target e set rule; em bundle, seu body é compilado pelo
target inferido da extensão PDF/SVG/PNG/HTML ou pelo `format`
explícito. Não tratar a forma direta sem path como metadata silenciosa.
O owner de Content transporta path/body/metadata sem I/O; o owner L3
compila os documentos. `#set document` continua a configurar metadata
em targets individuais e não produz um arquivo extra.

A troca do contrato público direto e dos dados de Content exige
confirmação ADR-0127. P1484 é a fatia de gate; P1485 completa o
constructor junto ao target, não o publica antes do exporter.
