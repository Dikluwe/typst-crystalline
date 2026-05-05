# Passo 192A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace ~1.802 verdes; zero violations.
- M9 ✅ 11/11.
- **M5 universal completo** (P200B).
- **M6 fechado completo** (P190I) — `CounterStateLegacy`
  eliminado.
- ADR-0070 ACEITE (P190I).
- ADR-0071 ACEITE (P191C — walk pipeline com
  `&mut TagIntrospector`).
- Trait `Introspector`: 20 métodos.
- `TagIntrospector`: 9 sub-stores.
- `LayouterRuntimeState`: 3 fields.
- 5 ADRs ACEITES no ciclo M5/M6.

**Material de partida** verificado:
- `00_nucleo/materialization/typst-passo-190-relatorio-consolidado.md`
  — M6 fechado.
- `00_nucleo/materialization/typst-passo-191-relatorio-consolidado.md`
  §4.1 — `apply_state_funcs` slim post-pass para Func
  eval em fixpoint, **chamada apenas em
  `fixpoint::run_fixpoint`** (mencionado como
  existente).

P192A é **diagnóstico de M7** (loop fixpoint runtime).
Verifica empiricamente se M7:
- **Estado A**: está completo (run_fixpoint + queries
  runtime location-aware funcionais; benchmarks
  estáveis).
- **Estado B**: está parcial (esqueleto existe; falta
  funcionalidade ou validação).
- **Estado C**: precisa de implementação substancial.

P192A é passo **L0-puro / diagnóstico-primeiro**.
Magnitude esperada **S-M**. Sub-passos subsequentes
(P192B+) dependem criticamente do estado descoberto.

**Particularidade**: diferente de P190A/P191A que
auditavam para planear trabalho futuro, P192A audita
**estado actual** que pode já estar fechado. Resultado
possível: M7 já está fechado e P192 série tem 1
sub-passo apenas (declaração formal + validação +
relatório).

---

## Postura do auditor / executor

P192A é passo **L0-puro / diagnóstico-primeiro**,
padrão estabelecido em 32 aplicações.

- **Zero código tocado** em camadas cristalinas.
- **Zero testes** modificados.
- **Pode criar** ADR `PROPOSTO` se decisão arquitectural
  emergir.
- **Pode declarar M7 fechado formalmente** se Estado A
  confirmado.
- **Pode abrir DEBT** se trabalho identificado for
  adiado.

**Magnitude diagnóstico**: S-M. Decisões expandidas
porque:
- Estado actual ambíguo.
- M7 envolve loop fixpoint + queries runtime
  location-aware (semântica complexa).
- Pode requerer benchmarks empíricos para validação.

---

## Escopo

**Primário**: auditar empiricamente estado de M7.

**Decisões a tomar** — 7 cláusulas:

1. **Estado de `fixpoint.rs`**:
   - Localização exacta.
   - Funções públicas (`run_fixpoint`, etc.).
   - Caller(s).

2. **Estado do loop fixpoint**:
   - Iteração até convergência implementada?
   - Critério de convergência (`Introspector` igual
     entre iterações)?
   - Limite máximo de iterações?

3. **Estado de queries runtime location-aware**:
   - Layouter consumers usam queries Introspector
     location-aware durante layout?
   - Queries activas: `is_numbering_active_at`,
     `flat_counter_at`, `formatted_counter_at`,
     `figure_number_at_index`, etc.

4. **Validação empírica**:
   - Tests E2E que exercitam loop fixpoint?
   - Tests que validam convergência?
   - Tests benchmark/performance?

5. **Comparação com vanilla typst**:
   - `lab/typst-original/` tem `Introspector::iterate`
     ou similar?
   - Estrutura paralela ou divergente?

6. **Estado do M7 (decisão final)**:
   - **Estado A**: completo. Declarar formalmente
     fechado em P192B; sem trabalho adicional.
   - **Estado B**: parcial. P192B+ implementa partes
     em falta.
   - **Estado C**: precisa implementação substancial.
     P192 série completa (A diagnóstico + B-D
     implementação).

7. **Critério de fecho M7**:
   - Loop fixpoint completo + queries runtime
     funcionais + tests verdes + benchmarks (se
     aplicável) + declaração formal em L0 + ADR (se
     necessário).

**Fora de escopo**:
- M8 (memoização comemo).
- F3 completo.
- Lacunas residuais.

---

## Critérios objectivos

### O1 — Inputs verificáveis

- `find 01_core/src -name "fixpoint*"` para localizar
  ficheiros.
- `grep -rn "run_fixpoint\|fixpoint" 01_core/src/`
  para callers e definição.
- `grep -rn "introspector.iterate\|Introspector::iterate\|fn iterate"
  lab/typst-original/crates/typst-library/src/introspection/`
  para comparação vanilla.
- Tests workspace: `cargo test --workspace --lib`
  baseline ~1.802.

### O2 — Alternativas

Cláusula 6 tem 3 estados (A/B/C). Demais cláusulas
são auditoria empírica directa.

### O3 — Critério de escolha

Decisão Estado A/B/C baseada em factualidade
empírica:
- Loop fixpoint implementado e exercitado por tests
  → Estado A.
- Esqueleto presente mas sem validação → Estado B.
- Ausência ou falha → Estado C.

### O4 — Magnitude

P192A diagnóstico: **S-M**.

Implementação P192B+ depende criticamente de Estado
A/B/C:
- Estado A: P192B = S puro (declaração formal +
  relatório consolidado).
- Estado B: P192B = M (implementar partes em falta).
- Estado C: P192 série = L (multi-sub-passo).

### O5 — Reversibilidade

Diagnóstico totalmente reversível (zero código
tocado).

---

## Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

P192A segue padrão diagnóstico-primeiro 32 aplicações
consecutivas. Particularidade: pode revelar que M7 já
está fechado — nesse caso série P192 reduz a 1
sub-passo declarativo.

### Q2 — Honestidade de magnitude

P192A diagnóstico é S-M. P192B+ implementação depende
empíricamente de findings.

### Q3 — Cobertura sem regressão

Diagnóstico não toca produção. Sem regressão por
construção.

### Q4 — M7 declaração

Após P192A:
- Estado A confirmado: P192B declara M7 fechado;
  série fecha em 2 sub-passos (A + B).
- Estado B/C: P192B+ executa trabalho identificado;
  série maior.

### Q5 — Granularidade

P192A é único sub-passo de diagnóstico. Sub-passos
subsequentes determinados por findings.

---

## Sub-passos de P192A

### Sub-passo 192A.A — Validação estado actual

Auditor confirma empiricamente:

#### Estado consolidado pós-P190I

1. Tests workspace verdes (~1.802 baseline).
2. Linter zero violations.
3. M5 universal completo + M6 fechado completo
   confirmados.

#### Localizar `fixpoint.rs`

4. `find 01_core/src -name "fixpoint*"`:
   - Ficheiro existe?
   - Localização exacta.
   - Tamanho aproximado (LOC).

5. Inspeccionar `fixpoint.rs`:
   - Funções públicas declaradas.
   - Tipos públicos.
   - Imports usados.

#### Inventário `run_fixpoint`

6. Localizar `run_fixpoint` (ou equivalente):
   - `grep -n "fn run_fixpoint" 01_core/src/`.
   - Signature exacta.
   - Body — loop completo ou esqueleto?

7. Identificar callers:
   - `grep -rn "run_fixpoint(" 01_core/src/`.
   - Quem chama e com que parâmetros.

#### Estado do loop fixpoint

8. Auditar implementação do loop:
   - Iteração até convergência?
   - Critério de comparação entre iterações?
   - Max iterations?
   - Recurso à `apply_state_funcs` (per P191B)?

9. Identificar pontos de divergência vs vanilla
   `Introspector::iterate`:
   - `lab/typst-original/crates/typst-library/src/introspection/introspector.rs`
     ou similar.
   - Estrutura comparada.

#### Estado de queries runtime location-aware

10. Layouter usa queries location-aware durante
    layout?
    - `grep -rn "is_numbering_active_at\|flat_counter_at\|formatted_counter_at\|figure_number_at_index"
      01_core/src/rules/layout/`.
    - Quais consumers Layouter chamam estas
      queries.

11. Confirmar que queries dependem de
    `current_location` ou similar populated durante
    layout.

#### Tests E2E loop fixpoint

12. `grep -rn "fixpoint\|iterate" 01_core/src/**/*test*`.
13. Tests que exercitam scenarios com TOC
    (caso típico que requer loop fixpoint —
    page numbers reverse-dependent).
14. Tests de convergência explícita (se houver).

#### Comparação vanilla typst

15. `find lab/typst-original -name "introspector.rs"`.
16. Identificar `Introspector::iterate` ou método
    equivalente.
17. Comparar estrutura: cristalino tem implementação
    paralela? Divergente?

#### Decisão de estado

18. Materializar decisão Estado A/B/C com base em
    factualidade empírica:
    - Estado A: loop fixpoint completo + tests +
      queries runtime activas.
    - Estado B: esqueleto presente; algo em falta.
    - Estado C: ausência substancial.

#### L0 alvos

19. Identificar L0s relevantes:
    - `rules/layout/fixpoint.md` (se existir).
    - L0 master sobre M7.
    - Referências em outras L0s.

Output: tabela com item + estado verificado +
inventário completo.

**Critério de saída**:
- `fixpoint.rs` localizado.
- `run_fixpoint` signature e body inventariados.
- Loop fixpoint estado verificado.
- Queries runtime location-aware estado verificado.
- Tests E2E inventariados.
- Comparação vanilla feita.
- Decisão Estado A/B/C materializada com
  factualidade.

### Sub-passo 192A.B — Decisão cláusulas 1–5

Conforme `.A`:

1. Estado de `fixpoint.rs` registado.
2. Estado do loop fixpoint registado.
3. Estado de queries runtime registado.
4. Validação empírica registada.
5. Comparação vanilla registada.

Output: 5 cláusulas fechadas com factualidade.

### Sub-passo 192A.C — Decisão cláusula 6 (Estado A/B/C)

Decisão final baseada em `.A.18`:

- **Estado A** (completo): P192 série = 2 sub-passos
  (A + B declaração formal).
- **Estado B** (parcial): P192 série = 3-5 sub-passos
  (A + B-D implementação parcial + E declaração).
- **Estado C** (substancial): P192 série = 5+
  sub-passos (A + B-G implementação completa +
  declaração).

Output: estado decidido + plano de sub-passos
correspondente.

### Sub-passo 192A.D — Decisão cláusula 7 (critério de fecho M7)

Critério literal verificável:
- Loop fixpoint funcional (iteração até convergência
  ou max).
- Queries runtime location-aware activas em
  Layouter.
- Tests E2E exercitam scenarios que requerem loop
  (TOC, reverse refs, etc.).
- Benchmarks (se aplicável) confirmam estabilidade.
- L0 documenta M7 fechado.
- (Eventual) ADR-0072 PROPOSTO se decisão
  arquitectural emergir.

Output: critério literal.

### Sub-passo 192A.E — Validação do plano de sub-passos

Plano dependente de Estado A/B/C:

**Plano para Estado A** (provável):

| Sub-passo | Escopo | Magnitude |
|---|---|---|
| `.B` | Validação empírica final (benchmarks se aplicável) + relatório consolidado P192 + declaração formal M7 fechado em L0 + ADR (se necessário) | S-M |

Total: 2 sub-passos (A + B). Magnitude S-M agregada.

**Plano para Estado B**:

| Sub-passo | Escopo | Magnitude |
|---|---|---|
| `.B` | Implementar partes em falta | M |
| `.C` | Validação + relatório + declaração | S |

Total: 3 sub-passos. Magnitude M agregada.

**Plano para Estado C**:

| Sub-passo | Escopo | Magnitude |
|---|---|---|
| `.B` | Esqueleto loop fixpoint | M |
| `.C` | Queries runtime location-aware | M |
| `.D` | Tests E2E + validação | M |
| `.E` | Relatório consolidado + declaração | S |

Total: 5 sub-passos. Magnitude L agregada.

Output: plano correspondente ao Estado materializado.

### Sub-passo 192A.F — ADR

Avaliar:

- Se M7 já estava implementado **sem ADR explícita**:
  considerar ADR-0072 retrospectiva para formalizar
  decisão.
- Se M7 precisa de implementação substancial:
  ADR-0072 PROPOSTO esperada.
- Se M7 está fechado e ADR existente cobre: sem
  ADR nova.

Output: decisão ADR fixada.

### Sub-passo 192A.G — DEBT

Avaliar:

- Defers identificados durante diagnóstico.
- Trabalho ortogonal (refactor Layouter F3 completo,
  etc.) — fora de escopo.

Output: estado actualizado.

### Sub-passo 192A.H — Outputs

Produzir 2-3 ficheiros (padrão):

1. **`00_nucleo/diagnosticos/diagnostico-m7-passo-192a.md`**
   — diagnóstico com 7-9 secções:
   - §1 Validação estado actual.
   - §2 Inventário `fixpoint.rs`.
   - §3 Inventário `run_fixpoint`.
   - §4 Estado loop fixpoint.
   - §5 Estado queries runtime.
   - §6 Inventário tests + comparação vanilla.
   - §7 Decisão Estado A/B/C + plano sub-passos.
   - §8 ADR + DEBT avaliação.
   - §9 Próximo sub-passo concreto.

2. **`00_nucleo/materialization/typst-passo-192a-relatorio.md`**
   — relatório com 14 secções padrão.

3. **(Eventualmente) ADR-0072 PROPOSTO** — se cláusula
   F decidir.

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora
  de `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não modificar `fixpoint.rs`** — P192B+.
- **Não modificar trait `Introspector`** — P192B+
  se necessário (improvável).
- **Não modificar `TagIntrospector`** — P192B+.
- **Não modificar Layouter** — P192B+.
- **Não materializar lacunas residuais**.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como
  bandeira retórica.
- **Aplicar regra dos 2 eixos** se aplicável.
- **Reaproveitar pattern ADR-0069 + ADR-0071** se
  aplicável.
- **Sem cláusulas condicionais nos sub-passos `.B`+
  do plano** — mas plano tem ramificação por Estado
  A/B/C.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-m7-passo-192a.md`
  com 7-9 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-192a-relatorio.md`
  com 14 secções produzido.
- 7 cláusulas fechadas com decisão literal.
- Estado A/B/C materializado empíricamente.
- Plano de sub-passos correspondente.
- Magnitude consolidada confirmada
  (S-M para diagnóstico).
- Critério de fecho M7 fixado.
- ADR avaliada (esperado: ADR-0072 PROPOSTO se
  Estado B/C; não ADR se Estado A com ADR existente).
- DEBT estado registado.
- Comparação vanilla typst feita.
- Regra dos 2 eixos aplicada se aplicável.
- Nenhum ficheiro de cristalino tocado.
- Tests workspace inalterados.
- `crystalline-lint .` zero violations.

P192A é instrumento. **Resultado abre o caminho**:
- Estado A: P192B declaração formal + série fecha
  em 2 sub-passos.
- Estado B: P192B-C implementação parcial.
- Estado C: P192B-E implementação substancial.

**Particularidade desta auditoria**: pode revelar que
**M7 já está fechado**. Sendo assim, série P192 é
curta (2 sub-passos: diagnóstico + declaração formal).
Pattern arquitectural reaproveitável — auditoria
diagnóstico-primeiro sobre estado existente em vez
de planeamento de trabalho futuro.

**Risco arquitectural baixo**: P192A é totalmente
reversível (zero código tocado). Decisão Estado A/B/C
guia trabalho subsequente sem comprometer estado
actual.
