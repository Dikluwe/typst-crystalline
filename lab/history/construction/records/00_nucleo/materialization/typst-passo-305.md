# Passo 305 — `P295.2` footnote overflow multi-página

**Frente**: `P295.2 — overflow multi-página` (pendente desde
P295 §8; reaparece P304 §8 e §9).
**Origem**:
- P304 §1.2 explicit: *"P295.2 (overflow multi-página)
  permanece scope-out"*.
- P304 §3.2 implementa flush single-page via `mem::take` +
  posicionamento Y absoluto a partir de `area_bot - total_h`.
- **Bug latente P304** (não-documentado explicitamente):
  se `total_h > área disponível rodapé`, bodies sobrepõem ao
  corpo da página silenciosamente.
- P304 §9 prioridade #1.
**Pré-requisitos**: P304 (`pending_footnote_bodies` buffer +
`flush_pending_footnote_bodies` method).
**Tipo declarado**: **extensão P304 com page break coupling
cross-page** — paradigma potencialmente novo (split state cross-
page) ou reaplicação DeferredX N=4 conforme A.0.0.
**Magnitude**: XS+ a M conforme A.0.0; subset reduzido viável.

---

## §1 — Objectivo

Estender flush de `pending_footnote_bodies` para suportar
overflow multi-página: quando bodies totais excedem espaço
rodapé disponível, items que não cabem **deferem para próxima
página** (não sobrepõem ao corpo nem são descartados).

### §1.1 — Bug latente exposto

P304 §3.2 flush:
```rust
let mut y_cursor = area_bot - total_h;
for (h, items) in measured { ... y_cursor += h; }
```

**Bug**: `y_cursor = area_bot - total_h` calcula posição inicial
**sem clamp** ao top da área disponível. Se `total_h > (area_bot
- top_corpo_pagina)`, `y_cursor` fica **acima** do top corpo →
bodies sobrepõem ao texto principal **silenciosamente**.

**Visibilidade**: bug existia desde P304; nenhum teste P304 cobriu
overflow (P304 §A.5 escopou apenas single-page). P305 expõe e
fixa.

### §1.2 — Comportamento vanilla a verificar (A.0.0)

`#footnote[body]` × overflow em vanilla typst — possibilidades:

| Hipótese | Comportamento |
|---|---|
| **HA** | Body inteiro spills se não cabe; vai inteiro para próxima página |
| **HB** | Body split per-item (linha); items que cabem na actual + sobras → próxima |
| **HC** | Body split a granularidade glyph |
| **HD** | Cristalino actual P304 produz overlap; **bug confirmado** |

**Default esperado**: **HA** — minimal viable; spill body inteiro
preserva integridade. **HB** mais sofisticado; **HC** vanilla-
mais-fidedigno mas mais complexo.

### §1.3 — Categoria sub-padrão DeferredX N=4

P304 §10.2 estabeleceu DeferredX N=3 (P245+P251+P304). P305
reaplicação:

| Cenário | Sub-padrão |
|---|---|
| **HA** + sobras simples carregadas para próxima página | DeferredX N=4 **directo** (mesmo pattern; diferente buffer state management) |
| **HB** + body split internal | Sub-padrão **distinto** — "DeferredX with item splitting"; N=1 inaugural |
| **HC** + glyph split | Categoria diferente — não DeferredX |

**Crítico**: distinção honesta categorias. Spec P304 §10.2 lição:
*"limiares devem distinguir N do sub-padrão correcto vs N
agregado de sub-padrões superficialmente similares"*.

**A.0.0 decide** subcategoria conforme inspecção vanilla.

### §1.4 — Riscos e razões

**Riscos**:
- Magnitude XS+ pode revelar-se M+ se HB/HC requerem split
  state.
- Sub-padrão DeferredX **promoção forçada** se N=4 inflacionado
  artificialmente. Mitigação: §A.5' critério estrito.
- **Recursão infinita potencial** se body único é maior que
  página inteira. Mitigação: A.5 cenário dedicado.

**Razões**:
1. **Bug factual P304** (não-documentado mas exposto por P305).
2. **Pendência registada** P295 §8 desde antes de P304.
3. **Completa paridade vanilla footnote** single-document.
4. **Reaplicação DeferredX N=4** candidata se HA confirmada.
5. **Reaplicação ADR-0098 + ADR-0099** — N=22 + N=17 cumulativos.

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória; 8 secções)

A.0.0 + A.0.0' (subdivision) + A.0-A.5 + A.5'.

### A.0.0 — Verificação literal estado overflow (N=12 cumulativo §8.7')

Inspecção literal:

1. **Reproduzir bug P304**:
   - Documento com 1 página + footnote body > área rodapé.
   - Verificar: bodies sobrepõem corpo? Descartam? Silenciosos?
2. **Inspeccionar P304 flush method literal** —
   `flush_pending_footnote_bodies` `cursor.rs`:
   - Como `area_bot - total_h` se comporta com `total_h` grande.
3. **Inspeccionar `area_bot`/`top_corpo` boundaries** —
   conceito Layouter de "área disponível rodapé".
4. **Inspeccionar `new_page()`** — fluxo actual page-break;
   compatível com retry flush?
5. **Inspeccionar vanilla** `lab/typst-original/.../model/footnote.rs`
   + integration layouter — algoritmo overflow vanilla.
6. **Verificar P245 `emit_deferred_float`** — pattern handling
   "doesn't fit"; reaplicável?
7. **Verificar P251 `pending_cell_tails`** — algum overflow
   handling?
8. **Cross-check com `outline()` P65-66** — TOC overflow
   multi-página (se houver) usa que pattern?

**Decisão A.0.0**:

| Hipótese confirmada | Acção |
|---|---|
| **HA** | P305 reaplica DeferredX N=4: split point = "body inteiro"; sobra → próxima página |
| **HB** | P305 inaugura sub-padrão "DeferredX with item splitting"; magnitude M+ |
| **HC** | P305 categoria distinta — fora DeferredX; magnitude M+ |
| **HD único** | Documentar bug; fix via clamp; sem overflow real implementado (subset reduzido) |

**Default sugerido**: **HA** (vanilla single-document
overflow mais simples). **HD** fallback se complexidade revela
M+ inesperado.

**Magnitude esperada A.0.0**: **média** — verificação empírica
do bug + inspecção vanilla.

### A.0.0' — Decisão de subdivisão

| Subset | Scope | Magnitude |
|---|---|---|
| **P305.A** | HD apenas: clamp Y position; descartar overflow (sem retry) | XS |
| **P305.B** | HA: body inteiro spill → próxima página | XS+ |
| **P305.C** | HB: body item split | M |
| **P305.D** | HC: glyph split paridade total vanilla | M+ |

**Default sugerido**: **P305.B** — fix bug + paridade single-
document overflow básico. **P305.A** se A.0.0 revelar
infraestrutura `new_page()` insuficiente para retry. P305.C/D
sub-passos dedicados.

**Permitir interrupção honesta** se P305.B requer cross-page
state complex.

### A.0 — Potencial de reuso ADR-0098

| Verificação | Esperado |
|---|---|
| `grep "footnote\|spill\|overflow" 03_infra/src/export.rs` | Zero hits — emit agnóstico |
| `FrameItem` variants suportam multi-page footnote? | **Sim** — Page-by-page já existe |
| Hash `export.rs 66cb8ac3` esperado | **Preservado bit-exact** pelo **22º passo consecutivo** P282-P305 |

### A.1 — Inventário literal

8 sub-secções:

1. **A.1.1 — Layouter pós-P304** — `pending_footnote_bodies`
   campo + `flush_pending_footnote_bodies` method.
2. **A.1.2 — `new_page()` flow** — capacidade trigger
   pré-flush.
3. **A.1.3 — `area_bot`/`top_corpo` calc** — boundaries
   literais.
4. **A.1.4 — Vanilla overflow algorithm** — HA/HB/HC
   discriminado.
5. **A.1.5 — P245 `emit_deferred_float`** — handling "doesn't
   fit".
6. **A.1.6 — `mem::take` vs partial drain** — patterns
   estabelecidos.
7. **A.1.7 — `Frame` size constraints** — cristalino enforce?
8. **A.1.8 — Diagrama fluxo overflow** — produzir genuinamente.

### A.2 — Estrutura overflow handling

**Decisão arquitectural** condicional A.0.0:

| Opção HA (default) | Mecanismo |
|---|---|
| **(a)** Partial drain: items que cabem flush; sobras stay no buffer; `new_page()` flush automático na próxima | Reusa DeferredX pattern N=4; minimal |
| **(b)** Trigger `new_page()` pre-emptive quando body não cabe; flush total na próxima | Cleaner; mas requer mid-flush state |
| **(c)** Calcular antecipadamente N páginas necessárias; reservar antes | Sofisticado; magnitude M |

Default sugerido: **(a)** — partial drain natural via condicional
no loop. **(b)** considerar se A.1.2 mostra `new_page()` trigger
mid-flush é complexo. **(c)** rejeitada — over-engineering.

### A.3 — Integração com `new_page()` flow

| Opção | Comportamento |
|---|---|
| **(α)** `new_page()` chama `flush_pending_footnote_bodies` ANTES de finish; bodies sobrantes ficam no buffer; próximo `new_page()` flush continua | Iterativo; convergente |
| **(β)** `flush` próprio detecta overflow e dispatch `new_page()` interno | Recursivo; mais limpo mas requer cuidado |
| **(γ)** Flag `overflow_detected` no Layouter; check antes do próximo flow | Indirecto; menos elegante |

Default sugerido: **(α)** — iterativo natural; convergência
quando buffer vazio.

### A.4 — Impacto em emit (ADR-0098)

Standard: `FrameItem::Text/Glyph` agnóstico. Hash `export.rs`
preservado bit-exact pelo **22º passo consecutivo**.

### A.5 — Detecção de bugs latentes

Cenários fronteira:

- **1 body cabe na página**: regressão P304 preservada.
- **3 bodies; total cabe**: regressão P304 preservada.
- **3 bodies; total > área**: spill primeiros → actual; restantes
  → próxima.
- **1 body > página inteira**: **caso degenerate**; comportamento
  vanilla? Pode requerer split forçado HC; **scope-out P305.B
  default** (documentar como limit).
- **0 bodies**: regressão zero-cost preservada.
- **Documento sem footnote**: regressão bit-exact P304 preservada.
- **Footnote no início vs fim página**: diferença comportamento?

**Matrix cross-construct** (P302 §6.7 / aplicado P303+P304):

```
                  1 footnote  3+ footnotes  body > página inteira
Cabe na página    ✓ regressão P304  ✓ regressão P304  n/a
Não cabe (overflow) ✓ P305.B  ✓ P305.B (parcial)  scope-out P305.D
Documento sem footnote ✓ regressão  ✓ regressão  ✓ regressão
```

### A.5' — Verificação anti-reflexão (N=15 do padrão §8.6)

**12ª reaplicação A.0.0** consecutiva.

4 verificações:

1. **Comparação literal A.1.6 P288-P305** — paradigma novo?
   - P293-P304 doze paradigmas distintos.
   - **P305**: paradigma **"DeferredX partial drain cross-
     page"** — extensão N=4 mesma categoria P245/P251/P304
     **SE HA confirmada**. Se HB/HC → categoria nova.
2. **A.0.0 produzido empiricamente** — magnitude.
3. **Elementos estructuralmente novos identificados**:
   - **Cross-page buffer state** — primeiro caso buffer
     persiste através de page break (P245/P251/P304 todos
     limpam buffer em new_page).
   - **Bug fix latente colateral** (paralelo P302/P303
     subcategoria §8.4) — bug P304 descoberto durante
     overflow inspecção.
4. **Decisão sobre promoção ADR meta**:
   - **Sub-padrão DeferredX N=4** — **NÃO promover** se HA é
     reaplicação directa (qualidade do 4º caso = 3º se trivial).
   - **Sub-padrão DeferredX N=4** candidato **se** HA introduz
     cross-page buffer state genuinamente novo.
   - **§8.4 subcat A/B/C** — P305 caso de bug derivado P304;
     pode ser subcat A (P304 expôs) ou C (bug em feature
     recém-materializada P304). **Subcat A esperada**;
     **§8.4 subcat A N=3 candidato robusto** (P288+P302+P305).
   - **§8.7' N=12** — adiamento standard.
   - **§8.3 N=14** candidato.
   - **Uma ADR meta por passo no máximo** (P273.17 §0).

**Critério P304 §10.2 mantido**: distinguir N do sub-padrão
correcto vs N agregado superficial. **§8.4 subcat A** se P305
bug = derivado P304 (paralelo P302 derivado P301). **Promoção
candidata robusta** se subcat A confirmada N=3.

---

## §3 — Materialização (condicional a A.0.0)

**Cenário default (HA + P305.B + A.2 → (a) + A.3 → α)**:

1. Modificar `flush_pending_footnote_bodies` em
   `01_core/src/engine/layout/cursor.rs`:
   - Calcular **available space** rodapé (não apenas
     `area_bot`).
   - Pass 1 measure (existe).
   - Loop place: para cada body measured, check se cabe; se
     sim, place; se não, **stop**.
   - Items não placed: re-inserir em
     `pending_footnote_bodies` (subset cauda do `bodies`).
   - **Não `mem::take` total**: usar drain condicional.
2. `new_page()` continua a chamar flush — bodies remanescentes
   da página anterior flush automático na nova (iterativo).
3. `finish()` continua chamar flush no fim.
4. **Bug fix coletáneo**: clamp `y_cursor` ao top da área
   rodapé (defensive).
5. Testes:
   - L1 unitário: overflow simples 3 bodies, 2 cabem + 1 spill.
   - L1 unitário: regressão P304 single-page.
   - L1 unitário: zero footnotes (regressão bit-exact).
   - L3 integração: PDF multi-página com footnotes
     overflow.
   - L3 integração: PDF sem footnote regressão.
   - L3 regressão: P304 single-page comportamento idêntico.
6. **Substituir P304 invariante** (se aplicável): teste P304
   que assume tudo flush single-page pode precisar atualização.
7. Promoção ADR meta condicional per A.5':
   - **§8.4 subcat A N=3** candidato — promover se subcat A
     confirmada genuína.
   - **DeferredX N=4** — adiar default (anti-padrão).
   - **Sem promoção** se ambíguo.
8. Actualizar L0:
   - `engine/layout.md` (se necessário; precedente P304:
     inalterado).
   - Tabela A.6 linha 178 — `parcial → parcial⁺` ou
     `implementado` conforme cobertura overflow.
9. Diagnóstico produzido com 8 secções.

**Sem caps**. Estimativa testes: 8-15.

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P304: 2 890 testes.
  Esperado: ~2 898-2 905.
- `crystalline-lint` zero violations.
- Hash L0 `content.md` **preserved**.
- Hash L0 `stdlib.md` **preserved**.
- Hash L0 `layout.md` **condicional** — preserved se P305
  segue precedente P304 (campos buffer sem L0 update); muda
  se requer fluxo `new_page` formal.
- Hash L0 `export.rs` **preserved bit-exact** pelo **22º passo
  consecutivo** P282-P305.
- **Regressão bit-exact validada** — P304 single-page tests
  preservados; documentos sem footnote idênticos.
- Bug factual P304 fixed (overlap silencioso eliminado).
- Tabela A.6 linha 178 reclassificada conforme cobertura.
- Diagnóstico 8 secções produzido.
- **Promoção ADR meta condicional**:
  - **§8.4 subcat A N=3** se subcat A confirmada genuinamente
    (promoção candidata robusta — 1ª promoção concreta possível
    em 13 passos).
  - **DeferredX N=4** apenas se HA introduz cross-page state
    genuinamente novo.
  - Default conservador: sem promoção (anti-padrão honrado 13ª
    vez).
- Frente P295.2 resolvida.

---

## §5 — Não-objectivos

- **Não** materializar HC/HD (item split / glyph split).
  Scope-out P305.C/D.
- **Não** materializar P304.B/C/D (reserved space dinâmico,
  separator, customization). Sub-passos dedicados.
- **Não** materializar `footnote.entry` / multi-ref (P295.X).
- **Não** alterar `Content::Footnote` semanticamente.
- **Não** promover múltiplas ADRs meta. Uma por passo
  (P273.17 §0).
- **Não** confundir P305 com P304 — P304 é single-page placement;
  P305 é cross-page overflow handling.
- **Não** assumir HA sem A.0.0 — comportamento vanilla pode
  ser HB/HC; documentar honestamente.

---

## §6 — Pendências relacionadas

Resolve:
- **P295.2 overflow multi-página** — pendente desde P295 §8.
- **Bug latente P304** (overlap silencioso quando total_h >
  área) — exposto e fixado.

Não resolve:
- P304.B/C/D — refinos cosméticos.
- P295.X footnote.entry / multi-ref.
- HC/HD item/glyph split.
- Outras pendências catálogo P300.

---

## §7 — Risco residual

Risco principal: **HB/HC revelados em A.0.0 — magnitude M+**.
Mitigação: A.0.0' subdivisão; abrir P305.0 dedicado se complexidade.

Risco secundário: **promoção §8.4 subcat A N=3 forçada**.
Mitigação: critério estrito; subcat A genuína exige bug
**derivado** (não pré-existente). P305 case: bug **exposto
por P304** (não criado por P304) — subcategoria ambígua A/B.
Default conservador per §A.5'.

Risco terciário: **cross-page state buffer interfere com `new_page()`
flow estabelecido**. Mitigação: A.1.2 inspecção; testes
regressão obrigatórios.

Risco quaternário: **recursão infinita se body > página
inteira**. Mitigação: A.5 cenário; scope-out P305.D + early-
return defensive em loop.

Risco quinário: **DeferredX N=4 promoção forçada** apesar de
ser reaplicação trivial. Mitigação: §A.5' critério qualidade
(paralelo P298 "cluster math handler" §6.5 adiado).

Risco senário: **Bug fix latente colateral** (clamp Y position)
introduz regressão silenciosa. Mitigação: §4 regressão
bit-exact obrigatória.

---

## §8 — Ponteiros

- Sítio bug: `01_core/src/engine/layout/cursor.rs`
  `flush_pending_footnote_bodies` (P304 §3.2).
- Buffer: `pending_footnote_bodies` campo Layouter (P304).
- Page break: `new_page()` flow.
- Vanilla: `lab/typst-original/crates/typst-library/src/model/footnote.rs`.
- Precedente DeferredX directo: **P304** (P245 + P251 + P304
  = N=3 cumulativo).
- Precedente §8.4 N=2 subcat A: **P302** (sin parens fix).
- Precedente §8.4 N=1 subcat B: **P303** (undef parens fix).
- ADR aplicável: **ADR-0098** + **ADR-0099**.
- ADR processual: ADR-0065 (inventariar-primeiro; 8 secções).
- ADR cultural: P273.17 §0 (anti-padrão; uma ADR meta por passo).
- Padrão §8.7' A.0.0 template (N=12 reaplicação).
- Sub-padrão DeferredX — N=3 cumulativo; **P305 candidato N=4
  conforme HA**.
- Sub-padrão §8.4 subcat A — N=2 cumulativo (P288+P302);
  **P305 candidato N=3 robusto** se subcat A confirmada.

---

*Spec P305 produzida 2026-05-19 pós-P304 (P295.1 footnote
single-page fechado; DeferredX N=3 cumulativo; bug overlap
silencioso exposto). Frente **P295.2 overflow multi-página** —
completa paridade vanilla footnote single-document. Fase A com
**8 secções** A.0.0+A.0.0'+A.0-A.5+A.5'. **Bug latente P304
exposto** (overlap silencioso quando total_h > área); fix
colateral via clamp Y. **Sub-padrão §8.4 subcat A N=3 candidato
robusto** se bug é derivado P304 — promoção candidata. **Sub-
padrão DeferredX N=4** candidato apenas se HA introduz cross-
page state genuinamente novo. **Anti-padrão P273.17 §0 honrado
13ª vez consecutiva** se sem promoção (default). Magnitude
XS+ a M conforme A.0.0; subset reduzido P305.B viável (HA
spill body inteiro). Lição P300 §10 mantida: critério é factual,
não pendência longa. Sem caps LOC ou magnitude (P282 §7).*
