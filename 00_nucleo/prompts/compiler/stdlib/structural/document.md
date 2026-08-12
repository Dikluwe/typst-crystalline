# Prompt L0 — `compiler/stdlib/structural/document` — metadados do documento
Hash do Código: bf84a49b

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/document.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada destas nativas (marcos P69…P962). Este L0
especifica **a superfície do nó**; o detalhe por marco vive no pai.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/{document,asset}.rs`. `asset` é extensão cristalina — não existe no vanilla com esta forma.

---

## Contexto

Nativas que declaram metadados em vez de produzir conteúdo visível.

## Instrução

| Nativa | Assinatura |
|---|---|
| `document` | `document(title: ?, author: ?, date: ?, keywords: ?)` — metadata pura |
| `asset` | `asset(path, kind: ?)` — placeholder de recurso externo (extensão cristalina) |

- `author` e `keywords` aceitam `Str` **ou** `Array<Str>` — normalizados por
  `extract_string_list`.
- `title` aceita content.
- `kind` de `asset`, quando ausente, é inferido da extensão do path por
  `infer_asset_kind`; extensão desconhecida → sem kind (não é erro).
- `path` de `asset` aceita-se posicional ou nomeado; ausente é erro.
- Nomeado desconhecido em `document` é erro.

## Restrições Estruturais

- L1 puro: `asset` **não** resolve nem lê o ficheiro — só guarda o path e o kind. A
  resolução é de L3.

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
#asset("x.png")                       → kind inferido
#asset("x.zzz")                       → sem kind, sem erro
#asset(path: "x.png")                 → nomeado aceite
#asset()                              → Err (path em falta)
```
