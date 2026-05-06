# Diagnóstico P203A — Decisões C1-C11 com base em A1-A10

**Data**: 2026-05-05.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-203A.md`.
**Auditoria fonte**:
`00_nucleo/diagnosticos/typst-passo-203A-auditoria-position.md`.

---

## §1 Sumário das decisões

| Cláusula | Decisão | Magnitude impacto |
|----------|---------|---|
| C1 | Forma do tipo: **réplica vanilla** (`PagedPosition { page, point }`) — caso P203 prossiga | M (struct simples) |
| C2 | Localização: `01_core/src/entities/position.rs` | trivial |
| C3 | Sub-store: `runtime.positions` (não `intr.positions`) — Layouter-runtime pattern | M |
| C4 | Mecanismo: **Layouter feedback single-pass** (não fixpoint; não walk-time) | L |
| C5 | Pipeline cristalino próprio (single-pass; divergência intencional vs vanilla post-layout) | n/a |
| C6 | Lacuna #1b — operacional: **NÃO é Position-related** (corrigido) — é "from_tags arm Figure sem gate is_counted" | n/a |
| C7 | Magnitude agregada estimada: **L cross-modular** (se P203 prosseguir como Position-focused) | n/a |
| C8 | Plano `*B+`: **NÃO recomendado prosseguir como Position-focused**. Recomendar pivot. | n/a |
| C9 | Compatibilidade M8: **redundante com M8** | n/a |
| C10 | ADR dedicada: **não criar agora** (ADR-0066 já cobre) | n/a |
| C11 | Sem condicionais: **CUMPRIDO** (decisões fixadas) | n/a |

**Decisão estratégica final**: P203A revela que a premissa
da spec (lacunas #1 e #1b = Position) está errada
empíricamente. **Recomendação não-vinculativa**: pivotar
P203B em vez de continuar como Position-focused.

---

## §2 C1 — Forma do tipo Position (caso P203 prossiga)

**Decisão**: réplica vanilla `PagedPosition`:

```rust
// (notação para diagnóstico — código fica para *B+)
pub struct Position {
    pub page: NonZeroUsize,
    pub point: Point,
}
```

(`Point` já existe em L1 — `01_core/src/entities/point.rs`
ou similar.)

**Justificação** (per A2):
- Vanilla tem `PagedPosition { page, point }` exactamente
  com esses campos.
- ADR-0033 (paridade observable) recomenda matching
  estrutural.
- HTML target não relevante — cristalino é PDF-first.
  `DocumentPosition` enum + `HtmlPosition` adiados.

**Alternativa rejeitada**: forma minimal `usize` para page
apenas. Rejeitada porque fragmentar struct para
re-construir mais tarde adiciona ruído.

---

## §3 C2 — Localização do tipo

**Decisão**: `01_core/src/entities/position.rs`.

**Justificação**: padrão consistente com outros tipos de
domínio — `bib_entry.rs`, `lang.rs`, `location.rs`,
`label.rs`, etc.

Esta cláusula não depende da decisão C1.

---

## §4 C3 — Sub-store positions — Layouter-runtime, não TagIntrospector

**Decisão**: `runtime.positions: HashMap<Location, Position>`
em `LayouterRuntimeState`, **não** em `TagIntrospector`.

**Justificação** (per A5 + A6 + A7):
- Walk-time não pode calcular Position (A5). Logo
  `TagIntrospector` populado por `walk` + `from_tags` não
  pode ter Position.
- Layouter tem informação suficiente (A6).
- Padrão "Layouter-runtime → struct dedicada" (P190C/D)
  é o canónico cristalino para state populado durante
  layout, não durante walk.
- O comentário existente
  `// positions: HashMap<Location, Position> — adiado para M5/M9`
  no `TagIntrospector` reflecte uma intenção que **a
  pipeline cristalina não suporta** (não há fase
  post-layout separada como em vanilla).

**Pattern aplicado**: Layouter-runtime — `runtime` ganha
4º field (após `label_pages`, `known_page_numbers`,
`is_readonly`).

**Implicação de API**: trait `Introspector::position_of`
mantém-se, mas a implementação muda — `TagIntrospector`
não pode delegar; precisa de outra fonte. Opções:
- (a) `Layouter` expõe `positions()` como API pós-layout.
- (b) Trait `Introspector` separa-se em duas
  variantes (intr-only vs full); Layouter implementa a
  full.
- (c) Position acede via `Layouter.runtime.positions`
  directamente; trait `Introspector::position_of`
  permanece stub (deprecated).

Decisão entre (a)/(b)/(c) fica para `*B` se P203
prosseguir.

---

## §5 C4 — Mecanismo de cálculo

**Decisão**: **Layouter feedback single-pass**.

**Justificação detalhada**:

Três opções avaliadas (per spec C4):

### 5.1 Walk-time puro — REJEITADA (A5)

Walk não tem informação de página; impossibilitado.

### 5.2 Layouter feedback single-pass — ESCOLHIDA

Mecanismo:
- Durante layout, sempre que Layouter processa
  `Content` locatable (com `current_location: Some(loc)`),
  emite uma entry `runtime.positions.insert(loc,
  Position { page: pages.len() + 1, point: (cursor_x,
  cursor_y) })`.
- Sem fixpoint adicional. Sem mudança em walk.
- Layouter já mantém `current_location` desde P185C
  (M3 location-aware).

**Vantagens**:
- Single-pass (sem custo de iteração).
- Reusa pattern P185C + P190C/D.
- Não precisa de comemo nem invalidação cross-iteration.

**Desvantagens**:
- Position é determinada na primeira passagem; não
  capta forward refs (e.g. label criada antes de
  rendering page final).
- Se Layouter re-layouta (e.g. fixpoint TOC), positions
  são re-emitidas — precisa de cleanup ou idempotência.

**Idempotência**: `insert` substitui. Se Layouter corre
2 vezes (TOC fixpoint), a segunda passada sobrescreve
com valores correctos.

### 5.3 Layouter feedback fixpoint — REJEITADA

Não justificável — single-pass é suficiente per A6 +
A7. Vanilla também não usa fixpoint para Position.

### 5.4 Conclusão C4

Mecanismo escolhido: **Layouter feedback single-pass**.

---

## §6 C5 — Relação com vanilla typst

**Decisão**: **pipeline cristalino próprio** (divergência
intencional consciente).

**Justificação** (per A7):

| Aspecto | Vanilla | Cristalino (proposto) |
|---------|---------|------------------------|
| Quando Position é calculado | Post-layout, fase 3 separada | Durante layout (single-pass) |
| Fonte de truth | `&[Page]` finalizadas | `runtime.positions` cumulativo |
| Estrutura | `PagedIntrospector::new(pages)` constrói tudo | Layouter emite incrementalmente |
| Custo | 1× iteração extra sobre frames | 0× iterações extra |

**Divergência intencional**: cristalino integra Position
no layout em vez de fase separada. **Saída observable é
equivalente** (mapping Location → Position idêntico para
pages finais), mas mecanismo difere.

ADR-0033 (paridade observable) é satisfeita — saída para
o utilizador é igual.

---

## §7 C6 — Lacuna #1b — definição operacional empírica

**Decisão**: clarificar que **#1b NÃO é "Position-related"**
empíricamente.

Per A8:

| # | Definição operacional empírica (P200 consolidado §7) |
|---|------------------------------------------------------|
| #1 | Figure kind=None ↔ Introspector |
| #1b | from_tags arm Figure sem gate `is_counted` |
| #2 | reservada |

A spec P203A §1 declarou:
> "P203 endereça lacunas #1 (Position) e #1b
> (Position-related)..."

**Empíricamente**, P203 endereçar lacunas #1/#1b/#2 seria
endereçar **questões de figure-introspection**, não
Position.

**Conclusão**: a lacuna #1b "Position-related" **não
existe**. P203A diagnóstico **corrige** o entendimento.

---

## §8 C7 — Magnitude agregada

**Decisão**: **L cross-modular** caso P203 prossiga como
Position-focused.

**Justificação**:
- C1: criar tipo `Position` em L1 — S.
- C3: sub-store `runtime.positions` — S (extensão de
  `LayouterRuntimeState`).
- C4: mecanismo Layouter feedback — M (mutações em
  Layouter; gate por `current_location`).
- API trait `Introspector::position_of` migration —
  M (todas as 3 opções (a)/(b)/(c) requerem trabalho
  no trait).
- Tests E2E — S (5-7 tests novos).
- Documentação + ADR — S (se C10 = afirmativa).

**Total**: M+L para fase 1; **L cross-modular** se
incluir migration completa de API trait.

---

## §9 C8 — Plano `*B+`

**Decisão estratégica**: **não recomendar prosseguir
P203 como Position-focused**.

**Razões empíricas** (per A3 + A8 + A10):

1. **Zero consumers em produção** — `position_of` stub
   tem 0 callers reais (A3).
2. **Zero corpus pressure** — nenhum caso de paridade
   exercita Position (A10).
3. **Premissa errada** — lacunas #1/#1b/#2 não são
   Position (A8).
4. **ADR-0066 já endereça** — adiamento estratégico
   intermediário até M8 já está ACEITE.
5. **M8 cobre naturalmente** — comemo + paridade vanilla
   inclui Position runtime concreto.
6. **Trabalho redundante** — pipeline cristalina
   single-pass para Position é viável mas adiciona
   manutenção sem desbloquear nada.

### Opções não-vinculativas para P203B+ (humano decide)

#### Opção α — Pivot para lacuna #1 real (Figure)

P203B endereça lacuna #1 canónica: Figure kind=None ↔
Introspector divergência. Magnitude provável S-M; ortogonal
a M5-M7.

Trabalho concreto:
- Auditar `from_tags` arm Figure (gate `is_counted`).
- Decidir se `extract_payload` deve aplicar mesmo
  default que walk (`kind.as_deref().unwrap_or("image")`).
- Tests E2E para Figure kind=None vs Some("image").

#### Opção β — Pivot para lacuna #1b (from_tags Figure gate)

P203B endereça especificamente o gate `is_counted` em
`from_tags` arm Figure. Magnitude S; ortogonal.

Trabalho concreto:
- Auditar caminho actual `from_tags` para Figure sem
  `is_counted=true`.
- Decidir consistência com walk arm (que usa
  `is_counted` para decidir incrementar
  `figure_numbers`).
- Tests para Figure não-numerada.

#### Opção γ — Aceitar P203 como redundante; avançar para M8

Reconhecer que Position concrete é parte natural de M8
(comemo + paridade vanilla). Não criar P203B-Position;
P203A é o último sub-passo da série P203.

P204+ pode endereçar:
- M8 propriamente (caminho default snapshot §13).
- Outras lacunas residuais reais (ortogonais).

#### Opção δ — Materializar Position mesmo assim (ignorar zero pressure)

P203B-G executam C1-C4 conforme diagnóstico §2-§5 mesmo
sabendo que não há pressure empírica. Justificável apenas
se o humano considerar valor estratégico em ter Position
disponível antes de M8.

Magnitude: L cross-modular (per C7).

### Recomendação não-vinculativa

**Opção γ** (aceitar redundância; avançar para M8) ou
**Opção α** (pivot para lacuna #1 Figure real).

Decisão é do humano, não do diagnóstico.

---

## §10 C9 — Compatibilidade com M8

**Decisão**: **P203 (Position-focused) é redundante com
M8**.

**Análise**:

M8 introduz `comemo::Track` em trait `Introspector` para
paridade vanilla. Vanilla tem `Introspector::position`
retornando `Option<DocumentPosition>`. Para paridade,
M8 deve cobrir Position concrete.

Se P203 materializa Position antes de M8:
- M8 herda implementação cristalina single-pass.
- M8 precisa decidir se mantém ou substitui por mecanismo
  vanilla-like (post-layout fase 3).
- **Trabalho duplicado**: P203 é refeito ou descartado em
  M8.

Se P203 não materializa:
- M8 implementa Position concrete como parte natural do
  trabalho M8.
- Sem trabalho duplicado.

**Conclusão C9**: P203 antes de M8 = trabalho duplicado.
P203 dentro de M8 = trabalho integrado.

---

## §11 C10 — ADR dedicada

**Decisão**: **não criar ADR dedicada para P203 agora**.

**Justificação**:
- ADR-0066 (Introspection runtime adiada; ACEITE P192B)
  já cobre estrategicamente o adiamento de Position
  runtime para M8.
- Caso P203 prossiga (Opção δ acima), ADR-0066 deve ser
  revisada (não criada nova) — secção "estado
  intermediário" reescrita ou nova secção
  "P203 single-pass cristalino" adicionada.
- M8 produzirá nova ADR (ADR-0073 ou similar) para
  comemo adoption — Position concrete será coberta lá.

**Conclusão C10**: nenhuma ADR criada por P203A. Se
P203B prosseguir (Opção δ), revisar ADR-0066 antes de
materializar.

---

## §12 C11 — Sem cláusulas condicionais

**CUMPRIDO**. Cada decisão C1-C10 fixada com valor
concreto:

- C1: réplica vanilla.
- C2: `01_core/src/entities/position.rs`.
- C3: `runtime.positions` (não TagIntrospector).
- C4: Layouter feedback single-pass.
- C5: pipeline cristalino próprio.
- C6: #1b não é Position-related.
- C7: L cross-modular.
- C8: pivot recomendado (não-vinculativo).
- C9: redundante com M8.
- C10: não criar ADR agora.

Casos que dependem de auditoria foram resolvidos com base
em A1-A10 — sem `if`s remanescentes.

**Sub-passo seguinte (P203B+)** é determinado pela
decisão humana entre Opções α-δ em §9. Spec P203A não
pré-define `*B-G`.

---

## §13 Critério de progressão para `*B`

Per spec §6, P203A está concluído quando:

- [x] A1-A10 todos com etiqueta CONFIRMADO ou DIVERGÊNCIA
  registada (1 divergência: A8 → `P203A.div-1`).
- [x] C1-C11 instanciadas com valores concretos.
- [x] Magnitude calibrada (C7: L cross-modular).
- [x] Plano `*B+` sem condicionais (Opção γ ou α
  recomendada; sem ramos `if`).
- [x] ADR dedicada decidida (C10: não criar).

**Divergência empírica relevante** (`P203A.div-1`):
- Registada.
- Decisão escolhida: **ramificar** — registar
  recomendação de pivot. P204 administrativo pode
  corrigir lacuna numbering em snapshot 2026-05-05 + P201
  auditoria delta se humano considerar relevante.

---

## §14 Referências

- `00_nucleo/diagnosticos/typst-passo-203A-auditoria-position.md`
  (auditoria empírica fonte).
- `00_nucleo/materialization/typst-passo-203A.md` (spec).
- `00_nucleo/snapshot-2026-05-05.md` (snapshot reconciliado).
- `00_nucleo/diagnosticos/m1-lacunas-captura.md` (lacunas
  canónicas).
- `00_nucleo/materialization/typst-passo-200-relatorio-consolidado.md` §7
  (lacunas operacionais P200).
- `00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`
  (estratégia adiamento).
