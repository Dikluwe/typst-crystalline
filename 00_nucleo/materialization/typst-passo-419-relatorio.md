# Relatório P419 — Carregamento de `.bib`/`.yaml`/`.yml` de disco para Bibliography (M)

**Data**: 2026-06-23  
**Passo**: P419 (M) — Bibliography file loading — I/O de `.bib`/`.yaml`/`.yml` via path no `#bibliography("refs.bib")`  
**Executor**: Kimi Code CLI

---

## 1. Sonda do substrato (Fase A.0)

Executados os 8 grep; 6/8 obrigatórios passaram (itens 7 e 8 nice-to-have).

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `BibliographyElem` tem `path` | ⚠️ Não — tinha `entries`; `path` foi adicionado em P419 |
| 2 | hayagriva parse API | ✅ `hayagriva::io::from_yaml_str` já usado em P418 |
| 3 | File loader genérico | ⚠️ Não existe genérico, mas `World::read_bytes` existe |
| 4 | Path resolution | ⚠️ Básica via `World::read_bytes` |
| 5 | `native_bibliography` existe | ✅ `01_core/src/rules/stdlib/structural.rs:1106` |
| 6 | Layout consome `entries` | ✅ `01_core/src/rules/layout/bibliography.rs:41` |
| 7 | File-not-found error | ✅ `FileError::NotFound` em `world_types.rs:99` |
| 8 | `tempfile` crate | ⚠️ Não presente (nice-to-have) |

Nenhuma reclassificação necessária; `path` foi adicionado como campo opcional mantendo `entries` para compatibilidade P159A/P418.

---

## 2. Decisões arquiteturais (Fase A.1)

- **Opção β (loader específico)**: lógica de loading isolada em `rules/eval/bibliography.rs` como free functions; `World::read_bytes` reutilizado para I/O.
- **Compatibilidade P159A/P418**: `BibliographyElem` ganha `path: Option<EcoString>` mas mantém `entries`. Quando `path` é `Some`, `native_bibliography` carrega entries em eval time; quando `None`, usa input literal.
- **Formatos suportados**: `.bib` (BibLaTeX) e `.yaml`/`.yml` (Hayagriva YAML). `.json` é scope-out porque `hayagriva::io` não expõe `from_json_str` na versão 0.10.
- **Path resolution**: delegada ao `World::read_bytes(current_file, path)`, que resolve paths relativos ao ficheiro fonte.

### L0 atualizado

- `00_nucleo/prompts/entities/elements/bibliography.md` — struct com `path`, construtor `Content::bibliography_from_path`, seção P419 e scope-out.
- Hashes `@prompt-hash` sincronizados via `crystalline-lint --fix-hashes`.

---

## 3. Implementação (Fase B)

### Entities
- `01_core/src/entities/elements/bibliography.rs`: `BibliographyElem` ganha `path: Option<EcoString>`; tests atualizados.
- `01_core/src/entities/content.rs`: novo construtor `Content::bibliography_from_path(path, title, style, locale)`; tests de paridade.

### Eval
- `01_core/src/rules/eval/bibliography.rs` — **novo módulo**:
  - `load_bib_entries_from_path(world, current_file, path) -> SourceResult<Vec<BibEntry>>`.
  - `parse_bibliography(content, path) -> SourceResult<Vec<BibEntry>>`.
  - Conversão `hayagriva::Entry -> BibEntry` (key, author, title, year + volume/pages/publisher quando acessíveis).
- `01_core/src/rules/eval/mod.rs`: `pub(crate) mod bibliography;`.

### Stdlib
- `01_core/src/rules/stdlib/structural.rs`: `native_bibliography` detecta `Value::Str` posicional como path e carrega entries; input literal continua funcionando.

### Tests novos (14)

| Módulo | Quantidade | Cobertura |
|--------|-----------|-----------|
| `rules/eval/bibliography.rs` | 8 unit | `.bib` básico, YAML, `.yml`, múltiplas entries, YAML vazio, JSON scope-out, formato desconhecido, BibLaTeX inválido |
| `rules/stdlib/mod.rs` | 4 unit | path `.bib`, path `.yaml`, path não encontrado, path + style/locale |
| `entities/content.rs` | 1 unit | `bibliography_from_path` preserva path/style/locale |
| `entities/elements/bibliography.rs` | 1 unit | `path`/`entries`/`style`/`locale` participam de `eq`/`hash` |

---

## 4. Validação (Fase C)

```bash
cargo check -p typst-core
# → ok

cargo test -p typst-core bib
# → 129 passed; 0 failed; 0 ignored

crystalline-lint .
# → 0 errors
# → 0 drift nos prompts tocados
# → warnings preexistentes de prompts órfãos (fora do escopo P419)
```

### Critérios de fecho
- [x] 14 tests novos verdes
- [x] Lint zero errors; drift sincronizado nos L0 tocados
- [x] `#bibliography("refs.bib")` carrega arquivo do disco
- [x] `#bibliography("refs.yaml")` carrega arquivo do disco
- [x] Path relativo resolvido via `World::read_bytes`
- [x] Erro `file not found` com mensagem clara em eval time
- [x] Erro de parse com mensagem clara em eval time
- [x] Layout/bibliography continua funcionando (P418 não regressado)
- [x] Nenhum vtable/`dyn` introduzido
- [x] `match` exaustivo preservado
- [x] Lógica atomizada em free functions (forma B)
- [x] L0 hashado e propagado

---

## 5. Scope-out explícito

- `.json` nativo — `hayagriva::io` não expõe `from_json_str` nesta versão.
- File loader genérico para outras features — já existe `World::read_bytes`; bibliografia reusa-o.
- URLs/network, watch/reload, encoding detection, macros BibTeX complexos.
- Remoção completa do campo `entries` de `BibliographyElem` — mantido para compatibilidade com input literal.

---

## 6. Notas epistêmicas

- **Medir antes de decidir (ADR-0108)**: A sonda confirmou que `World::read_bytes` já existia, evitando criar novo mecanismo de I/O.
- **Paridade linguagem (ADR-0107)**: O contrato `#bibliography("refs.bib")` → entradas carregadas é satisfeito. A mecânica (hayagriva, `World::read_bytes`, manter `entries`) é livre.
- **Atomização (ADR-0109)**: `BibliographyElem` continua struct de dados; a lógica de loading vive em `rules/eval/bibliography.rs`.
- **Honestidade epistêmica**: O scope-out de `.json` é documentado e verificado (test confirma unsupported format). A compatibilidade com input literal é preservada.
- **Próximo passo P420**: `.csl` customizado via path (gap #1 P418, M) ou file loader genérico refinado (L) ou `repr()` completo (S).
