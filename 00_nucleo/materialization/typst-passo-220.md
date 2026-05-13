# Passo 220 — `Content::Colbreak` agregado (variant + stdlib + arm graded)

**Série**: 220 (sexto sub-passo materialização Layout Fase
3; quarto e último sub-fase (b) DEBT-56; **fecha sub-fase
(b)**).
**Marco**: nenhum (nono passo pós-M9c; primeiro passo
agregado pós-M9c — variant + stdlib + arm em sub-passo
único paridade P156E pagebreak).
**Tipo**: aditivo agregado (variant + arm + stdlib) com
mudança observable graded (paridade P156E literal).
**Magnitude**: S+ (~1.5h).
**Pré-condição**: P219 concluído (`Content::Columns` consumer
real graded; width reduzida; sub-fase (b) DEBT-56: 3/4
sub-passos; 1972 tests verdes); ADR-0078 PROPOSTO anotada
P217+P218+P219; humano fixou Caminho 1 (continuação Caminho
1 P219 §8 recomendação); `Layouter::new_page` disponível em
`cursor.rs:128` (precedente P156E pagebreak); arm
`Content::Pagebreak` em `layout_content` como referência
estrutural.
**Output**: 1 ficheiro relatório curto + código alterado em
3 ficheiros L1 + L0 `entities/content.md` extensão minimal
(linha em tabela paridade decisão empírica P217+P218+P219)
+ ADR-0078 anotada (sem transição de status).

---

## §1 Trabalho

P217 + P218 + P219 materializaram `Content::Columns` em
**3 sub-passos atomizados** (variant + stdlib + arm). P220
fecha sub-fase (b) DEBT-56 (4/4) materializando
`Content::Colbreak { weak: bool }` em **sub-passo agregado
único** (variant + arm + stdlib) — paridade literal P156E
`Pagebreak` que também foi agregado.

**Decisão arquitectural central (3 decisões fixadas)**:

### Decisão 1 — Agregação vs separação

- **Argumentos a favor de agregação** (P220 único):
  - Precedente directo: P156E `Pagebreak` agregado (variant
    + arm + stdlib num passo — modelo Decisão 1 ADR-0061).
  - Arm é trivial (downgrade a pagebreak via
    `Layouter::new_page` reuso).
  - Sem consumer substantivo separado para extrair como
    sub-passo (vs P219 que era consumer substantivo).
- **Argumentos contra** (separar P220 + P220B):
  - Atomização ADR-0036 cumprida em P217+P218+P219.
  - Coerência metodológica recente.

**Decisão fixada**: **agregação P220 único** (paridade P156E
+ anti-inflação 15ª aplicação cumulativa). Atomização não
é dogma absoluto — separação ADR-0036 vale quando há
componentes substantivamente distintos (P217 variant +
P218 stdlib + P219 consumer). Em P220, arm é trivial;
separação seria over-engineering.

### Decisão 2 — Semantic do arm Layouter (Opção β graded)

Vanilla: `colbreak` força salto para próxima coluna; se
não há próxima coluna disponível, downgrade a pagebreak.

Cristalino pós-P219: Opção B graded fixada (sem multi-region
flow real); **não há "próxima coluna" disponível** a saltar.

**3 opções para arm Layouter**:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | Stub transparente puro (no-op) | colbreak invisible; viola paridade vanilla |
| **β** | `colbreak` → `pagebreak` (downgrade graded) | paridade vanilla literal — vanilla também downgrade fora de columns context |
| γ | Stub dentro de columns; erro fora | complica arm; viola pattern P217+ |

**Decisão fixada**: **Opção β — colbreak == pagebreak graded**.

**Justificação literal**:
- Vanilla também downgrade colbreak a pagebreak quando
  fora de columns context (consultar `lab/typst-original/.../
  layout/columns.rs` para verificação documental).
- Em P219 cristalino, **tudo** está fora de "columns context
  real" (single-region per Opção B P219); então colbreak ≡
  pagebreak sempre.
- Reusa `Layouter::new_page` (paridade P156E literal — zero
  refactor estrutural).
- Mudança observable controlada (colbreak produz break
  visível como pagebreak).

### Decisão 3 — Atributo `weak: bool` (paridade Pagebreak)

`Colbreak { weak: bool }` paridade literal `Pagebreak { weak,
to: Option<Parity> }` mas **sem `to`**:
- Vanilla `ColbreakElem` não tem campo de paridade (paridade
  só faz sentido para páginas).
- `weak: bool` armazenado mas semantic adiada (paridade
  P156D HSpace/VSpace, P156E Pagebreak).

Reuso de dados (sem recolha nova):

- P156E `Pagebreak` arm Layouter — precedente literal.
- `Layouter::new_page` em `cursor.rs:128`.
- ADR-0078 PROPOSTO §"Decisão" sub-passo 4.
- ADR-0054 graded autorizando downgrade.

---

## §2 Cláusulas (10)

### C1 — Confirmar contagem `Content` enum pós-P219

Auditoria empírica:

```
grep -c "^    [A-Z][A-Za-z]\+\(\s\|{\|(\)" 01_core/src/entities/content.rs
```

Hipótese: **55 variants** pós-P219 (P217 adicionou Columns;
P218+P219 sem alteração ao enum). Pós-P220: **56 variants**
(+1 Colbreak).

Se contagem divergir: registar `P220.div-1`.

### C2 — Adicionar `Content::Colbreak` variant

Editar `01_core/src/entities/content.rs` adicionando variant
imediatamente após `Pagebreak` (paridade ordem semântica):

```rust
/// Quebra de coluna manual — Fase 3 Layout per ADR-0078
/// PROPOSTO. Adicionado em P220 (DEBT-56 sub-fase b
/// 4/4 — fecha sub-fase b estructuralmente).
///
/// **Semantic graded P220**: em cristalino pós-P219
/// (Opção B graded — sem multi-region flow real), colbreak
/// downgrade a pagebreak literal. Paridade vanilla:
/// vanilla também downgrade fora de columns context.
///
/// Quando consumer multi-region real existir
/// (P-Layout-Fase4 candidato), arm pode ser refinado para
/// salto entre regions reais.
Colbreak {
    /// `weak: bool` — armazenado mas semantic de collapse
    /// adiada (paridade `Pagebreak { weak }` P156E).
    weak: bool,
},
```

**Decisão atributos**:
- **`weak: bool`** paridade `Pagebreak.weak` (P156E). Sem
  default per variant; stdlib (`native_colbreak`) tem
  default `false`.
- **Sem `to: Option<Parity>`** — paridade vanilla
  `ColbreakElem` que não tem (paridade só faz sentido em
  páginas).

Variant count Content: **55 → 56**.

### C3 — Arms exhaustivos em ~5-8 sítios L1

Cobertura paridade `Pagebreak` (P156E) literal:

**`entities/content.rs`** (5 arms):
- `is_empty` — sempre **`false`** (event observável; cf.
  Pagebreak + Divider).
- `plain_text` — `String::new()` (event sem texto).
- `PartialEq::eq` — comparação 1-field (`weak`).
- `map_content` — terminal (clone directo).
- `map_text` — terminal (clone directo).

**`rules/introspect.rs`** (2 arms):
- `materialize_time` — no-op (sem children; preserva).
- `walk` — no-op (sem children; sem tag).

**`rules/layout/mod.rs::layout_content`** (1 arm — Opção
β downgrade):

```rust
Content::Colbreak { weak: _ } => {
    // Opção β graded: downgrade a pagebreak literal
    // (paridade vanilla quando fora de columns context).
    if self.regions.current.cursor_x.0
        > self.regions.current.line_start_x.0 {
        self.flush_line();
    }
    self.new_page();
}
```

**`rules/layout/mod.rs::measure_content_constrained`** (1
arm — paridade Pagebreak measure):
- Retorna `(0.0, 0.0)` ou paridade Pagebreak (no-op em
  measure context).

**`rules/introspect/locatable.rs`** (catch-all):
- Colbreak **não-locatable**; cai em `_ => false`.

Total: **8 arms** em 4 ficheiros (paridade P217).

Helpers de construção:
- `Content::colbreak(weak: bool) -> Content` construtor
  Rust.

### C4 — `native_colbreak` em stdlib

Adicionar em `01_core/src/rules/stdlib/layout.rs` após
`native_columns` (paridade ordem ADR-0061 Fase 3 sub-passos
3→4):

```rust
/// Stdlib `colbreak(weak: false)` — Layout Fase 3 per
/// ADR-0078 PROPOSTO.
///
/// Forma: `#colbreak()` ou `#colbreak(weak: true)`.
///
/// Semantic graded P220 — colbreak downgrade a pagebreak
/// pós-P219 (Opção B graded; sem multi-region flow real).
pub fn native_colbreak(
    _ctx: &mut EvalContext<'_>,
    args: &Args,
    _world: &dyn World,
    _file: FileId,
    _figure_numbering: &FigureNumberingState,
) -> SourceResult<Value> {
    // 1. Sem argumentos posicionais.
    if !args.items.is_empty() {
        return Err(eco_format!(
            "colbreak: não aceita argumentos posicionais"
        ));
    }

    // 2. Extract weak (named opcional; default false).
    let weak = match args.named.get("weak") {
        Some(Value::Bool(b)) => *b,
        Some(other) => return Err(eco_format!(
            "colbreak(weak): espera Bool, recebeu {}",
            value_type(other)
        )),
        None => false,
    };

    // 3. Reject unknown named args.
    for key in args.named.keys() {
        if key != "weak" {
            return Err(eco_format!(
                "colbreak: named arg desconhecido `{}` (esperado: weak)",
                key
            ));
        }
    }

    // 4. Build variant.
    Ok(Value::Content(Content::Colbreak { weak }))
}
```

**Re-export** em `stdlib/mod.rs`:
```rust
pub use crate::rules::stdlib::layout::{
    ..., native_colbreak, native_columns, ...
};
```

**Scope register** em `eval/mod.rs` (paridade P218 pattern):
```rust
scope.define("colbreak", Value::Func(Func::native(
    "colbreak", native_colbreak,
)));
```

Stdlib funcs count: **54 → 55**.

### C5 — Sentinelas P220

Tests unitários P220 em `content.rs::tests` (paridade
P156E pagebreak — 5 unit + 4 layout E2E):

- `p220_colbreak_variant_existe` — instancia
  `Content::colbreak(false)` + verifica fields.
- `p220_colbreak_is_empty_sempre_false` — paridade
  Pagebreak/Divider.
- `p220_colbreak_plain_text_vazio`.
- `p220_colbreak_partial_eq_1_field` — `eq` compara
  `weak`.
- `p220_colbreak_map_content_terminal` — clone directo.

Tests unitários `stdlib/layout.rs::tests` (paridade
P218 native_columns — 6 unit tests):

- `p220_native_colbreak_sem_args_aceita` — `colbreak()`
  produz `Content::Colbreak { weak: false }`.
- `p220_native_colbreak_weak_true_aceita` — `colbreak(weak:
  true)` produz `weak: true`.
- `p220_native_colbreak_posicional_rejeita` —
  `colbreak("oops")` falha.
- `p220_native_colbreak_weak_nao_bool_rejeita` —
  `colbreak(weak: "true")` falha.
- `p220_native_colbreak_named_desconhecido_rejeita` —
  `colbreak(foo: bar)` falha.
- `p220_native_colbreak_to_rejeita` — `colbreak(to: "even")`
  falha (paridade `Pagebreak` mas sem `to` em colbreak).

Layout E2E tests em `tests.rs` (4 tests):

- `p220_colbreak_produz_new_page_downgrade` — `#colbreak()`
  + texto subsequente produz 2 páginas (1 vazia/curta + 1
  com texto).
- `p220_colbreak_dentro_columns_downgrade_graded` —
  `#columns(2)[#colbreak[after]]` produz pagebreak literal
  (P219 single-region scope-out preserva downgrade).
- `p220_colbreak_misturado_com_pagebreak` —
  `#colbreak()#pagebreak()` produz mesma quantidade de
  páginas que `#pagebreak()#pagebreak()` (downgrade graded).
- `p220_colbreak_no_inicio_documento_pagina_vazia` —
  `#colbreak[texto]` produz 2 páginas (paridade pagebreak).

Total tests P220: **5 unit content + 6 unit stdlib + 4 E2E
= 15 tests**.
Esperado pós-P220: **1972 + 15 = 1987 verdes**.

### C6 — L0 `entities/content.md` extensão minimal

Decisão sobre L0:
- **Opção α** — secção dedicada `## Variant Content::Colbreak
  (Passo 220)`.
- **Opção β** — linha minimal em tabela ou nota inline.
- **Opção γ** — sem extensão L0 (paridade decisão empírica
  P217+P218+P219).

**Hipótese provável**: **Opção γ** consolida pattern
"L0 minimal para refactors" N=3 → 4. P220 é aditivo + arm
trivial; documentação inline-doc em código suficiente.

Hash propagado **só se `entities/content.rs` mudar**
(content.md não tocado). Se Opção γ confirmada, "Nothing
to fix" esperado pós-`--fix-hashes`.

### C7 — Verificação tests workspace

Critério: 1972 verdes pré-P220 + 15 novos = **1987 verdes**.

```
cargo test --workspace 2>&1 | tail -20
```

**Erro tolerado**: zero. P220 é aditivo + arm trivial; sem
mudança a features existentes.

Hipótese provável: 1987 verdes; zero regressões
pre-existente.

### C8 — Verificação lint

```
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 0 violations. Hash content.rs propagado (variant
novo +5 arms).

### C9 — Inventário 148 reclassificação P220

**§A.5 Layout** linha `colbreak()`: reclassificação
`ausente` → **`parcial`** (paridade P219 `columns`).

Justificação literal:
- Variant + stdlib + arm real (downgrade graded) existem.
- Multi-region flow real ausente (paridade `columns`
  scope-out).
- Per ADR-0054 graded: `parcial` reflecte feature
  user-facing com limitação documentada.

**Tabela A.5 Layout**: `13/1/4/0/0 = 18 → 13/1/5/0/0 = 18`
(pós-P219 já tinha 4 parciais; P220 +1 → 5). Wait — recalcular:
- Pré-P219: `13/1/3/1/0 = 18` (1 ausente = columns).
- Pós-P219: `13/1/4/0/0 = 18` (columns ausente→parcial).
- Pós-P220: **erro** — colbreak também é parcial, mas
  estava ausente. Re-distribuir.

**Recálculo correcto**:
- Pré-P219 actual: `13/1/3/1/0 = 18` (columns ausente;
  colbreak não está em §A.5 separado? Verificar empíricamente
  em C1).
- **Possível**: §A.5 lista columns + colbreak como entradas
  separadas. Audit empírico necessário.
- Se colbreak listada separada: pós-P220 `13/1/5/(-1)/0`
  — recálculo conforme C1.

**Decisão fixada em C9**: re-audit §A.5 empíricamente
antes de fixar distribuição. Sem inflação documental
prematura.

**Tabela B.2 Content variants**: actualização **diferida a
P221** (paridade decisão empírica P217+P218+P219).

**Footnote 41 P220** documenta:
- Variant Colbreak + arm downgrade graded.
- Opção β graded (paridade vanilla).
- Reclassificação `ausente` → `parcial`.

### C10 — ADR-0078 anotação P220 + sub-fase (b) FECHADA

`00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md`
§"Plano de materialização" anotado com bloco
**`### P220 materializado 2026-05-12`**:

```markdown
Sub-fase (b) DEBT-56 — quarto e ÚLTIMO sub-passo (fecha
sub-fase b inteira):
- Variant `Content::Colbreak { weak: bool }` adicionado
  (Content variants 55 → 56).
- 8 arms exhaustivos em 4 ficheiros L1 (paridade P217
  Columns mas leaf sem body).
- `native_colbreak(weak: false)` stdlib registada (54 →
  55 stdlib funcs).
- Arm `layout_content` Opção β — downgrade a pagebreak
  literal (paridade vanilla quando fora de columns
  context).
- Reusa `Layouter::new_page` (paridade P156E Pagebreak
  literal).
- 15 tests adicionados (5 unit content + 6 unit stdlib +
  4 E2E). Tests workspace: 1972 → 1987 verdes.
- §A.5 `colbreak()` reclassificada `ausente` → `parcial`.
- **Sub-fase (b) DEBT-56 fechada: 4/4 sub-passos
  materializados** (P217 ✓, P218 ✓, P219 ✓, P220 ✓).

Status ADR-0078: **PROPOSTO mantido**. Transição
IMPLEMENTADO ocorre em P221 (encerramento Fase 3 +
DEBT-56 fecha).
```

**Status ADR-0078**: PROPOSTO mantido. Marco interno:
sub-fase (b) fechada estructuralmente. P221 é
encerramento documental.

---

## §3 Output

1 ficheiro relatório:
`00_nucleo/materialization/typst-passo-220-relatorio.md`.

Estrutura (~6-8 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Confirmação contagem Content (C1).
- §3 Variant Colbreak (C2; 1 field).
- §4 Arms exhaustivos (C3; 8 arms / 4 ficheiros).
- §5 `native_colbreak` + arm downgrade graded (C4; Opção
  β fixada).
- §6 Resultados verificação (15 tests + 1972 pre-existentes
  preservados).
- §7 Inventário 148 reclassificação `colbreak ausente →
  parcial` + ADR-0078 anotação P220.
- §8 Próximo sub-passo (**P221 encerramento Fase 3**;
  DEBT-56 fecha; ADR-0078 PROPOSTO → IMPLEMENTADO;
  ADR-0061 promoção candidata).

Código alterado:
- **Editado**: `01_core/src/entities/content.rs` (+ variant
  `Colbreak` + 5 arms + 5 unit tests).
- **Editado**: `01_core/src/rules/introspect.rs` (+ 2 arms).
- **Editado**: `01_core/src/rules/introspect/locatable.rs`
  (catch-all preserva; +1 arm explicit possível).
- **Editado**: `01_core/src/rules/layout/mod.rs` (+ 1 arm
  layout_content downgrade + 1 arm measure_content_constrained).
- **Editado**: `01_core/src/rules/stdlib/layout.rs`
  (`native_colbreak` + 6 unit tests).
- **Editado**: `01_core/src/rules/stdlib/mod.rs` (re-export).
- **Editado**: `01_core/src/rules/eval/mod.rs` (scope register).
- **Editado**: `01_core/src/rules/layout/tests.rs` (+ 4 E2E
  tests).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (§A.5 reclassificação + footnote 41).
- **Editado**: `00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md`
  (+ anotação P220 fecha sub-fase b).

**Sem novos ficheiros**.

---

## §4 Não-objectivos

- Implementar multi-region flow real para colbreak (salto
  entre colunas) — diferido a P-Layout-Fase4 candidato
  (Opção A multi-region; não-reservado per política P158).
- Reabrir decisão P216B (`Regions { current }` minimal) —
  preservada literal.
- Adicionar `backlog`/`last` a `Regions` — diferido a
  P-Layout-Fase4.
- Promover ADR-0078 → IMPLEMENTADO — só P221.
- Fechar DEBT-56 — só P221 (P220 fecha sub-fase b; P221
  encerramento documental).
- Reclassificar §A.5 `colbreak` a `implementado` —
  reclassificação a `parcial` (paridade `columns` P219).
- Show rules `#show colbreak: ...` — fora de escopo Fase
  3 cristalino.
- L0 extensão formal — Opção γ paridade empírica
  P217+P218+P219.
- Atualização Tabela B.2 Content — diferida P221.
- Separação P220 (variant) + P220B (stdlib) + P220C (arm)
  — agregação fixada (anti-inflação 15ª aplicação).

---

## §5 Riscos a evitar

1. **Esquecer arms em sítios exhaustivos**: ~8 sítios L1
   match exhaustivo. Mitigação: compiler errors detectam;
   iterar até zero errors (paridade P217 estratégia).
2. **Arm `walk` produz `Tag::Start/End` espúrio**:
   Colbreak não-locatable (paridade Pagebreak P156E).
   Arm walk deve ser no-op (sem tag).
3. **Downgrade no-op em vez de new_page**: tentação de
   "stub transparente" Opção α. Rejeitada — sem efeito
   observable viola paridade vanilla. Mitigação:
   `Layouter::new_page` reuso literal P156E.
4. **`weak` semantic implementado**: tentação de fazer
   collapse weak adjacentes. Rejeitada per consistência
   P156D/P156E — `weak` armazenado mas semantic adiada.
   Anti-inflação cumulativo.
5. **`to` field adicionado**: tentação de paridade total
   Pagebreak. Rejeitada — vanilla `ColbreakElem` não tem
   `to` (paridade só faz sentido em páginas). Audit
   `lab/typst-original/.../layout/columns.rs` confirma.
6. **Separação P220 + P220B + P220C**: tentação de
   atomização extrema. Rejeitada — arm trivial; agregação
   paridade P156E. Anti-inflação 15ª aplicação cumulativa.
7. **L0 extensão prematura**: tentação de criar secção
   dedicada (Opção α). Rejeitada — Opção γ paridade
   empírica P217+P218+P219; pattern N=3 → 4.
8. **Mudança observable em `Content::Pagebreak`**:
   refactor de arm pagebreak para reuso compartilhado
   com colbreak. Rejeitada — preservar pagebreak literal;
   colbreak duplica trivialmente o pattern.
9. **`#colbreak()` E2E test rendering**: tests E2E
   verificam `pages.len()` (não geometric); paridade
   estratégia P156E.
10. **Footnote 41 inflada**: paridade footnote 40 P219
    estrutura concisa. Sem detalhe técnico excessivo.

---

## §6 Hipótese provável

C1 confirmará Content em 55 variants pós-P219.

C2 adicionará `Colbreak { weak: bool }` em ~5 LOC + 5
sentinelas.

C3 cobrirá ~8 arms em 4 ficheiros (compiler-driven; paridade
P217 estratégia).

C4 criará `native_colbreak` em ~30 LOC + 6 unit tests +
re-export + scope register (paridade P218 pattern).

C5 fixará 4 E2E tests verificando downgrade graded.

C6 fixará Opção γ (sem extensão L0; consolida pattern
empírico N=3 → 4).

C7 reportará 1987 tests verdes (1972 + 15).

C8 reportará 0 violations.

C9 reclassificará §A.5 `colbreak` ausente → parcial; audit
empírico para confirmar distribuição Tabela A.5.

C10 anotará ADR-0078 P220 + sub-fase (b) DEBT-56 fechada
estructuralmente; promoção a IMPLEMENTADO diferida P221.

Custo real: S+ (~1.5h). Maior parcela em C3 (arms
exhaustivos compiler-driven) + C5 (15 tests).

Mas é hipótese, não decisão. C1-C10 fixam-se empíricamente.

---

## §7 Particularidade P220

P220 é estruturalmente distinto na trajectória pós-M9c:

- **Primeiro sub-passo agregado pós-M9c** — variant + arm +
  stdlib num único sub-passo. Paridade literal P156E
  pagebreak. Distinto de P217+P218+P219 atomizado.
  **Anti-inflação 15ª aplicação cumulativa** — atomização
  ADR-0036 não é dogma absoluto.
- **Pattern emergente "stub transparente → consumer real
  graded" N=1 estável** — P220 NÃO segue o pattern P219
  (consumer real graded único); P220 agrega num sub-passo.
  Razão: arm trivial vs arm substantivo P219.
- **Fecha sub-fase (b) DEBT-56 estructuralmente** — 4/4
  sub-passos materializados (P217 + P218 + P219 + P220).
  Marco interno; P221 é encerramento documental + ADR
  transição.
- **Opção β graded — downgrade colbreak = pagebreak** —
  paridade vanilla literal documentada. Refino multi-region
  flow real fica como P-Layout-Fase4 candidato
  (não-reservado).
- **Pattern "L0 minimal para refactors" N=3 → 4** — P217+
  P218+P219+**P220** consolidam convenção. Possível
  formalização como ADR meta documental se N≥5.
- **Reclassificação §A.5 `colbreak` ausente → parcial** —
  segunda reclassificação Layout pós-M9c (1ª foi P219
  `columns`). Padrão: features Fase 3 cristalino são
  `parcial` por scope-out multi-region flow real.

Por isso §5 risco 6 (separação over-engineering) é o mais
provável. Tentação óbvia é "manter atomização P217+P218+P219".
Defesa: precedente directo P156E pagebreak; arm é trivial;
agregação economiza overhead documental.

**Critério de aceitação P220**:
- 15 tests novos verdes (5 unit content + 6 unit stdlib +
  4 E2E).
- 1972 tests pre-existentes preservados verdes.
- 0 violations.
- Content variants 55 → 56.
- Stdlib funcs 54 → 55.
- §A.5 `colbreak()` reclassificada ausente → parcial.
- **Sub-fase (b) DEBT-56 fechada estructuralmente** (4/4).

**Estado pós-P220 esperado**:
- Sub-fase (a) DEBT-56: 2/2 ✓ (P216A + P216B).
- Sub-fase (b) DEBT-56: 4/4 ✓ (P217 + P218 + P219 + P220).
- DEBT-56 estructuralmente completo; falta só P221
  documental.
- ADRs: ADR-0078 PROPOSTO; ADR-0061 PROPOSTO ~75%
  concluído (Fase 1 ✓, Fase 2 ✓, Fase 3 4/5 features —
  só measure/place refinos pendentes).
- Layout cobertura agregada **78% preservada** (parcial
  fora numerador). 2 reclassificações qualitativas
  (`columns` + `colbreak` ausente → parcial; zero ausentes
  Layout pós-P220).
