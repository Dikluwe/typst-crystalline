> **Passo:** 450 — Bibliography `.bib` de disco
> **Data:** 2026-06-24
> **Executor:** assistente IA (Kimi Code CLI)
> **Ficheiro de instrução:** `00_nucleo/materialization/typst-passo-450.md`

---

## Resumo

Implementado parser BibTeX minimal em `01_core/src/rules/eval/bibtex.rs` e
integrado no carregamento de `#bibliography("refs.bib")`. Ficheiros `.bib`
usam agora o parser custom; `.yaml`/`.yml` continuam a usar `hayagriva::io`.
Foi também adicionado `SystemWorld::load_bibliography` em `03_infra/src/world.rs`
como helper L3 para consumidores directos do filesystem.

---

## Implementação

### Modelo

- **Parser custom (`01_core/src/rules/eval/bibtex.rs`)** — parser recursivo
  descendente que cobre o subset BibTeX usado pelo Typst:
  - Tipos: `article`, `book`, `inproceedings`, `misc`, `phdthesis`, `techreport`.
  - Campos obrigatórios: `title`, `author`, `year`.
  - Campos opcionais: `doi`, `url`, `journal`, `booktitle`, `volume`, `pages`.
  - Literais `{}` (com contagem de aninhamento) e `""`.
  - Autores `Last, First and Last2, First2` → string canónica.
- **Integração** — `parse_bibliography` em `rules/eval/bibliography.rs` encaminha
  `.bib` para `parse_bibtex`; YAML mantém `hayagriva`.
- **I/O em L3** — `SystemWorld::load_bibliography` reusa `World::read_bytes` +
  `parse_bibtex`.

### Ficheiros alterados

| Ficheiro | Alteração |
|----------|-----------|
| `01_core/src/rules/eval/bibtex.rs` | Parser BibTeX minimal + 8 testes L1 (novo ficheiro). |
| `01_core/src/rules/eval/mod.rs` | Declaração `pub mod bibtex;`. |
| `01_core/src/rules/eval/bibliography.rs` | `.bib` redireccionado para parser custom; YAML mantido. |
| `01_core/src/rules/eval/tests.rs` | Teste L3 E2E `p450_bibliography_path_bib_popula_entries`. |
| `03_infra/src/world.rs` | `SystemWorld::load_bibliography` + 2 testes L2 de file I/path relativo. |
| `00_nucleo/prompts/infra/bibtex.md` | Spec L0 do parser (novo). |
| `00_nucleo/prompts/infra/system-world.md` | Documentação de `load_bibliography`. |
| `00_nucleo/prompts/rules/stdlib/structural.md` | Actualização da secção `native_bibliography` (parser custom, .yaml/.yml, .json scope-out). |

### Notas

- A arquitectura actual não tem `Project`; usou-se `SystemWorld` como equivalente
  L3, conforme já adoptado noutros pontos do codebase.
- `native_bibliography` já estava registada no stdlib desde P419; este passo
  concentra-se na substituição do parser `.bib` e na exposição do helper L3.
- Os hashes `@prompt-hash` dos ficheiros cujos prompts mudaram foram actualizados
  com `crystalline-lint --fix-hashes`.

---

## Verificações

- `RUST_MIN_STACK=8388608 cargo test --workspace` ✅
- `crystalline-lint .` ✅ (apenas os 2 warnings órfãos pre-existentes)

---

## Código de fecho

- Parser BibTeX custom implementado e testado (8 testes L1).
- `parse_bibliography` usa parser custom para `.bib`.
- `SystemWorld::load_bibliography` implementado e testado (2 testes L2).
- Teste L3 E2E para `#bibliography("refs.bib")`.
- Spec L0 actualizada em 3 prompts.
