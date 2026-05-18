# Diagnóstico Fase A P273.15.A — Bbox medido pós-layout (Fase A com verificação demanda empírica)

**Data**: 2026-05-18.
**Passo**: typst-passo-273.15.A.
**Magnitude**: S documental (~30 min — diagnóstico viabilidade prioritário).
**Cluster**: Visualize / Gradient (sexto sub-passo na sequência terminar cluster).
**Tipo**: Fase A empírica com decisão **binária** go/no-go per ADR-0034 + ADR-0085.
**Vigésimo sexto consumo directo de fonte**.

---

## §A.1 — Inventário de demanda empírica

### Verificação literal: zero casos registados onde 3γ.2.γ produziu output incorrecto

`grep -rn "3γ.2.γ.*incorrecto\|3γ.2.γ.*errado\|3γ.2.γ.*falha"` nos
materialization files retorna apenas:
- 4 referências à pendência condicional "se 3γ.2.γ for empiricamente
  insuficiente" (P273.6, P273.7, P273.6-relatorio, P273.7-relatorio).
- 0 referências a casos onde 3γ.2.γ produziu output observado
  incorrecto.

### Adopção empírica 3γ.2.γ por arm

| Arm | Decisão Fase A | Cobertura |
|---|---|---|
| `Content::Block` | P273.6 — **3γ.2.γ** (popular apenas com `width+height` literais) | 8 sub-passos sem contraproba |
| `Content::Boxed` | P273.7 — **3γ.2.γ-inline-baseline-y** (idem; bbox.y baseline-relative) | 7 sub-passos sem contraproba |
| `Content::Grid` cell | P273.9 — **3γ.2.γ** (dimensions sempre literais por construção: `body_w`/`body_h`) | 5 sub-passos sem contraproba |
| `Content::Stack` | P273.9 — **3γ.2.β** (layout duplo via `measure_content_constrained`; sem dimensions literais) | 5 sub-passos sem contraproba |
| `Content::Pad` | P273.9 — **3γ.2.β** (idem; bbox INNER medido) | 5 sub-passos sem contraproba |
| `FrameItem::Group` | P273.10 — **L3 dispatcher override** (não Layouter; `frame.size` directo) | 4 sub-passos sem contraproba |

**Stack/Pad já usam 3γ.2.β** (layout duplo) porque por construção
não têm dimensions literais. **Block sem dimensions literais
permanece com 3γ.2.γ → cai no fallback page_bbox (P273.5)**.

### Casos teste exercitando gradient `relative=parent` aninhado

`grep -rn "relative=parent\|RelativeTo::Parent"` nos testes:
- P273.6 tests (`p273_6_*`) exercitam Block com dimensions literais —
  3γ.2.γ path activo, observable diff confirmado.
- P273.7 tests (`p273_7_*`) exercitam Boxed com/sem dimensions —
  observable diff confirmado para com-dims; sem-dims preservado
  outer.
- P273.9 tests (`p273_9_*`) exercitam Grid/Stack/Pad — todos cobertos.
- P273.10-P273.13 tests (`p273_10/12/13_*`) exercitam Group L3
  override.

**Zero tests** exercitam `Content::Block { width: None, height: None,
... }` com gradient `relative=parent` aninhado expecting bbox real
medida em vez de page_bbox fallback.

### Output PDF actual de tests com Block sem dimensions

Comportamento actual: gradient `relative=parent` aninhado em Block
sem dimensions cai no fallback page_bbox L3 P273.5 (identity
transform). **Nenhum test marca este comportamento como issue ou
regressão**. **Aceito por defaults P273.5-P273.13**.

### Issue tracker / documentos utilizador

Cristalino não tem issue tracker público externo. Documentação
utilizador (`00_nucleo/prompts/`) não menciona comportamento
incorrecto. Diagnósticos imutáveis (`00_nucleo/diagnosticos/`)
20 ficheiros — nenhum reporta este caso.

**Conclusão §A.1**: **zero demanda empírica registada**.

---

## §A.2 — Inventário dos 3 caminhos com custo perf concreto

### Caminho 1 — Eager measurement (todos os Blocks sem dimensions)

**Mecanismo**:
```rust
// L1 arm Content::Block:
let saved_parent_bbox = self.parent_bbox;
if let (Some(w), Some(h)) = (width, height) {
    // 3γ.2.γ literal P273.6 preserved
    self.parent_bbox = Some(Rect { x, y, w: w_pt, h: h_pt });
} else {
    // P273.15 — 3γ.2.β eager: measure body sempre.
    let avail_w = self.available_width();
    let (measured_w, measured_h) =
        self.measure_content_constrained(body, avail_w);
    if measured_w > 0.0 && measured_h > 0.0 {
        self.parent_bbox = Some(Rect { x, y, w: measured_w, h: measured_h });
    }
}
self.layout_content(body);
// flush + restore preserved
```

**Custo perf**:
- `measure_content_constrained` é recursivo sobre o body inteiro.
- **Executado em TODOS os Blocks sem dimensions** mesmo quando não
  há gradient `relative=parent` interno (Layouter não sabe a priori).
- Pipeline com N Blocks sem dimensions: N execuções layout duplo.
- Pior caso: documento estructurado com N Blocks aninhados, cada
  um media tudo recursivamente — complexidade O(N²) onde antes era
  O(N).

**Magnitude**: ~30-50 LOC L1 (lógica condicional simples).

**Comparação Stack/Pad P273.9**: Stack/Pad já fazem layout duplo;
custo aceitável **porque sem alternativa** (não há dimensions
literais para usar). Block tem alternativa (fallback page_bbox) —
custo perf não-justificado sem demanda.

### Caminho 2 — Lazy measurement (only when gradient relative=parent interno)

**Mecanismo**:
```rust
// Pre-walk no body para detectar gradient relative=parent:
fn has_relative_parent_gradient(content: &Content) -> bool {
    // Walk recursivo: Shape { fill/stroke gradient relative=parent } → true
    // Sequence/Block/Boxed/etc → recurse children
    // ...
}

// L1 arm Content::Block:
if (width, height) sem dimensions e has_relative_parent_gradient(body) {
    // Medir + popular
} else {
    // 3γ.2.γ literal
}
```

**Custo perf**:
- Walk extra **sempre** (mesmo que não haja gradient interno).
- Layout duplo só quando necessário (overhead reduzido).
- Walk é mais leve que measure (não calcula dimensions; só
  procura presença) — mas adicional.

**Magnitude**: ~60-100 LOC L1 (walker novo + lógica condicional).

**Trade-off**: redução de custo perf vs custo de implementação +
manutenção de walker novo.

### Caminho 3 — Scope-out preserved

**Mecanismo**: 3γ.2.γ literal mantido. Block sem dimensions →
`parent_bbox` outer preserved → cai no fallback page_bbox L3 P273.5
(identity transform).

**Custo perf**: zero.

**Magnitude**: zero código.

**Trade-off**: gradient `relative=parent` aninhado em Block sem
dimensions renderiza com page_bbox em vez de bbox real do Block.
Comportamento aceito por defaults P273.6-P273.13; **zero demanda
empírica registada** em 8 sub-passos.

---

## §A.3 — Decisão go/no-go primária — **NO-GO**

**Fixada**: **NO-GO via §A.5 critério #1 + #2 + #4 combinados**.

Razão concreta:

1. **§A.1 confirma zero demanda empírica** — 8 sub-passos consecutivos
   (P273.6-P273.13) sem caso registado onde 3γ.2.γ produziu output
   observable incorrecto.
2. **Caminho 1 (eager) tem custo perf inaceitável** — layout duplo
   sempre para todos Blocks sem dimensions, mesmo quando não há
   gradient `relative=parent` interno. Pior caso O(N²) onde antes
   era O(N).
3. **Caminho 2 (lazy) tem custo de implementação desproporcional** —
   walker novo + manutenção para resolver problema sem demanda
   registada.
4. **3γ.2.γ é aceito por ADR-0054 graded** — "menor mudança
   suficiente" preserved; refino sem demanda é over-engineering.

Esta decisão **não é falha** — é cumprimento honesto do critério
"verificar empíricamente" registado em todos os relatórios
anteriores P273.6-P273.13.

---

## §A.4 — Critério para GO — não cumprido

Per spec §A.4:

| Critério GO | Estado |
|---|---|
| Demanda empírica concreta identificada — pelo menos 1 caso real | ❌ Zero casos em 8 sub-passos |
| Caminho escolhido com magnitude + custo perf aceito | ❌ Caminho 1 perf inaceitável; Caminho 2 impl desproporcional |
| Tests E2E construídos verificando observable diff | ❌ Sem caso real, sem teste possível |

Critério #1 (demanda empírica) não cumprido → **NO-GO automático**
per spec §A.4 ("Sem demanda, refino é over-engineering").

---

## §A.5 — Critério para NO-GO — cumprido absoluto

Per spec §A.5:

| Critério NO-GO | Cumprido | Como |
|---|---|---|
| §A.1 confirma zero demanda empírica registada | ✅ | grep retorna 0 casos em 20 documentos |
| Caminho 1 (eager) tem custo perf inaceitável | ✅ | Layout duplo sempre; pior caso O(N²) |
| Caminho 2 (lazy) tem custo impl desproporcional | ✅ | Walker novo + manutenção sem demanda |
| 3γ.2.γ aceito por ADR-0054 graded | ✅ | "Menor mudança suficiente" preserved |

**4 critérios NO-GO cumpridos absolutos** — NO-GO honesto.

---

## §A.6 — Análise de risco

| Risco | Estado |
|---|---|
| Refino sem demanda empírica vira over-engineering | ✅ Mitigado — §A.4 critério 1 obrigatório bloqueou GO |
| Custo perf escondido | ✅ Mitigado — §A.5 critério 2 explícito; quantificado pior caso O(N²) |
| Regressão tests P273.6 Block 3γ.2.γ | N/A — zero código alterado |
| Scope-out parece falha | ✅ Mitigado — §A.5 explicita: NO-GO é cumprimento honesto; sub-padrão "Scope-out reconfirmado por Fase A" inaugural P273.14 |
| `measure_content_constrained` valores divergentes do layout real | N/A — não aplicado |

---

## §A.7 — Decisões fixadas Fase A

1. **Decisão 1 (caminho)**: **3 — scope-out preserved**.
2. **Decisão 2 (apenas se GO)**: **N/A** — NO-GO.
3. **Decisão 3 (sempre)**: documento
   `00_nucleo/diagnosticos/typst-passo-273-15-trabalho-previo-externo.md`
   produzido como output do passo per §6 workflow obrigação.

---

## §A.8 — Critério de aceitação Fase A

- ✓ §A.1 inventário de demanda empírica com factos literais (**zero
  casos** registados em 8 sub-passos via grep verificação).
- ✓ §A.2 inventário dos 3 caminhos com custo perf concreto
  (quantificado: eager pior caso O(N²); lazy walker novo; scope-out
  zero).
- ✓ §A.3 decisão **NO-GO** fixada com fundamento literal triplicado
  (zero demanda + Caminho 1 perf inaceitável + Caminho 2 impl
  desproporcional + ADR-0054 graded).
- ✓ §A.5 risco mitigado por 4 critérios explícitos.
- ✓ §A.7 documento de trabalho prévio externo produzido como output
  legítimo per ADR-0054 graded.

**Fase A produzida — critério §A.8 cumprido absoluto. Decisão
NO-GO confirmada empíricamente paralelo a P273.14.**

---

## §A.9 — Plano de implementação (Fase C — REDUZIDA por NO-GO)

Por NO-GO, Fase C reduzida a:

1. ADR-0091 anotação cumulativa décima quinta (template NO-GO).
2. L0 `entities/gradient.md` anotação P273.15 (NO-GO outcome).
3. Documento `typst-passo-273-15-trabalho-previo-externo.md`.
4. Relatório P273.15 com status **SCOPE-OUT-RECONFIRMED**.

**Zero alterações código L1/L3**. Tests workspace preserved bit-exact.

### Sub-padrão emergente reaplicado

- **"Scope-out reconfirmado por Fase A"** N=1 → **N=2 cumulativo**:
  - **N=1 (P273.14)**: CMYK-ICC scope-out via NO-GO (profile
    licensing + crate externa).
  - **N=2 (P273.15)**: Bbox medido pós-layout via NO-GO (zero
    demanda + custo perf).
  - Padrão consolidado por primeira reaplicação. Limiar formalização
    N=3-4 ainda longe — candidato meta-ADR futuro NÃO reservado.

### Comparação P273.14 vs P273.15

| Aspecto | P273.14 | P273.15 |
|---|---|---|
| Razão NO-GO | Profile licensing + crate externa | Zero demanda + custo perf |
| Trabalho prévio externo | 3 pré-requisitos (ADR + profile + size decision) | 2 pré-requisitos (caso real + custo perf decisão) |
| Sub-padrão | Inaugural N=1 | Reaplicação N=2 |

Distinção arquitectural: P273.14 NO-GO por **constraints externas**
(licensing, invariante L0); P273.15 NO-GO por **ausência de
demanda + custo perf**. Ambos legítimos per ADR-0054 graded.

---

*Diagnóstico imutável produzido em 2026-05-18. Vigésimo sexto
consumo. Decisão **NO-GO** confirmada empíricamente (zero demanda
em 8 sub-passos + custo perf inaceitável + ADR-0054 graded);
trabalho prévio externo documentado como output legítimo;
sub-padrão "Scope-out reconfirmado por Fase A" cresce N=1 → N=2
cumulativo consolidação.*
