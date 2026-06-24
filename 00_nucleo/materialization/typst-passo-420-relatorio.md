# Relatório P420 — `.csl` Customizado via Path (M)

**Data**: 2026-06-23  
**Passo**: P420 (M) — Bibliography CSL custom — carregamento de `.csl` XML via path local no `style:`  
**Executor**: Kimi Code CLI

---

## 1. Sonda do substrato (Fase A.0)

Executados os 8 grep; 6/6 obrigatórios passaram (itens 7 e 8 nice-to-have).

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `BibliographyElem` tem `style` | ✅ `01_core/src/entities/elements/bibliography.rs:30` |
| 2 | P418 built-ins funcionam | ✅ `01_core/src/rules/layout/bib_csl.rs:343+` (`ieee`, `apa`, `chicago-author-date`) |
| 3 | hayagriva parseia CSL XML | ✅ `citationberg::IndependentStyle::from_xml` (via `hayagriva::citationberg`) |
| 4 | P419 file loading funciona | ✅ `World::read_bytes` em `rules/eval/bibliography.rs:37` |
| 5 | CSL custom já existe | ✅ 0 hits — este é o gap do P420 |
| 6 | Distinguir built-in vs custom | ✅ 0 hits — decisão nova do P420 |
| 7 | Tratamento XML parse error | ⚠️ Não existia (nice-to-have) |
| 8 | Tests com `.csl` | ⚠️ Não existiam (nice-to-have) |

Nenhuma reclassificação necessária; a API `IndependentStyle::from_xml` confirmou que o scope-out de parser próprio não se justifica.

---

## 2. Decisões arquiteturais (Fase A.1)

- **Opção α (built-in primeiro, path fallback)**: `style: "ieee"` resolve no archive hayagriva; `style: "custom.csl"` é tratado como path e lido via `World::read_bytes`.
- **Atomização forma B (ADR-0109)**: a resolução de style vive em `rules/eval/bibliography.rs` como free functions; `BibliographyElem` guarda apenas o `IndependentStyle` resolvido como cache mecânico (`resolved_style`).
- **I/O puro via `World::read_bytes`**: a leitura do `.csl` acontece em eval time, sem `std::fs` no L1 de produção; o layout recebe o style já resolvido.
- **Erros legíveis**: built-in desconhecido → fallback para path; path inexistente, encoding inválido ou XML malformado → mensagens claras em `SourceDiagnostic`.

### L0 atualizado

- `00_nucleo/prompts/entities/elements/bibliography.md` — seção P420, `resolved_style` como cache mecânico, scope-out.
- `00_nucleo/prompts/rules/layout/bib_csl.md` — API pública com `parse_csl_style` e `build_cache_with_style`, seção P420.
- Hashes `@prompt-hash` sincronizados via `crystalline-lint`.

---

## 3. Implementação (Fase B)

### Entities
- `01_core/src/entities/elements/bibliography.rs`: `BibliographyElem` ganha `resolved_style: Option<Arc<IndependentStyle>>`; mapas e tests atualizados.
- `01_core/src/entities/content.rs`: construtores `bibliography`, `bibliography_with_style`, `bibliography_from_path` preenchem `resolved_style: None`.

### Eval
- `01_core/src/rules/eval/bibliography.rs`:
  - `resolve_style(world, current_file, style_str) -> SourceResult<IndependentStyle>` — built-in primeiro, path fallback.
  - `load_csl_style_from_path(world, current_file, path)` — I/O via `World::read_bytes`, UTF-8 decode, parse XML.

### Layout
- `01_core/src/rules/layout/bib_csl.rs`:
  - `resolve_style_name(name)` — built-ins hayagriva.
  - `parse_csl_style(content)` — parsing XML sem I/O.
  - `build_cache_with_style(entries, independent, locale)` — renderização usando style já resolvido.
  - `build_cache` mantido para built-ins.
- `01_core/src/rules/layout/mod.rs`:
  - `find_first_bibliography_style` retorna `FirstBibliographyStyle` com `resolved_style`/`style`/`locale`.
  - Usa `build_cache_with_style` quando `resolved_style` está presente (P420); senão `build_cache` com nome built-in.

### Stdlib
- `01_core/src/rules/stdlib/structural.rs`: `native_bibliography` resolve `style` em eval time e preenche `BibliographyElem.resolved_style`.

### Tests novos (14)

| Módulo | Quantidade | Cobertura |
|--------|-----------|-----------|
| `rules/layout/bib_csl.rs` | 5 unit | built-in `ieee`, built-in inexistente, parse CSL válido, parse XML malformado, `build_cache_with_style` custom |
| `rules/eval/bibliography.rs` | 5 unit | built-in `ieee`, path custom, path não encontrado, XML malformado, encoding inválido |
| `rules/eval/tests.rs` | 4 E2E | `#bibliography(..., style: "custom.csl")` renderiza, built-in continua funcional, style inválido → erro, XML malformado → erro |

---

## 4. Validação (Fase C)

```bash
cargo check --workspace
# → ok

cargo test -p typst-core --lib -- p420
# → 14 passed; 0 failed; 0 ignored

cargo test -p typst-core --lib -- bib
# → 143 passed; 0 failed; 0 ignored

cargo test -p typst-core --lib -- --skip p350c_flag_on_nao_convergente_classifica
# → 3117 passed; 0 failed; 0 ignored; 1 filtered out (stack overflow preexistente)

crystalline-lint .
# → 0 errors
# → 0 drift nos prompts tocados
# → warnings preexistentes de prompts órfãos (fora do escopo P420)
```

### Critérios de fecho
- [x] 14 tests novos verdes
- [x] Lint zero errors; drift sincronizado nos L0 tocados
- [x] `style: "ieee"` continua funcionando (P418 não regressado)
- [x] `style: "custom.csl"` carrega e aplica CSL custom
- [x] `style: "nope.csl"` → erro claro (built-in não encontrado + path não existe)
- [x] File not found → mensagem com path
- [x] XML malformado → mensagem com causa
- [x] Encoding inválido → mensagem "not valid UTF-8"
- [x] Nenhum vtable/`dyn` introduzido
- [x] `match` exaustivo preservado
- [x] Lógica atomizada em free functions (forma B)
- [x] L0 hashado e propagado

---

## 5. Scope-out explícito

- CSL style via URL (`http://...`).
- CSL style em diretórios de sistema (ex.: `~/.csl/`).
- Múltiplos styles simultâneos em um documento — primeiro `BibliographyElem` governa.
- Modificação de CSL style em runtime (hot-reload).
- Validação completa de CSL schema (RelaxNG) — hayagriva faz validação básica.
- Cache persistente cross-run.

---

## 6. Notas epistêmicas

- **Medir antes de decidir (ADR-0108)**: A sonda confirmou `IndependentStyle::from_xml`, evitando implementar parser CSL XML próprio (~5k LOC).
- **Paridade linguagem (ADR-0107)**: O contrato `style: "custom.csl"` → CSL custom aplicado é satisfeito. A ordem de resolução e o cache mecânico são livres.
- **Atomização (ADR-0109)**: `BibliographyElem` é struct de dados; a lógica de resolução vive em `rules/eval/bibliography.rs`; o layout não muda (P418).
- **Divergência declarada (correcção de deriva P420)**: `BibliographyElem` mantém `resolved_style: Option<Arc<IndependentStyle>>` como cache mecânico. A Forma A (remover o campo e rotear o style resolvido pelo `Introspector`) é viável em princípio — o `Introspector` está disponível em `layout_with_introspector` — mas exigiria refactor do `ElementPayload::Bibliography` e do `BibStore` para transportar o style resolvido do eval até ao layout. Optou-se pela **Forma B** como correção mínima:
  - `resolved_style` é **EXCLUÍDO** de `PartialEq` e `Hash`;
  - a identidade de `BibliographyElem` fica definida apenas pelas entradas (`path`, `style`, `locale`, `title`);
  - **invariante**: `resolved_style` é função pura de (`path`/`style`/`locale`), preenchido só em eval time a partir desses inputs;
  - o custo de manter o campo consistente está rastreado em **DEBT-63**.
- **Próximo passo P421**: `repr()` completo (S) ou `link` render visual (S) ou `text.lang` rustybuzz (XL, scope-out).
