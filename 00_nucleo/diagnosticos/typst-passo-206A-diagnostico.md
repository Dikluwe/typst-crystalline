# P206A — Diagnóstico: cláusulas de decisão C1–C13

**Data**: 2026-05-07.
**Spec**: `00_nucleo/materialization/typst-passo-206A.md`.
**Output 2 de 4** (diagnóstico).
**Auditoria empírica**:
`00_nucleo/diagnosticos/typst-passo-206A-auditoria-vanilla.md`.

---

## §1 C1 — Tipo de trabalho

**Decisão fixada: Caminho A — Reactivar**.

Justificação literal (per A12 + A13 + A14):

- **Custo A**: ~1-2h. **Custo B**: ~4-6h. Diferencial 2-3×.
- 2 breaks pre-existentes em `lab/parity/` são triviais
  (1-line cada).
- 5 ficheiros src + 3 tests reusáveis (frame_dto,
  value_dto, report, etc.).
- Bin `parity-runner` já funcional (smoke test passou).
- Convenção da matriz histórica preservada (latest +
  history em `reports/`).

Caminho B (Construir) rejeitado: clean slate sem
benefício observable; reintroduz risco de regressão.

Caminho C (Híbrido) rejeitado: "fix breaks + estender"
é exactamente o que Caminho A significa; Híbrido
inflaciona vocabulário sem distinção real.

Per spec §9 risco "subestimar custo de Construir do
zero porque parece mais limpo": rejeitado por evidência
— A12 mostra reuso máximo possível com 2 fixes
triviais.

---

## §2 C2 — Escopo concreto de validação

**Decisão fixada: 36 ficheiros INCLUDE com 1 SKIP
documentado**.

Lista literal:

| # | Categoria/Ficheiro | Etiqueta | Razão |
|---|---------------------|----------|-------|
| 1-2 | code/ (let, set) | INCLUDE | Smoke test compilação |
| 3 | markup/empty.typ | INCLUDE | Edge-case válido |
| 4 | markup/error.typ | **SKIP layout** | Sintaxe inválida intencional; INCLUDE para parse-only |
| 5-9 | markup/{heading, parbreak, plain, spaces, strong} | INCLUDE | Smoke compilação |
| 10-11 | math/{block, simple} | INCLUDE | Smoke math |
| 12-21 | semantic/* (10) | INCLUDE | P2 eval; companions têm `__resultado__` extraction |
| 22-36 | visual/* (15) | INCLUDE | P3 layout + P204F introspection |

**Total**: 36 INCLUDE; 1 SKIP layout (error.typ — que
o harness existente já skipa via `if file.starts_with("error")`).

Sem DEFERRED. Todas as 36 entradas são processáveis
empíricamente — vanilla CLI compila smoke tests
confirmados.

C2 não reduz escopo nominal. Excepção error.typ é
documentada e já estabelecida pre-P206 (linha 95 do
harness existente).

---

## §3 C3 — Mecanismo de comparação observable (PDF diff)

**Decisão fixada: Mecanismo C — Sem comparação
observable** (apenas estrutural; cristalino-only para
PDF).

Justificação:

- A7 confirmou ferramentas (compare, pdftocairo)
  disponíveis.
- **Mas pixel-perfect é inviável por design**:
  cristalino usa `FixedMetrics` (ADR-0054) enquanto
  vanilla usa `FontBookMetrics` (real fonts).
  Divergência geométrica é estrutural, não regressão.
  Pixel-perfect daria ~100% divergência observable.
- Fuzzy match exigiria escolha arbitrária de tolerância
  sem benchmark de baseline. Spec §C6 fixa critério
  apenas se C3 ≠ C; este caso obriga a decidir
  threshold sem evidência.
- ADR-0054 perfil graded já documenta divergência
  geométrica como **experimental por construção** —
  paridade observable não é objectivo realista no
  estado actual.

Mecanismo A (pixel-perfect) e B (fuzzy) **rejeitados**
por inviabilidade técnica documentada.

Cristalino continua a produzir PDF para inspecção
visual humana (P3 baseline preservado), mas sem
comparação automática contra vanilla.

---

## §4 C4 — Mecanismo de comparação estrutural (queries)

**Decisão fixada: Mecanismo D — Vanilla `typst query`
+ cristalino test helper**.

Justificação (per A8):

- Smoke test confirmou:
  ```
  typst query query-metadata.typ "metadata" --field value --format json
  → ["primeiro", {"tag": "secundario", "peso": 42}, "terceiro"]
  ```
- Match exacto com `expectations.cristalino` em
  companion.
- Cristalino test helper (~30 LOC) extrai output
  análogo via `Introspector::query_metadata()` +
  serialização JSON via `serde_json`.
- Diff via `serde_json::Value` comparison — pattern
  trivial.

Mecanismo E (sem comparação estrutural) rejeitado:
contradiria propósito P206 (fechar cond 9 ADR-0073).

Mecanismo F (dual D + observable C3) reduzido a
"D + cristalino-only PDF" porque C3 = sem comparação
observable. Substituído por D simples.

---

## §5 C5 — Vanilla typst binário acesso

**Decisão fixada: Caminho b — Pre-built binário**.

Justificação (per A5 + A6):

- `/usr/local/bin/typst` versão 0.14.2 disponível;
  paridade com `lab/typst-original/crates/typst-syntax v0.14.2`.
- `lab/typst-original/Cargo.toml` workspace-level
  ausente — Caminho a (compilar na quarentena)
  exigiria criar workspace ou compilar typst-cli
  explicitamente. Magnitude desnecessária.
- Caminho c (workspace member) **rejeitado**
  explicitamente: introduziria conflitos Cargo.lock
  (vanilla declara `[workspace.dependencies]` próprias)
  e violaria CLAUDE.md "lab/ é quarentena".

Pre-built binário trata vanilla como **dependência
ambiental externa**, não código cristalino. CI exigirá
install step (`cargo install --git ... typst-cli` ou
download binário) — documentado em ADR-0075.

DEBT-54 ("vanilla workspace setup") **fecha por
irrelevância**: workspace setup não é necessário
quando binário pre-built funciona.

---

## §6 C6 — Tolerância de comparação

**N/A** — C3 = C (sem comparação observable). C6 só
aplicável se C3 = A ou B.

---

## §7 C7 — Categorias de SKIP documentado

**Decisão fixada: Lista mínima**.

Per A11 (zero gaps categóricos):

| Ficheiro | SKIP de quê | Razão | Mantém em |
|----------|--------------|-------|-----------|
| `markup/error.typ` | layout/PDF compare | Sintaxe inválida intencional | parse comparison via parity-runner |

**Sem outros SKIPs identificados**. Hipótese
"here-locate.typ SKIP" (per spec) é resolvida porque
ficheiro **não existe** no corpus actual.

Documentação: nota inline no companion `.typ.toml`
(criar para markup/error.typ se aplicável). Alternativa:
manifest dedicado `lab/parity/SKIPS.md`. Decisão
deferida para P206B (escolha entre formato).

Edge cases potenciais a verificar em P206B (não SKIP
em P206A):

- `cite-bibliography.typ` — exige `refs.yaml` no mesmo
  dir. Se cristalino não consegue carregar yaml,
  documenta como SKIP-cristalino-only ou SKIP-vanilla-only
  conforme empírico.
- `multi-font.typ` — exige fonts disponíveis. Cristalino
  FontBook stub pode não cobrir fonts vanilla.

P206B audita estes em runtime e decide.

---

## §8 C8 — Tratamento de DEBT-53/54

**Decisão fixada: Fechar DEBT-53 em P206 série única;
DEBT-54 fecha por irrelevância**.

Justificação:

- **DEBT-53** (vanilla integration bloqueada por
  DEBT-54): P206 endereça directamente em sub-passos
  P206B-D. Pre-built binário (C5=b) elimina o
  bloqueador.
- **DEBT-54** (vanilla workspace setup): A6 mostra
  que workspace setup **não é necessário**. Pre-built
  binário é a solução pragmática. DEBT-54 fecha-se
  documentando esta resolução — sem código a manter,
  sem workspace a setup.

P206E formaliza fechos:
- DEBT-53 → CLOSED-by-P206 (vanilla integration
  funcional via pre-built CLI).
- DEBT-54 → OBSOLETED-by-P206 (workspace setup
  desnecessário).

Sem séries separadas. Magnitude C10 confirma série
única é viável.

---

## §9 C9 — ADR-0075 PROPOSTO?

**Decisão fixada: SIM — ADR-0075 PROPOSTO em P206A**.

Justificação (per spec C9 critério "decisão arquitectural
com alternativas reais"):

- **Reactivar vs Construir**: alternativa real (A vs B).
- **Workspace member vs quarentena vs pre-built**:
  alternativa real (a vs b vs c).
- **Pixel-perfect vs fuzzy vs estrutural-only**:
  alternativa real (A vs B vs C).
- **Vanilla CLI dependency strategy**: ambiental vs
  bundled — decisão arquitectural relevante.

Padrão consolidado: marcos arquitectónicos com decisão
estrutural ganham ADR dedicada — ADR-0072 (M7),
ADR-0073 (M8), ADR-0074 (F3). Vanilla integration
(P206 série) é estrutural; merece ADR-0075.

Conteúdo ADR-0075 PROPOSTO em
`00_nucleo/adr/typst-adr-0075-vanilla-integration.md`
(produzido como Output 4 do passo P206A).

Plano de validação ADR-0075:
- 7 condições paralelas a ADR-0073: P206B-D
  materializados; tests verdes; lint zero; corpus 36
  cobertos; cond 9 ADR-0073 fechada via P206; etc.

---

## §10 C10 — Magnitude agregada

**Output: M-S agregado**.

Decomposição por sub-passo (estimativas):

| Sub-passo | Tipo | Magnitude | Tempo estimado |
|-----------|------|-----------|----------------|
| P206A | Diagnóstico (este) | M | ~30-45 min |
| P206B | Reactivar harness + 2 fixes triviais + smoke vanilla CLI | S | ~30-45 min |
| P206C | Comparação estrutural via typst query (cristalino + vanilla helpers) | M | ~1-1.5h |
| P206D | Cobertura corpus 36 + matriz consolidada + sentinelas | S-M | ~45-60 min |
| P206E | Encerramento + ADR-0075 ACEITE + cond 9 ADR-0073 fechada + DEBT-53/54 fechos | S documental | ~30-45 min |

**Total série**: ~3.5-5h. Magnitude **M agregado**.

Hipótese específica testada: A12 + A13 mostraram custo
viável; C10 não obriga sub-séries. Série única P206A-E
é suficiente.

Per spec C10 hipótese: "se A12 ou A13 mostrarem custo
XL, C10 obriga a sub-séries (P206/P207)". Empírico:
custos S-M; **série única é a forma correcta**.

---

## §11 C11 — Sub-passos `*B+`

**Plano fixado** (4 sub-passos pós-A):

### P206B — Reactivar harness + smoke vanilla CLI

Magnitude S (~30-45 min).

- Fix `tests/layout_parity.rs:69` (1-line; remover
  `state` arg + linha `let state = introspect(...)`).
- Fix `src/value_dto.rs:83` (1-line; adicionar
  `Value::Location(_)` arm).
- Adicionar smoke test sentinel: `typst --version`
  available; abort gracefully se ausente.
- Confirmar `cargo check --manifest-path lab/parity --all-targets`
  sem erros pós-fixes.
- Confirmar `cargo test --manifest-path lab/parity`
  passa pelo menos com baseline cristalino-only
  preservado.
- Outputs: 2 ficheiros (inventário + relatório).

### P206C — Comparação estrutural via typst query

Magnitude M (~1-1.5h).

- Helper `lab/parity/src/vanilla_invoke.rs` —
  invocação vanilla CLI via `std::process::Command`.
- Helper `lab/parity/src/structural_compare.rs` —
  comparação JSON via serde_json.
- Cristalino test helper para serializar
  `Introspector::query_*` em JSON análogo.
- Verificar match em ficheiros introspection P204F (5
  inicialmente).
- Tests: 5+ unit tests para comparação estrutural.
- Outputs: 3 ficheiros (inventário + relatório +
  alterações de código).

### P206D — Cobertura corpus 36 + matriz consolidada

Magnitude S-M (~45-60 min).

- Estender `corpus_completo_p3` para incluir vanilla
  side via helper P206C.
- Adicionar `corpus_completo_p2` extension (semantic).
- Update `ParityMatrix` para colunas
  `text_content / structural` populadas (geometric
  permanece N/A).
- Render matriz consolidada para 36 entradas.
- Sentinelas: 2-3 (matriz produzida; N entradas
  cobertas; cond 9 cumprida).
- Outputs: 3 ficheiros.

### P206E — Encerramento + ADR transições + DEBT fechos

Magnitude S documental (~30-45 min).

- Auditoria das 7 condições ADR-0075.
- Forma de fecho (Completo se 7/7).
- Transições:
  - ADR-0075 PROPOSTO → ACEITE.
  - ADR-0073 "estruturalmente fechado" → "completo
    final" (cond 9 cumprida).
  - DEBT-53 → CLOSED.
  - DEBT-54 → OBSOLETED.
- Relatório consolidado da série P206A-E (paralelo a
  P204H/P205E).
- Blueprint actualizado §3.0ter [P206E] paridade
  vanilla operacional.
- Outputs: 4 ficheiros.

---

## §12 C12 — Sem cláusulas condicionais

C1–C11 fixadas com valores concretos baseados em
auditoria empírica. Sem ramos.

A possibilidade de C2 = "skip + reduzir escopo" foi
auditada em A11 e rejeitada — escopo 36 viável sem
SKIPs adicionais (1 SKIP markup/error já estabelecido
pré-P206; sem novos).

A possibilidade de C5 = compilar vanilla na quarentena
foi auditada em A5+A6 e rejeitada — pre-built
disponível.

---

## §13 C13 — Sem `P206A.div-N`

**Decisão fixada: sem divergência sobre escopo**.

Per spec C13: "se A12 + A13 + C10 mostrarem que '36
ficheiros' é inviável sem inflação XL, registar
divergência".

Empírico:
- A12 = S-M.
- A13 = M-L.
- C10 = M agregado série.

Nenhum mostra inflação XL. 36 ficheiros é viável com
custo per-ficheiro baixo (vanilla CLI + JSON diff são
operações constant-cost). Sem necessidade de reduzir
escopo a 6 ficheiros (P204F introspection inicialmente).

Pré-fixação do escopo (clarificação inicial) é
**confirmada por evidência empírica**, não acomodada
por inflação. Caminho A + C5=b + C3=C = custo viável
para 36 ficheiros.

---

## §14 Decisões durante a leitura

### D1 — Vanilla typst CLI install é divisor de águas

Esperava-se compilar vanilla typst na quarentena
(custo M-L) ou setup workspace (custo XL). Pre-built
binário em `/usr/local/bin/typst` versão certa
elimina ambos. Decisão arquitectural: tratar vanilla
CLI como dependência ambiental externa (CI exigirá
install step).

### D2 — Pixel-perfect comparison rejeitada por design

ADR-0054 perfil graded já estabeleceu que `FixedMetrics`
divergência face a `FontBookMetrics` é estrutural.
P206 não inverte isto — confirma e formaliza via C3=C.
Comparação observable seria 100% noise. Estrutural
queries são portáveis (mesma estrutura semântica
mesmo com geometria diferente).

### D3 — DEBT-54 obsoleta-se sem código

DEBT-54 ("vanilla workspace setup") era hipótese de
trabalho assumindo vanilla precisaria ser compilado
no workspace. Pre-built binário torna workspace
irrelevante. **Fechar DEBT por irrelevância é solução
honesta** — não se inflar trabalho para "fechá-lo
estruturalmente" se a hipótese inicial é obsoleta.

### D4 — `here-locate.typ` referenciado mas inexistente

Spec mencionou "P204F SKIP here-locate.typ". Empírico
mostra ficheiro **não existe** no corpus. Hipótese
spec foi precaução — não realidade. P206 não precisa
SKIP. Stdlib `here()`/`locate()` são pendência
separada (DEBT futuro), não corpus paridade.

### D5 — Companions sem `[expectations.vanilla]`

P204F adicionou `[expectations.cristalino]` em 5
ficheiros introspection. P206C pode adicionar
`[expectations.vanilla]` paralelo. **Mas** comparação
directa via `typst query` JSON em runtime dispensa
expectations explícitas — apenas precisa do JSON
output dinâmico. Decisão deferida para P206C runtime
empírico.

### D6 — Matriz histórica preservada (não reset)

Caminho A reusa `reports/{latest, history}/`. Caminho
B teria reset. **Preservação histórica** alinha com
padrão P201/P202/P204H/P205E (não reescrever histórico).
Argumento adicional contra Caminho B.

### D7 — `markup/error.typ` SKIP é pré-existente

Linha 95 do `tests/layout_parity.rs` actual:
`if file.starts_with("error") { continue; }`. SKIP
documentado pré-P206; P206 preserva. Não é nova
decisão; é continuação.

---

## §15 Resumo — métricas previstas

| Métrica | Valor |
|---------|-------|
| Caminho fixado | **A — Reactivar** |
| Mecanismo PDF | **C — sem comparação observable** |
| Mecanismo estrutural | **D — typst query JSON** |
| Vanilla acesso | **b — pre-built binário** |
| Sub-passos pós-A | **4 (B-E)** |
| Magnitude agregada | **M agregado** (~3.5-5h) |
| ADR-0075 PROPOSTO em P206A | **SIM** |
| `P206A.div-N` | **NÃO** |
| Tests workspace antes | 1860 |
| Tests workspace depois (estimativa) | 1865-1875 (∆+5 a +15) |
| LOC novas (código) estimadas | ~150-200 |
| LOC novas (docs) estimadas | ~3000+ (4 outputs P206A + outputs P206B-E) |
