# P429 — Fecho de débito: DEBT-63 (cache de style em `BibliographyElem`)

> **Data:** 2026-06-23  
> **Executor:** assistente IA (Kimi Code CLI)  
> **Branch:** Tekt  
> **Foco:** Fechar DEBT-63 — mover `resolved_style` para fora do struct de domínio ou fixar invariante em teste.

---

## Contexto

**DEBT-63** foi aberto no P420 (Forma B). O campo `resolved_style: Option<Arc<IndependentStyle>>` em `BibliographyElem` é um cache de estado computado dentro de um struct de dados de domínio. A exclusão de `PartialEq`/`Hash` já foi aplicada (P420), mas o risco permanece: se um passo futuro preencher o campo por outro caminho, dois `BibliographyElem` iguais nas entradas mas com styles resolvidos diferentes serão tratados como iguais, escondendo a divergência na deduplicação/memoização do `Introspector`.

**Critério de fecho original:** mover o style resolvido para fora do struct de domínio (`Introspector`/`BibStore` ou parâmetro de layout), repondo `BibliographyElem` puro; OU provar e fixar em teste a invariante "`resolved_style` é função pura das entradas" para que a exclusão permaneça sólida.

---

## ADR-0108 — Medir antes de decidir

### FASE A.0 — Sonda

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `BibliographyElem` tem `resolved_style`? | Sim, `Option<Arc<IndependentStyle>>` | ✅ |
| `PartialEq`/`Hash` excluem `resolved_style`? | Sim, implementações manuais (P420) | ✅ |
| Quem preenche `resolved_style`? | Pipeline `eval → ElementPayload → BibStore → Introspector → layout` | ✅ |
| Existe outro caminho que preencha o campo? | Não encontrado em grep por `resolved_style` | ✅ |
| `Introspector` deduplica `BibliographyElem`? | Via `headings_for_toc`/`figure_label_numbers` — não deduplica Bibliography diretamente | ✅ |
| Mover para `BibStore` é viável? | `BibStore` já existe em `CounterState` (P159A+); campo `resolved_style` pode viver lá | ✅ |
| Bloqueadores? | Nenhum externo; toca apenas `entities/elements/bibliography.rs` + `rules/introspect.rs` + `layout/mod.rs` | ✅ |

**Reclassificação:** S-M (refactor de campo entre structs; ~1-2h).

---

## ADR-0107 — Paridade linguagem

O contrato é **comportamental**, não estrutural: o PDF gerado para `#bibliography(...)` deve ser idêntico antes e depois do refactor. O cache de style é um detalhe de implementação; o utilizador não observa a mudança.

---

## ADR-0109 — Atomização forma B

### Opção α — Mover para `BibStore` (recomendada)

1. `entities/elements/bibliography.rs` — remover `resolved_style` de `BibliographyElem`; struct volta a ser puro
2. `entities/counter_state.rs` — adicionar `bib_styles: HashMap<Label, Arc<IndependentStyle>>` em `BibStore` (ou `CounterState` se BibStore não for struct separado)
3. `rules/introspect.rs` — preencher `bib_styles` no walk de `Bibliography` em vez de `resolved_style`
4. `rules/layout/mod.rs` — consumir `bib_styles` via `state` (ou `intr`) no arm `Bibliography` em vez de ler do elemento

### Opção β — Fixar invariante em teste

Adicionar teste que constrói dois `BibliographyElem` com mesmas entradas mas `resolved_style` diferente e asserta que `PartialEq` os trata como iguais. **Rejeitada** — o teste já existe (P420), mas a invariante é frágil: um futuro passo que adicione um segundo caminho de preenchimento quebra a invariante silenciosamente.

---

## Decisão arquitetural

| Decisão | Opção escolhida | Justificativa |
|---------|----------------|---------------|
| Destino do cache | `BibStore` (ou sub-store em `CounterState`) | Separação de responsabilidades: dados de domínio puros vs estado computado |
| `BibliographyElem` pós-refactor | Sem `resolved_style`; derive automático `PartialEq`/`Hash` | Elimina o smell; volta a ser struct de dados puro |
| Lookup no layout | Via `state.bib_styles.get(&label)` ou `intr.bib_style_for(label)` | Paridade com `headings_for_toc` e `figure_label_numbers` |
| Backward compat | Preservada — output PDF idêntico | Refactor interno sem observável externo |

---

## Scope-out explícito

- CSL styling completo (author-date, MLA, APA) — continua scope-out; DEBT-55 permanece parcial
- Hayagriva crate real — ADR-0062 continua PROPOSTO sem ficheiro
- `BibliographyElem` com campos adicionais (style, title, etc.) — fora do escopo; só remove `resolved_style`

---

## Critério de fecho

- [ ] `BibliographyElem` sem `resolved_style`; derive `PartialEq`/`Hash` automático restaurado
- [ ] `BibStore` (ou `CounterState`) ganha `bib_styles: HashMap<Label, Arc<IndependentStyle>>`
- [ ] Walk de `Bibliography` em `introspect.rs` popula `bib_styles` em vez de `resolved_style`
- [ ] Layout arm `Bibliography` consome `bib_styles` via lookup
- [ ] Teste P420 preservado (ou adaptado para validar a nova arquitetura)
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations
- [ ] DEBT-63 reclassificado como **FECHADO** em `DEBT.md`

---

## Próximo passo

Com P429 fechado, continuamos com:
- **DEBT-50** (show selector latente — S; depende de #set text migrar de bake-in, não acionável hoje)
- **DEBT-57** (specs L0 ausentes — M; trabalho documental, não funcional)
- **DEBT-42** (`get_unchecked` no scanner — bloqueado por infra de benchmark)
- **DEBT-43** (linter whitelist type-level — trabalho em `crystalline-lint` separado)

Indique se quer ajustar o escopo do P429 ou pivotar para outro débito.
