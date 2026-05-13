# Passo 208D — Encerramento série P208

**Série**: 208 (sub-passo `D` final).
**Marco**: M9c (encerramento série P208).
**Tipo**: encerramento série + decisão de captura.
**Magnitude**: S (~30min-1h) documental puro **ou** M+
(~3-5h) com `Content::Context` block — decidido em C1.
**Pré-condição**: P208C concluído; série P208
materializada em 3 sub-passos (A diagnóstico + B `here()`
+ C `locate()`); trait 26 métodos; stdlib funcs ~52;
tests 1907 verdes; 0 violations; ADR-0076 anotado
§P208B + §P208C.
**Output**: 1 ficheiro (relatório curto encerramento +
transição P209).

---

## §1 Trabalho

Encerrar série P208. Decisão sobre **`Content::Context`
block materialização** (sub-mecanismo (ii) deferido em
P208B C1) fixada em C1 com base em evidência empírica.

Reuso de dados P208A + P208B + P208C:

- Vanilla usa `ContextElem` + show-rule deferred eval
  (sub-mecanismo ii análogo per P208B C1.1).
- Cristalino: `EvalContext.current_location` field
  populated externamente (sub-mecanismo i minimal
  fixado P208B); `here()` lê directamente; `locate()`
  delega a `Introspector::query`.
- Zero consumers production de `here()`/`locate()`
  identificados em P208B C1.3 + P208C.
- Limitação herdada: `locate(<label>)` exige P209
  (`Selector::Label`).

---

## §2 Cláusulas (4)

### C1 — Diagnóstico breve: decisão sobre `Content::Context`

Antes de tocar código, inventário focado em **3
sub-secções**:

1. **Consumers reais emergentes**: re-grep production
   de `here()` + `locate()` para confirmar zero
   consumers persistente (esperado per P208B+C). Se
   algum consumer emergiu durante P208B+C,
   materialização de `Content::Context` justifica-se.
2. **Custo empírico de `Content::Context`**:
   - Novo variant em `Content` enum.
   - Hash impl manual (per P204B/P204D pattern).
   - Show-rule equivalente em cristalino.
   - Re-entry eval em layout time.
   - Walk advance de `current_location` durante essa
     re-entry.
   - Estimativa: **M+ (~3-5h)**.
3. **Roadmap M9c pendente**: P209 (Selector
   extensions), P210 (PageStore captura ou Counter/State
   extras), P211 (Outline configurável), P212
   (encerramento M9c). Há razão para acelerar
   `Content::Context` ou esperar consumer real?

Critério literal para C2:

- **Caminho 1 — encerramento documental puro** (~30min):
  se C1.1 confirmar zero consumers persistente E C1.2
  estimar custo M+ E C1.3 mostrar P209+ não desbloqueia
  consumer imediato. Pattern P205D/P207E "Caminho 1
  anti-inflação" aplica-se.
- **Caminho 2 — encerramento + `Content::Context`**
  (~3-5h): se algum consumer emergiu ou se P209
  Selector extensions exigem composição com `here()`
  via show-rule deferred eval (`Selector::Before/After`
  estaria nesta categoria mas foi adiado per
  `P207A.div-1`).
- **Caminho 3 — captura parcial**: zero precedente; só
  registar se C1 revelar opção intermediária real.

C1 fixa **uma**.

### C2 — Materializar caminho fixado em C1

Se Caminho 1 (puro): saltar C3 implementação; ir a C4
encerramento ADR.

Se Caminho 2 (com `Content::Context`):

- L0 update prompts relevantes (Content enum;
  show-rule pattern).
- L1 novo variant `Content::Context { body, ... }`.
- Hash impl manual (regra cristalina P204B/P204D).
- Show-rule + re-entry eval.
- Walk advance de `current_location` durante
  re-entry.
- Tests dedicados (~5-8 E2E).

### C3 — Tests (se C2 ≠ Caminho 1)

Apenas se Caminho 2:

- `p208d_context_block_here_retorna_loc_real` — `#context { here() }`
  dentro de heading retorna Location do heading.
- `p208d_context_block_locate_composicao` — `#context { locate(heading) }`
  composição funciona.
- 2-3 tests de regressão (Caminho 1 estado preservado).

Se Caminho 1: zero tests novos.

### C4 — Encerramento série P208

Independente de C2:

**ADR-0076 §Plano de materialização**:
- §P208 transita "EM CURSO" → "✅ MATERIALIZADO ({data})".
- §P208D anotado com forma fixada em C1.
- Bloco "Série P208 — encerrada" adicionado com sumário
  literal dos 4 sub-passos (A + B + C + D) + métricas
  agregadas.

**Blueprint**:
- Marca `§3.0quinquies [P208D]` adicionada adjacente a
  `§3.0quater [P207E]` per pattern marca-por-fecho.

```
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: tests verdes (1907 + N onde N depende de C2);
0 violations.

---

## §3 Output

1 ficheiro:
`00_nucleo/materialization/typst-passo-208D-relatorio.md`.

Estrutura (~4-6 KB) com 7 §s padrão:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Decisão de captura fixada (Caminho 1/2) com
  evidência empírica.
- §3 Alterações em código (se C2 ≠ Caminho 1).
- §4 Decisões substantivas.
- §5 Métricas (tabela compacta).
- §6 Encerramento série P208 (resumo 4 sub-passos +
  agregado).
- §7 Próximo sub-passo (P209A).

---

## §4 Não-objectivos

- `Selector::Label/And/Or/Regex/Location` (P209).
- Outline configurável (P211).
- Counter/State extras Q1β (P210+).
- Encerramento M9c inteiro — apenas série P208 fecha;
  M9c continua com P209+.
- Transição ADR-0076 PROPOSTO → ACEITE (P212).
- Page-aware captura (deferred per P207E Caminho 1).

---

## §5 Riscos a evitar

1. **Materializar `Content::Context` sem consumer**: se
   C1.1 mostrar zero E C1.2 mostrar M+, Caminho 1 é
   honesto. Pattern emergente P205D/P207E "Caminho 1
   anti-inflação" — 4ª aplicação se P208D fixar 1.
2. **Inflar encerramento série**: P208D é encerramento
   série, não consolidado M9c. Sumário literal 4
   sub-passos.
3. **Confundir encerramento série vs marco**: P208D
   fecha série P208; M9c continua. ADR-0076 mantém
   PROPOSTO até P212.
4. **Esquecer marca blueprint**: pattern §3.0/3.0bis/
   3.0ter/3.0quater/3.0quinquies — P208D adiciona
   §3.0quinquies.
5. **Forçar Caminho 2 por "completude vanilla"**: per
   `P205A.div-1` divergência arquitectónica legítima.
   Cristalino single-pass justifica forma distinta.
   Vanilla `ContextElem`+show-rule é específico do
   multi-pass; cristalino setter-based (P208B C2) é
   válido.

---

## §6 Hipótese provável

C1 fixará **Caminho 1** porque:

- Zero consumers persistente (per P208B/C C1.3).
- P209 Q-decisions (Q2=γ adiar Where; Q3=α Regex+Location
  só) confirmam que `Selector::Before/After` (que
  beneficiariam de `Content::Context`) **não estão no
  roadmap M9c**.
- Custo M+ sem benefício observable.

Pattern emergente "Caminho 1 anti-inflação" replicado:

- P205D — `SealedLabelPages` deferred.
- P207E — captura page-meta deferred.
- P208B C1 (parcial) — sub-mecanismo (i) minimal vs
  (ii)/(iii).
- **P208D (provável)** — `Content::Context` block
  deferred.

Mas é hipótese, não decisão. C1 fixa-se empíricamente.
