# P429 — Relatório de fecho (DEBT-63)

> **Data:** 2026-06-23  
> **Executor:** assistente IA (Kimi Code CLI)  
> **Branch:** Tekt  
> **Foco:** Fechar DEBT-63 — remover o cache `resolved_style` de `BibliographyElem` e transportar o style CSL resolvido através de uma tabela lateral no `BibStore`.

---

## Resumo executivo

**DEBT-63 (S-M → FECHADO):** o campo `resolved_style: Option<Arc<IndependentStyle>>` foi removido de `BibliographyElem`. O elemento voltou a ser um struct puro de dados de domínio (`entries`, `path`, `title`, `style`, `locale`), com `PartialEq`/`Eq`/`Hash` derivados sem exclusões artificiais. O style CSL resolvido em eval time é agora transportado numa tabela lateral:

1. `native_bibliography` resolve o style e regista-o em `EvalContext::bibliography_styles`, indexado por `BibliographyElem::style_key()`.
2. No fim do eval, os pares `(key, Arc<IndependentStyle>)` são transferidos para `Module::bib_styles`.
3. O pipeline (`03_infra`) injecta esses styles no `BibStore` do `TagIntrospector` antes do layout.
4. O layout reproduz a mesma chave a partir do primeiro `Content::Bibliography` e lê o style resolvido de `introspector.bib_store.style_for_key(...)`.

**Pipeline COMPLETO verde:** `cargo test --workspace` passa; `crystalline-lint` não reporta violações estruturais novas.

---

## 1. Mudanças de código

### 1.1 `01_core/src/entities/bib_store.rs`

| Linha(s) | Descrição |
|----------|-----------|
| `20` | `pub type BibStyleKey = u64;` — alias para a chave determinística. |
| `37` | Campo `bib_styles: HashMap<BibStyleKey, Arc<IndependentStyle>>` adicionado a `BibStore`. |
| `120-128` | `add_style(key, style)` — regista style resolvido. |
| `130-134` | `style_for_key(key)` — lookup do style resolvido. |
| `169-178` | Testes para `add_style`/`style_for_key`. |

### 1.2 `01_core/src/entities/elements/bibliography.rs`

| Linha(s) | Descrição |
|----------|-----------|
| `33` | `#[derive(Debug, Clone, PartialEq, Hash)]` — struct puro. |
| `39-43` | Campos sem `resolved_style`. |
| `47-55` | `impl Eq for BibliographyElem {}` manual ( `Content` não implementa `Eq`, mas `PartialEq` é equivalência). |
| `58-67` | `BibliographyElem::style_key()` — hash determinístico do elemento. |
| `82-95` | `map_content`/`map_text` reconstrói sem `resolved_style`. |
| `121-267` | Tests actualizados; novo teste `style_key_e_deterministica_e_igual_para_elementos_iguais`. |

### 1.3 `01_core/src/entities/module.rs`

| Linha(s) | Descrição |
|----------|-----------|
| `31-35` | `ModuleInner` ganha campo `bib_styles: HashMap<u64, Arc<IndependentStyle>>`. |
| `70-74` | `bibliography_styles()` — acessor read-only. |
| `76-81` | `set_bibliography_styles(...)` — usado por `eval_with_full_error`. |
| `103-106` | Teste `bibliography_styles_default_vazio`. |

### 1.4 `01_core/src/engine/eval/mod.rs`

| Linha(s) | Descrição |
|----------|-----------|
| `147-153` | Campo `bibliography_styles: HashMap<u64, Arc<IndependentStyle>>` em `EvalContext`. |
| `158` | Inicialização vazia em `EvalContext::new()`. |
| `160-168` | `register_bibliography_style(elem, style)` — regista style pela chave do elemento. |
| `313-316` | Transferência de `ctx.bibliography_styles` para o `Module` no fim do eval. |

### 1.5 `01_core/src/engine/stdlib/structural.rs`

| Linha(s) | Descrição |
|----------|-----------|
| `1110-1115` | `native_bibliography` passa a receber `ctx` (em vez de `_ctx`). |
| `1164-1182` | Constrói `BibliographyElem` sem `resolved_style`; resolve style e regista-o em `ctx` quando `style` é `Some`. |

### 1.6 `01_core/src/engine/introspect.rs`

| Linha(s) | Descrição |
|----------|-----------|
| `35` | Importa `std::sync::Arc` no topo do módulo. |
| `359-373` | `materialize_time` arm `Content::Bibliography` preserva `path`, `style` e `locale` (antes perdia tudo excepto entries/title). |

### 1.7 `01_core/src/engine/layout/mod.rs`

| Linha(s) | Descrição |
|----------|-----------|
| `1514-1520` | `find_first_bibliography_style(content, &introspector)` passa a consultar `introspector.bib_store.style_for_key(e.style_key())`. |
| `1604-1673` | `FirstBibliographyStyle` e `find_first_bibliography_style` actualizados com comentários P429. |

### 1.8 `03_infra/src/pipeline.rs`

| Linha(s) | Descrição |
|----------|-----------|
| `27-31` | Importa `introspect_with_introspector` e `layout_with_introspector`. |
| `116-126` | `compile_to_pdf_bytes_full_error` corre introspect, injecta styles do `Module` no `BibStore`, e invoca `layout_with_introspector`. |

### 1.9 `01_core/src/entities/content.rs`

| Linha(s) | Descrição |
|----------|-----------|
| `1625-1659` | Construtores `bibliography`, `bibliography_with_style`, `bibliography_from_path` deixam de preencher `resolved_style`. |

### 1.10 Testes directos

| Ficheiro | Teste | Descrição |
|----------|-------|-----------|
| `01_core/src/engine/stdlib/mod.rs` | `native_bibliography_style_ieee_aceite` | Verifica que o style IEEE fica em `ctx.bibliography_styles`. |
| `01_core/src/engine/stdlib/mod.rs` | `native_bibliography_path_e_style_preservam` | Verifica que a chave do elemento indexa o style resolvido. |
| `01_core/src/engine/eval/tests.rs` | `p420_bibliography_custom_csl_path_rende_titulo` | Usa helper `p420_layout_module` que replica o pipeline (introspect + inject styles + layout). |
| `01_core/src/engine/eval/tests.rs` | `p420_bibliography_built_in_ieee_continua_funcional` | Idem. |
| `01_core/src/entities/elements/bibliography.rs` | `style_key_e_deterministica_e_igual_para_elementos_iguais` | Garante que elementos iguais partilham a mesma chave. |

---

## 2. Decisão arquitetural

### 2.1 Por que não alterar `Content::Bibliography`?

Adicionar um campo extra à variante `Content::Bibliography` forçaria actualizações em dezenas de *match arms* e quebraria derives/Hash de `Content`. A alternativa escolhida — manter o elemento inalterado e transportar o style numa tabela lateral indexada por uma chave determinística — é localizada e equivalente em comportamento.

### 2.2 Chave determinística

`BibliographyElem` deriva `Hash`. A chave é produzida por `DefaultHasher` sobre o elemento (entradas, path, title, style, locale). Em layout, a mesma chave é reproduzida a partir do primeiro `Content::Bibliography`, garantindo que o style resolvido em eval é recuperado.

### 2.3 Backward compatibility

O PDF gerado para `#bibliography(...)` mantém-se idêntico; o cache é apenas um detalhe de implementação. O fallback de built-ins continua a funcionar: se não houver style resolvido na tabela lateral, `find_first_bibliography_style` devolve o nome do style e `bib_csl::build_cache` resolve-o por nome.

---

## 3. Verificação

### 3.1 Testes

```bash
RUST_MIN_STACK=8388608 cargo test --workspace
```

Resultado: **todos os testes passam** (3160+ em `typst-core`, 489 em `typst-infra`, 24 em `typst-shell`, 21 em CLI, etc.).

Testes directos:

```bash
cargo test -p typst-core native_bibliography
cargo test -p typst-core p420_bibliography
cargo test -p typst-core style_key
cargo test -p typst-core bib_store
```

Todos verdes.

### 3.2 Lint

```bash
crystalline-lint
```

Resultado: zero violações estruturais. Apenas 2 warnings preexistentes de "prompt órfão" (`adr-stub-vs-fallback.md`, `show-regex.md`), fora do escopo do P429.

---

## 4. Actualização em `00_nucleo/diagnosticos/debt/DEBT.md`

- **DEBT-63** reclassificado como **✅ FECHADO (P429)**, com descrição do fluxo eval → Module → pipeline → BibStore → layout.

---

## 5. Notas epistémicas / divergências declaradas

- **Tabela lateral vs campo no elemento:** o style resolvido é, por definição, uma função pura do input (`style`/`path`/`locale`). Mantê-lo numa tabela lateral separada do struct de domínio remove a fragilidade da exclusão manual de `PartialEq`/`Hash`.
- **Múltiplas bibliografias:** o layout ainda só considera o style do primeiro `Content::Bibliography` para o cache CSL. Múltiplas bibliografias com styles diferentes permanecem scope-out (conforme P420).
- **`materialize_time`:** o arm `Content::Bibliography` passou a preservar `path`, `style` e `locale` ao reconstruir o elemento. Isto garante que a chave calculada em layout coincide com a chave registada em eval, mesmo quando o título contém nós dinâmicos.
- **Stack de teste:** recomenda-se continuar a correr com `RUST_MIN_STACK=8388608` devido a testes de recursão artificial.

---

## 6. Próximo passo

Com DEBT-63 fechado, os próximos candidatos prioritários entre os débitos em aberto são:

- **DEBT-50** — show selector Strong/Emph não distingue origem (S; dívida latente).
- **DEBT-17** — caso patológico de fixpoint sem convergência (M; melhor esforço actual).

---

## 7. Checklist de fecho

- [x] `BibliographyElem` puro — sem `resolved_style`
- [x] `BibStore` transporta styles resolvidos indexados por `BibStyleKey`
- [x] Pipeline injecta styles no `TagIntrospector` antes do layout
- [x] Layout consulta `BibStore` via `BibliographyElem::style_key()`
- [x] Testes E2E `p420_bibliography_custom_csl_path_rende_titulo` e `p420_bibliography_built_in_ieee_continua_funcional` verdes
- [x] DEBT-63 reclassificado como **FECHADO** em `DEBT.md`
- [x] `cargo test --workspace` verde (com `RUST_MIN_STACK=8388608`)
- [x] `crystalline-lint` zero violações estruturais
