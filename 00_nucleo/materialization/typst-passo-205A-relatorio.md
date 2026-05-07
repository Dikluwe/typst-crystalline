# Relatório do passo P205A

**Data de execução**: 2026-05-07.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-205A.md`.
**Natureza**: diagnóstico-primeiro (zero código tocado) +
auditoria empírica.
**Sub-passo `A` da série P205** — primeiro de 5 (A-E).
**Magnitude planeada**: M (S–M auditoria + S diagnóstico).
**Magnitude real**: **M** (~30 min; 4 ficheiros novos;
zero código produção).

---

## §1 O que foi feito

P205A produziu auditoria empírica e diagnóstico para F3
(refactor de sub-stores trackable do Layouter
cristalino), com 4 outputs e ADR-0074 PROPOSTO.

### Output 1 — Auditoria empírica (A1–A14)

Localização:
`00_nucleo/diagnosticos/typst-passo-205A-auditoria-f3.md`.

Conteúdo:
- §1 Bloco 1: Layouter cristalino — 22 fields confirmados
  (A1); classificação 21 ortogonais em A/B/C/D (A2);
  mutabilidade actual (A3); 9 pares de aliasing
  identificados (A4).
- §2 Bloco 2: Vanilla Layouter — `Engine<'a>` + N
  Layouters especializados (A5); 8 sub-stores tracked
  vanilla com Padrão A literal (A6); mapeamento
  cristalino ↔ vanilla assimétrico (A7).
- §3 Bloco 3: Compatibilidade comemo — Categoria B
  paradox mutability/imutabilidade single-pass (A8);
  `PagedIntrospector::new` post-layout vanilla (A9).
- §4 Bloco 4: Loops fixpoint vs F3 — coexistência
  hash + tracking (A10); Position trackable exige
  sealing point (A11).
- §5 Bloco 5: Estado pós-M8 — 3 candidatos elegíveis
  (`positions`, `label_pages`, `known_page_numbers`)
  (A12); 18 fields ineligíveis (A13); benefício
  performance F3 minimal arquitectural não mensurável
  claro (A14).
- §6 Resumo + 2 divergências registadas (`P205A.div-1`,
  `P205A.div-2`).

Tamanho: ~17 KB.

### Output 2 — Diagnóstico (C1–C11)

Localização:
`00_nucleo/diagnosticos/typst-passo-205A-diagnostico.md`.

Cláusulas instanciadas:

| Cláusula | Decisão fixada |
|----------|---------------|
| C1 — Escopo | Mínimo (`positions` + `label_pages` opcional) |
| C2 — Modelo tracking | Híbrido sealing post-iteração |
| C3 — Mecanismo | Padrão A literal (paridade M8) |
| C4 — Sealing point | Fim de cada iteração fixpoint |
| C5 — Fixpoint compat | Coexistência (não substituição) |
| C6 — Position | Trackable via sealed sub-store; fecha P204D §C6a |
| C7 — Lacunas | Sem lacunas novas; divergência registada |
| C8 — Magnitude | M agregada (S+S+S+S) |
| C9 — ADR-0074 | **Sim, criar** |
| C10 — Plano `*B+` | 4 sub-passos B-E sem ramos |
| C11 — Sem condicionais | Confirmado |

Tamanho: ~12 KB.

### Output 3 — Relatório (este ficheiro)

### Output 4 — ADR-0074 PROPOSTO (condicional em C9 = afirmativa)

Localização:
`00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md`.

Estado: PROPOSTO. Estrutura per `template-adr.md`.

Conteúdo:
- Contexto (M8 ACEITE; pendência P204D; auditoria
  P205A).
- Decisão (escopo, modelo, mecanismo, sealing,
  fixpoint compat, Position).
- 5 alternativas consideradas (B–F) com razão de
  rejeição.
- Consequências (positivas, negativas, neutras).
- Plano de validação (7 condições).
- Plano de materialização (5 sub-passos P205A–E; A já
  ✅ MATERIALIZADO).
- Cross-references (ADR-0073, P204D, P190C, snapshot).
- Pattern emergente.

Tamanho: ~12 KB.

---

## §2 Tempo de execução

~30 minutos efectivos:

- ~5 min: leitura da spec + setup TaskList.
- ~10 min: A1-A4 auditoria Layouter cristalino (22
  fields, classificação, mutabilidade, aliasing).
- ~10 min: A5-A11 auditoria vanilla typst-layout
  (Engine, PagedIntrospector, fixpoint compat).
- ~5 min: A12-A14 + redacção dos 4 outputs em paralelo.

---

## §3 Decisões

### D1 — F3 não é paridade vanilla literal (`P205A.div-1`)

A5 confirmou empíricamente: vanilla **não tem Layouter
monolítico**. Tem `Engine<'a>` (6 fields) + N Layouters
especializados (Composer, Distributor, Work, Collector,
StackLayouter, GridLayouter). F3 cristalino é solução
**específica** ao monolítico cristalino, não paridade
literal.

ADR-0074 documenta esta divergência intencional como
decisão de design, não lacuna.

### D2 — Categoria B é restrita a 3 sub-fields (`P205A.div-2`)

Snapshot 2026-05-05 §5 falava de "21 fields ortogonais"
como universo elegível para refactor F3. Auditoria
empírica P205A reduziu o universo elegível para **3
sub-fields** (apenas dentro do `runtime: LayouterRuntimeState`):
`label_pages`, `known_page_numbers`, `positions`. Os
restantes 18 são Categoria A (runtime puro), C (config)
ou D (ambígua) — não candidatos a tracking.

Implicação: F3 escopo "mínimo" é praticamente o universo
elegível. Não há "F3 completo" plausível sem refactor
profundo da arquitectura cristalina.

### D3 — Position concrete (P204D) ainda incompleto

P204D materializou tipo `Position` + `runtime.positions`
populated single-pass, mas
`TagIntrospector::position_of` retorna sempre `None`
(per ADR-0073 §C6a — `TagIntrospector` é construído
pre-layout sem acesso a Layouter runtime).

**F3 minimal fecha essa pendência estruturalmente**:
`SealedPositions` é tracked sub-store; consumer
(novo `PagedTagIntrospector` ou `TagIntrospector`
enriquecido) acede via Tracked.

### D4 — Sealing post-iteração é o caminho viável

Categoria B fields populated single-pass durante
layout; tracking exige imutabilidade. Three caminhos:

- Single-pass puro: impossível (paradox).
- Post-layout vanilla-like: magnitude XL+, refactor
  cross-modular. Cristalino divergiu intencionalmente
  em P204D para single-pass.
- Híbrido sealing post-iteração: viável; mantém
  divergência intencional; magnitude S por sub-store.

C2 fixou Híbrido como única alternativa fundamentada.

### D5 — Padrão A literal (não B3) por coerência com M8

Vanilla usa Padrão A em 8 sub-stores tracked
auditados (Sink, Route, Traced, Locator, Context,
World, Introspector, LateLinkResolver). M8 cristalino
adoptou Padrão A literal. F3 segue mesmo padrão por
coerência arquitectónica.

C3 fixou Padrão A literal.

### D6 — ADR-0074 PROPOSTO criado

C9 = afirmativa. F3 é decisão arquitectural com
alternativas reais (escopo, modelo, mecanismo, sealing)
e divergência significativa face a vanilla. ADR
dedicada permite cross-reference com ADR-0073 e
documentar a divergência intencional explicitamente.

### D7 — Magnitude F3 é M (não L como M8)

C8 totaliza M agregado (S+S+S+S = 4 sub-passos
B-E). M8 envolveu 7 sub-passos B-H + diagnóstico = 8
sub-passos com magnitude L cross-modular. F3 é mais
focado.

---

## §4 Confronto de hipóteses do passo

A spec listou hipóteses específicas em §3 e §9:

| Hipótese | Resultado |
|----------|-----------|
| `LayouterRuntimeState` pode precisar Tracked (A8) | **CONFIRMADA com paradox** — populated mutably; tracking exige sealing |
| Vanilla Layouter sub-stores tracked (A6) | **REFUTADA** — vanilla não tem Layouter monolítico; 8 sub-stores tracked são todos cross-modular |
| Vanilla aplica Padrão B3 ou A para layout state (A6) | **CONFIRMADA Padrão A literal** em vanilla |
| `PagedIntrospector::new` post-layout (A9) | **CONFIRMADA** — sub-stores construídos depois do layout |
| Hash convergence vs tracking convergence (A10) | **CONFIRMADO coexistência** — F3 acelera queries sem mudar fixpoint |
| Position trackable exige sealing (A11) | **CONFIRMADA** — single-pass populated; tracking só pós-sealing |
| F3 escopo XL se C1 = completo + C2 = post-layout | **NÃO MATERIALIZOU** — auditoria fixou escopo mínimo viável; C8 = M |
| Tentação Padrão B3 sem verificação empírica | **EVITADA** — A6 verificou que vanilla usa Padrão A literal; C3 fixou A |

7 de 8 hipóteses confirmadas pela auditoria empírica;
1 refutada (paridade Layouter literal). Auditoria
reduziu incerteza efectivamente.

---

## §5 Sugestão para próximo passo

P205A fechado per critério §6 da spec:

- ✓ A1–A14 todos com etiqueta CONFIRMADO ou DIVERGÊNCIA
  registada.
- ✓ C1–C11 instanciadas com valores concretos.
- ✓ ADR-0074 PROPOSTO escrito (C9 afirmativa).
- ✓ Magnitude calibrada (C8 = M).
- ✓ Plano `*B+` sem condicionais (4 sub-passos).

**Próximo sub-passo**: **P205B — Sealing infrastructure
+ SealedPositions** (per C10 + ADR-0074 plano de
materialização):

- `pub struct SealedPositions(Arc<HashMap<Location, Position>>)` em L1.
- `#[comemo::track] impl SealedPositions { fn position_of(&self, loc: Location) -> Option<Position>; }`.
- `Layouter::finish` produz sealed sub-store.
- 2-3 sentinelas.

Magnitude estimada: S–M (~1-2h).

---

## §6 Cross-references

- **Spec**:
  `00_nucleo/materialization/typst-passo-205A.md`.
- **Outputs**:
  - `00_nucleo/diagnosticos/typst-passo-205A-auditoria-f3.md`.
  - `00_nucleo/diagnosticos/typst-passo-205A-diagnostico.md`.
  - `00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md`.
- **ADRs vinculadas**:
  - ADR-0073 (M8 ACEITE 2026-05-07) — paridade literal
    `#[comemo::track]` no trait Introspector.
  - ADR-0072 (M7 fixpoint estruturalmente fechado).
- **Predecessores**:
  - P204H (M8 estruturalmente fechado; pré-condição).
  - P204D (Position concrete; pendência fechada por
    F3 minimal).
  - P190C (`LayouterRuntimeState` pattern).
- **Vanilla referência**:
  - `lab/typst-original/crates/typst-layout/src/introspect.rs:38`
    (PagedIntrospector::new post-layout).
  - `lab/typst-original/crates/typst-library/src/engine.rs:19`
    (Engine + 6 Tracked fields).
- **Snapshot 2026-05-05** §5 (referência inicial dos
  21 fields; auditoria empírica reduziu para 3 elegíveis
  Categoria B).
