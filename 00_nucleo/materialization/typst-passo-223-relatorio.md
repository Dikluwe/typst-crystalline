# Relatório do passo P223 — `Content::Place` refino `float` + `clearance` (paridade graded)

**Data**: 2026-05-13.
**Spec**: `00_nucleo/materialization/typst-passo-223.md`.
**Tipo**: refino aditivo a variant existente `Content::Place`
(P84.5+P84.6 baseline); 2 fields novos armazenados mas
semantic real adiada graded (paridade pattern P156D/E
`weak`); DEBT-37 §"Divergência" fechada (Decisão 3 Opção α).
**Magnitude planeada**: M (~2-3h). **Magnitude real**: M
(~1.5h — abaixo do limite superior por C1 mostrar zero tests
afectados além de 1 integration test no 03_infra).
**Marco**: nenhum (décimo-primeiro passo pós-M9c; **segundo
sub-passo Fase 4 Layout candidata**; paridade estrutural
P218 para Fase 3 sub-fase b).

---

## §1 O que foi feito

P223 materializa refino aditivo a `Content::Place` adicionando
`float: bool` (default `false`; semantic real adiada per
pattern N=4 cumulativo `weak`/`breakable`/`float`) e
`clearance: Option<Length>` (default `None`; depende
`float: true` real; paridade Smart→Option N=7). **DEBT-37
§"Divergência" fechada** — restrição vanilla `scope: Parent
+ float: true` restaurada com erro hard (Decisão 3 Opção α).
**3 decisões fixadas**: Opção β float armazenado (pattern
graded N=4); Opção β clearance idem; Opção α DEBT-37
restrição vanilla literal restaurada. **Opção γ L0 sem
extensão** (pattern N=5 → 6). 14 tests novos (4 unit content
+ 8 unit stdlib + 2 E2E layout); workspace **1998 → 2012
verdes** (+14); 0 violations; 0 regressões reais (1 test
P84.6 adaptado adicionando `float: true` — DEBT-37 sentinela
intencional). §A.5 `place(...)` reclassificada `parcial ⁵`
→ `implementado⁺ ⁵ ⁴⁴`. **Cobertura Layout per metodologia:
78% → 83% real** (+5pp). Sem `P223.div-N`.

---

## §2 Inventário pré-P223 baseline P84.6 (C1)

`grep -n "Place {" entities/content.rs` + `grep -rn "Content::Place|PlaceScope::Parent"`:

- `entities/content.rs:301`: variant `Place { alignment:
  Align2D, dx: f64, dy: f64, scope: PlaceScope, body:
  Box<Content> }` — **5 fields baseline P84.5/P84.6**
  (`dx`/`dy` são `f64` — não `Length`; divergência face a
  spec C2 hipótese resolvida em §3).
- Arms cascata baseline:
  - `entities/content.rs:1340` (PartialEq), `1685`
    (map_content), `1912` (map_text).
  - `rules/introspect.rs:340` (materialize_time); `:1139`
    walk (recurse via destructure `{ body, .. }` —
    insensível ao refino).
  - `rules/layout/mod.rs:834` (layout_content arm).
  - `rules/introspect/locatable.rs:112` (catch-all —
    insensível ao refino).
- `native_place` em `stdlib/layout.rs:54-111` — validation
  3 named args (`dx`/`dy`/`scope`); estructura familiar
  paridade P156 patterns.

**Tests pre-existentes com `scope: "parent"` sem `float:
true`**: **N=1 test identificado** (`place_dentro_de_grid_com_scope_parent_ancora_a_pagina`
em `03_infra/src/integration_tests.rs:2087`). Paridade
hipótese spec N=1-3.

**Decisão crítica C1**: 1 test pre-existente precisará
adaptação intencional (adicionar `float: true`) per DEBT-37
restauração — paridade visual preservada literal porque
semantic real adiada per ADR-0054 graded.

Sem `P223.div-1` — empírico converge com hipótese.

---

## §3 Refino variant `Content::Place` (C2; +2 fields)

```rust
// ── Passo 84.5/84.6 baseline + P223 refino ──
Place {
    alignment: Align2D,
    dx:        f64,           // baseline P84.5
    dy:        f64,           // baseline P84.5
    scope:     PlaceScope,    // baseline P84.6
    float:     bool,          // P223 — semantic real adiada
    clearance: Option<Length>, // P223 — depende float real
    body:      Box<Content>,
},
```

**Decisões fixadas em C2**:
- 2 fields adicionados imediatamente antes de `body`
  (preserva ordem semântica baseline; minimiza diff
  textual).
- Docs inline na própria struct documentam Opção β graded
  + pattern N=4 cumulativo + DEBT-37 fecho.
- Sem helper construtor novo — `Content::Place { ... }`
  literal usado (paridade decisão P217 Columns).

**Divergência face a spec C2 hipótese**: spec antecipou
`dx: Length` + `dy: Length` mas baseline P84.5 usa `f64`.
P223 preserva literal — não converte. Sem `P223.div-N`
(divergência menor; afecta documentação não código).

---

## §4 Arms cascata + `native_place` refino + DEBT-37 restrição (C3 + C4)

**Arms cascata** (compiler-driven; 5 sítios refino):

1. `content.rs:1340` PartialEq — comparação 7 fields
   (paridade P217 Columns 3 fields refino).
2. `content.rs:1685` map_content — `*float` Copy + `*clearance`
   Copy via Option<Length>.
3. `content.rs:1912` map_text — idem map_content.
4. `introspect.rs:340` materialize_time — preserva
   `*float` + `*clearance`.
5. `layout/mod.rs:834` layout_content — destructure
   `float: _, clearance: _` (ignorados; semantic real
   adiada per ADR-0054 graded).

**Compiler reportou 10 errors E0027/E0063** — todos
endereçados sequencialmente.

**`native_place` refino**:
- `args.named.keys()` accept list expandida: `["dx", "dy",
  "scope"]` → `["dx", "dy", "scope", "float", "clearance"]`.
- Extract `float`: `Value::Bool` ou erro tipo + default
  `false`.
- Extract `clearance`: helper `extract_length` reuso
  **N=8 → 9** + validação negativo (paridade pattern P156I
  Stack.spacing).
- **DEBT-37 restrição vanilla restaurada** (Decisão 3
  Opção α):
  ```rust
  if matches!(scope, PlaceScope::Parent) && !float {
      return Err(/* "place: scope 'parent' requer float: true
          (paridade vanilla; sem float, scope 'parent' não
          tem semantic flutuante; fecha divergência DEBT-37
          documentada)" */);
  }
  ```
- `Content::Place { ..., float, clearance, ... }` construído
  literal.

---

## §5 Decisões substantivas

**Decisão 1 — `float: bool` Opção β (semantic adiada)**:
- vs Opção α float real (flow contorna; multi-pass layout)
  — rejeitada (L+ ~5-8h; reabre Opção B P219).
- vs Opção γ rejeitar `float: true` com erro hard —
  rejeitada (viola pattern P156+ graded).
- **Fixada**: armazenado mas semantic adiada per ADR-0054
  graded. **Pattern N=4 cumulativo** "Field armazenado
  semantic adiada" (P156D `weak` + P156E `weak` + P156G
  `breakable` + **P223 `float`**).

**Decisão 2 — `clearance: Option<Length>` Opção β
(semantic adiada; default None)**:
- vs Opção α default vanilla literal `1em` — rejeitada
  (Smart→Option pattern N=7 cumulativo).
- **Fixada**: `Option<Length>` default `None`; aceito
  silenciosamente sem `float: true` (paridade vanilla —
  vanilla também aceita; só tem efeito visível com float
  real).

**Decisão 3 — DEBT-37 restrição Opção α (restaurada)**:
- vs Opção β manter divergência DEBT-37 — rejeitada (viola
  comentário DEBT-37 explícito "quando float for adicionado,
  repor a restrição"; P223 é exactamente esse momento).
- vs Opção γ warning sem error — rejeitada (vanilla é
  hard error).
- **Fixada**: erro hard com mensagem clara referenciando
  DEBT-37. **Pattern emergente "fecho de divergência
  documentada via refino" N=1 inaugurado**.

**Decisão 4 — Opção γ L0 sem extensão** (pattern N=5 → 6):
- Refino aditivo a variant existente — **menos substantivo**
  que P156G+H+I (variants novos com secções L0 dedicadas).
- L0 `entities/content.md` Opção γ consolida pattern N=6
  P217+P218+P219+P220+P222+P223. N≥6 patamar empírico
  sólido; promoção formal Caminho 4 candidato.
- Hash `entities/content.md` preservado.

**Anti-inflação 17ª aplicação cumulativa** pós-P205D
(Opção β float adiada + Opção β clearance adiada + Opção
γ L0 sem extensão + sem helper construtor novo).

---

## §6 Resultados verificação (C7-C8)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde (após 10 errors E0027/E0063 sequenciais endereçados via compiler-driven cascada) |
| `cargo test --workspace` | 2012 verdes | **2012 verdes** (1723 + 242 + 24 + 2 + 21) ✓ |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | **"Nothing to fix"** ✓ (L0 `entities/content.md` Opção γ; hashes L1 propagados em content.rs + layout/mod.rs + stdlib/layout.rs + introspect.rs) |
| Tests P223 novos | 14 (4+8+2) | ✓ 14 verdes (4 unit content + 8 unit stdlib + 2 E2E layout) |
| Adaptações DEBT-37 | N=1-3 | **N=1** (`place_dentro_de_grid_com_scope_parent_ancora_a_pagina` em 03_infra) |
| Regressões reais pre-existente | 0 | **0** (1 adaptação intencional documentada) |
| Variant fields | 5 → 7 | ✓ 7 (`alignment`, `dx`, `dy`, `scope`, `float`, `clearance`, `body`) |
| Variants Content count | 56 (sem alteração) | ✓ 56 preservado |
| Stdlib funcs count | 56 (sem alteração) | ✓ 56 preservado |

**1 ajuste durante execução** (esperado per hipótese spec
N=1-3): test `place_dentro_de_grid_com_scope_parent_ancora_a_pagina`
adaptado adicionando `float: true` ao source typst;
paridade visual preservada literal (semantic real adiada
preserva geometria).

---

## §7 Inventário 148 reclassificação + ADR-0061 anotação (C9-C10)

**Inventário 148** (`typst-cobertura-vanilla-vs-cristalino.md`):

§A.5 Layout linha `place(alignment, ..., body)` (linha 137):
- Classificação: `parcial ⁵` → **`implementado⁺` ⁵ ⁴⁴**.
- Referência: "Passo 84.6" → "Passos 84.5 + 84.6 + 223".
- Nota: refino aditivo `float` + `clearance` armazenados
  (semantic real adiada per ADR-0054 graded; pattern N=4);
  DEBT-37 §"Divergência" fechada (Decisão 3 Opção α
  restaurada); flow real Fase 5 candidata NÃO-reservada.

Tabela A (linha 431 + 436):
- Layout footnotes: `⁴⁰ ⁴¹ ⁴² ⁴³` → `⁴⁰ ⁴¹ ⁴² ⁴³ ⁴⁴`.
- Distribuição §A.5: `12/2/4/0/0 = 18 → **12/3/3/0/0 =
  18**` (1 parcial → impl⁺; zero ausentes preservado).
- Total user-facing: `68/25/26/20/2 = 141 → **68/26/25/20/2
  = 141**` (1 parcial → impl⁺).
- **Cobertura Layout per metodologia**: `(12+3)/18 =
  **83%**` real (+5pp vs P222 78%; **divergência reaberta
  com paridade visual histórica §2.1 blueprint** que ainda
  lista 78% Opção γ baseline; actualização possível).
- Cobertura user-facing: `(68+26)/141 ≈ **67%**` (+1pp).

Tabela B.2 Content variants — linha `Place` actualizada
com 2 fields novos + nota DEBT-37 fechada + nota
implementado⁺ pela limitação semantic adiada.

**Footnote ⁴⁴ adicionada** (~60 linhas) documentando:
- 2 fields novos + semantic adiada.
- DEBT-37 §"Divergência" fechada + 1 test pre-existente
  adaptado.
- 3 decisões fixadas (Opção β float/clearance + Opção α
  DEBT-37).
- Pattern N=4 "Field armazenado semantic adiada".
- Pattern N=5 → 6 "L0 minimal para refactors".
- Pattern N=1 "refino aditivo a variant existente"
  inaugurado pós-M9c.
- Pattern N=1 "fecho de divergência documentada via
  refino" inaugurado.
- Helper `extract_length` reuso N=8 → 9.
- Δ Layout cobertura: 78% → 83% real (+5pp); cumulativo
  Fase 4: +11pp.
- 14 tests + 1 adaptação intencional.

**ADR-0061** (`typst-adr-0061-layout-fase-x-roadmap.md`):

Bloco `### P223 anotação — Fase 4 Layout candidata
sub-passo 2 (Place refino +float +clearance + DEBT-37
fecho)` adicionado após `### P222 anotação` em §"Aplicações
cumulativas":
- 2 fields + arms cascata + native_place refino +
  DEBT-37 fecho.
- 4 patterns emergentes registados (N=4, N=6, N=1, N=1).
- 14 tests + 1 adaptação intencional.
- Reclassificação + Δ cobertura.
- **Fase 4 Layout candidata 2/3 sub-passos** (P222 ✓;
  P223 ✓; P224 grid refino pendente).
- **Status ADR-0061 IMPLEMENTADO mantido**.

**DEBT-37** preservado ENCERRADO P84.6; divergência
documentada agora fechada estructuralmente em P223
(anotação histórica via ADR-0061 + inventário 148
footnote ⁴⁴).

---

## §8 Próximo sub-passo

P223 fecha segundo sub-passo Fase 4 Layout candidata
(série α "terminar Layout" Opção α P221 §8). Decisão
humana sobre próxima sessão entre opções:

| Caminho | Trabalho | Magnitude | Prioridade subjectiva |
|---------|----------|-----------|------------------------|
| **Caminho 1** | **P224 grid refino** (gutter/header/footer/colspan; DEBT-34d/e separado) — terceiro sub-passo Fase 4 candidata (Opção α P221 §8 completa série α) | M-L (~2-4h) | alta (sub-passo grandes; Layout cobertura 83% → potencial 89% se materializado completo; **fecha série α**) |
| **Caminho 2** | Pivot Bloco D — ADR-0064 saturação ou outra meta administrativa (paridade pattern N=6 L0 minimal + N=4 Field armazenado semantic adiada) | XS-S (~30min-1h) | baixa (consolidação metodológica) |
| **Caminho 3** | Pivot outro módulo (Visualize 54%; Text features 52%; Markup 78%; Model 50%) | varia | baixa-média |
| **Caminho 4** | Adiar Layout completo; outro objectivo arquitectural | varia | baixa |

**Recomendação subjectiva**: **Caminho 1 (P224)** — fecha
série α "terminar Layout" completa (3/3 sub-passos Fase 4
candidata); preserva momentum cumulativo P222+P223+P224;
grid refino é maior trabalho mas pode ser graded (focar 1-2
sub-features per sub-passo se preferível).

**Estado pós-P223**:
- Tests workspace: 1998 → **2012 verdes** (+14 P223; 1
  adaptação DEBT-37 intencional).
- Stdlib funcs: 56 (sem alteração; native_place refinado
  não-novo).
- Content variants: 56 (sem alteração; Place refinado +2
  fields).
- §A.5 distribuição: `12/3/3/0/0 = 18` (1 parcial →
  impl⁺; zero ausentes preservado).
- **Cobertura Layout per metodologia**: 78% → **83%**
  real (+5pp; +11pp cumulativo Fase 4 P222+P223).
- Cobertura user-facing total: 66% → **67%** (+1pp).
- ADR-0066 PROPOSTO mantido; ADR-0061 IMPLEMENTADO
  mantido; ADR-0078 IMPLEMENTADO mantido.
- DEBT-37 §"Divergência" fechada via P223 (anotação
  histórica; ENCERRADO P84.6 preservado).
- Saldo DEBTs: 13 abertos (preservado).
- **17 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern emergente "L0 minimal para refactors" N=5 →
  **6**** (P217+P218+P219+P220+P222+**P223**).
- **Pattern emergente "Field armazenado semantic adiada"
  N=3 → **4 cumulativo**** (P156D/P156E/P156G/**P223**).
- **Pattern emergente "refino aditivo a variant existente"
  N=1 inaugurado pós-M9c** (Place refino + 2 fields).
- **Pattern emergente "fecho de divergência documentada
  via refino" N=1 inaugurado** (DEBT-37 §"Divergência"
  fechada exactamente quando float adicionado).
- **Helper `extract_length` reuso N=8 → 9**.
- **Fase 4 Layout candidata 2/3 sub-passos** (P222 measure
  ✓; P223 place ✓; P224 grid refino pendente).
- **Divergência reaberta cobertura Layout real vs visual**:
  83% real per metodologia vs 78% Opção γ §2.1 blueprint
  histórica. Actualização possível "12 impl + 3 impl⁺ + 3
  parcial" — diferida (S documental se humano priorizar).
