# Relatório consolidado — Série P189

**Período**: 2026-05-04 (P189A diagnóstico + P189B implementação)
**Magnitude agregada**: S
**Estado**: ✅ Série fechada (A ✅ B ✅) — **M5 incremental**
(1 arm migrado + 6 excepções declaradas)
**ADR vinculada**: nenhuma
**DEBT**: M5-residual cobre 6 excepções (Cenário B)

---

## §1 Resumo executivo

Migração **incremental** de M5 walk puro materializada:

- **1 arm migrado**: `Content::Outline` — mutação
  `state.has_outline = true` removida (`introspect.rs:Content::Outline`).
  Consumer `mod.rs:layout_with_introspector` lê
  `intr.kind_index.contains_key(&ElementKind::Outline)`.

- **6 excepções declaradas** (E1–E6) com documentação
  obrigatória em 4 pontos cada:
  - **E1**: `Equation` walk arm — Reserva 1
    (`SetEquationNumbering` ausente).
  - **E2**: `Heading` walk arm (4 mutações) — Reserva 2
    alargada (sub-store `resolved_labels` ausente; cadeia
    Heading→Labelled→resolved_labels).
  - **E3**: `Figure` walk arm (2 mutações) — chained com
    E2.
  - **E4**: `Labelled` walk arm (2 mutações) — Reserva 2
    alargada.
  - **E5**: `SetHeadingNumbering` walk arm — chained com
    E2.
  - **E6**: `CounterUpdate` walk arm — chained com E2
    (decisão `.A.3` empírica em P189B).

**M5 universal não fecha em P189**. Análise empírica
(P189A) revelou cadeia de dependências que bloqueia
migração granular sem 4 pré-requisitos:
1. Sub-store `resolved_labels` (passo dedicado).
2. C4 migration (consumer Ref-arm em Layouter).
3. Sub-store `headings_for_toc` (lacuna #3 fechar).
4. `Content::SetEquationNumbering` materialização.

P189 é **início incremental de M5**, não fim. Δ tests
cumulativo: **+7** (1808 → 1815) com **zero regressões**.

---

## §2 Sub-passos materializados

| Passo | Magnitude planeada | Magnitude real | Δ tests | L0s tocados |
|-------|---------------------|-----------------|---------|-------------|
| **P189A** | S (diagnóstico) | S | 0 | nenhum |
| **P189B** | S (agregado) | S | **+7** | `rules/introspect.md` |
| **Total** | — | — | **+7** | 1 L0 produção |

P189B agregou em sub-passo único:
- `.A` auditoria + decisão CounterUpdate (excepção δ).
- `.B` migração Outline arm + consumer.
- `.C` documentação CounterUpdate como E6.
- `.D` 6 comentários inline + L0 secção + 7 tests sentinela.
- `.E` relatório consolidado (este ficheiro).
- `.F` verificação estrutural (13/13).

---

## §3 Decisões arquiteturais

### 7 cláusulas P189A fechadas + 1 decisão P189B

| # | Cláusula | Decisão | Sub-passo |
|---|----------|---------|-----------|
| 1 | Lista de arms não-puros | 6 arms / 11+ mutações empiricamente listadas | P189A `.A` |
| 2 | Estratégia por arm | 1 migrável (Outline = α); 5 excepções (δ); CounterUpdate decisão deferida a P189B | P189A `.B` |
| 3 | Sub-store por field | `resolved_labels` e `headings_for_toc` ausentes (bloqueadores) | P189A `.D` |
| 4 | Backward compat | Opção A — `from_tags` popula sub-store; legacy fica campo morto | P189A `.E` |
| 5 | `Content::Styled` | puro por design; sem trabalho | P189A `.F` |
| 6 | Excepções declaradas | 5 excepções (potencialmente 6 com CounterUpdate) | P189A `.G` + P189B `.A` |
| 7 | Critério fecho | Opção 3 — grep zero matches em arms migráveis + 6 excepções enumeradas | P189A `.H` |
| **+1** (P189B `.A`) | CounterUpdate decisão empírica | **Excepção δ (E6)** — chained com Reserva 2 alargada | P189B `.A` |

### Sem ADR — replicação de padrão

Substitution-with-fallback é padrão estabelecido (P184D,
P187B, P188B). Decisões registadas em P189A §2 + P189B `.A`.

---

## §4 Achados não-triviais durante execução

### P189A §11.1 — Reserva 2 é alargada, não isolada a C4

`state.resolved_labels` é populado por **2 arms** (Heading
auto-toc + Labelled explicit), e lido por Layouter Ref-arm.
Migrar qualquer elo exige sub-store para resolved_labels —
**que não existe**. Reserva 2 originalmente caracterizada
como "C4 não migrado". Análise correcta: Reserva 2 é
"sub-store resolved_labels ausente + C4 não migrado".

### P189A §11.2 — Cadeia de dependências bloqueia migração granular

Migrar `Heading.hierarchical` individualmente é tentador,
mas Labelled lê durante walk → popula resolved_labels.
Sem sub-store resolved_labels, não há para onde Labelled
escrever. Logo Heading.hierarchical fica gated por Reserva 2
alargada também.

Idem Figure: `figure_numbers` é populated por from_tags
P184B ✅, mas walk arm Labelled lê figure_numbers durante
walk.

**Resultado**: a migração tem que acontecer em ordem
inversa à mutação. Primeiro abrir sub-store
`resolved_labels` → migrar Labelled → migrar Heading +
Figure → restaurar (em última instância) walk universal
puro.

### P189A §11.3 — Outline é único migrável trivial

Eixo 1: snapshot final ✅ (`mod.rs:1470` lê o flag uma vez
antes do fixpoint TOC; não precisa walk-during).
Eixo 2: `kind_index[Outline]` populado por from_tags P178 ✅.

Migração: walk arm remove `state.has_outline = true`;
consumer lê
`intr.kind_index.contains_key(&ElementKind::Outline)`.

### P189A §11.6 — P189 é "início incremental de M5", não fim

Spec original sugeria "M5 fecha com excepções declaradas".
Análise §1 mostrou que **maioritariamente é excepções**.

Honestidade: P189 fecha 1 arm + documenta 6 excepções.
M5 universal precisa de 4 passos pré-requisito antes de
fechar.

### P189B `.A.3` — CounterUpdate decisão δ

Análise empírica:
- **Eixo 1**: `Labelled` arm pode ler counter mutado via
  CounterUpdate durante walk (cadeia chained com E2).
- **Eixo 2**: CounterRegistry pode ser populado se
  `from_tags` cobrir `ElementPayload::CounterUpdate`, mas
  ainda assim o consumer Labelled lê durante walk.

**Decisão**: Excepção δ (E6) — chained com Reserva 2
alargada. Mesma cadeia que Heading/Figure.

### P189B `.D` — 4 pontos de documentação obrigatória materializados

Per padrão P188B (replicação literal):
1. ✅ 6 comentários inline em `introspect.rs` (E1–E6).
2. ✅ Secção "Walk puro M5 incremental" em L0
   `rules/introspect.md` com tabela de 6 excepções.
3. ✅ 7 tests sentinela em `mod p189b_walk_puro_m5`
   (1 paridade Outline + 6 excepções).
4. ✅ §5 deste relatório consolidado dedicada a Excepções
   M5.

---

## §5 Excepções M5 (secção dedicada)

Walk arms que **continuam a mutar state directamente** após
P189B, com justificação literal e plano de fechamento:

### E1 — Equation walk arm

**Localização**: `introspect.rs:Content::Equation` (linha
~387-399 após edits P189B).

**Mutação**: `state.step_flat("equation")` quando
`block && state.is_numbering_active("equation")`.

**Razão**: `Content::SetEquationNumbering` ausente em
cristalino (Reserva 1; P186A §11.2 confirmado em P188 §5).
Sem ele, gate em `from_tags` arm Equation (P186E) nunca
dispara → counter introspector vazio → P188B fallback
legacy é caminho funcional permanente.

**Pré-requisito**: materialização de
`Content::SetEquationNumbering` (passo dedicado fora série
P186-P189).

**Plano de fechamento**:
1. Materializar `Content::SetEquationNumbering`.
2. State `numbering_active:equation` populado via tag
   StateUpdate.
3. Gate em P186E dispara → counter introspector populado.
4. P188B Introspector path activa em produção; fallback
   legacy fica redundante.
5. **E1 fecha**: walk arm Equation pode remover mutação
   directa.
6. Janela compat M6 abre para Equation.

### E2 — Heading walk arm (4 mutações)

**Localização**: `introspect.rs:Content::Heading` (linha
~348-376).

**Mutações**:
- `state.step_hierarchical("heading", *level)` (linha 349).
- `state.auto_label_counter += 1` (linha 352).
- `state.resolved_labels.insert(auto_label, ...)` (linha 367).
- `state.headings_for_toc.push(...)` (linha 372).

**Razão**: `Labelled` arm lê counter durante walk para
popular `resolved_labels`. Sub-store `resolved_labels` **não
existe**. Cadeia Heading→Labelled→resolved_labels bloqueia
migração granular.

**Pré-requisitos**:
1. Sub-store `resolved_labels` (passo dedicado).
2. C4 migration (consumer Ref-arm em Layouter).
3. Sub-store `headings_for_toc` (lacuna #3, E4 chained).

**Plano de fechamento**: ver §"Ordem inversa à mutação" em
§9 abaixo.

### E3 — Figure walk arm (2 mutações)

**Localização**: `introspect.rs:Content::Figure` (linha
~407-426).

**Mutações**:
- `state.local_figure_counters.entry(...).or_insert(0)`.
- `state.figure_numbers.entry(...).or_default().push(...)`.

**Razão**: walk arm Labelled lê `state.figure_numbers`
durante walk para popular `figure_label_numbers`.
Sub-stores existem (P184B figure_numbers + P168
figure_label_numbers) mas chained com E2.

**Pré-requisito**: E2 fecha primeiro.

### E4 — Labelled walk arm (2 mutações)

**Localização**: `introspect.rs:Content::Labelled` (linha
~430-475).

**Mutações**:
- `state.figure_label_numbers.insert(label, n)`.
- `state.resolved_labels.insert(label, text)`.

**Razão**: consumer Ref-arm em Layouter lê durante layout.

**Pré-requisitos**: idêntico a E2 (sub-store + C4).

### E5 — SetHeadingNumbering walk arm

**Localização**: `introspect.rs:Content::SetHeadingNumbering`
(linha ~485-487).

**Mutação**:
`state.numbering_active.insert("heading".to_string(), *active)`.

**Razão**: `Heading` arm lê
`state.is_numbering_active("heading")` durante walk para
resolver auto-toc text. Tag StateUpdate emitida
paralelamente via P182C; `StateRegistry` populado
independentemente — legacy mutation é write paralelo
durante janela compat.

**Pré-requisito**: E2 fecha primeiro; legacy mutation
removida orgânicamente.

### E6 — CounterUpdate walk arm

**Localização**: `introspect.rs:Content::CounterUpdate`
(linha ~497-510).

**Mutações**:
- `state.step_hierarchical("heading", 1)` ou
  `state.step_flat(key)` ou `state.update_flat(key, val)`.

**Razão**: Labelled arm pode ler counter mutado via
CounterUpdate durante walk. Chained com E2 (Reserva 2
alargada).

**Pré-requisito**: E2 fecha primeiro.

### Padrão de cadeia

5 das 6 excepções (E2–E6) fecham em sequência após
desbloquear sub-store `resolved_labels` + C4 migration. E1
é independente (Reserva 1 distinta).

---

## §6 Estado final M9 e M5

### M9 (counter-feature) — inalterado: 11/11

P189 não introduz feature M9 nova.

### M5 — **incremental**: 1 arm migrado + 6 excepções

P189B materializou:
- Outline arm puro (1 mutação removida).
- Consumer `layout_with_introspector` adapta a `kind_index`.

Restantes 6 walk arms permanecem com mutações directas
(E1–E6) — documentadas com plano de fechamento.

### M5/M4 (read-sites) — inalterado: 8/12

P189 não migra read-sites — migra walk arms.

### Trait `Introspector` — 18 métodos (inalterado)

### Layouter — sem mudança em fields

`mod.rs:layout_with_introspector` adapta consumer Outline
(linha 1470). Field `state.has_outline` em
`CounterStateLegacy` fica morto; cleanup em M6.

---

## §7 Estado final lacunas

- **Lacuna #3** (`headings_for_toc` sub-store): activa
  ainda. Bloqueia E4 (parcialmente) e E2 (parcialmente).
- Outras lacunas: inalteradas.

---

## §8 Pendências cumulativas + DEBT M5-residual

### DEBT M5-residual (Cenário B)

**Sem DEBT formal aberto**. Nota preventiva neste relatório:

> Após P189, DEBT M5-residual cobre 6 excepções declaradas
> (E1–E6). Quando pré-requisitos fecharem, excepções fecham
> incrementalmente:
> - Sub-store `resolved_labels` + C4 migration → E2, E3,
>   E4, E5, E6 fecham em cadeia.
> - `Content::SetEquationNumbering` materialização → E1
>   fecha.
> - Sub-store `headings_for_toc` → E2 fecha residual.
>
> Walk torna-se universalmente puro após esses 3 trabalhos
> sequenciais. Segue M6 (eliminação `CounterStateLegacy`).
>
> P183F formal pode ser dispensado (DEBT M4-residual fica
> vazio em prática per P188; M5-residual incremental).

---

## §9 Próximos passos sugeridos

### Trabalho identificado para fechar M5 universalmente

**Ordem inversa à mutação** (camada baixa → alta):

1. **Abrir sub-store `resolved_labels`** (passo dedicado) —
   estrutura paralela a `BibStore` (P181) ou simples
   `HashMap<Label, String>`. Exposição via trait
   `Introspector::resolved_label_for(&Label)`.

2. **Migrar consumer Ref-arm em Layouter** (C4 migration;
   P183E retomado ou novo) — substituir
   `self.counter.resolved_labels.get(label)` por
   `self.introspector.resolved_label_for(label)`.

3. **Migrar walk arm `Labelled`** — emitir Tag em vez de
   mutar directamente. **E2 + E4 fecham**.

4. **Migrar walk arm `Heading`** — auto-toc generation
   migra para from_tags. **E2 fecha residual**.

5. **Migrar walk arm `Figure`** — `figure_numbers` mutation
   removida; consumer (Labelled) já lê do sub-store. **E3
   fecha**.

6. **Migrar walk arms `SetHeadingNumbering` + `CounterUpdate`**
   — write paralelo legacy removido. **E5 + E6 fecham**.

7. **Materializar `Content::SetEquationNumbering`** — passo
   independente. **E1 fecha**.

Após esses 7 passos sequenciais, walk torna-se
universalmente puro; M5 fecha universalmente; segue M6
(eliminação `CounterStateLegacy`).

### Independente de M5

- **`Content::SetEquationNumbering` materialização** —
  passo 7 acima; pode acontecer em paralelo com 1-6.
- **Trabalhos M9 slot 11** — independente.
- **Sub-store `headings_for_toc`** (lacuna #3) — relacionado
  com E2 mas pode ser passo separado.

---

## §10 Conclusão

P189 fechou em 2 sub-passos (A diagnóstico + B
implementação agregada) com magnitude correctamente
estimada (S em ambos). **Não fecha M5 universalmente** —
fecha 1 arm migrável (Outline) e documenta 6 excepções
honestamente.

Achados centrais:
- **M5 universal exige 7 passos sequenciais pré-requisito
  antes de fechar**. Análise empírica confirmou cadeia de
  dependências (Heading→Labelled→resolved_labels) que
  bloqueia migração granular.
- **Outline é único arm independente da cadeia** — migrado
  trivialmente.
- **6 excepções declaradas com 4 pontos de documentação
  cada** — replicação de padrão P188B.
- **DEBT M5-residual em Cenário B** — sem formal; notas
  preventivas.

P189 termina como **início incremental de M5**, não fim.
Próximos 7 passos identificados e sequenciados em §9.

**60 passos executados** após P189B. Padrão
diagnóstico-primeiro mantido — 14/14 acertaram a magnitude
planeada ±1 nível
(P131A/132A/140A/148/154A/181A/182A/183A/184A/185A/186A/
187A/188A/189A).

Próximo passo sugerido: abrir sub-store `resolved_labels`
(passo 1 da sequência §9). Magnitude esperada S — replicação
de padrão BibStore (P181B).
