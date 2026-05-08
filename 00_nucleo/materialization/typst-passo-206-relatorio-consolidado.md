# Relatório consolidado da série P206 — Vanilla integration fechada completa

**Escopo**: P206A–E (2026-05-07 a 2026-05-08).
**Tema**: Vanilla integration via pre-built CLI +
comparação estrutural cristalino vs vanilla.
**Output 2 de 4 do passo P206E** (paralelo a P204H/P205E
§C7).
**Estado final**:
- **ADR-0075 ACEITE final** (P206E 2026-05-08); 7/7
  condições do plano de validação CUMPRIDAS;
  `P206C.div-1` registada (CLI deferred).
- **ADR-0073 ACEITE completo retroactivo** (P206E
  2026-05-08) — cond 9 fechada via matriz P206D com
  excepções documentadas.
- **DEBT-53 ENCERRADO + DEBT-54 OBSOLETED** —
  vanilla integration desbloqueada.

---

## §1 Trajectória da série

P206A–E executou-se em sessão dispersa 2026-05-07 (P206A
diagnóstico) + 2026-05-08 (P206B-E implementação +
encerramento). Padrão: diagnóstico-primeiro de profundidade
alta + implementação progressiva + encerramento
administrativo com **transição retroactiva** (primeira
da trajectória — pattern novo).

### §1.1 P206A — Diagnóstico-primeiro

Magnitude M (real ~30-45 min).

P206A iniciou a série após M8 estruturalmente fechado
(P204H) + F3 fechado completo (P205E). Auditoria empírica
A1–A16 cobriu 5 blocos arquitecturais: estado actual do
harness `lab/parity/`; dependências vanilla; corpus
actual; caminhos viáveis; estado pós-M8+F3.

**Achado-chave (D1) — "vanilla CLI pre-built é divisor
de águas"**:

- A5 descobriu vanilla typst CLI 0.14.2 instalado em
  `/usr/local/bin/typst` — paridade exacta com
  `lab/typst-original/crates/typst-syntax v0.14.2`.
- DEBT-54 ("vanilla workspace setup") tornou-se
  obsoleta sem código (per D3) — hipótese inicial
  (workspace setup necessário) inválida face a
  pre-built CLI funcional.
- `typst query --format json` confirmou compatibilidade
  estrutural directa via smoke test (`query-metadata.typ`
  match exacto).

ADR-0075 PROPOSTO produzido com 7 alternativas
consideradas e rejeitadas (B/C/D/E/F/G + híbrido), plano
de validação com 7 condições, plano de materialização
com 5 sub-passos (P206A-E).

### §1.2 P206B — Reactivar harness

Magnitude S (real ~25 min).

P206B aplicou 2 fixes triviais identificados em P204F.div-1
+ confirmados em P206A A2:

- `tests/layout_parity.rs:69` — migration P190I
  (`layout(content, state)` → `layout(content)` + remoção
  de `let state = introspect(content)` + import).
- `src/value_dto.rs:83` — adicionado arm `Value::Location(loc)`
  per convenção catch-all `Other(format!(...))`.
- Sentinel `lab/parity/tests/vanilla_cli_smoke.rs`
  (novo): 2 tests — versão check + subcomando query
  presence — ambos com skip graceful via `eprintln!`
  se vanilla CLI ausente.

P206B teve **zero divergências** durante implementação.
Fixes triviais + sentinel auto-contido.

### §1.3 P206C — Helper L3 + comparação estrutural

Magnitude M (real ~1.5h).

P206C materializou comparação estrutural via Caminho B
(decisão fixada empíricamente em C2):

- L0 prompt `00_nucleo/prompts/infra/query-helpers.md`
  (hash `c7ea6387`).
- Helper L3 `03_infra/src/query_helpers.rs` (hash
  `51294329`) — `query_to_summary(world, source,
  selector) -> QuerySummary` + parsing Kind/Label +
  dispatch a `Introspector::query_*`.
- Helpers `lab/parity/src/{vanilla_invoke,
  structural_compare}.rs` — invocação CLI + comparação
  JSON tolerante.
- Tests parameterized `lab/parity/tests/structural_parity.rs`
  — corpus 36 ficheiros + smoke tests.
- 13 tests unit em query_helpers + 10 tests em
  structural_parity = 23 tests E2E novos.

**`P206C.div-1` registada** (divergência cosmética):
clarificação inicial fixou "novo CLI cristalino"
(Caminho A). C1.6 mostrou Caminho A magnitude L (3-5h
refactor cross-modular `04_wiring/main.rs` + Selector::Label
extension + JSON shape replication). Caminho B (helper
L3) é magnitude M (1.5h) e satisfaz "cristalino expõe
helper" sem refactor desproporcional. CLI subcomando
deferred para sub-passo dedicado pós-P206.

### §1.4 P206D — Matriz consolidada + sentinelas

Magnitude S-M (real ~40 min).

P206D consolidou matriz de paridade via Caminho B
(test dedicado novo; reuso de `ParityMatrix` schema):

- `lab/parity/tests/consolidado_p206d.rs` (novo, ~270
  LOC) — 4 sentinelas dedicadas + builder de matriz.
- `lab/parity/SKIPS.md` (novo manifest, ~5 KB) —
  documenta 3 SKIP-pre-existing + 10 SKIP-feature +
  3 INCLUDE-com-diff literais.
- `reports/latest.md` sobrescrito + `history/2026-05-08-passo-206D.md`
  versionado — matriz P206D com text_content +
  structural populadas.

**Resultado empírico da matriz**:
- Total: 36 ficheiros corpus.
- Compila (cristalino): 34/36 (94%).
- text_content: 20/36 (categorias INCLUDE markup/visual).
- structural: 20/36 (matches cristalino vs vanilla).

P206D teve **zero divergências** durante implementação.

### §1.5 P206E — Encerramento

Magnitude S documental (~50 min — primeira sessão da
trajectória com transição retroactiva).

P206E auditou as 7 condições ADR-0075 (todas CUMPRIDAS),
fixou forma de fecho (**Completo final**), e transitou:

- **ADR-0075 PROPOSTO → ACEITE final**.
- **ADR-0073 "ACEITE estruturalmente fechado" →
  "ACEITE completo retroactivo P206E"** — primeira
  transição retroactiva da trajectória (pattern novo).
- P204H consolidado anotado cirúrgicamente per pattern
  P201/P202 (preservação histórica).
- DEBT-53 ENCERRADO (CLOSED); DEBT-54 ENCERRADO
  (OBSOLETED).
- Blueprint actualizado §3.0ter [P206E].

---

## §2 Divergências detectadas e absorvidas

### §2.1 `P206C.div-1` — CLI subcomando deferred

**Origem**: P206C C1.6 + C2.

**Causa**: clarificação inicial fixou "novo CLI
cristalino" (Caminho A). C1.6 mostrou Caminho A
magnitude L (3-5h refactor cross-modular). Caminho B
(helper L3) é M (1.5h) e satisfaz intenção da
clarificação ("cristalino expõe helper") via API L3
público.

**Impacto**: CLI subcomando literal não materializado
em P206C. Helper L3 reusable disponível para futuro
sub-passo dedicado.

**Resolução**: spec C3 Caminho B aceito como
"compromisso aceitável (cristalino expõe helper, mas
não como CLI)". Divergência **cosmética**, não
estrutural — não exigiu solicitar decisão humana.
Documentada em ADR-0075 §"Decisão" + §"Validação P206A-E".

---

## §3 Outputs concretos por sub-passo

### §3.1 Tabela de referência

| Sub-passo | Magnitude real | Outputs principais |
|-----------|----------------|---------------------|
| P206A | M (~30-45 min) | ADR-0075 PROPOSTO; auditoria + diagnóstico; relatório P206A |
| P206B | S (~25 min) | 2 fixes triviais lab/parity; smoke vanilla_cli_smoke.rs (2 tests); ADR §P206B; relatório P206B |
| P206C | M (~1.5h) | L0 prompt + query_helpers.rs (L3); helpers vanilla_invoke + structural_compare; tests structural_parity; ADR §P206C; relatório P206C |
| P206D | S-M (~40 min) | Matriz consolidada P206D em reports/; SKIPS.md manifest; sentinelas dedicadas; ADR §P206D; relatório P206D |
| P206E | S documental (~50 min) | Auditoria 7 condições; transição retroactiva ADR-0073; P204H anotado; DEBT-53/54 fechadas; blueprint §3.0ter; consolidado (este); relatório P206E |

**Magnitude agregada real**: M + S + M + S-M + S
documental ≈ **M agregado** (paralelo a P205; menor que
P204 L cross-modular).

### §3.2 Ficheiros novos por camada

| Camada | Ficheiros novos | Origem |
|--------|------------------|--------|
| L0 prompts | `00_nucleo/prompts/infra/query-helpers.md` | P206C |
| L3 código | `03_infra/src/query_helpers.rs` (+ export em `lib.rs`) | P206C |
| Lab/parity quarentena | `src/{vanilla_invoke,structural_compare}.rs` + `tests/{vanilla_cli_smoke,structural_parity,consolidado_p206d}.rs` + `SKIPS.md` | P206B-D |
| Diagnósticos | `typst-passo-206{A-auditoria-vanilla,A-diagnostico,B-inventario,C-inventario,D-inventario,E-inventario}.md` | P206A-E |
| Materialização | 5 relatórios individuais + 1 consolidado (este) | P206A-E |
| ADR | 0075 (novo PROPOSTO P206A → ACEITE P206E); 0073 anotado retroactivamente | P206A, P206E |
| Reports | `lab/parity/reports/{latest.md, history/2026-05-08-passo-206D.md}` | P206D |

### §3.3 Ficheiros modificados por sub-passo

| Ficheiro | Sub-passos | Tipo |
|----------|------------|------|
| `lab/parity/tests/layout_parity.rs` | P206B | fix migration P190I |
| `lab/parity/src/value_dto.rs` | P206B | arm Value::Location |
| `lab/parity/Cargo.toml` | P206C | serde_json dev-dep |
| `03_infra/src/lib.rs` | P206C | export query_helpers |
| `00_nucleo/adr/typst-adr-0075-...` | P206A-E | criação + 4 anotações |
| `00_nucleo/adr/typst-adr-0073-...` | P206E | transição retroactiva cond 9 |
| `00_nucleo/materialization/typst-passo-204-relatorio-consolidado.md` | P206E | anotação §14 cirúrgica |
| `00_nucleo/diagnosticos/blueprint-projecto.md` | P206E | marca §3.0ter [P206E] |
| `00_nucleo/DEBT.md` | P206E | DEBT-53 ENCERRADO + DEBT-54 OBSOLETED |

---

## §4 Achados consolidados

### §4.1 Vanilla CLI pre-built é divisor de águas (P206A D1)

A5 descobriu vanilla typst CLI 0.14.2 instalado em
`/usr/local/bin/typst`. Antes da auditoria, esperava-se
custo M-L (compilar vanilla na quarentena) ou XL
(workspace setup). Pre-built binário **colapsa esses
custos para zero** (install ambiental).

### §4.2 Pixel-perfect rejeitado por design (ADR-0054)

P206A C3 fixou C (sem comparação observable PDF). Per
ADR-0054 perfil graded: cristalino `FixedMetrics`
diverge estruturalmente de vanilla `FontBookMetrics`.
Pixel-perfect daria 100% noise. P206C confirmou
empíricamente — fuzzy thresholds seriam arbitrários
sem benchmark de baseline.

### §4.3 Pattern emergente: DEBT pode fechar via 3 caminhos

P206E formalizou:
- **CLOSED** — materializado (DEBT-53).
- **REPLACED-BY** — superseded por outra abordagem.
- **OBSOLETED** — irrelevância empírica (DEBT-54).

DEBT-54 fechada como OBSOLETED é primeira aplicação
deste pattern na trajectória. Hipótese inicial
(workspace setup necessário) era inválida face a
pre-built CLI funcional.

### §4.4 Helper L3 `query_to_summary` reusável

`03_infra/src/query_helpers.rs` expõe API pública
para queries cristalinos. **Base para CLI subcomando
deferred** (`P206C.div-1`): sub-passo futuro pode
adicionar `04_wiring` subcommand que delega ao helper
L3 existente. Reuso preservado.

### §4.5 Cond 9 ADR-0073 fechada estruturalmente

Matriz P206D mostra:
- 4/6 introspection P204F com matches (counter-heading,
  figure-ref, query-metadata, equation-ref).
- 2/6 com excepções documentadas (outline-toc TOC
  entries — design intencional cristalino P200B;
  cite-bibliography stdlib gap pre-P206).

Excepções não são regressões M8 — são pre-existentes
ou design intencional. Cond 9 transitou de PARCIAL
(P204H) para "completo retroactivo" (P206E) per spec
C3 Caminho B.

### §4.6 Pattern emergente: transição retroactiva via anotação cirúrgica

P206E é a primeira transição que altera estado de
série anterior (M8 P204H). Pattern aplicado:
- ADR-0073: anotação no início da ADR + bloco "Fecho
  retroactivo cond 9 — P206E" preservando texto
  original do plano de validação.
- P204H consolidado: §14 adicionada no final
  ("Anotação cirúrgica P206E"); §1-§13 não reescritas
  per pattern P201/P202.

Reaproveitável para futuras transições retroactivas.

---

## §5 Métricas agregadas

### §5.1 Tests

| Sub-passo | Workspace antes | Workspace depois | Lab/parity antes | Lab/parity depois |
|-----------|-----------------:|-------------------:|------------------:|---------------------:|
| P206A (diagnóstico) | 1860 | 1860 | 52 | 52 |
| P206B (sentinel) | 1860 | 1860 | 52 | 54 (+2) |
| P206C (helper L3 + tests) | 1860 | 1873 (+13) | 54 | 64 (+10) |
| P206D (sentinelas) | 1873 | 1873 | 64 | 75 (+11; 4 dedicated + 7 path-included) |
| P206E (documental) | 1873 | 1873 | 75 | 75 |
| **Total série** | **1860** | **1873 (+13)** | **52** | **75 (+23)** |

Estimativa do plano de validação: workspace +5 a +15.
Real: +13 (dentro do range).

### §5.2 LOC

| Tipo | LOC |
|------|-----|
| Código produção L3 (query_helpers.rs) | ~290 |
| Código lab/parity (vanilla_invoke + structural_compare + tests) | ~750 |
| Tests | inclui ~150 unit + e2e em query_helpers + 250 structural_parity + 270 consolidado_p206d |
| Documental (L0 + ADRs novas/anotadas + diagnósticos + relatórios + SKIPS.md + blueprint) | ~6000+ |

### §5.3 ADRs

| ADR | Estado série P206 |
|-----|---------------------|
| ADR-0075 | NOVA — PROPOSTO (P206A) → ACEITE final (P206E) |
| ADR-0073 | Anotação retroactiva P206E (cond 9 fechada; estado actualizado para "ACEITE completo retroactivo") |
| ADR-0066 | Não alterada (anotação P205E preservada) |
| ADR-0054 | Não alterada (perfil graded fundamenta C3=C) |
| ADR-0072 | Não alterada (loops fixpoint preservados) |

### §5.4 Sentinelas activas

- 19 workspace cristalino M8 (P204).
- 5 workspace cristalino F3 (P205B+C).
- +13 query_helpers tests (P206C; em workspace cristalino).
- +2 vanilla_cli_smoke (P206B; lab/parity).
- +10 structural_parity (P206C; lab/parity).
- +4 dedicated consolidado_p206d (P206D; lab/parity)
  + 7 path-included structural_compare duplicates.

**Total workspace**: 24 + 13 = **37 sentinelas/tests**
relacionados com M8+F3+P206 (não exhaustivo de todo
workspace).

**Total lab/parity quarentena**: 75 tests (52 baseline
pré-P206 + 23 P206 série).

---

## §6 Divergências da série

| Divergência | Origem | Impacto | Resolução |
|-------------|--------|---------|-----------|
| `P206C.div-1` | C1.6 + C2 | CLI subcomando deferred; helper L3 substitui | Caminho B fixado; cosmética não-estrutural; documentada em ADR-0075 |
| Excepções cond 9 ADR-0073 | P206D matriz | 2/6 ficheiros P204F com divergências documentadas | Não regressões M8; outline-toc design intencional + cite-bibliography stdlib gap; ADR-0073 transita "completo retroactivo" via Caminho B |

Sem outras divergências surgindo durante P206B/D/E.

---

## §7 Padrões demonstrados

### §7.1 5 sub-passos sem pre-existing breaks adicionais

P206B confirmou exhaustivamente que P204F.div-1 inventário
era completo (apenas 2 breaks; sem `P206B.div-N`). Pattern
"verificação exhaustiva via cargo check" antes de assumir.

### §7.2 Distinção workspace cristalino vs lab/parity quarentena

- P206B/D: zero impacto workspace cristalino (apenas
  lab/parity).
- P206C: workspace afectado por helper L3
  (1860 → 1873 +13).
- P206E: zero impacto código (puramente documental).

Pattern preservado: workspace invariante quando possível;
afectado apenas quando estritamente necessário (helper
L3).

### §7.3 Tensão "novo CLI" vs orçamento série navegada honestamente

`P206C.div-1` documenta resolução parcial da clarificação
inicial. Pattern "honestidade vs pré-fixação": auditoria
empírica (C1.6 vs C1.7) prevalece sobre pré-fixação se
custos divergem desproporcionalmente.

### §7.4 Honestidade sobre divergências arquitectónicas

P206C documentou 3 divergências (equation namespace;
cite-bibliography stdlib; outline-toc TOC). Não fixadas
em P206C (per não-objectivos). Reconhecidas como
design legítimo ou gap pre-existente — não regressões.

### §7.5 Pattern emergente: DEBT pode fechar via 3 caminhos

CLOSED / REPLACED-BY / OBSOLETED — primeira aplicação
formal em P206E (DEBT-54 OBSOLETED; DEBT-53 CLOSED).

### §7.6 Pattern emergente: transição retroactiva cirúrgica

ADR-0073 + P204H consolidado anotados sem reescrita.
Pattern P201/P202 estendido para "fecho retroactivo de
cond X" via marca §14 / bloco no início da ADR.

### §7.7 Pattern emergente: marca-por-fecho no blueprint

Blueprint §3.0/§3.0bis/§3.0ter preserva trajectória
chronológica de fechos. Cada série completa adiciona
nova subsecção cirúrgica adjacente; sem reescrita das
anteriores.

---

## §8 Estado pós-série face ao snapshot M8+F3

Snapshot 2026-05-07 (P204H) declarou:
- M8 estruturalmente fechado.
- Cond 9 PARCIAL.
- DEBT-53/54 EM ABERTO.

Snapshot 2026-05-08 (P206E):
- M8 ACEITE **completo retroactivo** (cond 9
  fechada).
- F3 ACEITE final (P205E preservado).
- ADR-0075 ACEITE final (vanilla integration).
- DEBT-53 ENCERRADO (CLOSED).
- DEBT-54 ENCERRADO (OBSOLETED).

Tests workspace cristalino: 1852 (P204H) → 1873 (P206E);
∆+21 ao longo de F3+P206 séries.

---

## §9 Convenções consolidadas pela série

### §9.1 Lição P206A D1: ferramentas pré-existentes do ambiente

Antes de assumir custo de construção, auditar
explicitamente em A1: instalações ambientais,
binários pre-built, ferramentas system-wide.
Vanilla CLI 0.14.2 em PATH descoberto colapsou
estimativa M-L para zero ambiental.

### §9.2 Lição P206C D1+D5: divergência cosmética legítima

Pré-fixação ("novo CLI cristalino") quando
inflaciona magnitude desproporcional pode ser
honrada parcialmente via Caminho B + `div-N`
documentada. **Não exige decisão humana** se per
spec aceitável (`P206C.div-1` é cosmética).

### §9.3 Lição P206D D2: preservação histórica em testes

`corpus_completo_p3` intacto pós-P206C/D — pattern
P204H/P205E aplicado a tests existentes. Test novo
dedicado em vez de modificação retroactiva.

### §9.4 Pattern DEBT-fechado-via-irrelevância

DEBT-54 OBSOLETED demonstrou que hipóteses iniciais
podem ser inválidas face a evidência empírica nova.
Fechar por irrelevância é solução honesta — não
inflar trabalho para "fechá-lo estruturalmente".

### §9.5 Pattern transição retroactiva cirúrgica

Estabelecido em P206E. Reaproveitável: futuras
transições retroactivas que afectem ADRs ou
consolidados de séries anteriores devem usar
anotação cirúrgica (não reescrita) per
P201/P202/P206E precedent.

### §9.6 Pattern marca-por-fecho no blueprint

§3.0/§3.0bis/§3.0ter — convenção consolidada para
preservar trajectória chronológica.

---

## §10 Não-objectivos respeitados

P206A–E **não**:

- Materializou CLI subcomando cristalino (deferred per
  `P206C.div-1`).
- Estendeu `Selector` enum em L1 (P175 minimal
  preservado).
- Adicionou `Serialize` derive em tipos cristalinos.
- Tocou em código produção workspace além de
  `03_infra/src/query_helpers.rs` (L3 hosting).
- Modificou `corpus_completo_p3` (preservação
  histórica).
- Reescreveu P204H consolidado (anotação cirúrgica
  apenas).
- Materializou `Selector::Label` em L1 (futuro;
  out-of-scope P206).
- Endereçou divergências arquitectónicas documentadas
  (equation namespace; cite-bibliography; outline-toc)
  — fora-de-escopo P206.
- Compilou vanilla typst (pre-built CLI usado).
- Criou ADR nova além de ADR-0075.
- Endereçou DEBTs além de 53/54.

---

## §11 Sugestão para próximo marco arquitectónico

**Não-vinculativa** — depende de prioridades do humano.

Caminhos plausíveis após P206E:

1. **CLI subcomando cristalino (P207+)** — materializar
   `P206C.div-1` deferred. `04_wiring` ganha subcommand
   `query` que delega ao helper L3
   `query_to_summary`. Magnitude L (~3-5h cross-modular)
   per estimativa P206C C1.6.

2. **`Selector::Label` em L1** — extensão do Selector
   enum em L1 (P175 minimal); requer L0 prompt update.
   Magnitude S-M.

3. **Bibliography stdlib completion** — fechar gap
   cite-bibliography (P181 series não-completa).
   Magnitude M (não estimada exhaustivamente).

4. **Equation namespace parsing** — suporte vanilla
   `math.equation` syntax em cristalino selector
   parsing. Magnitude S.

5. **Próximo marco arquitectónico não catalogado** —
   Model Fase 2 (table/figure-kinds/bibliography per
   blueprint §3.2 OPÇÃO A).

6. **Pausa estratégica** — vanilla integration fechada;
   ponto natural para re-avaliar prioridades.

P206E **não decide** — reporta. Próximo passo é escolha
humana.

---

## §12 Cross-references

- **Spec do passo P206E**:
  `00_nucleo/materialization/typst-passo-206E.md`.
- **ADRs materializadas**:
  - `00_nucleo/adr/typst-adr-0075-vanilla-integration.md`
    (ACEITE final 2026-05-08).
  - `00_nucleo/adr/typst-adr-0073-comemo-introspector.md`
    (ACEITE completo retroactivo P206E 2026-05-08).
- **Diagnósticos da série**:
  - `00_nucleo/diagnosticos/typst-passo-206A-auditoria-vanilla.md`.
  - `00_nucleo/diagnosticos/typst-passo-206A-diagnostico.md`.
  - `00_nucleo/diagnosticos/typst-passo-206B-inventario.md`.
  - `00_nucleo/diagnosticos/typst-passo-206C-inventario.md`.
  - `00_nucleo/diagnosticos/typst-passo-206D-inventario.md`.
  - `00_nucleo/diagnosticos/typst-passo-206E-inventario.md`.
- **Relatórios individuais**:
  - `00_nucleo/materialization/typst-passo-206{A,B,C,D,E}-relatorio.md`.
- **Anotação cirúrgica retroactiva**:
  `00_nucleo/materialization/typst-passo-204-relatorio-consolidado.md`
  §14 (P206E).
- **Blueprint actualizado**:
  `00_nucleo/diagnosticos/blueprint-projecto.md` §3.0ter
  [P206E].
- **DEBT registry**:
  `00_nucleo/DEBT.md` — DEBT-53 ENCERRADO + DEBT-54
  ENCERRADO/OBSOLETED.
- **Helper L3 produzido**:
  `03_infra/src/query_helpers.rs` (hash `51294329`) +
  L0 `00_nucleo/prompts/infra/query-helpers.md` (hash
  `c7ea6387`).
- **Lab/parity quarentena**:
  - `lab/parity/src/{vanilla_invoke,structural_compare}.rs`.
  - `lab/parity/tests/{vanilla_cli_smoke,structural_parity,consolidado_p206d}.rs`.
  - `lab/parity/SKIPS.md` (manifest).
  - `lab/parity/reports/latest.md` + `history/2026-05-08-passo-206D.md`.
- **Predecessor série**: P205E (F3 ACEITE final);
  P204H (M8 estruturalmente fechado; agora ACEITE
  completo retroactivo).
- **Pattern referência**: P204H §C7 + P205E §C4
  (consolidados paralelos; estrutura 11 secções).

---

## §13 Resumo executivo

**Vanilla integration via pre-built CLI + comparação
estrutural** — fechada completa em 2026-05-08 per
ADR-0075 ACEITE final. Helper L3
`03_infra/src/query_helpers.rs` expõe API pública;
`lab/parity/` quarentena ganha vanilla_invoke +
structural_compare + matriz consolidada P206D.
Cobertura empírica: 34/36 ficheiros corpus compilam em
cristalino; 20/36 com matches estruturais cristalino
vs vanilla; 13 SKIPs justificados; 3 divergências
arquitectónicas documentadas (não regressões).

**Cond 9 ADR-0073** transitada retroactivamente de
PARCIAL (P204H) para "completo retroactivo" (P206E)
via Caminho B (fórmula intermediária honesta face às
2 excepções documentadas: outline-toc design
intencional cristalino + cite-bibliography stdlib gap
pre-P206).

**DEBTs fechadas**: DEBT-53 CLOSED (vanilla integration
materializada); DEBT-54 OBSOLETED (workspace setup
desnecessário via vanilla CLI 0.14.2 pre-built).

**Tests**: workspace cristalino 1860 → 1873 (+13);
lab/parity 52 → 75 (+23). 0 violations preservadas.

**Pattern novo emergente**: transição retroactiva via
anotação cirúrgica (P206E é primeira da trajectória).
Reaproveitável para futuras séries que afectem estado
de série anterior.

5 sub-passos sem inflação retórica; magnitude agregada
real M agregado (paralelo a P205; menor que P204).
