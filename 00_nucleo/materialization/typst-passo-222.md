# Passo 222 — `native_measure` stdlib expose (Bloco C ADR-0066)

**Série**: 222 (oitavo sub-passo Layout pós-M9c; **primeiro
sub-passo Fase 4 Layout** — `measure(body)` stdlib expose;
abre série α de "terminar Layout" — 4 sub-passos cumulativos).
**Marco**: nenhum (décimo passo pós-M9c; primeiro Fase 4
Layout — paridade estrutural P217 para Fase 3; pattern
"encerramento Fase pós-M9c" N=1 → 2 esperado em P225).
**Tipo**: aditivo trivial à stdlib expondo helper privado
existente como function pública (paridade P156J `repeat`
single-render + P218 `native_columns`); zero refactor
estrutural.
**Magnitude**: S+ (~1-2h).
**Pré-condição**: P221 concluído (Fase 3 Layout fechada
estructuralmente; DEBT-56 ENCERRADO; ADR-0078+ADR-0061
IMPLEMENTADO; 1987 tests verdes; 0 violations); humano
fixou Opção α (4 sub-passos cumulativos); helper privado
`measure_content` em `01_core/src/rules/layout/helpers.rs`
existe (per inventário 148 §A.5 linha 151); ADR-0066
PROPOSTO existe (P160A criou; sem promoção a IMPLEMENTADO
em P222 per decisão graded).
**Output**: 1 ficheiro relatório curto + código alterado em
`stdlib/layout.rs` + `stdlib/mod.rs` (re-export) +
`eval/mod.rs` (scope register) + ADR-0066 anotação (sem
transição de status) + L0 stdlib.md decisão paridade P217-P220
(Opção γ provável — pattern N=4 → 5).

---

## §1 Trabalho

`measure(body)` foi declarado `parcial` em inventário 148
§A.5 linha 151 desde origem — helper privado
`measure_content` em `01_core/src/rules/layout/helpers.rs`
existe e é usado internamente pelo Layouter (em arms
`measure_content_constrained` e similar). **Sem stdlib
expose** — `measure` não é invocável de markup ou code.

P222 materializa expose graded — `native_measure(body) ->
Value::Dict { "width": Length, "height": Length }` —
paridade vanilla retornando dicionário com dimensões em
Length. Sem dependência de promoção ADR-0066 (helper é
puro single-pass; opera sobre Content já evaluated — não
precisa runtime queries genuínas).

**Decisão arquitectural central — 3 decisões fixadas**:

### Decisão 1 — Tipo de retorno: Dict vs Length composto

Vanilla `measure(body)` retorna **dictionary** com keys
`width` + `height` (ambos `length` type):

```typst
let dims = measure([Hello world])
// dims.width: 4.5em
// dims.height: 1.2em
```

**3 opções para cristalino**:

| Opção | Tipo retorno | Trade-off |
|-------|--------------|-----------|
| α | `Value::Dict { width: Length, height: Length }` | paridade vanilla literal |
| β | Tuple `(Length, Length)` ou novo `Value::Size` | mais ergonómico mas divergência observable |
| γ | Stub que retorna Dict vazio (transparente) | viola paridade vanilla |

**Decisão fixada**: **Opção α — Dict { "width": Length,
"height": Length }**.

**Justificação literal**:
- Paridade vanilla observable (`#measure(body).width` ↔
  `measure(body)[width]` literal funcional).
- `Value::Dict` já existe — sem novo tipo a introduzir.
- ADR-0033 paridade observable preservada.

### Decisão 2 — Promoção ADR-0066 vs paridade graded

ADR-0066 §"Plano promoção futuro" lista 3 condições para
PROPOSTO → IMPLEMENTADO:
1. Feature runtime queries genuína materializada (e.g.
   `state(key, init)`).
2. Pipeline `introspect` extendido com 2-pass.
3. Tests E2E feature observable user-facing.

`measure(body)` **NÃO satisfaz condição 1** — não é
runtime query genuína; é função pura sobre Content
evaluated. **Não promove ADR-0066** em P222.

**3 opções para promoção**:

| Opção | Acção | Trade-off |
|-------|-------|-----------|
| α | Promover ADR-0066 PROPOSTO → IMPLEMENTADO | viola §"Plano promoção" 3 condições; observable parcial |
| **β** | Anotar ADR-0066 §"Plano promoção" — `measure()` materializado independente | preserva integridade ADR-0066; reconhece graded |
| γ | Materializar sem mencionar ADR-0066 | viola pattern documentação |

**Decisão fixada**: **Opção β — anotação ADR-0066 sem
promoção**. ADR-0066 §"Plano promoção" bloco C cross-módulo
ganha anotação P222: `measure()` stdlib materializado
graded; runtime queries genuínas continuam diferidas.

### Decisão 3 — Width override aceito ou não

Vanilla `measure(body)` aceita opcional `width` parameter
para medir em região constrained:
```typst
measure(body, width: 5cm)  // width override
```

**3 opções para cristalino P222**:

| Opção | Suporte width | Trade-off |
|-------|---------------|-----------|
| α | Aceita `width: Length` named opcional | paridade vanilla completa |
| **β** | Sem width override (paridade graded) | scope-out ADR-0054; refino futuro candidato |
| γ | Aceita mas ignora silenciosamente | viola explicit error pattern P157-P218 |

**Decisão fixada**: **Opção β — sem width override**.

**Justificação literal**:
- Paridade ADR-0054 graded.
- Helper `measure_content` actual provavelmente assume width
  full-page (auditar em C1).
- Width override exige refactor multi-region scope (paridade
  decisão columns Opção B P219).
- Refino futuro candidato (não-reservado per política P158).
- `width` named arg em P222 **rejeitado** com erro hard
  (paridade pattern P217+ "scope-out atributos vanilla").

Reuso de dados (sem recolha nova):

- `measure_content` helper privado existente em
  `01_core/src/rules/layout/helpers.rs` (audit em C1).
- `Value::Dict` infraestrutura existente em `entities/value.rs`.
- Pattern P218 `native_columns` + P220 `native_colbreak`
  para stdlib registo (re-export + scope define).
- ADR-0066 §"Plano promoção" Bloco C cross-módulo
  documenta `measure()` como primeira feature do bloco.
- ADR-0054 graded para Opção β width override scope-out.

---

## §2 Cláusulas (10)

### C1 — Inventário pré-P222: confirmar `measure_content` signature

Auditoria empírica:

```
grep -n "fn measure_content\|pub fn measure" 01_core/src/rules/layout/helpers.rs
grep -n "measure_content_constrained" 01_core/src/rules/layout/mod.rs
```

Hipótese (per inventário 148 §A.5):
- `measure_content` é função privada (provavelmente
  `pub(super)` ou `pub(crate)`) em
  `01_core/src/rules/layout/helpers.rs`.
- Provavelmente signature: `fn measure_content(content:
  &Content, ...) -> (f64, f64)` ou `Size` ou `(Pt, Pt)`.
- Provavelmente assume width full-page.

Empíricamente verificar:
1. Localização exacta.
2. Signature (params + return).
3. Visibility (pub/pub(super)/pub(crate)/privada).
4. Se aceita width parameter ou não.
5. Se já tem testes próprios.

Se signature diferir significativamente da hipótese:
registar `P222.div-1` com ajustes ao plano.

**Decisão crítica em C1**: se helper for **privada
absoluta** sem visibilidade, precisa promoção a
`pub(crate)` ou `pub(super)` para uso da stdlib.
Promoção a visibility expandida pode ser sub-passo
implícito (não-óbvio).

### C2 — Promover visibility do helper (se necessário)

Se C1 confirma helper privada absoluta:
- Promover a `pub(crate)` (mínimo necessário para
  `stdlib/layout.rs` em mesmo crate).
- **Não** promover a `pub` (helper continua interno).
- Pattern consistente com `extract_length` privado mas
  acessível ao stdlib mesmo crate.

Se C1 confirma já `pub(crate)` ou similar: sem alteração.

Magnitude isolada: XS (~5min) se necessária; zero se já
acessível.

### C3 — `native_measure` function

Adicionar em `01_core/src/rules/stdlib/layout.rs` após
`native_colbreak` (paridade ordem ADR-0061 Fase 3 → Fase 4
candidata):

```rust
/// Stdlib `measure(body) -> dict(width: length, height: length)` —
/// Fase 4 Layout candidata per ADR-0066 §"Plano promoção"
/// Bloco C cross-módulo.
///
/// Forma: `#measure([Hello world])`.
///
/// Semantic graded P222 — opera sobre Content evaluated
/// (single-pass); runtime queries genuínas (counter values,
/// labels resolution) continuam diferidas per ADR-0066
/// PROPOSTO. Sem width override (paridade ADR-0054 graded;
/// refino futuro candidato).
///
/// Retorna `Value::Dict` com keys "width" + "height" ambos
/// `Value::Length` (paridade vanilla `measure(body).width`
/// observable).
pub fn native_measure(
    _ctx: &mut EvalContext<'_>,
    args: &Args,
    _world: &dyn World,
    _file: FileId,
    _figure_numbering: &FigureNumberingState,
) -> SourceResult<Value> {
    // 1. Extract body (posicional [0], Content ou Str).
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.clone()),
        Some(other) => return Err(eco_format!(
            "measure(body): espera Content ou Str, recebeu {}",
            value_type(other)
        )),
        None => return Err(eco_format!(
            "measure: argumento posicional body obrigatório ausente"
        )),
    };

    // 2. Reject extra positionals.
    if args.items.len() > 1 {
        return Err(eco_format!(
            "measure: aceita 1 posicional (body), recebeu {}",
            args.items.len()
        ));
    }

    // 3. Reject all named args (paridade Opção β graded;
    //    `width` override scope-out per ADR-0054).
    for key in args.named.keys() {
        return Err(eco_format!(
            "measure: named arg `{}` não suportado (paridade
            graded; refino futuro candidato)",
            key
        ));
    }

    // 4. Call helper privado (visibility pub(crate)).
    let (width_pt, height_pt) = measure_content(&body, /* full_width */);
    //   ^ ajustar conforme signature real auditada em C1

    // 5. Build Dict { width: Length, height: Length }.
    let mut dict = Dict::new();
    dict.insert("width".into(), Value::Length(Length::pt(width_pt)));
    dict.insert("height".into(), Value::Length(Length::pt(height_pt)));

    Ok(Value::Dict(dict))
}
```

**Decisões fixadas em C3**:
- Body posicional obrigatório (Content ou Str shortcut
  paridade P218 native_columns).
- 1 posicional max (rejeita >1 explicit).
- 0 named args aceitos (paridade Opção β graded).
- Retorno `Value::Dict { "width", "height" }` ambos
  `Value::Length` (pt internamente).

### C4 — Registar `native_measure` em scope + re-export

**Re-export** em `01_core/src/rules/stdlib/mod.rs`:
```rust
pub use crate::rules::stdlib::layout::{
    ..., native_measure, ...
};
```

**Scope register** em `01_core/src/rules/eval/mod.rs`
(paridade P218 pattern):
```rust
scope.define("measure", Value::Func(Func::native(
    "measure", native_measure,
)));
```

Posição: ordem alfabética com outras stdlib funcs Layout
(provavelmente após `m...` ou similar).

Stdlib funcs count: **55 → 56**.

### C5 — Sentinelas P222

Tests unitários P222 em `stdlib/layout.rs::tests` (paridade
P218 native_columns + P220 native_colbreak):

- `p222_native_measure_body_content_aceita` — `measure([texto])`
  produz `Dict { width: Length, height: Length }`.
- `p222_native_measure_body_str_aceita` — `measure("texto")`
  produz Dict (shortcut Str → Content).
- `p222_native_measure_body_ausente_rejeita` — `measure()`
  falha.
- `p222_native_measure_body_tipo_errado_rejeita` —
  `measure(42)` falha.
- `p222_native_measure_extra_positional_rejeita` —
  `measure([a], [b])` falha (>1 posicional).
- `p222_native_measure_named_arg_rejeita` — `measure([texto],
  width: 5cm)` falha (paridade Opção β; refino futuro
  candidato; mensagem deve indicar isso).
- `p222_native_measure_retorna_dict_com_width_height` —
  Dict resultante tem keys "width" e "height".
- `p222_native_measure_dimensoes_positivas_para_texto_nao_vazio`
  — width > 0 e height > 0 para body com texto.
- `p222_native_measure_dimensoes_zero_para_empty` —
  width == 0 e height == 0 para `Content::Empty`.

Layout E2E test em `tests.rs` (2 tests):
- `p222_measure_stdlib_via_let_binding` —
  `let dims = measure([Hello]); dims.width` produz Length
  positivo via parse + eval pipeline.
- `p222_measure_resultado_usavel_em_dict_access` —
  `measure([a]).height` acessa via Dict indexing
  (paridade vanilla observable).

Total tests P222: **9 unit + 2 E2E = 11 tests**.
Esperado pós-P222: **1987 + 11 = 1998 verdes**.

### C6 — L0 `stdlib.md` decisão paridade P217-P220

Decisão sobre L0:
- **Opção α** — linha minimal em tabela (paridade decisão
  P218 spec — não materializada porque P218 rejeitou).
- **Opção β** — secção dedicada `## native_measure` (paridade
  P156I native_stack documentation antigo).
- **Opção γ** — sem extensão L0 (paridade pattern N=4
  P217+P218+P219+P220).

**Hipótese provável**: **Opção γ** — pattern "L0 minimal
para refactors" N=4 → 5 consolida-se. Distinct de P156C-J
pré-M9c onde L0 era actualizada. Convenção "inline-doc"
para stdlib funcs aditivas pós-M9c.

Hash `stdlib.md` preservado (não tocado).

**Promoção formal a ADR meta documental**: Caminho 4
identificado em P221 §8. **NÃO** promove em P222 (paridade
política "sem novas reservas" P158); registo cumulativo em
relatório P222 §5.

### C7 — Verificação tests workspace

Critério: 1987 verdes pré-P222 + 11 novos = **1998 verdes**.

```
cargo test --workspace 2>&1 | tail -20
```

**Erro tolerado**: zero. P222 é aditivo puro (visibility
promotion + nova stdlib func); sem refactor estrutural.

Hipótese provável: 1998 verdes; zero regressões
pre-existente.

### C8 — Verificação lint

```
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 0 violations. Hash propagado em
`rules/stdlib/layout.rs` (L1). L0 stdlib.md não tocado
(Opção γ) — "Nothing to fix" esperado.

Se C2 promover visibility do helper: hash propagado em
`rules/layout/helpers.rs` também.

### C9 — Inventário 148 reclassificação P222

Editar `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

**§A.5 Layout linha 151 `measure(body)`**: reclassificação
**`parcial` → `implementado⁺`**.

Justificação literal:
- Helper privado expose como stdlib (decisão crítica
  cumprida).
- Width override scope-out (paridade Opção β graded —
  `implementado⁺` paridade graded com asterisco).
- ADR-0066 anotada sem promoção (semantic runtime queries
  genuínas continuam diferidas).

**Tabela A.5 Layout**: distribuição actualizada:
- Pré-P222 (pós-P221): `12/1/5/0/0 = 18`.
- Pós-P222: **`12/2/4/0/0 = 18`** (1 parcial → impl⁺).

**Cobertura Layout per metodologia §A.9**:
- Pré: `(12+1)/18 = 72%`.
- Pós: `(12+2)/18 = **78%**` ✓ (paridade visual histórica
  Opção γ §2.1 — coincidência aritmética agradável).

**Tabela A user-facing total**: re-distribuição:
- Pré: `68/24/27/20/2 = 141`.
- Pós: `68/25/26/20/2 = 141` (1 parcial → impl⁺).
- Cobertura total: `(68+25)/141 ≈ **66%**` (+1pp real).

**Footnote ⁴³ P222** adicionada documentando:
- `measure` stdlib expose graded; helper privado
  promovido a `pub(crate)` se necessário.
- Opção β graded; width override scope-out.
- ADR-0066 anotada sem promoção; Bloco C cross-módulo
  primeira materialização parcial.
- Reclassificação parcial → impl⁺.
- Δ Layout cobertura: 72% → 78% (+6pp real per
  metodologia).
- Δ user-facing total: 65% → 66% (+1pp).

**Tabela B.2 Content variants**: sem alteração P222
(measure é stdlib func; não toca enum Content).

### C10 — ADR-0066 anotação P222 + ADR-0061 nota refino Fase 4

**ADR-0066** §"Plano promoção futuro" anotada com bloco
**`### P222 materializado 2026-05-12`**:

```markdown
**Bloco C cross-módulo — primeira materialização parcial**:
- `measure(body)` stdlib expose graded.
- Helper privado `measure_content` em `layout/helpers.rs`
  promovido a `pub(crate)` (se necessário per audit C1) e
  exposto via `native_measure` em `stdlib/layout.rs`.
- Retorna `Value::Dict { width: Length, height: Length }`
  (paridade vanilla observable).
- **Width override scope-out** per Opção β graded ADR-0054;
  refino futuro candidato.
- 11 tests adicionados (9 unit + 2 E2E). Tests workspace:
  1987 → 1998 verdes.
- §A.5 `measure(body)` reclassificada `parcial` →
  `implementado⁺`.

Status ADR-0066: **PROPOSTO mantido**. Materialização
independente per natureza single-pass (helper opera sobre
Content evaluated; sem runtime queries genuínas). 3 condições
§"Plano promoção" continuam pendentes (state(), 2-pass
pipeline, E2E feature observable).

Bloco C cross-módulo restante: cross-document cite refs
(depende multi-document pipeline; não-reservado per P158).
```

**ADR-0061** §"Status" anotação cumulativa P222 (sem
transição de status; ADR-0061 já IMPLEMENTADO em P221):

```markdown
### P222 anotação — Fase 4 Layout candidata sub-passo 1

`measure(body)` stdlib expose graded materializado.
**Fase 4 Layout candidata 1/3** (P222 measure + P223 place
+ P224 grid pendentes per Opção α P221).

ADR-0061 status: **IMPLEMENTADO mantido** (Fase 3 cumprida
P221; Fase 4 candidata em curso sem nova reserva formal).
```

**Status ADR-0066**: PROPOSTO mantido. Anotação cumulativa
sem transição.
**Status ADR-0061**: IMPLEMENTADO mantido. Anotação
cumulativa Fase 4.

---

## §3 Output

1 ficheiro relatório:
`00_nucleo/materialization/typst-passo-222-relatorio.md`.

Estrutura (~6-8 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Inventário pré-P222 helper (C1).
- §3 Visibility promotion se necessária (C2).
- §4 `native_measure` function + scope register (C3 + C4).
- §5 Decisões substantivas (Opção α Dict; Opção β width
  scope-out; Opção γ L0 sem extensão pattern N=4 → 5;
  ADR-0066 sem promoção).
- §6 Resultados verificação (11 tests + 1987 pre-existentes
  preservados).
- §7 Inventário 148 reclassificação `measure` parcial →
  impl⁺ + ADR-0066 anotação Bloco C + ADR-0061 anotação
  Fase 4.
- §8 Próximo sub-passo (P223 place refino float +
  clearance; Caminho 1 Opção α continuação).

Código alterado:
- **Editado**: `01_core/src/rules/stdlib/layout.rs` (+
  `native_measure` ~30 LOC + 9 unit tests ~80 LOC).
- **Editado**: `01_core/src/rules/stdlib/mod.rs` (re-export).
- **Editado**: `01_core/src/rules/eval/mod.rs` (scope
  register).
- **Possivelmente editado**: `01_core/src/rules/layout/helpers.rs`
  (visibility promotion `pub` → `pub(crate)` se necessário
  per C2).
- **Editado**: `01_core/src/rules/layout/tests.rs` (+ 2 E2E
  tests).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (Tabela A.5 + §A.5 reclassificação + footnote ⁴³ P222).
- **Editado**: `00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`
  (+ anotação P222 Bloco C cross-módulo).
- **Editado**: `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`
  (+ anotação Fase 4 candidata sub-passo 1).

**Sem novos ficheiros**.

---

## §4 Não-objectivos

- Promover ADR-0066 PROPOSTO → IMPLEMENTADO — 3 condições
  §"Plano promoção" não satisfeitas em P222 (helper puro
  single-pass não satisfaz "runtime queries genuínas").
- Implementar `state(key, init)` ou outras Introspection
  runtime features — fora de escopo Layout Fase 4;
  candidatos P160B+ separados.
- Width override em `measure(body, width: 5cm)` —
  diferido a Fase 5 candidata (refino futuro
  não-reservado per política P158).
- Show rules `#show measure: ...` — fora de escopo.
- Reabrir decisão P216B (`Regions` minimal) — preservada
  literal.
- Materializar Bloco C cross-document cite refs —
  candidato separado (depende multi-document pipeline).
- Tocar em arms Layouter — P222 é stdlib + helper visibility
  apenas.
- L0 stdlib.md extensão — Opção γ pattern N=4 → 5.
- Reclassificar `place` ou `grid` — diferidos a P223 e
  P224 respectivamente.
- ADR meta administrativa formal pattern "L0 minimal" —
  Caminho 4 P221 §8 diferido (não-reservado).

---

## §5 Riscos a evitar

1. **Helper `measure_content` signature divergente**: se
   C1 revela signature diferente da hipótese (e.g. retorna
   `Result` ou requer mais params), ajustar plano. Mitigação:
   C1 explícito como primeira cláusula.
2. **Visibility promotion implícita**: se helper é privada
   absoluta, promoção a `pub(crate)` é mudança de API. Embora
   minimal, registar em relatório.
3. **Retorno `Value::Dict` vs novo tipo**: tentação de criar
   `Value::Size` novo. Rejeitada — Dict preserva ADR-0033
   paridade observable; Value::Size seria divergência.
4. **Width override aceito por engano**: tentação de
   implementar width override "porque é trivial". Rejeitada
   — paridade ADR-0054 graded explícita; rejeitar com
   mensagem clara documenta scope-out.
5. **Promover ADR-0066 prematuramente**: tentação de fazer
   PROPOSTO → IMPLEMENTADO porque "feature ADR-0066 foi
   materializada". Rejeitada — 3 condições §"Plano promoção"
   não satisfeitas. Promoção exigirá runtime queries
   genuínas em passo futuro (P160B+).
6. **Mudança observable em features existentes**: P222 é
   aditivo puro; zero impacto em features pré-existentes.
   Mitigação: 1987 tests pre-existentes verdes preservados.
7. **L0 stdlib.md actualização inflada**: tentação de criar
   secção dedicada (Opção β). Rejeitada — Opção γ pattern
   N=4 → 5 estável.
8. **`Dict::new()` API**: dependente do constructor real
   de `Value::Dict` em cristalino. Auditar em C1 ou C3
   se necessário.
9. **`Length::pt(f64)` API**: dependente do constructor
   real. Auditar em C1 ou C3 se necessário.
10. **Materializar Bloco C completo**: tentação de fazer
    cross-document cite refs no mesmo passo. Rejeitada —
    P222 escopo isolado `measure`; cite refs é separado.

---

## §6 Hipótese provável

C1 confirmará `measure_content` em
`01_core/src/rules/layout/helpers.rs` — provavelmente
`pub(super)` ou `pub(crate)` (já acessível ao stdlib mesmo
crate).

C2 — sem promoção visibility (já acessível); zero alteração.

C3 criará `native_measure` em ~30 LOC com 4 validações
(body, extra positional, named arg rejeição, body tipo).

C4 registará em scope + re-export (paridade P218 pattern).

C5 criará 11 tests novos (9 unit + 2 E2E).

C6 fixará Opção γ (sem extensão L0; pattern N=4 → 5).

C7 reportará 1998 tests verdes (1987 + 11).

C8 reportará 0 violations; "Nothing to fix" hashes.

C9 reclassificará `measure(body)` parcial → impl⁺;
cobertura Layout 72% → 78%; user-facing 65% → 66%.

C10 anotará ADR-0066 §"Plano promoção" Bloco C primeira
materialização parcial; ADR-0061 Fase 4 candidata
sub-passo 1.

Custo real: S+ (~1-2h). Maior parcela em C5 (11 tests)
+ C9 (recálculo cumulativo Tabela A).

Mas é hipótese, não decisão. C1-C10 fixam-se empíricamente.

---

## §7 Particularidade P222

P222 é estruturalmente distinto na trajectória pós-M9c:

- **Primeiro sub-passo Fase 4 Layout** — paridade
  estrutural P217 (primeiro Fase 3 sub-fase b).
- **Pattern emergente "L0 minimal para refactors" N=4 →
  5** — P217+P218+P219+P220+**P222** todos Opção γ. N≥5
  reforça candidatura formal ADR meta documental
  (Caminho 4 P221 §8 diferido per política P158).
- **ADR-0066 PROPOSTO anotada sem promoção** — primeira
  materialização parcial Bloco C cross-módulo. Pattern
  emergente "ADR PROPOSTO com materialização parcial
  graded" N=1 inaugurado (paridade ADR-0078 antes de
  IMPLEMENTADO; mas ADR-0078 transitou; ADR-0066 mantém).
- **Cobertura Layout 72% → 78%** real per metodologia.
  **Coincidência aritmética agradável**: paridade visual
  histórica Opção γ §2.1 ("78% (12 impl + 5 parcial)")
  agora coincide com cobertura real per metodologia rígida
  pós-P222 ("78% (12 impl + 2 impl⁺ + 4 parcial)" — 14
  no numerador agora). Próxima sessão pode actualizar Opção
  γ blueprint para preservar paridade ou recalcular nota.
- **Anti-inflação 16ª aplicação cumulativa** pós-P205D —
  Opção β width override scope-out + Opção γ L0 sem
  extensão.
- **Primeira reclassificação Layout pós-Fase 3 fechada** —
  `measure` parcial → impl⁺. Pattern emergente "Fase 4
  candidata reclassifica entradas parcial → impl⁺" N=1
  (paridade Fase 3 que reclassificou ausente → parcial em
  P219+P220).

Por isso §5 risco 5 (promover ADR-0066 prematuramente) é
o mais provável. Tentação óbvia é "feature ADR-0066
materializada → promover". Defesa: 3 condições §"Plano
promoção" não satisfeitas; helper é puro single-pass não
runtime queries genuínas.

**Critério de aceitação P222**:
- 11 tests novos verdes (9 unit + 2 E2E).
- 1987 tests pre-existentes preservados.
- 0 violations.
- §A.5 `measure(body)` reclassificada parcial → impl⁺.
- Cobertura Layout: 72% → **78%** real (+6pp).
- Cobertura user-facing total: 65% → **66%** (+1pp).
- Fase 4 candidata Layout: 1/3 sub-passos (P222 ✓; P223
  pendente; P224 pendente).

**Estado pós-P222 esperado**:
- Tests workspace: 1987 → 1998 verdes.
- Stdlib funcs: 55 → 56 (+native_measure).
- Content variants: 56 (sem alteração).
- §A.5 distribuição: `12/2/4/0/0 = 18` (1 parcial → impl⁺;
  zero ausentes preservado).
- Cobertura Layout per metodologia: 72% → 78%.
- Cobertura user-facing total: 65% → 66%.
- ADR-0066 PROPOSTO; ADR-0061 IMPLEMENTADO; ADR-0078
  IMPLEMENTADO.
- Saldo DEBTs: 13 abertos (preservado).
- 16 aplicações cumulativas anti-inflação.
- Pattern "L0 minimal para refactors" N=4 → 5.
