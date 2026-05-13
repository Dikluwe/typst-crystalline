# Passo 210C — Encerramento série P210

**Série**: 210 (sub-passo `C` final).
**Marco**: M9c (encerramento série P210; M9c continua
com P211+P212).
**Tipo**: encerramento série documental puro.
**Magnitude**: S (~20-30min).
**Pré-condição**: P210B concluído; série P210 materializada
em 2 sub-passos (A diagnóstico + B counter_step);
`counter.step()` stdlib func registada; trait 26 métodos;
stdlib funcs ~53; tests 1939 verdes; 0 violations;
ADR-0076 PROPOSTO anotado §P210B; blueprint §3.0sexies
[P209E].
**Output**: 1 ficheiro (relatório curto encerramento +
transição P211).

---

## §1 Trabalho

Encerrar série P210. Trabalho documental puro — P210B
já implementou subset Caminho 3 (counter.step);
display/get permanecem deferred per P210A C3 sem
trabalho material remanescente em P210.

**Distinção face a P207E / P208D / P209E**:
encerramentos anteriores tinham decisões de Caminho 1/2
em C1 interno. **P210C não tem essa decisão** — P210A C3
já fixou Caminho 3 subset; P210B implementou; P210C
apenas formaliza encerramento.

Reuso de dados trajectória M9c:

- Pattern marca-por-fecho blueprint: §3.0/§3.0bis/
  §3.0ter/§3.0quater/§3.0quinquies/§3.0sexies → P210C
  adiciona §3.0septies.
- Pattern encerramento documental puro consolidado (P207E,
  P208D, P209E).
- Sem ADR nova em P210 (trabalho 100% sob ADR-0076).

---

## §2 Cláusulas (3)

### C1 — Anotações documentais

**ADR-0076** (`00_nucleo/adr/typst-adr-0076-introspector-completion.md`)
§Plano de materialização:

- Série P210 transita "EM CURSO" → "✅ MATERIALIZADO
  ({data})".
- §P210C anotado com forma documental pura.
- Bloco "Agregado série P210" adicionado com:
  - Sumário 3 sub-passos (A + B + C).
  - Métricas Δ série (stdlib funcs +1; tests +4; sem
    L0 novo; sem L1 novo; sem ADR nova).
  - **Deferreds explícitos**: `counter.display(numbering)`
    + `state.get()` here-aware — critério de reabertura:
    "quando walk advance for implementado em sub-passo
    futuro pós-M9c (ou se consumer real emergir)".

**Blueprint** (`00_nucleo/diagnosticos/blueprint-projecto.md`):

- §3.0septies marca adicionada (paralelo a §3.0sexies
  [P209E]). Conteúdo:
  - Série P210 fechada.
  - Caminho 3 honest subset 8ª aplicação anti-inflação
    cumulativa.
  - Deferreds counter.display + state.get com critério.
  - Estado M9c pós-P210: 4 séries fechadas (P207 + P208
    + P209 + P210); restam P211 + P212.

### C2 — Verificação final

```
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: tests 1939 verdes (Δ 0 — sem código tocado);
0 violations.

### C3 — Resumo agregado série P210

Estrutura no relatório (§6) — paralelo a P207E §6 / P208D
§6 / P209E §6:

| Sub-passo | Tipo | Magnitude real | Output principal |
|-----------|------|---------------|------------------|
| P210A | Diagnóstico-primeiro reduzido | S (~30min) | Auditoria A1-A5 + decisões C1-C5 + plano P210B-C. Caminho 3 subset fixado (só counter.step; display/get deferred). |
| P210B | Materializar counter.step | S (~30min) | `native_counter_step(key)` stdlib + scope register + 4 tests. Sem L0; sem trait extension. |
| P210C | Encerramento série | S documental (~20-30min) | ADR-0076 anotada; blueprint §3.0septies; relatório este. Zero código tocado. |

**Custo agregado real**: ~1.5h (estimado ~1.5-2h per P210A
C5). Magnitude **S-M** confirmada empíricamente — abaixo
do estimado L original em P207A para Bloco V completo
(reduzido por Caminho 3 subset honest).

**Padrões consolidados em P210**:

1. **Caminho 3 honest subset** (P210A C3 + P210B
   implementação): pattern emergente novo distinto de
   Caminho 1 puro. Materializa parte; defere parte com
   critério de reabertura. 8ª aplicação cumulativa
   anti-inflação.
2. **Funcs stdlib sem walk advance dependência** vs
   **funcs que dependem**: distinção qualitativa
   confirmada empíricamente (P210A A3). `counter.step`
   emite Content layout-time-resolved (sem
   current_location); `counter.display`/`state.get`
   resolvem at-this-location (com current_location).
3. **Convenção L0 stdlib funcs P169+ inline-documentadas**
   reusada (P208B / P208C / P209D C6 / P210B). Sem L0
   prompt novo em toda a série P210.

---

## §3 Output

1 ficheiro:
`00_nucleo/materialization/typst-passo-210C-relatorio.md`.

Estrutura (~4-5 KB) com 7 §s paralelos a P207E / P208D /
P209E:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Forma do encerramento (documental puro; sem decisões
  C1 a tomar).
- §3 Alterações documentais (anotações ADR + blueprint).
- §4 Decisões substantivas (nenhuma material; apenas
  formalização).
- §5 Métricas (compacta).
- §6 Encerramento série P210 (sumário 3 sub-passos +
  agregado).
- §7 Próximo sub-passo (P211).

---

## §4 Não-objectivos

- Materializar `counter.display`/`state.get` (deferred per
  P210A C3 + P210B implementação).
- Walk advance (deferred per P208B).
- Outline configurável (P211).
- Encerramento M9c inteiro (P212).
- Transição ADR-0076 PROPOSTO → ACEITE (P212).
- Novas ADRs (nenhuma em P210).

---

## §5 Riscos a evitar

1. **Inflar encerramento série**: P210C é encerramento
   série, não consolidado M9c. Sumário literal 3
   sub-passos.
2. **Confundir encerramento série vs marco**: P210C
   fecha série P210; M9c continua. ADR-0076 mantém
   PROPOSTO até P212.
3. **Esquecer marca blueprint**: pattern §3.0/3.0bis/
   3.0ter/3.0quater/3.0quinquies/3.0sexies → P210C
   adiciona §3.0septies (7ª marca cirúrgica).
4. **Esquecer deferreds documentados em ADR-0076**:
   `counter.display` + `state.get` ficam adiados com
   critério explícito. Sem critério explícito,
   reabertura futura fica sem trigger; com critério,
   sub-passo dedicado tem gatilho claro.
5. **Inventar decisão de Caminho 1/2 em C1**: P210C
   **não tem** essa decisão. P210A C3 já fixou
   Caminho 3 e P210B implementou. Distinto de P207E /
   P208D / P209E.
