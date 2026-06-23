# Passo 388 — Materialização (Fase 1): `bibliography` / `cite` sobre hayagriva

**Tipo:** Materialização (L1 + composição; 2 variants `Content` novos; reusa L3 `read_bytes` do P387). **XL — faseado.**
**Data:** 2026-06-21.
**Padrão:** achado já diagnosticado (Lista A — Model, parcial, XL; DEBT-55); mas a Fase 1 abre com **sonda de viabilidade** do runtime de introspecção (ADR-0108) antes de codar.
**Destravado por:** Passo 387 (`loading` — carregar `.bib`/CSL já é possível).
**ADRs relevantes:** ADR-0026 (Content enum fechado — 2 variants novos), ADR-0029 (pureza L1), ADR-0033 (paridade vanilla), ADR-0054 (perfil graded), ADR-0062 (hayagriva autorizada), ADR-0066 (runtime de introspecção — **confirmar número**), ADR-0107 (paridade é com a língua). DEBT-55 (bibliography).

> **Numeração de ADR.** Este passo propõe uma ADR de **perfil graded de bibliografia** (que CSL/recursos entram na Fase 1). Número `ADR-NNNN`: varrer `00_nucleo/adr/`, primeiro livre (ADR-0111 ocupada por loading). Precedente P160A/P376.

---

## 1. Contexto e o porquê do faseamento

`bibliography`/`cite` é a maior dívida Model (XL, DEBT-55). O `loading` (P387) removeu o primeiro bloqueante (carregar o ficheiro de dados). Sobra o peso real, que são **dois** substratos:

1. **Formatação CSL** — transformar entradas (`.bib`/CSL-JSON/hayagriva-YAML) + estilo em `Content` formatado. hayagriva (ADR-0062) faz isto, e é **computação pura** → cabe em L1, no mesmo padrão decode do loading.
2. **Coleta cross-document dos `cite`** — montar a bibliografia exige saber **quais** chaves foram citadas, **em que ordem**, com numeração e back-references. Isto é o problema 2-pass clássico, e depende do **runtime de introspecção** (ADR-0066), o módulo mais fraco do projeto.

O escopo da Fase 1 depende de quanto do substrato 2 já existe. Por isso o passo **não assume**: sonda primeiro (§2), faseia depois (§4).

---

## 2. Sonda de viabilidade (ADR-0108) — primeiro, antes de qualquer código

Medir o estado actual do runtime de introspecção/query no HEAD. Perguntas factuais:

1. O runtime já coleta elementos por tipo ao longo do documento (query de todos os `CiteElem`)? Sim/não, com o sítio no código.
2. Há acesso à **ordem de aparição** dos elementos (necessária para estilos numéricos `[1][2]`)?
3. Há resolução de label/location estável entre passes (back-reference "ver [3]")?

A resposta classifica o que é single-pass vs 2-pass e **fixa o escopo da Fase 1**. Registar a sonda no relatório (não é narrativa — é o gatilho de escopo).

> **Nota retroativa (correcção de deriva):** a sonda de viabilidade foi executada a posteriori.
> O runtime de introspecção confirma:
> - `query_by_kind` (`01_core/src/entities/introspector.rs:46`) devolve `Vec<Location>` em ordem de aparição no walk.
> - `query` (`01_core/src/entities/introspector.rs:132`) suporta `Selector::And`/`Or` (intersecção/ união preservando ordem).
> - `position_of` (`01_core/src/entities/introspector.rs:72`) tem implementação real via `SealedPositions`.
> - `layout_with_introspector` (`01_core/src/rules/layout/mod.rs:1484`) existe como entry point com introspeitor.
> Portanto a Fase 1 proposta (autor-data + bibliografia alfabética) é compatível com o substrato; o faseamento
> não precisou de ajuste. A deriva foi processual (spec antes da sonda), não factual.

> Regra: o que a sonda disser que **não** existe vira **graded** na Fase 1, com DEBT, não bloqueia o passo. O que existir, materializa-se.

---

## 3. Arquitetura por estrato (o estrato força a separação — ADR-0029)

Mesmo padrão do loading, estendido:

- **L1 — parse puro:** `parse_bib(&[u8], format) -> SourceResult<Library>` (bytes → entradas hayagriva). Zero I/O; prova-se com bytes `.bib` literais.
- **L1 — format puro:** `render_citation(entry, style, ctx) -> Content` e `render_bibliography(cited, style, order) -> Content`. CSL render é puro; entra `ctx`/`order` como **dados de entrada** (vêm da coleta, não de I/O).
- **L3 — leitura:** reusa `World::read_bytes` (P387). **Não criar L3 novo.**
- **Variants `Content` (L1):** `Content::Bibliography`, `Content::Cite` (ADR-0026 — enum fechado, adição justificada).
- **Coleta (substrato 2):** via runtime de introspecção/query — **só na medida que a sonda §2 autorizar**. A composição (`native_bibliography`/`native_cite`) é o único ponto que liga coleta + format.

Fronteira (precedente V14 do loading): nenhum tipo hayagriva aparece em contrato L1 público — as funções devolvem `Content`/`Value` cristalino; hayagriva é interno.

---

## 4. Faseamento

### Fase 1 — este passo (o que single-pass permite)

- `parse_bib` (L1) para os formatos que hayagriva já dá: BibLaTeX `.bib` + hayagriva-YAML (+ CSL-JSON se trivial). Graded por formato se algum custar.
- `cite(key)` → `Content::Cite`: rótulo que **não exige ordem global** (estilo **autor-data**, ex.: APA-like) renderiza pleno; estilo numérico fica graded (depende de §2).
- `bibliography(...)` → `Content::Bibliography`: lista das entradas citadas, ordenação **alfabética** (não exige ordem de aparição). Numeração por ordem de citação = graded.
- **Um estilo default** (ou subset pequeno) dos que hayagriva embute. O conjunto de estilos é decisão da ADR §perfil-graded.

### Fase 2 — passo dedicado posterior (pós runtime 2-pass)

Estilos numéricos, numeração por ordem de aparição, back-references ("ver [3]"), `ibid`/`op. cit.`. Materializa quando ADR-0066 (runtime) suportar a coleta ordenada. Registrar como continuação no roadmap da ADR, **não** neste passo.

---

## 5. O que produzir

1. **L1** novo `01_core/src/rules/stdlib/bibliography.rs` (ou módulo `model/bib`): `parse_bib`, `render_citation`, `render_bibliography`. Puro.
2. **Variants** `Content::Bibliography`, `Content::Cite` + arms de `match` afetados.
3. **Stdlib funcs** `bibliography`, `cite` registadas (ABI `native_*`, igual a loading), compondo L3 read + L1 parse/format + coleta (na medida da sonda).
4. **Opções** mínimas vanilla: `bibliography(style:, title:)`, `cite(key, form:, style:)` — subset graded documentado.
5. **Testes:** parse L1 com `.bib` literal; render de citação autor-data com saída de paridade; E2E `#cite` + `#bibliography` num doc single-pass. Erros distintos: ficheiro (L3) vs `.bib` malformado (L1) vs chave inexistente (resolução).
6. **ADR-NNNN** perfil graded de bibliografia (estilos in/out; recursos 2-pass diferidos; critério).
7. **Inventário 148:** `bibliography`/`cite` `parcial`→`implementado⁺` graded (ou mantém `parcial` com ressalva encolhida).
8. **DEBT:** abrir/atualizar DEBT da Fase 2 (recursos 2-pass). DEBT-55 transita para "Fase 1 feita; Fase 2 aberta".

---

## 6. O que NÃO fazer (scope-out)

- **Não** implementar 2-pass aqui se a sonda §2 disser que o runtime não o suporta — vira graded + DEBT.
- **Não** perseguir o universo CSL. Um default + subset; o resto graded por ADR.
- **Não** pôr I/O em L1 (parse recebe bytes).
- **Não** expor tipos hayagriva em contrato L1.
- **Não** perseguir paridade da mecânica do parser CSL (ADR-0107) — paridade é o `Content` de saída.
- **Não** abrir reservas fora da ADR ("sem novas reservas").

---

## 7. Critérios de aceitação

| # | Critério |
|---|----------|
| 1 | Sonda §2 registada no relatório, com o escopo da Fase 1 derivado dela (não assumido). |
| 2 | `cite` autor-data + `bibliography` alfabética materializam pleno; saída de paridade provada em L1 com `.bib` literal. |
| 3 | Zero I/O em L1; leitura só via `read_bytes` reusado. |
| 4 | Nenhum tipo hayagriva em contrato L1 público (fronteira tipo-V14). |
| 5 | Recursos 2-pass (numérico/ordem/back-ref) **graded e em DEBT**, não silenciosos. |
| 6 | ADR-NNNN documenta o perfil graded por estilo/recurso, com critério. |
| 7 | Tests verdes; lint zero; linhagem `@prompt`/hash propagada; protocolo de nucleação cumprido (L0 + ADR antes do código, paragem para aprovação). |

---

## 8. O que pode sair errado

- **Sonda revela runtime sem coleta cross-document.** Então a Fase 1 encolhe para autor-data puro (cada `cite` independente) + bibliografia alfabética das chaves referidas localmente. Ainda é entrega real; o numérico todo vai para Fase 2. Mitigação: o passo já assume esse piso.
- **Estilo CSL default exige recurso 2-pass mesmo em autor-data** (ex.: desambiguação "2020a/2020b" entre autores homónimos precisa do conjunto global). Mitigação: escolher um default que funcione single-pass; desambiguação vira graded.
- **hayagriva acopla parse e format de forma difícil de partir nos estratos.** Mitigação: se o format exigir o `Library` inteiro (não só uma entrada), passar o `Library` como dado para a função L1 de format — continua puro, só muda a granularidade do input.
- **Resolução de chave inexistente.** Erro user-facing claro (chave `@foo` não encontrada), distinto de `.bib` malformado.

---

## 9. Referências

- `typst-falta-migrar-lista-A-passo-386.md` — `bibliography`/`cite` parcial, XL; `typst-passo-387-relatorio.md` — `loading` (destrava; padrão decode/ABI a reusar).
- ADR-0029 (pureza L1, razão da separação parse/leitura), ADR-0062 (hayagriva), ADR-0066 (runtime introspecção — alvo da sonda), ADR-0107 (paridade de saída).
- Vanilla: `lab/typst-original/crates/typst-library/src/model/bibliography.rs`.

---

## 10. Nota sobre o Tekt

`bibliography` é o caso que expõe um padrão Tekt: uma feature XL costuma sê-lo porque assenta em **mais de um substrato**, e o trabalho honesto é **separá-los e fasear pelo mais fraco** — aqui, sondar o runtime de introspecção e gradar o 2-pass, em vez de tratar "bibliography" como um bloco. A sonda-antes-de-fasear (ADR-0108 aplicado a dependência interna, não a crate) é candidato a lição. Registar; não materializar no Tekt aqui.
