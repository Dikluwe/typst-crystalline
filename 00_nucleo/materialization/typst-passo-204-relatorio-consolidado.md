# Relatório consolidado da série P204 — M8 estruturalmente fechado

**Escopo**: P204A–H (8 sub-passos da série M8).
**Data de fecho**: 2026-05-07.
**Forma de fecho**: estruturalmente fechado (análogo a M7
P192B per ADR-0072).
**ADR vinculante**: ADR-0073 (ACEITE 2026-05-07; 8/9
condições CUMPRIDAS; condição 9 PARCIAL).
**ADR superseded**: ADR-0066 (SUPERSEDED-BY 0073 em
P204H).

---

## §1 Trajectória da série

A série P204 implementou a adopção de `#[comemo::track]`
no trait `Introspector` cristalino — o último marco
arquitectónico da fase de paridade vanilla literal por
M5/M6/M7/M8/M9.

### §1.1 Diagnóstico-primeiro de profundidade máxima (P204A)

**Magnitude planeada**: M+ (auditoria empírica). **Real**:
~3h.

P204A produziu uma **auditoria de 16 cláusulas A1–A16**
cobrindo 5 blocos arquitecturais (estado pré-M8;
compatibilidade comemo 0.4.0; impactos cross-modular;
política invalidação; plano de validação). Output:
ADR-0073 PROPOSTO + plano de materialização P204B–H com
13 cláusulas (C1–C13).

Padrão estabelecido: spec da série segue
"diagnóstico-primeiro" (per P203 §9.1) — mesmo `*B+`
começam com inventário empírico.

### §1.2 Implementação progressiva (P204B–G)

| Sub-passo | Magnitude planeada | Magnitude real | Output principal | Tests |
|-----------|-------------------|----------------|------------------|-------|
| P204B | S-M | **M** | `#[comemo::track]` aplicado ao trait | 1824 → 1827 (+3) |
| P204C | M | **M** | Layouter ganha lifetime + Tracked field | 1827 → 1829 (+2) |
| P204D | S-M | **S-M** | `Position` concreto + `position_of` retorna `Option<Position>` | 1829 → 1836 (+7) |
| P204E | S | **S** (~30 min) | `crystalline_evict(n)` wrapper L4 | 1836 → 1838 (+2) |
| P204F | M | **M** (~70 min) | 6 ficheiros corpus paridade + 6 smoke tests | 1838 → 1844 (+6) |
| P204G | S | **S+** (~50 min) | `typst_infra::measurements` (L3) + wrapper newtype | 1844 → 1852 (+8) |

**Trajectória total**: 1824 → 1852 (+28 tests; estimativa
P204A foi +10 a +23 — cobertura excedida sem regressão).

### §1.3 Encerramento (P204H)

**Magnitude planeada**: S documental. **Real**: ~30 min.

P204H executou auditoria das 9 condições de validação de
ADR-0073, fixou forma de fecho ("estruturalmente
fechado"), fixou caminho de resolução (A — aceitar
parcialmente), produziu este relatório consolidado,
transitou ADR-0073 PROPOSTO → ACEITE e ADR-0066 ACEITE
→ SUPERSEDED-BY 0073, e anotou marca cirúrgica no
blueprint.

---

## §2 Divergências detectadas e absorvidas

A série P204 detectou e absorveu 2 divergências sem
inflar magnitude:

### §2.1 `P204B.div-1` — 4 métodos do trait sem `Hash`

**Sub-passo**: P204B.
**Sintoma**: `#[comemo::track]` exigia que returns dos
métodos fossem `Hash`. 4 dos 20 métodos retornavam
`Vec<Value>`, `Option<&BibEntry>`, `&[(Label, Content,
usize)]` que não tinham `Hash` impl.

**Resolução**: 3 `impl Hash` adicionais em L1 (`Value`,
`BibEntry`, `Content`). Resolvido dentro de P204B sem
recuar a P204A. Magnitude real M (planeada S-M).

### §2.2 `P204F.div-1` — Vanilla integration deferred

**Sub-passo**: P204F.
**Sintoma**: spec P204A C13.1 anteviu observable harness
para sanity-check cristalino vs vanilla. Realidade
empírica em P204F: lab/parity harness vanilla **não
funcional desde antes de M8** (DEBT-53/54 pre-existing
P151/P152).

**Resolução**: Caminho B reduzido (cristalino-only
baseline). 6 smoke tests cristalino verdes; vanilla
comparison adiada para sub-passo dedicado pós-M8.
Registado em ADR-0073 §P204F como `P204F.div-1` e em
P204H bloco "Validação P204A–H" como condição 9 PARCIAL.

---

## §3 Outputs concretos por sub-passo

### §3.1 Tabela de referência

| Sub-passo | Outputs |
|-----------|---------|
| P204A | ADR-0073 PROPOSTO; spec P204B–H; inventário + relatório |
| P204B | `#[comemo::track]` aplicado; 3 `Hash` impls (L1); 3 sentinelas; inventário + relatório |
| P204C | Layouter `<'a>` + `Tracked` field; consumers migrados; 2 sentinelas; inventário + relatório |
| P204D | `01_core/src/entities/position.rs`; `runtime.positions`; trait API estável (`Option<Position>`); 2 sentinelas; inventário + relatório |
| P204E | `04_wiring/src/eviction.rs` (`crystalline_evict`); L0 prompt `wiring/eviction.md` (hash `7ac7b48b`); 2 sentinelas; inventário + relatório |
| P204F | 6 ficheiros `.typ` + companions em `lab/parity/corpus/visual/`; 6 smoke tests cristalino; inventário + relatório |
| P204G | `03_infra/src/measurements.rs` (`CacheStats`, `CallCounts`, wrapper `CountingIntrospector`); L0 prompt `infra/measurements.md` (hash `c89617ca`); logging opt-in `CRYSTALLINE_MEASUREMENTS=1` em `main.rs`; 2 sentinelas + 6 tests; inventário + relatório |
| P204H | Este relatório consolidado; ADR-0073 ACEITE + bloco "Validação P204A–H"; ADR-0066 SUPERSEDED-BY 0073; blueprint anotado [P204H]; inventário + relatório |

### §3.2 Ficheiros novos por camada

**L1** (3 ficheiros novos):

- `01_core/src/entities/position.rs` (P204D).
- `01_core/src/entities/value.rs` — `impl Hash` adicionado
  (P204B.div-1).
- `01_core/src/entities/bib_entry.rs` — `impl Hash`
  adicionado (P204B.div-1).
- `01_core/src/entities/content.rs` — `impl Hash`
  adicionado (P204B.div-1).

**L3** (1 ficheiro novo):

- `03_infra/src/measurements.rs` (P204G).

**L4** (1 ficheiro novo):

- `04_wiring/src/eviction.rs` (P204E).

**Corpus paridade** (13 ficheiros novos):

- 6 `.typ` + 6 `.toml` companions + 1 asset `refs.yaml`
  em `lab/parity/corpus/visual/` (P204F).

**L0 prompts** (2 ficheiros novos):

- `00_nucleo/prompts/wiring/eviction.md` (P204E).
- `00_nucleo/prompts/infra/measurements.md` (P204G).

---

## §4 Achados consolidados

Per auditoria das 9 condições em P204H C1:

| # | Condição | Estado |
|---|----------|--------|
| 1 | P204B materializado | ✅ CUMPRIDA |
| 2 | P204C materializado | ✅ CUMPRIDA |
| 3 | P204D materializado | ✅ CUMPRIDA |
| 4 | P204E materializado | ✅ CUMPRIDA |
| 5 | P204F materializado | ✅ CUMPRIDA |
| 6 | P204G materializado | ✅ CUMPRIDA |
| 7 | Tests workspace verdes | ✅ CUMPRIDA (1852, +28) |
| 8 | Lint 0 violations | ✅ CUMPRIDA |
| 9 | Sanity-check cristalino vs vanilla observable | ⚠️ PARCIAL (`P204F.div-1`) |

**8/9 CUMPRIDAS, 1 PARCIAL**.

A condição 9 PARCIAL é registada como excepção
justificada per:

- DEBT-53/54 pre-existing (não criado por M8).
- Spec P204A C13.1 assumiu, sem verificar empíricamente,
  que harness vanilla estaria operacional.
- P204F descobriu empíricamente o gap e registou
  divergência com fundamento.
- Caminho A (P204H C3) documenta excepção sem inflar.

---

## §5 Métricas agregadas

### §5.1 Tests

| Métrica | Antes M8 (P203) | Pós M8 (P204H) | ∆ |
|---------|-----------------|----------------|---|
| Tests workspace | 1824 | **1852** | **+28** |
| Tests ignored | 6 | 6 | 0 |
| Tests falhantes | 0 | 0 | 0 |

Distribuição P204H:

- `typst_core` unit: 1576.
- `typst_infra` unit: 229 (+ 6 ignored).
- `typst_shell` unit: 24.
- `typst-wiring` binary unit: 2.
- `typst-wiring` integration `tests/cli.rs`: 21.
- Doc-tests: 1 ignored.

### §5.2 LOC

| Categoria | LOC novas (estimadas) |
|-----------|----------------------|
| L1 produção (`Position`, `Hash` impls) | ~150 |
| L3 produção (`measurements`) | ~290 |
| L4 produção (`eviction` + edição main.rs) | ~70 |
| Tests (sentinelas + cláusula-C6 + smoke + integration) | ~600 |
| L0 prompts (eviction + measurements) | ~270 |
| Documental (relatórios + inventários + ADR + blueprint) | ~3500 |

### §5.3 ADRs

| ADR | Antes M8 | Depois M8 |
|-----|----------|-----------|
| ADR-0073 | (não existia) | ACEITE 2026-05-07 |
| ADR-0066 | ACEITE com nota intermediário | SUPERSEDED-BY 0073 |
| ADR-0072 | EM VIGOR (M7 fechado) | EM VIGOR (preservado) |

P204 **não** criou ADRs novas além de 0073. Todas as
restantes ADRs preservadas sem alteração.

### §5.4 Sentinelas activas

**17 sentinelas** introduzidas pela série:

- P204B: 3 (`p204b_trait_e_send_sync`,
  `p204b_dyn_trait_implementa_track`,
  `p204b_tagintrospector_pode_ser_tracked_via_dyn`).
- P204C: 2.
- P204D: 2.
- P204E: 2 (`p204e_crystalline_evict_existe`,
  `p204e_crystalline_evict_aceita_max_age_parametro`).
- P204F: 6 smoke tests (`p204f_corpus_*_compila`).
- P204G: 2 (`p204g_cache_stats_existe`,
  `p204g_introspector_call_counts_existe`).

**Total: 17**. Todas verdes em `cargo test --workspace`.

P204H não adiciona nem remove sentinelas (encerramento
documental).

---

## §6 Divergências da série

| Divergência | Sub-passo | Causa | Resolução |
|-------------|-----------|-------|-----------|
| `P204B.div-1` | P204B | 4 métodos retornam tipos sem `Hash` impl | 3 `impl Hash` adicionados (L1); resolvido dentro de P204B |
| `P204F.div-1` | P204F | lab/parity harness vanilla não funcional (DEBT-53/54 pre-existing) | Caminho B reduzido (cristalino-only baseline); registada como condição 9 PARCIAL em P204H |

Ambas detectadas empíricamente, registadas formalmente,
absorvidas sem inflação de magnitude. P204B.div-1
resolvida; P204F.div-1 registada como excepção
justificada (DEBT pre-existing fora-de-escopo de M8).

---

## §7 Padrão demonstrado

A série P204 aplicou em **8 aplicações consecutivas** o
padrão consolidado por P181/P184/P190/P192/P200/P203:

1. **Diagnóstico-primeiro de profundidade máxima** — P204A
   produziu 16 cláusulas (A1–A16) antes de qualquer
   alteração de código.
2. **Inventário empírico antes de implementação** — P204B–G
   começam com C1 inventário de 3–6 sub-secções.
3. **Decisões fixadas em alternativas** — cada sub-passo
   tem C2/C3 que fixam **uma** alternativa entre 2–4
   listadas (não condicionais estruturais na spec; decisão
   baseada em evidência).
4. **Hipóteses de obstrução listadas, não pré-fixadas como
   ramos** — quando obstrução materializa, sub-passo
   detecta e regista divergência (`*.div-N`).
5. **Paridade vanilla literal quando viável** — padrão A
   (paridade literal) preferido sobre B/C/D em ADR-0073.
6. **Magnitude calibrada por soma de componentes** — M8
   foi M cross-modular = M+M+S-M+S+M+S+S documental.
7. **Sentinelas mínimas por sub-passo** — 1–6 sentinelas
   por passo; total 17 acumuladas.
8. **L0 prompts criados via `--fix-hashes` para
   sincronização automática** — eviction (P204E) e
   measurements (P204G).

**2 divergências detectadas e absorvidas sem inflação**:
P204B.div-1 (resolução interna), P204F.div-1 (excepção
documentada). Padrão consolidado: divergência empírica
NÃO É falha; é *informação* que reforma o passo sem
expandir.

---

## §8 Estado pós-série face ao snapshot 2026-05-05

**Snapshot 2026-05-05** (per P203 consolidado §13):

- 1824 tests workspace.
- 0 violations.
- 70 ADRs (-1 slot 0063).
- M5 universal fechado (P200B).
- M6 fechado (P190I).
- M7 estruturalmente fechado (P192B; ADR-0072).
- M9 fechado 11/11 (P182F).
- M8 ainda **NÃO** materializado (ADR-0066 ACEITE com
  nota "intermediário até M8").

**Estado pós-P204H** (2026-05-07):

- **1852 tests workspace** (+28).
- **0 violations** (preservado).
- **71 ADRs** (+1 ADR-0073; 0 revogadas).
- M5/M6/M7/M9 preservados (sem regressão).
- **M8 estruturalmente fechado** (P204H; ADR-0073 ACEITE;
  ADR-0066 SUPERSEDED-BY 0073).
- 17 sentinelas P204 activas.
- 2 L0 prompts novos (`wiring/eviction`,
  `infra/measurements`).
- `P204F.div-1` (vanilla integration deferred) registada
  como condição 9 PARCIAL — DEBT pre-existing
  fora-de-escopo de M8.

**Pre-existing DEBTs preservados**: DEBT-34d/34e
(grid layouting), DEBT-52 (text.font dict + regex),
DEBT-53/54 (vanilla integration), DEBT-55 (bibliography
cite). Nenhum endereçado por M8 (fora-de-escopo).

---

## §9 Convenções consolidadas pela série

Lições aprendidas (formalizadas para futuros marcos):

### §9.1 Mesmo sub-passos `*B+` começam com inventário empírico

P203 §9.1 já formalizara isto. P204 confirmou em 6
aplicações consecutivas: nenhum sub-passo iniciou
implementação sem C1 inventário concreto. Pattern
reaproveitável.

### §9.2 L4 é estritamente para wiring sem criação de tipos

Disciplina V12 confirmada empíricamente em P204G:
implementação inicial em L4 disparou warnings;
refactor para L3 eliminou-os sem custo arquitectural.
Nota literal `04_wiring/src/main.rs:101` ("L4 faz I/O
trivial sem criar tipos — composição pura") é referência
canónica para futuros sub-passos.

### §9.3 L0 prompts criados via `--fix-hashes` para sincronização automática

P204E e P204G demonstraram fluxo:

1. Redigir L0 com `Hash do Código: 00000000`.
2. Escrever código com `@prompt-hash 00000000`.
3. Correr `crystalline-lint --fix-hashes .` para
   sincronizar ambos automaticamente.

Sem este fluxo, V5 (drift) acumula. Pattern
reaproveitável para qualquer sub-passo que crie L0.

### §9.4 Hipóteses de obstrução listadas reduzem iteração

Specs da série P204 listaram em §"Erro a não repetir" e
§"Hipóteses de obstrução" cenários antecipáveis. Quando
realidade divergiu (P204B.div-1, P204F.div-1), sub-passo
identificou rapidamente o cenário antecipado e aplicou
caminho alternativo sem desperdiçar tempo. Pattern
reaproveitável.

### §9.5 "Estruturalmente fechado" é etiqueta legítima quando 1+ condição PARCIAL é justificável

M7 (P192B) e M8 (P204H) ambos fechados como
"estruturalmente fechado" por terem condição não
totalmente cumprida com fundamento. **Não é
acomodação cosmética**: é honestidade arquitectural.
Etiqueta "fechado completo" exige 100% de condições
materializadas; "estruturalmente fechado" admite
excepções documentadas.

---

## §10 Não-objectivos respeitados

A série P204 **não**:

- Modificou trait `Introspector` além de adicionar
  `#[comemo::track]` (assinaturas dos 20 métodos
  preservadas; só `position_of` mudou
  `Option<()>` → `Option<Position>`).
- Tocou em loops fixpoint (mantidos sem alteração).
- Criou ADRs além de 0073 (ADR-0066 transitada, não
  reescrita).
- Reescreveu blueprint (apenas marca cirúrgica [P204H]).
- Endereçou DEBTs pre-existing (53/54 preservados).
- Adicionou `crystalline_evict` em CLI (pós-M8).
- Implementou hits/misses por sub-store (granularidade
  per-method via comemo é suficiente).
- Adicionou benchmarks comparativos com vanilla.
- Reescreveu ADR-0066 (preservada como histórico per
  padrão P201/P202).
- Acrescentou ficheiros ao corpus paridade além dos 6
  introspection.

Honestidade preservada — nenhum não-objectivo violado.

---

## §11 Sugestão para próximo marco arquitectónico

P204H não decide próximo passo. Reporta opções plausíveis
para o humano avaliar:

1. **Sub-passo dedicado a `P204F.div-1`** — fechar
   condição 9 da ADR-0073: investigar DEBT-53/54,
   tentar restaurar lab/parity harness vanilla, executar
   sanity-check observable nos 6 ficheiros corpus
   introspection. Magnitude estimada M (depende do
   estado actual do harness; pode ser XL se harness
   exige refactor profundo).
2. **Próximo marco arquitectónico** — F3 completo (Layout
   Fase X — block, columns, stack, hide, repeat, pad,
   ...; opção B no blueprint §3.2); Model Fase 2
   completa (P156–P158 — table, figure-kinds,
   bibliography com hayagriva); Introspection cobertura
   user-facing (here(), locate(), query() expostos como
   stdlib).
3. **Optimizações pós-M8** — re-walks parciais via
   constraint tracking granular do comemo (mencionado em
   ADR-0073 "Consequências positivas" como "fundação
   para optimizações futuras"); benchmarks comparativos
   cristalino vs vanilla (P204G §7 não-objectivo);
   integração `crystalline_evict` em CLI watch mode.
4. **Higiene documental** — actualização ampla do
   blueprint (datado 2026-04-25 com 1145 tests vs estado
   actual 2026-05-07 com 1852); reorganização de DEBTs
   (13 abertos; alguns já resolvidos implicitamente
   desde P155).
5. **Pausa estratégica** — M8 fechou um marco grande;
   consolidação antes de novo investimento.

P204H **não** pré-define. Humano decide com base no
estado pós-série e prioridades não-codificadas.

---

## §12 Cross-references

- **Specs**:
  - `00_nucleo/materialization/typst-passo-204A.md` a
    `typst-passo-204H.md`.
- **Inventários**:
  - `00_nucleo/diagnosticos/typst-passo-204A-auditoria-comemo.md`.
  - `00_nucleo/diagnosticos/typst-passo-204A-diagnostico.md`.
  - `00_nucleo/diagnosticos/typst-passo-204{B,C,D,E,F,G,H}-inventario.md`.
- **Relatórios individuais**:
  - `00_nucleo/materialization/typst-passo-204{A,B,C,D,E,F,G,H}-relatorio.md`.
- **ADRs**:
  - `00_nucleo/adr/typst-adr-0073-comemo-introspector.md`
    (ACEITE 2026-05-07).
  - `00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`
    (SUPERSEDED-BY 0073).
  - `00_nucleo/adr/typst-adr-0072-fixpoint-runtime-estruturalmente-fechado.md`
    (preservado).
- **L0 prompts**:
  - `00_nucleo/prompts/wiring/eviction.md` (hash `7ac7b48b`).
  - `00_nucleo/prompts/infra/measurements.md` (hash `c89617ca`).
- **Blueprint**:
  - `00_nucleo/diagnosticos/blueprint-projecto.md` §3.0
    (marca cirúrgica [P204H]).
- **Vanilla referência**:
  - `lab/typst-original/crates/typst-eval/.../introspector.rs:28`
    (paridade literal `#[comemo::track] pub trait
    Introspector: Send + Sync`).
  - `lab/typst-original/crates/typst-cli/src/watch.rs:81`
    (`comemo::evict(10)`; paralelo a `crystalline_evict`).
- **comemo 0.4.0**:
  `~/.cargo/registry/src/index.crates.io-*/comemo-0.4.0/`.

---

## §13 Resumo executivo

| Métrica | Valor |
|---------|-------|
| Sub-passos | 8 (P204A–H) |
| Tempo total estimado | ~10–14h |
| Tests adicionados | +28 (1824 → 1852) |
| Tests falhantes | 0 |
| Lint violations | 0 |
| ADRs novas | 1 (ADR-0073) |
| ADRs transitadas | 2 (0073: PROPOSTO→ACEITE; 0066: ACEITE→SUPERSEDED-BY 0073) |
| ADRs revogadas | 0 |
| Sentinelas | 17 (3+2+2+2+6+2) |
| Divergências | 2 (P204B.div-1 resolvida; P204F.div-1 documentada) |
| Condições ADR cumpridas | 8/9 (1 PARCIAL — `P204F.div-1`) |
| Forma de fecho | Estruturalmente fechado |
| L0 prompts novos | 2 (eviction + measurements) |

**M8 fechado estruturalmente em 2026-05-07**. Adopção de
`#[comemo::track]` em `Introspector` materializada em
paridade vanilla literal; Position concrete; corpus
paridade introspection; measurements infra. `P204F.div-1`
(vanilla integration) é trabalho separado pós-M8.

---

## §14 Anotação cirúrgica P206E — Fecho retroactivo cond 9 (2026-05-08)

**Adicionado em**: 2026-05-08 (P206E).
**Pattern aplicado**: anotação cirúrgica per P201/P202
preservação histórica — secções §1–§13 acima **NÃO
reescritas**; este bloco regista o ponto final da
trajectória cond 9 sem alterar narrativa P204H.

### Estado em 2026-05-07 (P204H)

Per §5.3 + §6 acima:
- Cond 9 ADR-0073 era **PARCIAL** por `P204F.div-1`
  (vanilla integration deferred; DEBT-53/54
  pre-existing).
- Forma de fecho fixada como "estruturalmente fechado"
  com excepção justificada.
- Trabalho separado pós-M8 sugerido.

### Evolução em 2026-05-08 (série P206)

P206 série (A-E) materializou vanilla integration:

- **P206A** descobriu vanilla CLI 0.14.2 pre-built
  disponível (DEBT-54 obsoleto sem código).
- **P206B** reactivou harness `lab/parity/` (2 fixes
  triviais).
- **P206C** materializou helper L3
  (`03_infra/src/query_helpers.rs`) + comparação
  estrutural via `typst query` JSON.
- **P206D** produziu matriz consolidada cobrindo 36
  ficheiros corpus.
- **P206E** transitou ADR-0075 PROPOSTO → ACEITE final
  + cond 9 ADR-0073 fechada retroactivamente.

### Cond 9 fechada em 2026-05-08

ADR-0073 transitou de "ACEITE estruturalmente fechado"
para **"ACEITE completo retroactivo, P206E 2026-05-08"**
per spec C3 Caminho B (fórmula intermediária honesta
face às excepções).

Análise da cond 9 em P206E (per matriz P206D):

- 4/6 ficheiros introspection P204F com matches limpos
  (counter-heading, figure-ref, query-metadata,
  equation-ref).
- 2/6 com excepções documentadas:
  - `outline-toc` heading count diff (design
    intencional cristalino P200B; não regressão M8).
  - `cite-bibliography` eval fail (bibliography stdlib
    gap pre-P206; não regressão M8).

DEBTs colaterais fechadas em P206E:
- **DEBT-53** → ENCERRADO (vanilla integration
  materializada).
- **DEBT-54** → ENCERRADO (workspace setup obsoleto
  via vanilla CLI pre-built).

### Cross-references retroactivas

- ADR-0073 §"Fecho retroactivo cond 9 — P206E"
  (anotação no início da ADR).
- ADR-0075 (ACEITE final P206E 2026-05-08).
- `00_nucleo/materialization/typst-passo-206-relatorio-consolidado.md`.
- `00_nucleo/diagnosticos/typst-passo-206E-inventario.md`.

### Preservação histórica

§1–§13 acima **mantêm-se inalteradas** — reflectem
estado real em 2026-05-07. Auditor futuro lê secções
em ordem cronológica:
- §1–§13: trajectória M8 P204A-H 2026-05-07.
- §14 (este bloco): fecho retroactivo cond 9 via
  série P206 2026-05-08.

Pattern reaproveitável: futuras transições
retroactivas que afectem ADRs ou consolidados de séries
anteriores devem usar anotação cirúrgica (não
reescrita) per P201/P202/P206E precedent.
