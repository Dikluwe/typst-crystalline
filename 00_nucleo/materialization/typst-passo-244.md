# Spec do passo P244 — Reconciliação documental Linhas A (P216-P221) e B (P239-P243) + ajuste contagens factuais (passo administrativo XS; M9d quinta sub-passo se contado, ou sub-passo administrativo isolado se contado paralelo a M9d)

**Data**: 2026-05-14.
**Tipo**: passo administrativo XS puramente documental.
**NÃO materializa código.** **NÃO altera comportamento observable.**
**NÃO cria ADRs novas.** Reconcilia inconsistência factual entre
o relatório P243 (que recomendou "M7+3 fase (b)" como próximo
trabalho) e o estado empírico do repositório (Linha A já materializou
todo o trabalho descrito como "fase (b)" em P217-P221 a 2026-05-12).
**Magnitude planeada**: XS (~30-60 min documental).
**Marco**: sexto passo administrativo XS pós-P156A (paridade
P156A historiograma + P156K ADRs meta + P160A ADR-0066-create +
ADR-0062-create + P238 audit); **primeira aplicação do padrão
"reconciliação documental pós-divergência factual entre planeamento
e materialização"** N=1 inaugurado; sétima aplicação cumulativa
pattern "spec C1 audit obrigatório bloqueante pós-P236.div-1"
N=6 → 7 cumulativo — distintamente neste passo, **audit C1 é o
substantivo material do passo** (não preâmbulo a materialização).

---

## §1 O que será feito

P244 reconcilia documentação inconsistente entre duas linhas
paralelas de trabalho cumulativas:

### Linha A — série Fase 3 columns/colbreak (2026-05-12; pré-existente)

| Passo | Trabalho |
|-------|----------|
| P215 | diagnóstico Fase 3 Layout + ADR-0078 PROPOSTO (column flow algorithm) |
| P216A | `Region` entity em `01_core/src/entities/region.rs` |
| P216B | `Regions` minimal (campo `current` apenas) |
| P217 | `Content::Columns { count, gutter, body }` variant |
| P218 | `native_columns` stdlib + helper `extract_count` |
| P219 | Consumer multi-column real (Opção B graded) |
| P220 | `Content::Colbreak { weak: bool }` + `native_colbreak` |
| P221 | Encerramento: ADR-0078 IMPLEMENTADO + ADR-0061 IMPLEMENTADO + DEBT-56 ENCERRADO |

### Linha B — série M9d / M7+ pipeline restructuring (2026-05-14; em curso)

| Passo | Trabalho |
|-------|----------|
| P239 | ADR-0081 PROPOSTO (M7+ pipeline restructuring; 5 sub-passos) |
| P240 | M7+1 state.display walk-time real |
| P241 | M7+2 counter.display walk-time real |
| P242 | M7+5 A.4 radius/clip + `Corners<T>` |
| P243 | M7+3 "fase (a)" — extensão `Regions { backlog, last }` + `advance` method + 3 scope-outs promovidos |

### Inconsistência factual detectada

O relatório P243 §8 recomenda "M7+3 fase (b)" como próximo
trabalho, descrevendo-a como criar `Content::Columns` +
`Content::Colbreak` + `native_columns` + `native_colbreak` +
ADR column flow + Layouter consumer multi-column —
**trabalho factualmente já materializado em Linha A (P217-P220),
DEBT-56 ENCERRADA P221, ADR-0061 + ADR-0078 IMPLEMENTADOS**.

**Origem provável da inconsistência** (audit C1 confirma): o
audit C1 do P243 capturou parcialmente a realidade (descobriu
que `Regions` já existia desde P216A/B — Decisão 2 da §3 do
spec P243 anotou "Migração field-by-field: Já feita P216A/P216B")
mas **não auditou que `Content::Columns`/`Colbreak` também já
existiam**. Decisão 5 do spec P243 ("Sem `Content::Columns`/
`Colbreak` em P243 — fase (b) DEBT-56 pendente") foi
internamente coerente com o spec, mas o spec assumiu base
factual incorrecta — situação que o **audit C1 deveria ter
detectado e accionado `P243.div-1`** mas não detectou.

P244 corrige a documentação alinhando-a com o estado empírico
do repositório, sem afectar comportamento observable.

**Tests esperados**: 2198 → **2198 verdes preservado** (passo
documental; zero código alterado).
**Adaptações pre-existentes**: N=0 (paridade absoluta P156A +
ADR-0062-create + P160A).

---

## §2 Verificação empírica pré-P244 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=6 → 7 cumulativo

**Audit C1 é o substantivo material de P244**, não preâmbulo.
Distintamente dos passos precedentes (P240-P243) onde audit C1
informava materialização subsequente, em P244 o audit **é** o
trabalho — fixar o estado factual real.

**Verificação empírica via comandos `grep`** (já confirmada pelo
humano; documentar literalmente em §2 do relatório):

| Aspecto | Verificação empírica | Achado |
|---------|---------------------|--------|
| `Content::Columns` em `01_core/src/entities/content.rs` | `grep -n "Columns\|Colbreak"` | **CONFIRMADO** existe — variant em linha 901 com fields `count`/`gutter`/`body`; construtor `Content::columns` em linha 1134; arms cascata em is_empty/plain_text/PartialEq/map_content/map_text presentes |
| `Content::Colbreak` em `content.rs` | mesmo `grep` | **CONFIRMADO** existe — variant em linha 555 com field `weak`; construtor em linha 1066; arms cascata presentes |
| `native_columns` em `01_core/src/rules/stdlib/` | `grep -rn "native_columns"` | **CONFIRMADO** — function em `stdlib/layout.rs:1138`; registado em `stdlib/mod.rs:51`; ~13 unit tests `p218_native_columns_*` em `stdlib/mod.rs` linhas 2865-3000 |
| `native_colbreak` em stdlib | mesmo `grep` | **CONFIRMADO** — function em `stdlib/layout.rs:1209`; registado em `stdlib/mod.rs:51`; ~6 unit tests `p220_native_colbreak_*` |
| Status DEBT-56 em `00_nucleo/DEBT.md` | `grep "DEBT-56"` | **ENCERRADO (Passo 221) ✓** confirmado |
| Status ADR-0078 em `00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md` | `head -10` | **IMPLEMENTADO** (P215 PROPOSTO 2026-05-12 → P221 IMPLEMENTADO 2026-05-12) confirmado |
| Status ADR-0061 em `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md` | `head -10` | **IMPLEMENTADO** (PROPOSTO 2026-04-25 → IMPLEMENTADO 2026-05-12 P221; Fase 1+2+3 cumpridas) confirmado |
| Relatórios P217-P221 em `00_nucleo/materialization/` | `ls typst-passo-21*-relatorio.md` | **CONFIRMADO** existem |
| Content variants pós-P243 reportado em P243 §8 | Contagem real | P243 reporta **62**; pré-P217 era 56 (P156I) + 1 (P217 Columns = 57) + 1 (P220 Colbreak = 58); falta verificar empíricamente quais 4 variants adicionais (P227-P232 Categoria A Fase 5 + P240/P241 StateDisplay/CounterDisplayCallback são candidatos) |
| Stdlib funcs pós-P243 reportado em P243 §8 | Contagem real | P243 reporta **64**; pré-P217 era 42 (P156I) + 1 (P218 native_columns = 43) + 1 (P220 native_colbreak = 44); falta verificar quais 20 funcs adicionais foram registadas em P222+ |
| §A.5 Layout distribuição pós-P243 | Contagem real | P243 reporta **12/4/2/0/0** mas P221 reporta `12/1/5/0/0` pós-fechamento Fase 3; divergência material a clarificar |
| Cobertura Layout per metodologia pós-P243 | Contagem | P243 reporta **~91-92%** após scope-outs promovidos; P221 já reportava saturação Fase 3 — qualquer valor abaixo de 100% Layout indica refinos qualitativos restantes (`place` parcial, `measure` parcial scope-out Fase 4 candidata) |

**Sem `P244.div-N`** — passo é puramente documental; divergências
detectadas em §2 são objecto do trabalho do passo, não bloqueadores.

---

## §3 Decisões fixadas P244 — 7 decisões

### Decisão 1 — Naming "M7+3 fase (a)" preservado como descritivo, não factual

Preservar terminologia "M7+3 fase (a)" no relatório P243 + spec
P243 como **descrição interna da intenção P243** (extensão
`Regions` + scope-outs promovidos), **mas anotar publicamente
em ADR-0081 + relatório P244** que:
- "Fase (a) DEBT-56" e "Fase (b) DEBT-56" são labels históricos
  cuja **materialização real ocorreu em Linha A** (P216A+P216B =
  fase a; P217-P220 = fase b; P221 = encerramento).
- "M7+3 fase (a) P243" é **extensão posterior** ortogonal a Linha
  A — adiciona `backlog`/`last`/`advance` à `Regions` minimal
  (Linha A só populava `current`). Útil para multi-region flow
  real **Fase 4 candidata futura** (declarado scope-out P219).
- Os 3 scope-outs promovidos em P243 (Pad.right + Block.width +
  Boxed.width) **não dependem de Linha A** — são promoção real
  isolada via `regions.current.width` save/restore. Beneficiam
  da infraestrutura Linha A mas não são Linha A.

### Decisão 2 — ADR-0081 anotada para clarificar M7+3

ADR-0081 §"Plano materialização" tem 5 sub-passos (M7+1 a M7+5).
Pós-P244, M7+3 fica anotado:
- **M7+3 fase (a) infrastructure** ✓ P243 — extensão `Regions`
  + scope-outs ≥3 promovidos.
- **M7+3 fase (b)** — **NÃO APLICÁVEL** porque já materializado
  em Linha A P217-P221 anterior à abertura de ADR-0081 P239.
- M7+3 fica **CUMPRIDO cumulativamente** sem precisar de passo
  novo "fase (b)" — fechamento estrutural via reconhecimento
  empírico Linha A pré-existente.

ADR-0081 IMPLEMENTADO parcial: **4/5 → 4.5/5** (M7+1, M7+2,
M7+3 fase (a) + Linha A consumindo fase (b), M7+5) com M7+4
Place float pendente.

### Decisão 3 — Ajuste §A.5 Layout distribuição factualmente

P221 reporta `12/1/5/0/0` (~89-94% se métrica final P221).
P243 reporta `12/4/2/0/0`. Divergência material: P243 conta
4 parciais (provavelmente `columns`/`colbreak`/`place`/`measure`
recategorizados) + 2 ausentes (qual?).

**Decisão fixada**: P244 audita §A.5 empíricamente em
`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
e fixa a distribuição factual real. Se P243 erradamente
descontou `columns`/`colbreak` para ausente (assumindo "fase
(b) pendente"), corrigir para parcial paridade P221.

### Decisão 4 — Sem ADR nova; sem reabertura de ADR existente

P244 não cria ADR nova nem reabre ADR pré-existente. Anotações
cumulativas em:
- ADR-0081 §"Plano materialização" — esclarecimento M7+3
  cumulativo (Decisão 2).
- ADR-0080 §"Excepções" — registar lição P244 sobre detecção
  de divergências factuais via audit C1.

**Padrão "audit C1 obrigatório bloqueante"** N=6 → **7
cumulativo** — P244 é primeira aplicação onde o **falhanço
do audit C1 prévio** (P243 não detectou Columns/Colbreak
pré-existentes) é objecto explícito do passo subsequente.
Lição refinada: audit C1 deve grep variants `Content::*`
candidatas antes de assumir ausência.

### Decisão 5 — Patterns emergentes anotados; nenhum atinge limiar formalização sozinho

**Sub-padrão inaugurado P244 N=1**: "Reconciliação documental
pós-divergência factual entre planeamento e materialização".
Candidato a formalização N=3-4 se outras aplicações ocorrerem.

**Sub-padrões já N≥2 que tocam P244**:
- "Passo administrativo XS" N=5 → **6** (P156A + P156K +
  ADR-0062-create + P160A + P238 + **P244**). **N=6 ATINGE
  limiar formalização sólido**. Candidato a ADR meta dedicada
  em passo administrativo XS futuro (não-reservada per política
  P158).
- "Spec C1 audit obrigatório bloqueante" N=6 → **7** (P237 +
  P238 reescrito + P240 + P241 + P242 + P243 + **P244**) —
  primeira aplicação onde audit C1 É o substantivo, não
  preâmbulo.

### Decisão 6 — Tests baseline preservados literal

Zero código alterado em P244 → zero adaptações tests → **2198
verdes preservado** (paridade absoluta P156A + ADR-0062-create
+ P160A administrativos XS precedentes).

### Decisão 7 — L0 prompts intocados

P244 toca apenas ADRs + DEBT.md + inventário 148 + relatório.
Zero L0 hashes propagam. Paridade L0-baseline P156A + P156K
+ administrativos XS precedentes.

---

## §4 Ficheiros a editar (C2+C3+C4+C5)

### §4.1 ADR-0081 — anotação clarificação M7+3

Editar `00_nucleo/adr/typst-adr-0081-m7-pipeline-restructuring.md`
(naming exacto do ficheiro a confirmar empiricamente; pode ser
`adr-0081-m7-mais.md` ou similar):

Adicionar bloco no fim de §"Plano de materialização" ou §"Status":

```markdown
### Anotação P244 — Clarificação M7+3 cumulativo (2026-05-14)

**M7+3 fase (a)** ✓ materializado P243 (extensão `Regions {
backlog, last, advance }` + 3 scope-outs promovidos:
Pad.right + Block.width + Boxed.width).

**M7+3 fase (b)** — **NÃO APLICÁVEL via Linha B** porque
trabalho factualmente já materializado em **Linha A
pré-existente** (P217-P221, 2026-05-12) cobrindo:
- `Content::Columns { count, gutter, body }` variant (P217).
- `Content::Colbreak { weak: bool }` variant (P220).
- `native_columns` + `native_colbreak` stdlib (P218 + P220).
- Consumer multi-column real graded Opção B (P219).
- ADR-0078 IMPLEMENTADO; ADR-0061 IMPLEMENTADO; DEBT-56
  ENCERRADO (P221).

Linha A foi materializada **anterior à abertura de ADR-0081**
em P239; coexistência paralela inadvertida das duas linhas
até detecção empírica via grep em P244.

**M7+3 fica CUMPRIDO cumulativamente** sem precisar de passo
novo "fase (b)" em Linha B — fechamento estrutural via
reconhecimento empírico Linha A pré-existente.

Status pós-P244: **IMPLEMENTADO parcial 4.5/5** (M7+1 ✓; M7+2 ✓;
M7+3 ✓ via cumulativo; M7+5 ✓; M7+4 Place float pendente).
Promoção a IMPLEMENTADO total requer materialização M7+4 ou
scope-out formal humano.
```

### §4.2 ADR-0080 — anotação lição P244 sobre audit C1

Editar `00_nucleo/adr/typst-adr-0080-l0-minimal-refactors.md`
(naming a confirmar):

Adicionar entrada em §"Excepções" ou §"Lições refinadas":

```markdown
### Lição P244 — Audit C1 deve detectar variants Content já-existentes (2026-05-14)

P243 audit C1 capturou parcialmente o estado factual (descobriu
`Regions` já existia P216A/B → Decisão 2 spec corrigida) mas
**não auditou que `Content::Columns`/`Colbreak` também já
existiam** em `content.rs`. Resultado: Decisão 5 spec P243
("Sem `Content::Columns`/`Colbreak` em P243 — fase (b) DEBT-56
pendente") foi internamente coerente mas assumiu base factual
incorrecta.

**Refino procedural lição N=6 → 7 cumulativo "audit C1
obrigatório bloqueante"**: audit C1 deve incluir grep empírico
sistemático de variants `Content::*` candidatas, não apenas
abstracções estruturais (Regions, traits, etc.).

**Procedimento recomendado pós-P244**:
1. Identificar variants candidatas mencionadas no spec
   (`Content::Columns`, `Content::Colbreak`, etc.).
2. `grep -n "VariantName" 01_core/src/entities/content.rs`
   antes de assumir ausência.
3. Se variant existe → ajustar spec ou criar `Pxxx.div-N`
   conforme magnitude da divergência.
4. Se variant ausente → prosseguir com materialização.

Padrão N=6 → 7 cumulativo. Limiar formalização N=4
ultrapassado largamente; candidato a ADR meta em passo
administrativo XS futuro NÃO reservado.
```

### §4.3 Inventário 148 — ajuste §A.5 Layout

Editar
`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
§A.5 Layout:

Auditar empiricamente o estado actual:
- `columns(n)` — provavelmente **parcial** (per P221: variant
  + stdlib + consumer graded Opção B; multi-region flow real
  scope-out Fase 4 candidata).
- `colbreak()` — provavelmente **parcial** (per P220:
  variant + stdlib + arm downgrade graded; multi-region flow
  real scope-out paralelo).
- `place(...)` — preservado **parcial** (sem float/clearance
  — refino column scope na Fase 4 candidata; nada material
  em P217-P243 alterou isto).
- `measure` — preservado **parcial** (helper privado; sem
  stdlib expose; depende ADR-0066 Introspection PROPOSTO).

Distribuição factual a fixar empíricamente. **Hipótese**:
P221 distribuição `12/1/5/0/0` corrigida pelos refinos
cumulativos pós-P221 (P227-P232 Categoria A Fase 5, P240/P241
Categoria D walk-time real, P242 A.4 radius/clip parcial,
P243 scope-outs promovidos).

### §4.4 DEBT.md — confirmação DEBT-56 ENCERRADO

Verificar que DEBT-56 §"Justificação literal de fecho" inclui
referência cumulativa a P244 reconciliação se aplicável. Provável:
**sem alteração necessária** — DEBT-56 fechou correctamente
em P221; P244 só formaliza que Linha B não duplica este fecho.

### §4.5 Relatório P244

Criar `00_nucleo/materialization/typst-passo-244-relatorio.md`
com estrutura canónica passos administrativos XS (paridade
P156A, ADR-0062-create, P160A, P238):

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Verificação empírica via grep (literal output).
- §3 Linha A vs Linha B — tabela comparativa.
- §4 Origem da inconsistência (audit C1 P243 incompleto).
- §5 Decisões fixadas (7 decisões).
- §6 Ficheiros editados (ADR-0081, ADR-0080, inventário 148).
- §7 Patterns emergentes (1 inaugurado + 2 N≥6 atingindo
  limiar consolidação).
- §8 Próximo sub-passo pós-P244.

---

## §5 Critério aceitação P244 (C6+C7)

| Critério | Esperado |
|----------|----------|
| `cargo build --workspace` | **verde preservado** (zero código tocado) |
| `cargo test --workspace` | **2198 verdes preservado** (zero código tocado; N=0 adaptações) |
| `crystalline-lint .` | **0 violations preservado** |
| `crystalline-lint --fix-hashes` | **"Nothing to fix"** (paridade L0-baseline P156A + P156K + administrativos XS) |
| Content variants | **62 preservado** (zero alterações) |
| ShapeKind variants | **5 preservado** |
| Layouter fields | preservados |
| Regions fields | **3 preservado** (`current`, `backlog`, `last`) |
| Stdlib funcs | **64 preservado** |
| Scope-outs promovidos cumulativos | **3 preservado** (P242 radius+clip não contam aqui; ver §A.4 separadamente) |
| ADR-0081 status | IMPLEMENTADO parcial **4/5 → 4.5/5** (M7+3 cumulativo reconhecido) |
| ADR-0080 §"Lições refinadas" | entrada P244 lição audit C1 N=6→7 anotada |
| ADR-0061 status | **IMPLEMENTADO preservado** P221 (inalterado) |
| ADR-0078 status | **IMPLEMENTADO preservado** P221 (inalterado) |
| DEBT-56 status | **ENCERRADO preservado** P221 (inalterado) |
| Inventário 148 §A.5 | distribuição factualmente fixada (empírico audit C1) |
| L0 hashes propagados | **0** |
| Regressões reais | **0** (paridade absoluta) |

**Tests P244**: **zero novos** (paridade administrativos XS
precedentes). Verificação é via `cargo test --workspace`
comprovando preservação 2198 verdes.

**3 pré-condições obrigatórias verificadas** (per ADR-0081
§"Pré-condições obrigatórias" P239 — agora pós-P244
formalmente expandida):
1. **Tests baseline preservados**: 2198 verdes pré-P244 →
   2198 verdes pós-P244 (paridade absoluta; zero código).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   P244 não toca trait Introspector nem methods nem qualquer
   código L1; invariants preservados literal.
3. **Backward compat**: zero código alterado; toda funcionalidade
   pré-P244 preservada literal.

**Promoções ADR esperadas**:
- ADR-0081 IMPLEMENTADO parcial **4/5 → 4.5/5** (reconhecimento
  M7+3 cumulativo via Linha A pré-existente). Distribuição
  preservada literal — sem novos ADRs criados; sem PROPOSTO ↔
  IMPLEMENTADO global.
- ADR-0080 §"Lições refinadas" entrada P244 anotada N=6→7.
- ADR-0061 + ADR-0078 + DEBT-56 todos preservados sem alteração
  (factuais desde P221).

**Sem inventário 148 footnote nova** — P244 é reconciliação,
não materialização. Anotação inline em §A.5 distribuição
suficiente.

---

## §6 Próximo sub-passo pós-P244

P244 fecha reconciliação documental. Restantes pendentes do
roadmap pós-P243 §8 reconciliado:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **M7+4 Place float real** | Reabertura Opção B P219 graded; desbloqueia C.1 | L (~5-8h) | **alta** (último sub-passo M7+ pendente; fecha ADR-0081 IMPLEMENTADO total 5/5; magnitude isolada) |
| Cell layout migration → `regions.current.height` | Decisão 7 P243 diferida; activa A.4 breakable per-cell | M (~2-4h) | média (refino sequente P243 natural; activa scope-out P242) |
| Refino A.4 — `outset`+`fill`+`stroke` Block+Boxed | 3 dos 4 scope-outs restantes pós-P242 | S-M por attr | baixa-média |
| ADR meta admin XS — formalizar pattern "passo administrativo XS" N=6 | Promoção formal pattern N≥4 (atinge limiar sólido) | XS | média (patamar cumulativo claro pós-P244) |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |
| Pausa M-fase | Fase 5 graded ~93-94% estado preservado | XS | baixa |

**Recomendação subjectiva pós-P244**: **M7+4 Place float
real**. Último sub-passo M7+ pendente; fecha ADR-0081
IMPLEMENTADO total 5/5; magnitude L isolada (~5-8h); desbloqueia
C.1. Alternativa: ADR meta admin XS formalizar pattern N=6
"passo administrativo XS" — limiar sólido atingido.

**Decisão humana fica em aberto literal** pós-P244.

**Estado esperado pós-P244**:
- Tests workspace: **2198 verdes preservado**.
- Content variants: **62 preservado**.
- Regions fields: **3 preservado**.
- Stdlib funcs: **64 preservado**.
- Cobertura Layout per metodologia: **~91-94% preservado**
  (ajuste empírico §A.5 pode marginalmente alterar; sem ganho
  cumulativo material).
- Cobertura user-facing total: **~74-75% preservado**.
- **ADRs distribuição preservada literal**: PROPOSTO 12; EM VIGOR
  29; IMPLEMENTADO 22; total **68 preservado**. ADR-0081 transita
  4/5 → **4.5/5** internamente (reconhecimento M7+3 cumulativo);
  ADR-0061 + ADR-0078 + DEBT-56 inalterados (preservados P221).
- **Saldo DEBTs: 11 preservado**.
- **36 aplicações cumulativas anti-inflação** pós-P205D (+1 P244
  reconciliação preserva sem inflar).
- **Patterns emergentes pós-P244** (1 inaugurado + 2 consolidados
  N≥6):
  - "Reconciliação documental pós-divergência factual planeamento
    vs materialização" N=1 inaugurado P244.
  - "Passo administrativo XS" N=5 → **6 cumulativo** atinge
    limiar formalização sólido (P156A + P156K + ADR-0062-create
    + P160A + P238 + **P244**); candidato a ADR meta XS futuro.
  - "Spec C1 audit obrigatório bloqueante" N=6 → **7 cumulativo**;
    P244 primeira aplicação onde audit C1 é o substantivo,
    não preâmbulo; refinamento procedural "grep variants
    candidatas antes de assumir ausência".
- **Categoria D Fase 5 Layout: 3/? sub-passos materializados**
  preservado.
- **Categoria A.4 Fase 5 Layout**: parcial P242 preservado.
- **Fase 5 Layout candidata: 14/13-15 → 14/13-15 sub-passos
  preservado** (P244 administrativo não soma).
- **M9d / M7+ progresso**: **4.5/5 sub-passos materializados**
  (M7+1 ✓; M7+2 ✓; M7+3 ✓ via cumulativo Linha A + extensão
  Linha B P243; M7+5 ✓; M7+4 pendente). Cumulativa restante
  ~5-8h (M7+4 isolada).
- **Marco interno**: reconciliação documental Linhas A e B
  completa; inconsistência factual P243 corrigida; ADR-0081
  M7+3 cumulativo reconhecido formalmente; padrão administrativo
  XS atinge N=6 (limiar formalização sólido); padrão audit C1
  refinado procedimentalmente (grep variants antes de assumir
  ausência).

---

## §7 Notas operacionais para o executor

1. **Verificar empíricamente §2 antes de editar §4**. Os achados
   reportados pelo humano em 2026-05-14 são canónicos mas
   re-verificar via `grep` é cheap insurance contra mudança
   intermédia.

2. **Confirmar nome exacto do ficheiro ADR-0081** antes de
   editar. Provavelmente
   `typst-adr-0081-m7-pipeline-restructuring.md` mas naming
   pode variar (`-mais.md`, `-pipeline-mais.md`, etc.).

3. **Auditar §A.5 Layout empiricamente** — abrir o ficheiro
   `typst-cobertura-vanilla-vs-cristalino.md` e ler a
   distribuição actual. Se inconsistente com estado factual,
   corrigir. Distribuição `12/4/2/0/0` reportada em P243
   precisa de validação cruzada com factualidades P221 + P242
   + P243.

4. **Não criar ADR nova nem reabrir ADR-0061/0078/0061/DEBT-56**.
   Estado P221 desses artefactos é canónico; P244 apenas
   reconhece P243 não-conflito.

5. **Sem `P244.div-N` antecipado**. Passo é puramente documental;
   divergências detectadas são objecto do trabalho, não
   bloqueadores. **EXCEPÇÃO**: se §2 verificação empírica
   revelar que `Content::Columns`/`Colbreak` foram **removidos**
   entre P221 e o estado actual (regressão silenciosa não
   detectada em P222-P243), criar `P244.div-1` formal e
   parar para investigação humana.

6. **Custo real esperado**: ~30-60 min (paridade ADR-0062-create
   + P160A + P238 administrativos XS precedentes). Maior parcela:
   redacção do relatório P244 + auditoria empírica §A.5 (não
   código).

7. **Anti-inflação 36ª aplicação cumulativa** pós-P205D
   preservar: Opção α anotação cumulativa minimal + Opção α
   reconhecimento Linha A pré-existente + Opção β L0
   intocados + Opção α ADR-0081 promoção interna parcial 4/5
   → 4.5/5 (não completo).
