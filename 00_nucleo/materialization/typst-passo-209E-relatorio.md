# Relatório do passo P209E

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-209E.md`.
**Tipo**: encerramento série + decisão de transição ADR-0077.
**Magnitude planeada**: S (~30min-1h) documental puro.
**Magnitude real**: S (~30min).
**Marco**: M9c (encerramento série P209; M9c continua com P210+).

---

## §1 O que foi feito

Encerramento da série P209 (5 sub-passos materializados:
A + B + C + D + E). 2 decisões fixadas empíricamente em C1
+ executadas em C2: **C1.1 = Caminho A** (ADR-0077 PROPOSTO
→ ACEITE após verificação dos 8 critérios §Plano de validação);
**C1.2 = Caminho 1** (encerramento documental puro — Caminho
1 anti-inflação **7ª aplicação consecutiva**). Zero código
tocado; só ADR-0076, ADR-0077 e blueprint anotados.

---

## §2 Decisões fixadas em C1 — evidência empírica

### C1.1 = Caminho A — ADR-0077 transição PROPOSTO → ACEITE

**Verificação dos 8 critérios §Plano de validação ADR-0077**:

| # | Critério | Estado |
|---|----------|--------|
| 1 | `regex` em `crystalline.toml:64` allowlist | ✓ (P209D C3) |
| 2 | `regex` em `Cargo.toml` `[workspace.dependencies]` | ✓ (P209D C3) |
| 3 | `regex` em `01_core/Cargo.toml` `[dependencies]` | ✓ (P209D C3) |
| 4 | `01_core/src/entities/regex.rs` existe com Hash/Eq manuais | ✓ (P209D C2) |
| 5 | `Selector::Regex(Regex)` variant existe | ✓ (P209D C5) |
| 6 | `cargo build --workspace` verde | ✓ (P209D verify) |
| 7 | `crystalline-lint` 0 violations | ✓ (P209D verify) |
| 8 | Tests `regex_*` (6+) em `entities::regex` verdes | ✓ (7 tests P209D) |

Todos 8 satisfeitos. ADR-0077 é dep-específica (cobre `regex`
em allowlist + wrapper L1) e **independente de ADR-0076**
(escopo distinto — não cobre marco M9c inteiro). Sem
dependência futura que possa invalidar. Caminho A fixado.

### C1.2 = Caminho 1 — encerramento documental puro

Re-grep `native_regex`/`Value::Regex` em production:

- Production matches: **zero**.
- Tests-only: confinados a P209D selector tests + stdlib mod
  query stub tests.

Roadmap M9c pendente:
- **P210** — Counter/State extras (se Q1=β reabrir; humano
  fixou Q1=β = "manter forma minimal"). Não invoca regex.
- **P211** — Outline configurável (se aplicável). Não invoca
  regex.
- **P212** — encerramento M9c. Trabalho documental,
  não material.

Nenhum sub-passo M9c pendente desbloqueia consumer
imediato de `native_regex`. Caminho 1 fixado — **7ª aplicação
consecutiva** do pattern anti-inflação (P205D, P207E, P208B
C1, P208D, P209C-vazios, P209D C6, P209E C1.2).

---

## §3 Alterações documentais

**Caminho 1 = zero código tocado.**

| Ficheiro | Edição |
|----------|--------|
| `00_nucleo/adr/typst-adr-0077-regex-l1.md` | Status **PROPOSTO → ACEITE** 2026-05-12; Data actualizada; Histórico +1 linha registando transição P209E + verificação empírica. |
| `00_nucleo/adr/typst-adr-0076-introspector-completion.md` | §Plano de materialização: série P209 transita "EM CURSO" → "✅ MATERIALIZADO 2026-05-12"; P209E anotado com Caminho A + Caminho 1; bloco "Agregado série P209" adicionado com sumário 5 sub-passos + métricas Δ série + 2 patterns emergentes formalizados. |
| `00_nucleo/diagnosticos/blueprint-projecto.md` | §3.0sexies Marca de actualização adicionada (paralelo a §3.0quinquies [P208D]): regista série P209 fechada + ADR-0077 ACEITE + pattern Caminho 1 7ª aplicação + limitações herdadas (`Regex` query stub; `And/Or` vazios; And/Or Rust-only). |
| `00_nucleo/materialization/typst-passo-209E-relatorio.md` | Este ficheiro (novo). |

---

## §4 Decisões substantivas

- **Caminho A para ADR-0077** (preferida a Caminho B "manter
  PROPOSTO até P212"): ADR-0077 é dep-específica e
  independente; critério §4 verificado empíricamente em
  P209D. ADR-0023/0024 (deps anteriores L1) seguiram pattern
  similar (transição rápida pós-materialização). Manter
  PROPOSTO seria espera artificial sem critério adicional.
- **Caminho 1 puro** (preferida a Caminho 2 reabrir
  `native_regex` Opção α): zero consumers production + P210/
  P211 não desbloqueiam consumer + pattern anti-inflação
  consolidado em 7 aplicações. Per `P205A.div-1` —
  divergências arquitectónicas legítimas; cristalino single-pass
  + paridade Rust API only para regex é válido.
- **ADR-0076 mantém PROPOSTO**: independente da decisão
  ADR-0077. ADR-0076 cobre marco M9c inteiro; transição
  PROPOSTO → ACEITE fica para P212 quando 7 condições do
  ADR forem verificadas (per §Plano de validação ADR-0076).
- **Marca blueprint §3.0sexies**: 6ª marca cirúrgica do
  pattern P204H/P205E/P206E/P207E/P208D. Documenta
  encerramento série P209 + ADR-0077 ACEITE sem reescrita
  ampla.

---

## §5 Métricas

| Métrica | Antes (P209D) | Depois (P209E) | Δ |
|---------|---------------|----------------|---|
| Trait `Introspector` métodos | 26 | 26 | 0 |
| `Selector` variants | 6 | 6 | 0 |
| Stdlib funcs registadas | ~52 | ~52 | 0 |
| Tests workspace | 1935 | 1935 | 0 |
| `crystalline-lint` violations | 0 | 0 | 0 |
| ADRs PROPOSTO | 2 (0076, 0077) | 1 (0076) | -1 |
| ADRs ACEITE | (várias pré-existentes) | +1 (0077) | +1 |
| L0 prompts modificados | — | 0 | — |
| L1 ficheiros modificados | — | 0 | — |
| Documentação modificada | — | 4 | +4 |

**Agregado série P209** (A diagnóstico + B + C + D + E):

| Métrica | Pré-P209 | Pós-P209E | Δ série |
|---------|----------|-----------|---------|
| Trait `Introspector` métodos | 26 | 26 | 0 |
| `Selector` variants | 1 | 6 | +5 |
| `Introspector::query` arms | 1 | 6 | +5 |
| L1 entities (sub-stores + types) | 24 | 25 | +1 (`Regex` wrapper) |
| Allowlist L1 deps externas | 11 | 12 | +1 (`regex`) |
| Stdlib helpers privados novos | — | 1 (`parse_selector_arg`) | +1 |
| ADRs novas (PROPOSTO+ACEITE) | — | 1 (ADR-0077 ACEITE) | +1 |
| Tests workspace | 1907 | 1935 | +28 |
| L0 prompts novos | — | 2 (`regex.md`, ADR-0077) | +2 |
| L0 prompts modificados | — | 2 (`selector.md` em B+C; +1 em D) | +2 |
| L1 ficheiros novos | — | 1 (`regex.rs`) | +1 |
| L1 ficheiros modificados | — | 3-4 ao longo da série | — |
| Workspace files modificados | — | 2 (`Cargo.toml`, `crystalline.toml`) | +2 |
| Caminho 1 anti-inflação aplicações | 5 (cumulativas) | 7 (cumulativas) | +2 (P209C-vazios, P209D C6, P209E C1.2) |

---

## §6 Encerramento série P209 — sumário literal

Série P209 fechou em 5 sub-passos. Pattern emergente do
projecto (P204A-H, P205A-E, P206A-E, P207A-E, P208A-D)
replicado: diagnóstico-primeiro reduzido (A) →
materialização incremental (B, C, D) → encerramento
documental (E).

| Sub-passo | Tipo | Magnitude | Output principal |
|-----------|------|-----------|------------------|
| P209A | Diagnóstico-primeiro reduzido | S-M (real ~45min) | Auditoria A1-A5 + decisões C1-C5 + plano P209B-E. 1 ficheiro `00_nucleo/diagnosticos/typst-passo-209A-relatorio.md`. Caminho B Regex full; Caminho 2 (5 sub-passos); Opção c Rust-only. |
| P209B | Variants triviais | S (real ~45min) | `Selector::Label(Label)` + `Selector::Location(Location)`. Query arms +2. Stdlib refactor: helper `parse_selector_arg`; type dispatch trinário. 8 tests. |
| P209C | Composição N-ária | S-M (real ~50min) | `Selector::And(EcoVec<Self>)` + `Selector::Or(EcoVec<Self>)`. Query arms intersecção (filter+contains) + união dedupliquada (HashSet preservando ordem). Opção A para vazios. 9 tests. |
| P209D | Regex + ADR-0077 + dep regex | M (real ~1h) | Wrapper L1 `entities::regex::Regex` + ADR-0077 PROPOSTO + `regex` em allowlist + workspace + 01_core deps. `Selector::Regex(Regex)` variant + query arm stub `vec![]`. C6 = Opção γ deferred `native_regex` (Caminho 1 anti-inflação 6ª aplicação). 11 tests. |
| P209E | Encerramento série + decisões | S (real ~30min) | C1.1 Caminho A: ADR-0077 PROPOSTO → ACEITE. C1.2 Caminho 1: encerramento documental puro (7ª aplicação anti-inflação). ADR-0076 anotado; blueprint §3.0sexies; relatório este. Zero código tocado. |

**Custo agregado real**: ~4h (estimado ~4-5h per ADR-0076).
Magnitude **M** confirmada empíricamente.

**Padrões emergentes consolidados em P209**:

1. **Caminho 1 anti-inflação 7ª aplicação consecutiva**:
   pattern operacional formalizado para materialização
   honesta sem over-engineering. Documentado em §3.0sexies.
2. **Stdlib funcs + Selector variants sem trait extension**:
   Toda a série P209 não tocou o trait `Introspector` (mantém
   26 métodos). Regra empírica P207B §5 **não acionada** em
   P208 e P209 inteiras. Consolida paridade vanilla atingida
   em P207D.
3. **Recursive Hash transparente em enum com EcoVec<Self>**:
   P209C confirmou; P209D estendeu com `Regex` wrapper +
   Hash manual via pattern. Pattern reusável para futuros
   sub-stores recursivos.

---

## §7 Próximo sub-passo

**P210 série** — Counter/State extras (se Q1=β reabrir) ou
**P211 série** — Outline configurável (se aplicável).

Per P207A C10 humano Q-decisões fixadas:
- **Q1=β** — "manter forma minimal" (Counter/State sem
  rich types). P210 série **não desbloqueia** materialização
  imediata salvo se Q1 for reaberta. Estado: provavelmente
  **P210 skip** ou minimal documental.
- **Q4=β** — `query_count_before` adiado.

**P211 série** — Outline configurável: depende de critério
empírico (consumer real `outline()` actual cobre necessidades
M9c?). Provavelmente Caminho 1 anti-inflação 8ª aplicação
se zero consumers persistir.

**P212** — encerramento M9c (auditoria 7 condições ADR-0076
§Plano de validação + transição PROPOSTO → ACEITE).
Magnitude S documental.

Estado M9c: 3 séries fechadas (P207 + P208 + P209). 2
séries possíveis remanescentes (P210/P211) — provavelmente
minimal ou skip. P212 fecha M9c.

ADR-0076 mantém `PROPOSTO`; ADR-0077 transitou **ACEITE
2026-05-12**.
