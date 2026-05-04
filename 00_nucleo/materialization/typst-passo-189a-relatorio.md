# Relatório P189A — Diagnóstico Walk Puro M5

**Data**: 2026-05-04
**Magnitude**: S (diagnóstico puro)
**Pré-condição**: M4-residual fechado funcionalmente (P188B);
tests workspace 1.808; zero violations.

---

## §1 Escopo

P189A é o passo de diagnóstico-primeiro que precede a
migração walk puro M5. Replica registo de
P181A/P182A/P183A/P184A/P185A/P186A/P187A/P188A.

P189A produziu 2 artefactos documentais e zero código:

- `00_nucleo/diagnosticos/diagnostico-walk-puro-passo-189a.md` (8 secções).
- `00_nucleo/materialization/typst-passo-189a-relatorio.md` (este, 14 secções).

Sem ADR. Sem DEBT formal.

---

## §2 Inputs verificados empiricamente (10+ greps)

| # | Input | Resultado |
|---|-------|-----------|
| 1 | Mutações em walk fn (linhas <600) | 11+ mutações em 6 arms (Heading, Equation, Figure, Labelled, SetHeadingNumbering, CounterUpdate, Outline) |
| 2 | `SetEquationNumbering` | zero hits em produção (Reserva 1 confirmada) |
| 3 | `Bibliography`/`Cite` walk | puros (P181H/P162) |
| 4 | `Content::Styled` walk | puro por design — `walk(body, ...)` sem mutação |
| 5 | `Outline.has_outline` consumer | `mod.rs:1423` em `layout_with_introspector` |
| 6 | `from_tags` arms cobrindo cada field | inventário completo |
| 7 | `resolved_labels` sub-store | **AUSENTE** — Reserva 2 alargada confirmada |
| 8 | `headings_for_toc` sub-store | ausente — lacuna #3 não fechada |
| 9 | `figure_numbers` sub-store | populado por from_tags P184B ✅ |
| 10 | Cadeia de dependências Heading → Labelled → resolved_labels | bloqueia migração universal |

Crítico descoberto (per P186C aprendizado spec vs realidade):

**A Reserva 2 é mais alargada do que C4 isolado**. Cadeia
empírica: Heading muta hierarchical → Labelled lê durante
walk → popula resolved_labels → Layouter Ref-arm consume.
Migrar QUALQUER elo exige sub-store para `resolved_labels`,
que **não existe**.

Resultado: **apenas 1 arm é trivialmente migrável**
(`Outline.has_outline`). Restantes 5+ arms ficam excepcionados.

---

## §3 Decisões cláusulas 1–7 (resumo)

| # | Cláusula | Decisão |
|---|----------|---------|
| 1 | Lista de arms não-puros | **6 arms, 11+ mutações** identificadas empiricamente |
| 2 | Estratégia por arm | 1 migrável (Outline = α); 5 excepções (δ); CounterUpdate a auditar |
| 3 | Sub-store por field | resolved_labels e headings_for_toc **ausentes** (bloqueadores) |
| 4 | Backward compat | Opção A modificada — `from_tags` popula sub-store; legacy permanece campo morto |
| 5 | `Content::Styled` | puro por design; sem trabalho |
| 6 | Excepções declaradas | **5 excepções** (Equation, Heading.*, Figure+Labelled, headings_for_toc, SetHeadingNumbering) |
| 7 | Critério fecho | Opção 3 — grep zero matches em arms migráveis + tests E2E paridade + 5 excepções enumeradas |

---

## §4 Plano de sub-passos B-D (sem condicionais)

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Migrar Outline arm + consumer + L0 + test paridade | S |
| `.C` | Documentar 5 excepções (4 pontos cada) + tests sentinela | S |
| `.D` | Relatório consolidado P189 + actualizar nota DEBT M5-residual | S |

Total agregado: ~50 LOC produção + ~150 LOC tests + ~200
LOC documentação ≈ S agregada.

---

## §5 Magnitude agregada

**P189 série = S puro** (3 sub-passos triviais a S).

Honestidade chave: **P189 não fecha M5 universalmente**.
Fecha 1 arm migrável (Outline) e documenta 5 excepções.
Trabalho real de M5 é maioritariamente bloqueado por
ausência de:
- Sub-store `resolved_labels` (bloqueia 5+ mutações).
- `Content::SetEquationNumbering` (bloqueia Equation arm).
- C4 migration (consumer de resolved_labels).
- Sub-store `headings_for_toc` (lacuna #3).

Sem estes pré-trabalhos, M5 não pode ser universal.
P189 honra esta realidade.

---

## §6 Dependências de outros passos

### §6.1 — Pré-requisitos (cumpridos)

- M4-residual fechado funcionalmente (P188B).
- Trait `Introspector` 18 métodos (P185B).
- Sub-stores P181 (BibStore), P184B (figure_numbers),
  P186E (equation counter dormente), P178
  (kind_index[Outline]).

### §6.2 — Bloqueadores M5 universal

- **`Content::SetEquationNumbering`** materialização —
  fora série actual; abre Excepção 1.
- **Sub-store `resolved_labels`** + **C4 migration** —
  passo dedicado ou P183E retomado; abre Excepção 2 +
  3 + 5.
- **Sub-store `headings_for_toc`** (lacuna #3) — passo
  dedicado; abre Excepção 4.

### §6.3 — Independente

- M5 universal só fecha após pré-requisitos acima.
- P189 fecha **a primeira peça** (Outline) e documenta
  todas as excepções.

---

## §7 ADR avaliação

**Sem ADR criada.** Replicação de padrão estabelecido
(P162/P165/P181E). Excepções são honestidade documental.

Decisão futura possível: ADR formalizando "M5 incremental,
não universal" — mas pode esperar até pré-requisitos
fecharem.

---

## §8 DEBT avaliação

### Cenário B (replica M4-residual)

**Sem DEBT formal aberto**. Nota preventiva no relatório
consolidado P189:

> DEBT M5-residual cobre 5 excepções declaradas. Quando
> pré-requisitos fecharem (`SetEquationNumbering`,
> sub-store `resolved_labels`+C4, sub-store
> `headings_for_toc`), excepções fecham incrementalmente
> e walk torna-se universalmente puro. Segue M6
> (eliminação `CounterStateLegacy`).

---

## §9 Restrições honradas

- **Zero código tocado**.
- **Zero testes** modificados.
- **Sem reservas de identificadores**.
- **Não modifica walk** — P189B+.
- **Não toca `from_tags`** — P189B+.
- **Não modifica trait `Introspector`** — P185B fechou.
- **Não materializa `SetEquationNumbering`** — fora série.
- **Não migra C4** — fora série.
- **Sem inflação retórica**.
- **Honestidade obrigatória sobre Reservas 1+2**:
  documentado em §1.4-1.6 + §2.6 do diagnóstico.
- **Regra dos 2 eixos aplicada** a cada mutação (§1.2 do
  diagnóstico).
- **Sem cláusulas condicionais** nos sub-passos.

---

## §10 Verificações

- ✅ `cargo check --workspace` passa (sem código tocado).
- ✅ `cargo test --workspace` passa: **1.808** inalterado.
- ✅ `crystalline-lint .` zero violations.
- ✅ Diagnóstico produzido (8 secções).
- ✅ Relatório produzido (este ficheiro).
- ✅ Sem ADR; sem DEBT formal.
- ✅ Reservas 1+2 confirmadas empiricamente.

---

## §11 Achados não-triviais

### §11.1 — Reserva 2 é alargada, não isolada a C4

Análise empírica revelou que `state.resolved_labels` é
populado por **2 arms** (Heading auto-toc + Labelled
explicit), e lido por Layouter Ref-arm. Migrar qualquer
elo exige sub-store para resolved_labels — **que não
existe**.

Reserva 2 originalmente caracterizada como "C4 não
migrado". Análise correcta: Reserva 2 é "sub-store
resolved_labels ausente + C4 não migrado". Trabalho mais
alargado do que P183E retomado pode cobrir.

### §11.2 — Cadeia de dependências bloqueia migração granular

Migrar Heading.hierarchical individualmente é tentador,
mas Labelled lê durante walk → popula resolved_labels.
Sem sub-store resolved_labels, não há para onde Labelled
escrever. Logo Heading.hierarchical fica gated por
Reserva 2 alargada também.

Idem Figure: figure_numbers é populated por from_tags P184B
✅, mas walk arm Labelled lê figure_numbers durante walk.

**Resultado**: a migração tem que acontecer em ordem
inversa à mutação. Primeiro abrir sub-store
`resolved_labels` → migrar Labelled → migrar Heading +
Figure → restaurar (em última instância) walk universal
puro.

P189 não pode fazer este trabalho todo. Fecha apenas
**Outline** (independente da cadeia) e documenta
honestamente.

### §11.3 — Outline.has_outline é o único migrável trivial

Eixo 1: snapshot final ✅ (`mod.rs:1423` lê o flag uma
vez antes do fixpoint TOC; não precisa walk-during).
Eixo 2: `kind_index[Outline]` populado por from_tags P178
✅.

Migração: walk arm remove `state.has_outline = true`;
consumer lê
`intr.kind_index.contains_key(&ElementKind::Outline)`.

Sub-passo trivial. P189B materializa.

### §11.4 — `Content::Styled` puro por design

Linha 608 confirma: `walk(body, state, locator, tags,
None)` — apenas recursão, sem mutação. P189A não
identifica trabalho aqui. Caso fechado por design.

### §11.5 — `CounterUpdate` arm a auditar empiricamente

Arms `Content::CounterUpdate { key, action }` mutam
`state.step_*` ou `update_flat`. Necessário verificar:
- Se `from_tags` cobre via `ElementPayload::CounterUpdate`
  ou similar.
- Se Layouter ainda muta `self.counter` durante layout
  (paralelo).

P189B `.A` audita; decisão pode ser migração α/β ou
excepção δ.

### §11.6 — P189 é "primeira peça" de M5, não M5 fechado

Spec original sugeria "M5 fecha com excepções declaradas".
Análise §1 mostra que **maioritariamente é excepções**.

Honestidade: P189 fecha 1 arm + documenta 5 excepções.
M5 universal precisa de 3-4 passos pré-requisito (sub-store
resolved_labels, sub-store headings_for_toc,
SetEquationNumbering, C4 migration) antes de fechar.

Recomendação: P189B materializar é trabalho útil para
desbloquear ciclos (Outline migrado liberta um pequeno
test sub-store). Mas chamar P189 de "fim de M5" seria
desonesto. P189 é **início incremental de M5**.

---

## §12 Snapshot pós-P189A

- **Tests workspace**: 1.808 (inalterado).
- **Trait `Introspector`**: 18 métodos.
- **M4-residual**: fechado funcionalmente (P188B).
- **M5 progresso**: 0 arms migrados; auditoria completa
  identifica 1 arm migrável + 5 excepções.
- **59 passos executados** (P188B = 58 + P189A = 59).
- **Padrão diagnóstico-primeiro**: 14ª aplicação consecutiva
  (P189A na lista).

---

## §13 Próximo passo

**P189B** — migrar Outline arm + documentar 5 excepções:

- Editar `introspect.rs:611`: remover mutação
  `state.has_outline = true`; comentário walk puro.
- Editar consumer em `mod.rs:1423`: ler
  `intr.kind_index.contains_key(&ElementKind::Outline)`.
- Adicionar 5 comentários de excepção inline em arms
  pendentes.
- L0 `rules/introspect.md` actualizado: secção "Walk puro
  M5 incremental" + 5 excepções declaradas.
- Tests E2E:
  - `outline_walk_puro_paridade` (Outline migrado).
  - 5 tests sentinela paridade legacy ↔ Layouter para
    excepções.
- Actualizar nota DEBT M5-residual no relatório
  consolidado P189.

Magnitude: S. Sem cláusulas condicionais.

---

## §14 Conclusão

P189A fechou 7 cláusulas com decisão literal e plano em
3 sub-passos. Magnitude S agregada confirmada.

Achado central: **M5 universal não é alcançável em P189**.
Análise empírica revelou cadeia de dependências
(Heading → Labelled → resolved_labels) que bloqueia
migração de 5 arms. Apenas Outline é independente e
migrável.

Honestidade obrigatória aplicada: P189 fecha **a primeira
peça** (Outline). Restantes 5 excepções ficam declaradas
com cross-references aos pré-requisitos (Reservas 1+2,
lacuna #3).

P189 termina como **início incremental de M5**, não fim.
M5 universal exige 3-4 passos pré-requisito antes de
fechar. Trabalho identificado:
1. Sub-store `resolved_labels` (Reserva 2 alargada).
2. C4 migration (P183E retomado ou novo).
3. Sub-store `headings_for_toc` (lacuna #3).
4. `Content::SetEquationNumbering` (Reserva 1).

Padrão diagnóstico-primeiro mantido — 14/14 acertaram
magnitude planeada ±1 nível
(P131A/132A/140A/148/154A/181A/182A/183A/184A/185A/186A/
187A/188A/189A).

P189 segue. Plano fica acessível em diagnóstico.
