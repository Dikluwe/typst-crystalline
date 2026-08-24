# Prompt L0 — `compiler/stdlib/structural/bibliography` — bibliografia e citação
Hash do Código: 2e904617

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/bibliography.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada destas nativas (marcos P69…P962). Este L0
especifica **a superfície do nó**; o detalhe por marco vive no pai.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/{bibliography,cite}.rs`. Co-mudança: P159a/c/d/e/g movem `extract_bib_entries`, `native_bibliography` e `native_cite` sempre juntos.

---

## Contexto

`bibliography` e `cite` são um par com acoplamento semântico no próprio vanilla: a
citação só resolve contra as entradas registadas pela bibliografia.

**Fronteira de camada**: este nó **não lê ficheiros**. As entradas chegam já parseadas
como `Value::Array<Value::Dict>` — o parsing de BibTeX vive em `compiler/eval/bibtex.rs`
e o carregamento em L3. É o que mantém o nó em L1.

## Instrução

| Nativa | Assinatura |
|---|---|
| `bibliography` | `bibliography(entries: array, title: ?, style: ?, locale: ?)` → `Content::Bibliography` |
| `cite` | `cite(key, supplement: ?, form: ?, style: ?)` → `Content::Cite` |

### Extractores (privados ao nó)

- `extract_bib_entries` — coage `Array<Dict>` para `Vec<BibEntry>` conforme P159A §5 +
  P159D/E/G §5.1.
- `extract_citation_form` (P159C) e `extract_citation_style` (P468) — parsing de
  `Value::Str` com **matching estrito e sensível a maiúsculas**; `auto`/`none` → `None`.

## Restrições Estruturais

- L1 puro: zero I/O, zero acesso a `World` para ficheiros `.bib`.
- O matching de forma/estilo é estrito por decisão registada — não aceitar variantes
  case-insensitive nem abreviaturas.

## Critérios de Verificação

```
#bibliography(entries)                  → Content::Bibliography
#cite("key")                            → Content::Cite
#cite("k", form: "prose")               → form reconhecida
#cite("k", form: "Prose")               → Err (matching sensível a maiúsculas)
#cite("k", form: auto)                  → None
#bibliography(entries, style: "ieee")   → estilo reconhecido
```

---

## P1034 — `bibliography.style` tem default `"ieee"`

**Achado 2 do P1031**, medido: `A @netwok B @netwok C @other D @netwok` +
`#bibliography("works.bib")` sem `style` dava, no vanilla, `A [1] B [1] C [2] D [1]` com
entradas IEEE; no cristalino, `B` saía como `ibid.` e `D` como `op. cit.`, com formato CSL
diferente. Com `style: "ieee"` explícito os dois já coincidiam — logo o caminho CSL estava
correcto e faltava só o default.

`native_bibliography` passa a distinguir três casos, como o `figure.numbering` (§P1034 de
`layout_figure.md`):

| `args.named["style"]` | resultado |
|---|---|
| `Some(Value::Str(s))` | `s` |
| `Some(Value::None)` — `style: none` | `None` (desactiva explicitamente) |
| ausente | **`"ieee"`** |

Verificação (2026-08-13, binários do dono do passo): o texto extraído do PDF passa a ser
idêntico ao do vanilla no caso acima, incluindo as duas entradas formatadas.

## P1141 — entrada bibliográfica `path | str` (gate ADR-0127)

### Medição antes da decisão

O cristalino aceita `Value::Str` como primeiro posicional e delega a
`compiler/eval/bibliography::load_bib_entries_from_path`, que chama
`World::read_bytes(current_file, path)`. A infraestrutura vanilla de paths
mede `PathOrStr` como união pública para funções que leem ficheiros; a sonda
cross-file P1141 demonstra que um path pré-resolvido não pode ganhar a base do
consumer.

### Decisão

O primeiro posicional de `bibliography` aceita `Value::Path | Value::Str`, além
das entradas in-memory já suportadas. A leitura usa o helper único P1141:
`RootedPath` segue direto a `World::read_path`; string resolve no caller e então
lê pelo mesmo método. Parsing, registro de entries, `title`, `style`, `locale`
e semântica de `cite` não mudam. O parâmetro textual `style` não é promovido a
path neste lote: sua resolução builtin/ficheiro é contrato separado e não foi
medida como `PathOrStr`.

O cast público muda; implementar somente após confirmação do gate.
