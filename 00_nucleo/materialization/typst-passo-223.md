# Passo 223 — `Content::Place` refino `float` + `clearance` (paridade graded)

**Série**: 223 (nono sub-passo Layout pós-M9c; **segundo
sub-passo Fase 4 Layout candidata**; série α "terminar
Layout" Opção α P221 §8 — 4 sub-passos cumulativos).
**Marco**: nenhum (décimo-primeiro passo pós-M9c; segundo
Fase 4 Layout — paridade estrutural P218 para Fase 3 sub-fase
b).
**Tipo**: refino aditivo a variant existente `Content::Place`
(P84.5+P84.6 baseline); 2 fields novos armazenados mas
semantic adiada graded (paridade pattern P156D/E `weak`).
**Magnitude**: M (~2-3h).
**Pré-condição**: P222 concluído (`native_measure` stdlib
expose; helper visibility promotion; 1998 tests verdes;
§A.5 `measure(body)` impl⁺; cobertura Layout 78% real per
metodologia coincide com paridade visual histórica; Fase
4 Layout candidata 1/3 sub-passos); `Content::Place {
alignment, dx, dy, scope, body }` existe desde P84.5/P84.6
(DEBT-37 ENCERRADO); `PlaceScope { Column, Parent }` em
`entities/layout_types.rs`; `native_place` em `stdlib/layout.rs`
com argumentos `alignment`, `dx`, `dy`, `scope`; humano
fixou continuação série α P222.
**Output**: 1 ficheiro relatório curto + código alterado em
`entities/content.rs` (variant refino + arms cascata) +
`stdlib/layout.rs` (native_place validação + helpers) +
`rules/introspect.rs` (arm cascata) + L0 `entities/content.md`
extensão Opção γ (decisão pattern N=5→6 ou reabertura Opção
α para refactor substantivo) + ADR-0061 anotação Fase 4
candidata 2/3.

---

## §1 Trabalho

`Content::Place` existe desde P84.5+P84.6 com 5 fields
(`alignment`, `dx`, `dy`, `scope`, `body`). **2 fields
vanilla ausentes** per inventário 148 §A.5 linha 137: `float`
e `clearance`. DEBT-37 §"Divergência face ao vanilla a
documentar" anotou explicitamente: "o vanilla restringe
`Parent` a `float: true` (erro caso contrário, collect.rs:309).
O cristalino não tem `float` implementado — `Parent` é aceite
incondicionalmente, com efeito visual de ancoragem à página
sem layout flutuante. **Quando `float` for adicionado, repor
a restrição**".

**P223 é esse momento.** Refino aditivo:
- **`float: bool`** field novo (default `false`) — armazenado;
  semantic real adiada (paridade pattern P156D/E `weak`).
- **`clearance: Option<Length>`** field novo (default `None`)
  — armazenado; semantic real adiada (depende float real).
- **Restrição vanilla `Parent` exige `float: true`** restaurada
  — `place(..., scope: "parent")` sem `float: true` agora
  rejeitado com erro hard (fecha divergência DEBT-37
  documentada).

**Decisão arquitectural central — 3 decisões fixadas**:

### Decisão 1 — `float: bool` semantic real ou adiada?

Vanilla `float: true` muda comportamento substantivo:
elemento flutua para topo/fundo da página/coluna; conteúdo
fluído contorna-o. Cristalino actual não tem multi-region
flow real (decisão Opção B P219 graded preserva-se).

**3 opções consideradas**:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | Float real (flow contorna; multi-pass layout) | **L+ ~5-8h**; reabre Opção B P219 ; risco estrutural alto |
| **β** | `float: bool` armazenado; semantic adiada | **S+**; paridade ADR-0054 graded; consistente P156D/E pattern |
| γ | Rejeitar `float: true` com erro hard | Impede paridade gradual; viola Opção β graded P156+ pattern |

**Decisão fixada — Opção β** (paridade ADR-0054 graded
literal; pattern N=3 cumulativo `weak`/`breakable`/`float`
armazenados mas semantic adiada).

**Justificação literal**:
- Pattern P156D `HSpace.weak`/`VSpace.weak` armazenado mas
  collapse defere — precedente N=1.
- Pattern P156E `Pagebreak.weak` armazenado mas collapse
  defere — precedente N=2.
- Pattern P156G `Block.breakable` armazenado mas layouter
  não impede quebra — precedente N=3.
- **P223 `Place.float` armazenado mas flow real adiado** —
  precedente N=4 cumulativo.

### Decisão 2 — `clearance: Option<Length>` semantic real?

Vanilla `clearance` só faz sentido com `float: true`. Sem
float real, clearance é semantic vazia.

**Decisão fixada — Opção β** (consistente com `float`).

- `clearance: Option<Length>` armazenado mas adiada.
- Default vanilla é `1em`; cristalino usa `None` (paridade
  pattern Smart→Option N=6 P217 cumulativo).
- Validação semântica `clearance` sem `float: true`: warning?
  erro? **Decisão fixada**: aceitar silenciosamente
  (paridade vanilla — vanilla também aceita; clearance
  só tem efeito visível com float).

### Decisão 3 — Restrição `Parent` exige `float: true`?

DEBT-37 §"Divergência" anotou explicitamente: "**quando
`float` for adicionado, repor a restrição**". P223 é
exactamente esse momento.

**3 opções consideradas**:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | Restrição vanilla literal: `scope: Parent` sem `float: true` → erro hard | Paridade vanilla literal restaurada; fecha divergência DEBT-37 |
| β | Sem restrição (manter divergência) | Paridade reduzida |
| γ | Warning sem error | Inferior a α (vanilla é hard error) |

**Decisão fixada — Opção α** (restaura paridade vanilla;
fecha divergência DEBT-37 documentada).

**Validação em `native_place`**:
```
scope: "parent" + float: false  →  ERRO "place: scope 'parent' requer float: true"
scope: "column" + float: any    →  OK
scope: "parent" + float: true   →  OK
```

**Cuidado com regressões**: P84.6 testes pre-existentes
que usem `scope: "parent"` sem `float: true` quebram. Audit
empírico em C1 + adaptação testes (paridade pattern P217+).

Reuso de dados (sem recolha nova):

- `Content::Place { alignment, dx, dy, scope, body }` baseline
  P84.5/P84.6.
- `PlaceScope { Column, Parent }` em `entities/layout_types.rs`.
- `native_place` em `stdlib/layout.rs` — refino aditivo.
- Pattern P156D/E `weak` semantic adiada — precedente.
- DEBT-37 §"Divergência" — autorização documentada para Opção α.
- ADR-0054 graded — base teórica.
- ADR-0064 Caso D — atributos aditivos paridade.

---

## §2 Cláusulas (10)

### C1 — Inventário pré-P223: confirmar `Content::Place` baseline + testes existentes

Auditoria empírica:

```
grep -n "Place {" 01_core/src/entities/content.rs
grep -c "Content::Place" 01_core/src/
grep -n "scope:" 01_core/src/rules/stdlib/layout.rs | head -10
grep -rn "scope.*parent\|scope.*Parent" 01_core/src/ | head -5
```

Hipótese:
- `Content::Place` variant em `entities/content.rs` com 5
  fields.
- Arms em `entities/content.rs` (5: `is_empty`, `plain_text`,
  `PartialEq`, `map_content`, `map_text`), `rules/introspect.rs`
  (2: `materialize_time`, `walk`), `rules/layout/mod.rs`
  (1: `layout_content`), `rules/introspect/locatable.rs`
  (catch-all ou explicit). Total ~7-8 arms.
- `native_place` validation atrás de `extract_alignment` +
  `dx`/`dy`/`scope`.
- Testes pre-existentes P84.5/P84.6 + alguns L3 com `scope:
  "parent"` — possível N=1-3 testes que precisam ajuste.

Se contagem ou estado divergir: registar `P223.div-1`.

**Decisão crítica C1**: identificar testes pre-existentes
que usem `scope: "parent"` sem `float: true` — devem ser
adaptados (adicionar `float: true`) ou movidos para sentinela
de regressão (validation P223 error).

### C2 — Refino variant `Content::Place` (+2 fields)

Editar `01_core/src/entities/content.rs`:

```rust
// ── Passo 84.5/84.6 baseline (já existe) ──
// ── Passo 223 refino: +float +clearance ──
Place {
    alignment: Align2D,
    dx: Length,
    dy: Length,
    scope: PlaceScope,
    /// `float: bool` — Passo 223. Armazenado; semantic
    /// real adiada per ADR-0054 graded (precedente N=4 
    /// `weak`/`breakable`/`float` cumulativos).
    /// Default `false` (paridade vanilla).
    float: bool,
    /// `clearance: Option<Length>` — Passo 223. Armazenado;
    /// semantic real adiada (depende float real).
    /// Default `None` (paridade pattern Smart→Option N=7
    /// cumulativo).
    clearance: Option<Length>,
    body: Box<Content>,
},
```

**Decisões fixadas em C2**:
- 2 fields adicionados ao final da struct (antes de `body`)
  — preserva ordem semântica baseline; minimiza diff.
- Defaults documentados na própria struct (paridade decisão
  P217).
- Sem helper construtor novo `Content::place_floating(...)`
  — overhead inflacionário; construtor existente
  `Content::place(...)` recebe 2 args novos.

### C3 — Arms cascata em ~7-8 sítios L1

Compiler-driven (paridade P217 estratégia):

**`entities/content.rs`** (5 arms — refino, não-aditivo):
- `is_empty` — `Content::Place { body, .. } => body.is_empty()`
  preservado (atributos não afectam).
- `plain_text` — recurse no body preservado.
- `PartialEq::eq` — **mudança**: comparação 7-fields (2 novos).
- `map_content` — preservar `float`, `clearance` como Copy.
- `map_text` — idem.

**`rules/introspect.rs`** (2 arms — preservados):
- `materialize_time` — recurse no body; preserva 6 atributos
  (incluindo 2 novos).
- `walk` — recurse no body; preservado.

**`rules/layout/mod.rs::layout_content`** (1 arm — refino
mínimo):
```rust
Content::Place { alignment, dx, dy, scope, float, clearance, body } => {
    // Pattern existing P84.6 preservado.
    // float + clearance armazenados mas IGNORADOS no layout
    // (semantic real adiada per ADR-0054 graded).
    // [...lógica P84.6 existing...]
}
```

**`rules/layout/mod.rs::measure_content_constrained`** (se
existir arm Place; auditar em C1):
- Preservar; sem mudança.

**`rules/introspect/locatable.rs`** (catch-all ou explicit):
- Content::Place continua não-locatable (sem mudança).

Total: **5 arms refino** + **3 arms preservados** = ~8 arms
total.

### C4 — Refino `native_place` stdlib

Editar `01_core/src/rules/stdlib/layout.rs` função `native_place`:

```rust
pub fn native_place(_ctx, args, ...) -> SourceResult<Value> {
    // ... existing alignment/dx/dy/scope extraction ...

    // Passo 223 — extract `float` (default false).
    let float = match args.named.get("float") {
        Some(Value::Bool(b)) => *b,
        Some(other) => return Err(eco_format!(
            "place(float): espera Bool, recebeu {}",
            value_type(other)
        )),
        None => false,
    };

    // Passo 223 — extract `clearance` (default None;
    //                                  reuso extract_length N=9).
    let clearance = match args.named.get("clearance") {
        Some(val) => Some(extract_length(val, "place", "clearance")?),
        None => None,
    };

    // Passo 223 — validar clearance >= 0 (paridade pattern P156I).
    if let Some(c) = &clearance {
        if c.is_negative() {
            return Err(eco_format!(
                "place(clearance): negativo rejeitado"
            ));
        }
    }

    // Passo 223 — DEBT-37 restrição restaurada (Decisão 3 Opção α).
    if matches!(scope, PlaceScope::Parent) && !float {
        return Err(eco_format!(
            "place: scope 'parent' requer float: true \
             (paridade vanilla; sem float, scope 'parent' \
             não tem semantic flutuante)"
        ));
    }

    // ... existing named arg loop check ...
    // Adicionar "float" e "clearance" à lista de keys aceites.

    Ok(Value::Content(Content::Place {
        alignment, dx, dy, scope, float, clearance,
        body: Box::new(body),
    }))
}
```

**Magnitude isolada**: XS (~15min).

**Helper `extract_length` reuso N=8 → 9** via `clearance`.

### C5 — Sentinelas P223

Tests unitários P223 em `content.rs::tests` (refino variant):

- `p223_place_variant_aceita_float_clearance` — instancia
  `Content::Place { ..., float: true, clearance: Some(1em),
  ... }`.
- `p223_place_default_float_false_clearance_none` —
  construtor Rust com defaults preservados.
- `p223_place_partial_eq_inclui_float_clearance` — `eq`
  compara 7 fields.
- `p223_place_map_content_preserva_atributos` — `map_content`
  retorna Place com mesmos float/clearance.

Tests unitários `stdlib/layout.rs::tests` (refino native_place):

- `p223_native_place_float_aceita` — `place(top, float: true,
  body)` produz `float: true`.
- `p223_native_place_float_default_false` — `place(top, body)`
  produz `float: false`.
- `p223_native_place_float_nao_bool_rejeita` —
  `place(top, float: "yes", body)` falha.
- `p223_native_place_clearance_length_aceita` — `place(top,
  float: true, clearance: 2em, body)` produz `clearance:
  Some(2em)`.
- `p223_native_place_clearance_negativo_rejeita`.
- `p223_native_place_parent_sem_float_rejeita` (Decisão 3
  Opção α) — `place(top, scope: "parent", body)` SEM
  `float: true` → erro com mensagem clara.
- `p223_native_place_parent_com_float_aceita` —
  `place(top, scope: "parent", float: true, body)` OK.
- `p223_native_place_column_sem_restricao` — `place(top,
  scope: "column", body)` OK independente de float.

Layout E2E tests em `tests.rs` (2 tests):
- `p223_place_float_armazenado_layout_preservado` — body
  com `place(top, float: true, body)` renderiza preservando
  baseline P84.6 (semantic adiada — sem flow real).
- `p223_place_clearance_armazenado_layout_preservado` —
  body com `place(top, float: true, clearance: 2em, body)`
  renderiza preservando baseline.

**Sentinela de regressão DEBT-37**:
- Possíveis testes pre-existentes P84.6 com `scope: "parent"`
  sem `float: true` — adaptar adicionando `float: true`.
  Documentar em §6 risco 7.

Total tests P223: **4 unit content + 8 unit stdlib + 2 E2E
= 14 tests**.
Esperado pós-P223: **1998 + 14 = 2012 verdes**.

### C6 — L0 `entities/content.md` decisão Opção γ vs Opção α

Decisão sobre L0:
- **Opção α** — secção dedicada `## Variant Content::Place
  refino — Passo 223` (paridade Stack/Block secções
  P156G+).
- **Opção β** — linha em tabela existente.
- **Opção γ** — sem extensão L0 (pattern P217+P218+P219+P220+P222
  N=5).

**Hipótese provável**: **Opção γ** consolida pattern N=5 →
6. Refino aditivo (não-novo variant) tem ainda menos
justificação para L0 extensão formal que refactor estrutural.

Hash `entities/content.md` preservado se Opção γ.

**Reabertura Opção α justificada?**: refino aditivo a variant
existente é **menos substantivo** que P156G+H+I (variants
novos com secções dedicadas). Opção γ é consistente. Pattern
"L0 minimal para refactors" N=5 → 6 fortalece-se.

### C7 — Verificação tests workspace

Critério: 1998 verdes pré-P223 + 14 novos = **2012 verdes**.

**Risco regressão DEBT-37 documentado**: tests pre-existentes
P84.6 que usem `scope: "parent"` sem `float: true` quebram
após Decisão 3 Opção α. **Estes não são regressões** — são
restauros de paridade vanilla intencionais. Adaptação:
adicionar `float: true` ao test, ou converter o test em
sentinela de regressão verificando que erro é correctamente
disparado.

Hipótese provável N tests afectados: 1-3 (audit C1
determinará).

**Critério ajustado**: 1998 verdes preservados (após
adaptação de N tests pre-existentes; conta zero como regressão
real) + 14 novos = 2012 verdes.

### C8 — Verificação lint

```
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 0 violations. Hash propagado em
`entities/content.rs` (variant refino +2 fields, +arms
mudança em PartialEq/map) + `rules/stdlib/layout.rs`
(native_place refino). L0 `entities/content.md` não tocado
(Opção γ) — "Nothing to fix" esperado em L0.

### C9 — Inventário 148 reclassificação P223

Editar `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

**§A.5 Layout linha 137 `place(alignment, ..., body)`**:
- Classificação: `parcial ⁵` → **`implementado⁺` ⁴⁴**.
- Coluna "Cristalino": "Passo 84.6" → "Passos 84.5 + 84.6 +
  223".
- Nota: refino aditivo `float` + `clearance` armazenados
  (semantic adiada per ADR-0054 graded); restrição vanilla
  `Parent + float: true` restaurada (fecha divergência
  DEBT-37); reusa pattern P156D/E weak.

**Tabela A.5 Layout**: distribuição actualizada:
- Pré-P223 (pós-P222): `12/2/4/0/0 = 18`.
- Pós-P223: **`12/3/3/0/0 = 18`** (1 parcial → impl⁺).

**Cobertura Layout per metodologia §A.9**:
- Pré: `(12+2)/18 = 78%`.
- Pós: `(12+3)/18 = **83%**` (+5pp real).

**Tabela A user-facing total**: re-distribuição:
- Pré: `68/25/26/20/2 = 141`.
- Pós: `68/26/25/20/2 = 141` (1 parcial → impl⁺).
- Cobertura total: `(68+26)/141 ≈ **67%**` (+1pp real).

**Footnote ⁴⁴ P223** adicionada documentando:
- `Content::Place` refino aditivo +2 fields graded.
- 3 decisões fixadas (Opção β float adiada; Opção β
  clearance adiada; Opção α DEBT-37 restrição restaurada).
- Pattern emergente "Field armazenado semantic adiada" N=3
  → 4 cumulativo.
- Pattern "L0 minimal" N=5 → 6.
- Helper `extract_length` reuso N=8 → 9.
- Reclassificação parcial → impl⁺.
- Δ Layout cobertura: 78% → 83% real (+5pp).
- Δ user-facing total: 66% → 67% (+1pp).

**Tabela B.2 Content variants**: linha `Place` actualizada
com `float: bool` e `clearance: Option<Length>` fields novos
(variant existente refinado; count Content variants
preservado em 56).

### C10 — ADR-0061 anotação Fase 4 candidata 2/3 + DEBT-37 atualização

**ADR-0061** §"Aplicações cumulativas" anotação P223 (sem
transição de status; ADR-0061 IMPLEMENTADO mantido):

```markdown
### P223 anotação — Fase 4 Layout candidata sub-passo 2

`Content::Place` refino aditivo +2 fields graded materializado.
**Fase 4 Layout candidata 2/3** (P222 measure ✓; **P223
place ✓**; P224 grid pendente per Opção α P221 §8).

- 2 fields novos: `float: bool` (default false; semantic
  adiada paridade pattern P156D/E weak), `clearance:
  Option<Length>` (default None; depende float real).
- DEBT-37 §"Divergência" fechada — restrição vanilla `Parent
  + float: true` restaurada (Decisão 3 Opção α).
- Pattern emergente "Field armazenado semantic adiada" N=3
  → 4 (P156D weak; P156E weak; P156G breakable; **P223
  float**).
- 14 tests adicionados (4 unit content + 8 unit stdlib + 2
  E2E).
- Reclassificação §A.5 `place(...)` parcial → impl⁺.
- Cobertura Layout: 78% → 83% real (+5pp).

ADR-0061 status: **IMPLEMENTADO mantido** (Fase 3 fechada
P221; Fase 4 candidata 2/3).
```

**DEBT-37 §"Divergência face ao vanilla"**: anotar **fecho
em P223** (Decisão 3 Opção α restaurada; restrição vanilla
literal).

DEBT-37 está **já ENCERRADO** desde P84.6 (CLOSED) — não
reabre nem fecha; apenas anotação histórica da divergência
documentada agora ser fechada.

**Status ADR-0061**: IMPLEMENTADO mantido. Anotação Fase 4
candidata 2/3 sub-passo.
**Status DEBT-37**: ENCERRADO (P84.6) preservado; divergência
documentada fechada em P223 (anotação histórica).

---

## §3 Output

1 ficheiro relatório:
`00_nucleo/materialization/typst-passo-223-relatorio.md`.

Estrutura (~6-8 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Inventário pré-P223 baseline P84.6 (C1).
- §3 Refino variant `Content::Place` (C2; +2 fields).
- §4 Refino `native_place` stdlib + DEBT-37 restrição (C4).
- §5 Decisões substantivas (Opção β float adiada; Opção β
  clearance adiada; Opção α DEBT-37 restrição restaurada;
  Opção γ L0 pattern N=5→6).
- §6 Resultados verificação (14 tests + 1998 pre-existentes
  preservados após N adaptações DEBT-37).
- §7 Inventário 148 reclassificação `place` parcial → impl⁺
  + footnote ⁴⁴ + ADR-0061 anotação Fase 4 candidata 2/3.
- §8 Próximo sub-passo (P224 grid refino; Caminho 1 Opção
  α continuação).

Código alterado:
- **Editado**: `01_core/src/entities/content.rs` (variant
  refino +2 fields + arms cascata em 5 sítios + 4 unit tests).
- **Editado**: `01_core/src/rules/introspect.rs` (arms
  preservados — possível ajuste mínimo).
- **Editado**: `01_core/src/rules/layout/mod.rs` (arm
  Place desestrutura 2 fields novos; possível arm em
  `measure_content_constrained` ajustado).
- **Editado**: `01_core/src/rules/stdlib/layout.rs`
  (`native_place` refino: +2 named args + DEBT-37 validation;
  +8 unit tests).
- **Editado**: `01_core/src/rules/layout/tests.rs` (+2 E2E
  tests + adaptação N tests pre-existentes DEBT-37).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (Tabela A.5 + §A.5 reclassificação + Tabela B.2 actualização
  + footnote ⁴⁴ P223).
- **Editado**: `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`
  (+ anotação Fase 4 candidata sub-passo 2).

**Sem novos ficheiros**.

---

## §4 Não-objectivos

- Implementar float real (flow contorna; multi-pass layout)
  — diferido a Fase 5 candidata L+ não-reservado per
  política P158.
- Implementar clearance real — depende float real; idem
  diferido.
- Reabrir decisão P216B (`Regions { current }` minimal) —
  preservada literal.
- Refactor `Content::Place` para enum tagged — preserva
  struct + 7 fields actual.
- Tocar em `Align2D` — preservado literal P84.5.
- Tocar em `PlaceScope` enum — preservado literal P84.6
  (apenas `Parent` ganha validação obrigatória em
  `native_place`).
- Promover ADR-0066 → IMPLEMENTADO — fora de escopo P223;
  paridade decisão P222.
- Reclassificar `grid` ou `columns`/`colbreak` — diferidos
  a P224 e scope-out respectivamente.
- L0 extensão formal — Opção γ pattern N=5 → 6.
- Adicionar helper construtor `Content::place_floating()` —
  overhead inflacionário; usar `Content::place(...)` existente
  com args novos.

---

## §5 Riscos a evitar

1. **Refactor `Content::Place` em vez de aditivo**: tentação
   de reordenar fields ou refactor struct. **Rejeitada** —
   aditivo minimaliza diff e preserva baseline P84.6.
2. **Implementar float real "porque clearance precisa"**:
   tentação de "ir mais longe" implementando flow real.
   **Rejeitada** — Opção β graded literal; precedente N=3
   `weak`/`breakable` cumulativos.
3. **Default `clearance: Length::em(1.0)` em vez de None**:
   tentação de paridade vanilla literal default. **Rejeitada**
   — pattern Smart→Option N=7 cumulativo; `None` representa
   "default vanilla resolvido em uso".
4. **DEBT-37 restrição NÃO restaurada**: tentação de "manter
   divergência DEBT-37 actual porque float é semantic vazia".
   **Rejeitada** — DEBT-37 §"Divergência" explicitamente
   anotou "**quando float for adicionado, repor a restrição**".
   P223 é esse momento; fechar a divergência documentada.
5. **Aceitar tests pre-existentes P84.6 com `scope: "parent"`
   sem `float: true`**: regressão real OU adaptação
   intencional? **Decisão fixada em C7**: adaptação intencional;
   tests adicionam `float: true` ou viram sentinela de
   regressão verificando error correcto.
6. **Mensagem de erro DEBT-37 confusa**: vanilla diz "place
   with `scope: "parent"` must have `float: true`". Cristalino
   pode reescrever mais clara. **Decisão**: mensagem
   cristalino mais explícita do que vanilla (paridade
   ADR-0033 — texto reescrito mais claro mas sentido
   idêntico).
7. **Esquecer ajuste em `materialize_time` / `walk`**: arms
   em introspect.rs desestruturam 7 fields agora (em vez de
   5). Compiler-driven; iterar até zero errors.
8. **Esquecer ajuste em `PartialEq::eq`**: novo comparação
   tem 7 fields. Crítico para tests de igualdade.
9. **L0 extensão prematura**: tentação de criar secção
   dedicada (Opção α). Rejeitada — Opção γ pattern N=5 → 6
   estável.
10. **Mudança a `PlaceScope` enum**: tentação de adicionar
    variant `Both` ou similar. Rejeitada — DEBT-37 fechou
    PlaceScope com 2 variants literal vanilla; P223 só
    valida combinação na stdlib.

---

## §6 Hipótese provável

C1 confirmará `Content::Place { alignment, dx, dy, scope,
body }` em `entities/content.rs` + ~7-8 arms cascata. N=1-3
tests pre-existentes com `scope: "parent"` sem `float: true`
identificados.

C2 adicionará `float: bool` + `clearance: Option<Length>`
no final da struct; defaults documentados.

C3 cobrirá ~5 arms refino + ~3 preservados (compiler-driven).

C4 adicionará 2 named args extract + DEBT-37 validation +
helper `extract_length` reuso N=9.

C5 criará 14 tests novos (4+8+2) + sentinela regressão DEBT-37.

C6 fixará Opção γ (sem extensão L0; pattern N=5 → 6).

C7 reportará 2012 tests verdes (1998 + 14 novos; N=1-3
adaptados).

C8 reportará 0 violations.

C9 reclassificará `place(...)` parcial → impl⁺; cobertura
Layout 78% → 83%; user-facing 66% → 67%.

C10 anotará ADR-0061 Fase 4 candidata 2/3 + DEBT-37 anotação
histórica fecho divergência.

Custo real: M (~2-3h). Maior parcela em C5 (14 tests) +
C7 (audit + adaptação N=1-3 tests pre-existentes DEBT-37).

Mas é hipótese, não decisão. C1-C10 fixam-se empíricamente.

---

## §7 Particularidade P223

P223 é estruturalmente distinto na trajectória pós-M9c:

- **Segundo sub-passo Fase 4 Layout candidata** — paridade
  estrutural P218 (segundo Fase 3 sub-fase b).
- **Refino aditivo a variant existente** — distinto de
  P217 (variant novo) + P218 (stdlib novo) + P220 (variant
  + stdlib novo) + P222 (stdlib expose existente). P223 é
  **primeiro refino aditivo a variant existente pós-M9c**.
  Pattern emergente "refino aditivo +N fields" N=1
  inaugurado.
- **Pattern emergente "Field armazenado semantic adiada"
  N=3 → 4** — P156D weak + P156E weak + P156G breakable +
  **P223 float**. N=4 atinge limiar formalização N=3-4
  ultrapassado. **Promoção formal a ADR meta** documental
  fica como Caminho 4 candidato (P221 §8 diferido per
  política P158).
- **Pattern emergente "L0 minimal para refactors" N=5 →
  6** — P217+P218+P219+P220+P222+**P223** todos Opção γ.
  N≥6 patamar empírico extremamente sólido. Promoção formal
  fortemente justificada se humano priorizar Caminho 4.
- **Pattern "Smart→Option" N=6 → 7** — `clearance: Option<Length>`
  default None paridade vanilla default `1em` simplificada.
- **`extract_length` reuso N=8 → 9** — patamar crescente
  reforça candidatura helper público.
- **DEBT-37 §"Divergência" fechada** — primeira aplicação
  pós-M9c de fecho de divergência documentada. Pattern
  emergente "fecho de divergência documentada via refino"
  N=1 inaugurado.
- **Anti-inflação 17ª aplicação cumulativa** pós-P205D —
  Opção β float adiada + Opção β clearance adiada +
  Opção γ L0 sem extensão + sem helper construtor novo.
- **Cobertura Layout 78% → 83% real** per metodologia.
  **Segundo aumento real cumulativo** pós-Fase 3 fechada
  (P222 6pp + P223 5pp = 11pp cumulativo no início Fase 4).

Por isso §5 risco 2 (implementar float real) é o mais
provável. Tentação óbvia é "clearance só faz sentido com
float; implementar float". Defesa: Opção β graded literal;
pattern N=3 cumulativo precedente forte; recomendação
metodológica explícita.

**Critério de aceitação P223**:
- 14 tests novos verdes (4 unit content + 8 unit stdlib + 2
  E2E).
- 1998 tests pre-existentes preservados (após N=1-3
  adaptações DEBT-37; conta zero como regressão real).
- 0 violations.
- §A.5 `place(...)` reclassificada parcial → impl⁺.
- Cobertura Layout: 78% → **83%** real (+5pp).
- Cobertura user-facing total: 66% → **67%** (+1pp).
- Fase 4 candidata Layout: 1/3 → **2/3 sub-passos** (P222
  ✓; P223 ✓; P224 grid pendente).

**Estado pós-P223 esperado**:
- Tests workspace: 1998 → 2012 verdes.
- Stdlib funcs: 56 (sem alteração).
- Content variants: 56 (sem alteração; variant refinado).
- §A.5 distribuição: `12/3/3/0/0 = 18` (1 parcial → impl⁺;
  zero ausentes preservado).
- Cobertura Layout per metodologia: 78% → 83%.
- Cobertura user-facing total: 66% → 67%.
- ADR-0066 PROPOSTO; ADR-0061 IMPLEMENTADO; ADR-0078
  IMPLEMENTADO.
- DEBT-37 §"Divergência" fechada via P223 (anotação
  histórica; ENCERRADO preservado).
- Saldo DEBTs: 13 abertos (preservado).
- 17 aplicações cumulativas anti-inflação.
- Pattern "L0 minimal para refactors" N=5 → 6.
- Pattern "Field armazenado semantic adiada" N=3 → 4.
