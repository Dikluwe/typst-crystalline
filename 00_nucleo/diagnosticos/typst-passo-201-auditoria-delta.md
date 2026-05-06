# Auditoria delta P156A → P200

**Escopo**: passos P157 a P200 inclusive sub-passos
(P156B-L produzidos no mesmo dia que P156A; tratados como
parte do delta).
**Data de produção**: 2026-05-05.
**Geração**: Claude Code executando o passo P201.
**Propósito**: leitura rápida do que mudou desde P156A.
**Complementa**: `00_nucleo/historiograma-passos.md`
(referência exaustiva).
**Snapshot referência**: estado pré-M8 declarado pela spec
P201 ("snapshot 2026-05-05").

---

## §1 — Estado consolidado pré-M8 (C6)

Validação empírica face ao snapshot declarado em P201 (§4 C6).
Cada item etiquetado **CONFIRMADO** / **DIVERGÊNCIA** /
**NÃO APLICÁVEL** com evidência.

| # | Item declarado pela spec | Etiqueta | Evidência |
|---|---|---|---|
| 1 | 7 ADRs ACEITES no ciclo M5/M6/M7 | **DIVERGÊNCIA (marginal)** | Verificadas 6 ACEITES estritas: 0066, 0068, 0069, 0070, 0071, 0072. Plus 2 EM VIGOR (0064, 0065) — interpretação ampla rende 8. Spec valor 7 cai entre ambas as interpretações. Possível que ADR-0067 (PROPOSTO mas referida no consolidado P190 como ACEITE) seja a 7ª; ficheiro tem `Status: PROPOSTO`. |
| 2 | 1.802 tests verdes | **DIVERGÊNCIA** | `cargo test --workspace` em 2026-05-05 reporta **1823 testes verdes** (1563 + 215 + 24 + 21). Diferença +21. Causa provável: Δ tests P200B (+5), P195+ (+4-5 cada). Snapshot pode estar desactualizado de 1-2 dias. |
| 3 | 0 violations | **CONFIRMADO** | `crystalline-lint .` retorna `✓ No violations found`. |
| 4 | `CounterStateLegacy` eliminado | **CONFIRMADO** | `grep -rn "CounterStateLegacy"` em 01_core/02_shell/03_infra/04_wiring retorna apenas comentários doc, headers `@updated`, e o ficheiro `entities/counter_state_legacy.rs` (35 linhas, body vazio, preservado para histórico arquivístico do prompt-hash). Struct `pub struct CounterStateLegacy` não existe no código activo. |
| 5 | Walk fn 7 parâmetros | **CONFIRMADO** | `01_core/src/rules/introspect.rs:714` declara: `pub(crate) fn walk(content, locator, tags, intr, auto_label_counter, lang, label_from_parent)`. Sete parâmetros. Sig matches ADR-0070 §validação empírica (`5 params → 7 params (+intr +auto_label_counter +lang -state)`). |
| 6 | 2 loops fixpoint, MAX=5 | **CONFIRMADO** | `01_core/src/rules/introspect/fixpoint.rs:33`: `pub const MAX_FIXPOINT_ITERATIONS: usize = 5`. `01_core/src/rules/layout/mod.rs:1506`: `const MAX_ITERATIONS: usize = 5`. Dois loops paralelos, ambos cap 5. |
| 7 | 33 aplicações de diagnóstico-primeiro | **DIVERGÊNCIA (marginal interpretativa)** | Reconstrução por inventário identifica ~35 aplicações formais com sufixo *A ou diagnóstico declarado. Spec 33 cai dentro do range plausível (depende se P156L e P159B são contadas como diag formal ou refino-via-ADR-0065). Sem divergência semântica relevante. |
| 8 | `comemo` em uso | **CONFIRMADO** | `Cargo.toml` workspace declara `comemo = "0.4"`. Uso intermédio (não comemo nativo) para hash-based convergence; per ADR-0072 e P192B, comemo nativo é trabalho de M8. |
| 9 | `Introspector` 20 métodos | **CONFIRMADO** | Trait `Introspector` em `01_core/src/entities/introspector.rs` declara 20 métodos: `query_by_kind`, `query_by_label`, `query_first`, `query_unique`, `position_of`, `figure_number_for_label`, `query_metadata`, `formatted_counter`, `state_value`, `state_final_value`, `query`, `formatted_counter_at`, `bib_entry_for_key`, `bib_number_for_key`, `is_numbering_active`, `figure_number_at_index`, `is_numbering_active_at`, `flat_counter_at`, `resolved_label_for`, `headings_for_toc`. |
| 10 | `TagIntrospector` 9 sub-stores | **CONFIRMADO** | Struct `pub struct TagIntrospector` (entities/introspector.rs:173) tem 9 fields: `labels`, `counters`, `kind_index`, `figure_label_numbers`, `metadata`, `state`, `bib_store`, `resolved_labels`, `headings_for_toc`. Comentário interno declara `// positions: HashMap<Location, Position> — adiado para M5/M9.`. |
| 11 | `Layouter` 19 fields, sem `counter` | **DIVERGÊNCIA** | Struct `Layouter<M, S>` em `01_core/src/rules/layout/mod.rs:69` tem **22 fields**, não 19. `counter` field eliminado **CONFIRMADO** (P190I). Lista actual: `metrics`, `sizer`, `font_size_pt`, `style`, `chain`, `page_config`, `pages`, `current_items`, `cursor_x`, `cursor_y`, `line_start_x`, `current_line`, `introspector`, `figure_progress`, `is_height_unconstrained`, `cell_available_h`, `cell_origin_x`, `cell_origin_y`, `cell_origin_w`, `locator`, `current_location`, `runtime`. Diferença +3 face ao snapshot 19. Causa provável: snapshot conta apenas fields nomeados pelo desenvolvimento M3-location-aware (P185C +2 fields: `locator`, `current_location`) e P190C/D (+1 field `runtime`). Total =19 no momento do snapshot inicial; P190+ adicionou 3 fields. Snapshot precisa actualização: **22 fields, sem `counter`**. |

**Resumo C6**:
- 7 itens **CONFIRMADO** (3, 4, 5, 6, 8, 9, 10).
- 4 itens **DIVERGÊNCIA**: #1 (ADRs 6 ou 8 — não 7);
  #2 (testes 1823 — não 1802); #7 (~35 — não 33; marginal);
  #11 (Layouter 22 fields — não 19; **divergência
  estrutural relevante**).

Apenas a divergência #11 é estruturalmente relevante para M8.
As outras três são derivas de snapshot de 1-2 dias.

---

## §2 — Lacunas residuais (C8)

> **CORRECÇÃO RETROACTIVA aplicada por P203B (2026-05-05)**
>
> A tabela abaixo atribui "Position", "Position-related"
> e "Counter at locations" às lacunas #1/#1b/#2. Esta
> atribuição está **empíricamente errada**. As lacunas
> reais (per `m1-lacunas-captura.md` + P200 consolidado
> §7) são:
>
> - **#1** — `figure.kind` literal em tags vs colapsado
>   em counter (default `"image"`).
> - **#1b** — gate `is_counted` no caminho de população
>   do counter Figure.
> - **#2** — reservada / vazia.
>
> Ambas #1 e #1b foram fechadas estruturalmente por
> P190H/P191C; formalizadas em P203B (test
> `p203b_lacuna_1_e_1b_fecho_formal_4_casos`).
>
> **Position** continua um concern real (stub
> `position_of() -> Option<()>`) mas **não é catalogado
> como lacuna**. ADR-0066 cobre o adiamento até M8.
>
> O conteúdo abaixo é preservado para histórico mas não é
> canónico. Para estado actual, ver
> `00_nucleo/diagnosticos/snapshot-2026-05-05.md` §7.

Estado actual face à lista declarada (preservado para
histórico; **não canónico**):

| Lacuna | Tópico | Estado | Último passo a tocar | Bloqueia M8? |
|--------|--------|--------|----------------------|--------------|
| #1 | Position | **residual** | P165 (TagIntrospector criada com `position_of` stub `Option<()>`) | provavelmente sim — M8 traz Position concreta |
| #1b | Position-related | **residual** | (relacionada com #1; não materializada) | depende de #1 |
| #2 | Counter at locations | **residual** | P185B (`flat_counter_at`, `is_numbering_active_at` adicionados). API existe; uso location-aware activado em P187B+. **Lacuna remanescente** é provavelmente snapshot vs runtime granularity. | parcial — M8 deve refinar |
| #3 | headings_for_toc | **fechada P200B** | P200B materializa sub-store dedicado, payload, walk arm, helper, trait method (19→20), consumer outline migrado. | não bloqueia — fechada |
| #4 | numbering_active StyleChain-like | **fechada P182F** | série P182A-F | não bloqueia |
| #5 | CounterRegistry hierárquico | **fechada P170** | P170 (apply_hierarchical, format) | não bloqueia |
| #6 | Bibliography full-stack | **fechada P181I** | série P181A-J | não bloqueia |
| #7 | Outline locatable | **fechada P178** | P178 (ElementKind/Payload::Outline) | não bloqueia |

**Apenas lacunas #1 e #1b permanecem residuais**. Lacuna #2
tem progresso parcial. As lacunas que existiam à data de
P156A foram parcialmente resolvidas e enriquecidas (a
sequência de pipeline Introspection runtime descobriu
lacunas adicionais #4-#7 em P167, todas fechadas).

---

## §3 — Padrões novos do ciclo (subset C2 marcado novo)

Padrões que **emergiram desde P156A** (não detectados em
P156A, ou detectados em forma diferente):

### 3.1 ADR-0069 com 5 variantes operacionais

Pattern post-recursion tag emission para resolver
"extract_payload puro sem state". 5 variantes em produção
(ver historiograma §2.2). Família de 4 helpers paralelos
(`compute_labelled`, `compute_heading_auto_toc`,
`compute_figure`, `compute_heading_for_toc`).

P200B introduz **trabalho híbrido sem nova variante** —
combinatória de variantes existentes cobre novos casos sem
expansão da família. Sinal de maturidade do pattern.

### 3.2 ADR-0070 — Eliminação write paralelo

8 aplicações sequenciais (P190B-I) eliminam
`CounterStateLegacy` (16 fields → 0). Forma canónica:
substituir mutação legacy pela leitura via Introspector path;
fallback eliminado quando legacy é dead code factual
confirmado.

### 3.3 ADR-0071 — Walk pipeline com Introspector

Resolveu barreira arquitectural P190F (walk fn sem acesso a
Introspector). `walk` ganha `&mut TagIntrospector`;
`from_tags::from_tags` eliminado (-969 LOC).

### 3.4 Layouter-runtime → struct dedicada

2 aplicações (P190C `LayouterRuntimeState`; P190D extensão).
Pattern para extrair estado runtime do Layouter para struct
própria.

### 3.5 Auditoria sobre estado existente (P192A)

Variante reflexiva do diagnóstico-primeiro: o passo `A`
revela que o marco está estruturalmente fechado sem ADR
explícita. Δ tests = 0; resultado: passo `B` declarativo.

### 3.6 Substitution-with-fallback (M4-residual)

7 aplicações sequenciais (P168 estabelece; P181G, P182D,
P184D, P187B, P188B, P194B). Forma canónica
`introspector.<primitive>().or_else(|| legacy.<primitive>())`.
P188B introduz subvariante "Introspector dormente; fallback
funcional permanente" (legacy é caminho activo permanente
quando setter location-aware não é exercitado em produção).

### 3.7 Auditor #1 — fixture vs restrição

Documentado em P185D, P191B, P200B. Quando restrição (ex:
ADR-0068 sincronização Locator) seria violada por adaptação
simples de teste, prefere ajustar o fixture (filtro
`hash != 0`) em vez de violar restrição na produção.

### 3.8 Pattern auditoria-2-eixos para gates substanciais

P183C, P183D bloqueiam em `.B` com zero código tocado
quando 2 eixos independentes precisam de ser desbloqueados
antes de prosseguir. Bloqueio formal documentado motiva
ramificação arquitectural (P184/P185/P186).

### 3.9 Pause-resume documentado em série

P190F **pause** (barreira arquitectural) → ramo paralelo
P191A-C → **retomar** P190G. Reorganização de ordem
cronológica documentada; tracker pause-resume em P191A.
Forma evoluída de "reformulação de série" (P156A §3.1.2).

---

## §4 — ADRs novas no ciclo

| ADR | Tópico | Proposta | Aceite/EM VIGOR | Estado actual |
|-----|--------|----------|-----------------|---------------|
| **0061** | Layout Fase X roadmap | P156B (PROPOSTO) | mantém PROPOSTO | PROPOSTO (decisão humana 2026-04-25) |
| **0062** | hayagriva (parsing bibliography) | reserva P156B; ficheiro em "ADR-0062-create" admin | PROPOSTO | PROPOSTO |
| **0063** | (não existe) | — | — | slot conceptual reservado column flow |
| **0064** | Smart→Option/default | P156K | EM VIGOR imediato | EM VIGOR |
| **0065** | Inventariar primeiro (6 critérios) | P156K | EM VIGOR imediato | EM VIGOR |
| **0066** | Introspection runtime adiada | P160A (PROPOSTO) | ACEITE P192B | ACEITE (com nota "intermediário até M8") |
| **0067** | Attribute grammar scoping | (pré-janela?) | PROPOSTO | PROPOSTO |
| **0068** | Layouter location-aware (M3) | P185A (PROPOSTO) | ACEITE P185E | ACEITE |
| **0069** | Post-recursion tag emission | P195A (PROPOSTO) | ACEITE P195E | ACEITE |
| **0070** | Eliminação CounterStateLegacy | série P190 | ACEITE P190I | ACEITE |
| **0071** | Walk pipeline com Introspector | P191A (PROPOSTO) | ACEITE P191C | ACEITE |
| **0072** | M7 fixpoint runtime fechado | P192B (NOVA ACEITE) | ACEITE P192B | ACEITE |

**Total**: 11 ADRs novas (0061, 0062, 0064-0072).
**ADRs ACEITES estritas no ciclo M5/M6/M7**: 6 (0066, 0068,
0069, 0070, 0071, 0072).
**ADRs EM VIGOR no ciclo**: 2 (0064, 0065).
**ADRs PROPOSTAS pendentes**: 3 (0061, 0062, 0067).

---

## §5 — Marcos fechados no ciclo

| Marco | Fechou em | Contexto |
|-------|-----------|----------|
| **M1** (Walk emite tags + payload) | P163 | Tags determinísticas; bracketing válido |
| **M2** (is_locatable função pública) | P164 | Match exaustivo 56 variants; invariante |
| **M3** (Introspector + sub-stores + from_tags) | P165 | trait + TagIntrospector + 4 stores + from_tags |
| **M3 location-aware** | P185E | ADR-0068 ACEITE; current_location field; resolve eixo 1 |
| **M4** (expose Introspector) | P166 | introspect_with_introspector público |
| **M4-residual** (consumers C1/C2/C3 migrados) | P188B (funcionalmente) | C1 P187B; C2 P188B (Introspector dormente); C3 P184D |
| **M5** (incremental — 1 arm migrado) | P189B | Outline arm puro; 6 excepções E1-E6 declaradas |
| **M5 universal** | **P200B** (2026-05-04) | 0 excepções activas, 0 residuos, 0 pré-requisitos. Desbloqueia M6. |
| **M6** (eliminação CounterStateLegacy) | P190I (2026-05-05) | ADR-0070 ACEITE; struct 16→0 fields; F1 fechado |
| **M7** (loop fixpoint runtime) | P192B (2026-05-05) | ADR-0072 NOVA ACEITE; estruturalmente fechado por P174→P179→M9→P190→P191 |
| **M9** (Introspection vanilla 11/11) | P182F | query, query_by_label, query_first/unique, figure_number_for_label, query_metadata, formatted_counter, state_value/final, query Selector, formatted_counter_at, bib_entry/number_for_key, is_numbering_active |
| **F1** (Layouter slim — counter elim) | P190I | "F1 fechado" |
| **F2** (não documentado como fechado nesta janela) | — | — |
| **F3** (Layouter -1 field; 18 ortogonais pendentes) | parcial em P190I | "F3 parcialmente fechado" |

**Total**: 9 marcos fechados no ciclo (M1, M2, M3, M3
location-aware, M4, M4-residual, M5 incremental + universal,
M6, M7, M9, F1; F3 parcial).

---

## §6 — Métricas comparadas P156A → P200

| Métrica | P156A (2026-04-25) | P200 (2026-05-05) | Δ |
|---------|:--:|:--:|:--:|
| Tests workspace | 1145 | **1823** | +678 |
| ADRs total | 60 | **70** (-1 slot 0063) | +10 |
| ADRs ACEITES (cumulativo) | (não declarado) | inclui 6 novas | — |
| Violations | 0 | **0** | = |
| Aplicações diagnóstico-primeiro | 7 | ~35 | +28 |
| DEBTs novos abertos no ciclo | (P156A registou ~13 abertos finais) | 1 (DEBT-56) | +1 |
| ADRs revogadas no ciclo | — | 0 | — |
| Marcos fechados | até Fase 1 Model | M1-M7, M9, F1 | +9 |
| LOC líquido série P190 | n/a | -990 | — |
| LOC eliminados from_tags | n/a | -969 | — |
| Walk fn parâmetros | 5 (P162) | **7** | +2 |
| TagIntrospector sub-stores | 4 (P165 inicial) | **9** | +5 |
| Introspector trait métodos | (P165 inicial ~10) | **20** | ~+10 |
| Layouter fields | ~19 (P185 snapshot) | **22** (sem `counter`) | +3 (líq) |

---

## §7 — Divergências detectadas face ao snapshot 2026-05-05

Quatro divergências detectadas (todas em §1 C6); apenas
uma é estruturalmente relevante:

### 7.1 Divergência estrutural — Layouter 19 → 22 fields

**Snapshot**: 19 fields, sem `counter`.
**Empírico**: 22 fields, sem `counter`.
**Causa**: snapshot foi capturado num estado intermédio
(provavelmente pré-P185C ou pré-P190C). P185C adicionou
`locator` + `current_location` (+2). P190C/D adicionaram
`runtime` (+1). Total +3.
**Impacto em M8**: M8 deve trabalhar com 22 fields como
baseline; quaisquer estimativas de "redução" devem
recalibrar.

### 7.2 Divergência menor — testes 1802 → 1823

**Snapshot**: 1.802 verdes.
**Empírico**: 1823 verdes.
**Causa**: 21 tests adicionais em séries terminais
(P195+, P200B). Snapshot com 1-2 dias de atraso.

### 7.3 Divergência marginal — 7 ADRs ACEITES

**Snapshot**: 7 ADRs ACEITES no ciclo M5/M6/M7.
**Empírico**: 6 strictamente ACEITE; 8 incluindo EM VIGOR.
**Causa**: definição de "ACEITE" ambígua entre status formal
e "decidida em definitivo".

### 7.4 Divergência marginal — 33 vs ~35 aplicações diag-1º

**Snapshot**: 33 aplicações consecutivas.
**Empírico**: ~35 (depende se P156L e P159B contam).
**Causa**: ambiguidade entre diagnóstico formal e
refino-via-ADR-0065.

---

## §8 — Notas de auditor

### 8.1 ADR-0067 referência sem proposição na janela

Consolidado P190 §1 lista ADR-0067 como "ACEITE" entre as 5
ADRs do ciclo M5/M6 mas o ficheiro
`typst-adr-0067-attribute-grammar-scoping.md` declara
`Status: PROPOSTO`. Possíveis interpretações:
- ADR-0067 foi proposta em sub-passo administrativo não
  consultado (anterior a P185).
- Consolidado P190 incorre em erro de status.

Auditor pode investigar para clarificar.

### 8.2 ADR-0063 slot vazio — convenção implícita

Não há ficheiro `typst-adr-0063-*.md`. P160A documenta a
descoberta ("número 0017 ocupado; usar 0066"). 0063 ficou
reservado conceptualmente para column flow. **Convenção
implícita** que não é explicitamente registada como tal —
auditor pode querer formalizar.

### 8.3 Datação inconsistente em P157-P160

Cabeçalhos de P157, P157A-C, P158, P158A-C, P159, P159A-G,
P160 não declaram data explícita. Inferíveis (entre P156L
2026-04-26 e P161 2026-04-30). **Auditor pode enriquecer**
estes relatórios com datas explícitas.

### 8.4 Datação anómala P190-P192 vs P195-P200

Datas em ficheiros relatório:
- P190G/H/I: 2026-05-05
- P191A-C: 2026-05-05
- P192A-B: 2026-05-05
- P193-P200: 2026-05-03/04

Mas **a sequência §9 P189 (P193-P200) depende de M6
fechado** (estructuralmente, via ADR-0070 ACEITE em P190I
2026-05-05). Possível interpretação: a sequência §9 P189 foi
preparada estructuralmente em paralelo com M5 universal
fechar (P200B 2026-05-04), e M6 fechou logo a seguir
(P190I 2026-05-05). Datas de relatório reflectem ordem de
publicação, não dependência semântica.

Auditor pode considerar enriquecer relatórios com "ordem de
dependência semântica" vs "ordem de publicação".

### 8.5 Relatórios individuais ausentes

- P190B-H: sem relatórios individuais (consolidado P190
  absorve).
- P183B: sem relatório individual (P183C absorve com
  documentação retroactiva como "primitiva inadequada").
- P160B: sem relatório (descartado por P161).
- P181J: relatório existe mas curto (consolidado P181 absorve).

---

## §9 — Sugestão de próximos passos (não-vinculativa)

Per spec P201 §6: P201 não decide. **As notas que se seguem
são observações, não plano**.

### 9.1 Caminho A — P202A diagnóstico-primeiro de M8

Caminho declarado pela spec P201 §6 como default. Aplicaria
o pattern diagnóstico-primeiro consolidado (~35 aplicações)
ao próximo marco M8 (substituir hash-based convergence por
comemo nativo).

### 9.2 Caminho B — Reconciliação do snapshot 2026-05-05

Se a divergência §7.1 for considerada bloqueante, P202 pode
ser administrativo (actualizar snapshot 2026-05-05 com
22 fields Layouter; testes 1823; 6 ACEITES vs 7).

### 9.3 Caminho C — Atender lacuna ADR-0067

Se §8.1 for considerada relevante, P202 pode ser
administrativo (clarificar status ADR-0067; promover ou
manter PROPOSTO formalmente).

### 9.4 Caminho D — Atender lacunas residuais #1, #1b, #2

Lacunas #1 e #1b (Position) são as únicas residuais
não-progredidas no ciclo. Se forem dependência crítica de
M8, P202 deve abordá-las antes de mecanismo M8.

**P201 reporta. Não decide.**

---

## §10 — Referências

- `00_nucleo/historiograma-passos.md` (referência completa
  P0-P200).
- `00_nucleo/historiograma-passos.P156A.md` (versão anterior).
- `00_nucleo/materialization/typst-passo-201.md` (spec).
- `00_nucleo/materialization/typst-passo-201-relatorio.md`
  (relatório).
- ADRs do ciclo: 0061, 0062, 0064-0072.
- Consolidados: P181, P182, P184-P200.
