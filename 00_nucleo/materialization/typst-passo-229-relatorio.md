# Relatório do passo P229 — Promoção ADR-0080 PROPOSTO → EM VIGOR (passo administrativo XS dedicado pós-N=9 validação)

**Data**: 2026-05-13.
**Spec**: `00_nucleo/materialization/typst-passo-229.md`.
**Tipo**: passo administrativo XS puro — zero código tocado;
transição status ADR-0080 PROPOSTO → EM VIGOR + anotações
cumulativas (N=8/N=9 P227+P228).
**Magnitude planeada**: XS (~30min). **Magnitude real**: XS
(~15min).
**Marco**: **primeira promoção formal pós-M9c de ADR meta
documental cristalino**; pattern emergente "passo administrativo
XS dedicado para promoção formal pós-N≥critério" N=1
inaugurado P229 (paridade estrutural P160A).

---

## §1 O que foi feito

P229 promove ADR-0080 "L0 minimal para refactors aditivos
pós-M9c" PROPOSTO → **EM VIGOR** via satisfação dupla dos
critérios §"Promoção" criados P226 (N=9 cumulativo atingido +
passo administrativo XS dedicado fixado humano).

**Ficheiros editados**: 2 — ADR-0080 (status; tabela +2
entradas P227+P228; §"Promoção" preservada [HISTÓRICO P226];
§"Promoção executada P229" novo bloco) + README ADRs
(distribuição PROPOSTO 13 → 12; EM VIGOR +1 → 29).

**Tests workspace 2071 verdes preservados**; 0 violations;
"Nothing to fix" hashes.

---

## §2 Auditoria pré-P229 + audit N=9 cumulativo (C1)

**Audit empírico**:
- `cargo test --workspace`: **2071 verdes preservados** ✓.
- `crystalline-lint .`: **0 violations** ✓.
- ADR-0080 status `PROPOSTO` ✓ (pré-transição confirmada).
- ADR-0033 status `EM VIGOR` ✓ (precedente meta cristalino).
- ADR-0034 status `EM VIGOR` ✓ (precedente meta cristalino).
- ADR-0065 status `EM VIGOR` ✓ (precedente meta cristalino).

**Audit pattern N=9 cumulativo confirmado**:
- P217 (Columns) + P218 (native_columns) + P219 (Layouter
  arm) + P220 (Colbreak agregado) + P222 (native_measure) +
  P223 (Place +float+clearance) + P224 (Grid substantivo)
  + P227 (Grid+Table +stroke) + P228 (Grid+Table +fill)
  = **N=9 cumulativo**.

**Critério §"Promoção" satisfeito duplamente**:
- ✓ N=8+ aplicação cumulativa atingido (N=9 ultrapassa).
- ✓ Sem decisão explícita contrária em 9 aplicações.
- ✓ Passo administrativo XS dedicado fixado humano.

Sem `P229.div-N`.

---

## §3 ADR-0080 transição PROPOSTO → EM VIGOR (C2)

**1. Header status**:
```
**Status**: `EM VIGOR`
**Data**: 2026-05-13 (PROPOSTO P226; EM VIGOR P229)
**Validado**: 9 aplicações cumulativas pós-M9c
(P217+P218+P219+P220+P222+P223+P224+P227+P228; N=9 patamar
empírico extremamente sólido).
```

**2. §"Aplicações cumulativas" expandida** (7 → 9 entradas):
- Header `## 7 aplicações cumulativas` → `## 9 aplicações
  cumulativas`.
- +2 entradas tabela:
  - **P227**: Grid+Table +stroke field; Value::Stroke variant
    novo; native_stroke constructor; renderização Opção β
    — "N=7 → 8 primeira validação real pós-formalização".
  - **P228**: Grid+Table +fill field; sem Value variant +
    sem constructor stdlib (anti-inflação) — "N=8 → 9
    segunda validação real; promoção EM VIGOR P229".

**3. §"Promoção" preservada [HISTÓRICO P226]** — paridade
pattern P204H+ "histórico textual preservado"; marker
[HISTÓRICO P226] adicionado ao título.

**4. §"Promoção executada — P229 (2026-05-13)"** novo bloco
adicionado após §"Promoção" histórica documentando:
- Critério satisfeito (3 ✓): N=9; sem decisão contrária;
  passo administrativo XS humano.
- Trajectória cumulativa P226 → P227 → P228 → P229.
- Pattern emergente "L0 minimal" formalizado EM VIGOR.
- 7 patterns emergentes secundários preservados como
  candidatos a ADRs meta separadas futuras.
- Status pós-P229: `EM VIGOR` literal.

---

## §4 README ADRs distribuição actualizada (C3)

`00_nucleo/adr/README.md` editado:

- **PROPOSTO**: `13` → **`12`** (-1: ADR-0080 transita).
  Lista actualizada: 0005, 0006, 0008-0015, 0062, 0066,
  0079 (ADR-0080 removido).
- **EM VIGOR**: `28` → **`29`** (+1: ADR-0080 transita).
  Lista actualizada inclui 0080 P229.
- **IMPLEMENTADO**: 21 preservado.
- **Total**: 67 preservado.

---

## §5 Resultados verificação (C4)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo test --workspace` | 2071 preservado | **2071 verdes** ✓ (zero código tocado) |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | **"Nothing to fix"** ✓ (L0 não tocado validação literal) |
| ADR-0080 status `EM VIGOR` | sim | ✓ |
| Tabela aplicações expandida +2 | sim | ✓ (P227 + P228) |
| §"Promoção" preservada [HISTÓRICO P226] | sim | ✓ |
| §"Promoção executada P229" novo bloco | sim | ✓ |
| README distribuição (PROPOSTO 12; EM VIGOR 29) | sim | ✓ |
| Sem marca cirúrgica blueprint | sim (anti-inflação Decisão 6) | ✓ (skip empírico) |
| Sem footnote inventário 148 | sim (skip empírico Decisão 7) | ✓ |
| Sem código tocado | sim | ✓ |
| Sem L0 prompts alterados | sim (validação literal!) | ✓ |

**11 critérios aceitação satisfeitos integralmente**.

---

## §6 Próximo trabalho

P229 fecha "ciclo PROPOSTO → EM VIGOR" para pattern ADR-0080
antes de mais materialização Fase 5. Decisão humana sobre
próxima sessão:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **A.3 per-cell** | GridCell +stroke/+fill per-cell + precedence rules | M (~2-3h) | alta (continuação Categoria A; valida pattern "refino aditivo paralelo entre variants irmãos" N=2 → 3 estendido a GridCell) |
| **A.4 Block/Boxed** | outset/radius/clip P156G+H scope-outs | M (~2-3h cada) | média (refinos ortogonais) |
| **A.5 Place per-cell** | Place dentro Grid com align: ? per-cell | S+ (~1h) | média (Categoria A 5/5 fecha; cobertura cosmética completa) |
| **B.1 DEBT-34d** | Auto track sizing algorítmico (fecha DEBT-34d) | M | média (algorítmico isolado; fecha DEBT preservado) |
| **D.1 state** | runtime mutable; desbloqueia ADR-0066 IMPLEMENTADO | M | média (+33pp Introspection) |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |

**Recomendação subjectiva**: **A.3 per-cell** — consolida
Categoria A continuação (3/5 sub-passos); valida pattern
"refino aditivo paralelo entre variants irmãos" N=2 → 3
estendido (GridCell estende paralelismo Grid+Table de
P227+P228 para per-cell precedência); momentum cumulativo
P227 → P228 → P229 → A.3 natural pós-promoção EM VIGOR
ADR-0080.

**Decisão humana fica em aberto literal** pós-P229.

**Estado pós-P229**:
- Tests workspace: **2071 verdes preservados** (zero código).
- Content variants: 59 preservado.
- Value variants: 55 preservado.
- Stdlib funcs: 60 preservado.
- Grid fields: 10 preservado.
- Table fields: 5 preservado.
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada.
- **ADR-0080 status `EM VIGOR`** ✓ (transição P229).
- ADR-0066 PROPOSTO preservado; ADR-0061 IMPLEMENTADO
  preservado; ADR-0078 IMPLEMENTADO preservado; ADR-0079
  PROPOSTO preservado.
- **Distribuição ADRs**: PROPOSTO **12** (-1); EM VIGOR **29**
  (+1); IMPLEMENTADO 21; total 67 preservado.
- Saldo DEBTs: 12 preservado.
- **21 aplicações cumulativas anti-inflação** pós-P205D
  preservadas (P229 documental puro não altera).
- **Pattern "L0 minimal para refactors" N=9 EM VIGOR** —
  regra metodológica formal cristalina para refactors
  aditivos pós-M9c.
- **Pattern emergente "passo administrativo XS dedicado
  para promoção formal" N=1 inaugurado P229** (paridade
  P160A "passo administrativo XS" mas para promoção
  específica).
- **Pattern emergente "passos administrativos XS NÃO
  ganham marca cirúrgica blueprint" N=1 inaugurado P229**.
- **Pattern "spec opcional → skip empírico pragmático"
  N=1 → 2 cumulativo** (P225 C9 + P229 C7).
- **Pattern "ADR PROPOSTO com materialização parcial
  graded" reduzido** — pós-P229: 2 ADRs PROPOSTAS
  (ADR-0066 + ADR-0079; ADR-0080 transitou EM VIGOR).
- **Categoria A Fase 5 Layout**: 2/5 preservado (A.1 ✓;
  A.2 ✓; A.3 candidato próximo).
- **Fase 5 Layout candidata**: 2/13-15 sub-passos
  materializados; **trajectória aberta consolidada com
  ADR-0080 EM VIGOR formal**.
