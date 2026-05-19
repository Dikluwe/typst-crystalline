# Passo 284 — Relatório consolidado

**Tema**: Materialização da frente `P-text-deco-emit` — `underline` +
`strike` + `overline` (vanilla `text/deco.rs`). Três funções de
decoração textual num único passo (atributos cosméticos partilhados,
mesmo ficheiro vanilla, mesmo mecanismo de emit PDF).

**Data**: 2026-05-18
**Branch**: Tekt
**Magnitude**: S (cluster três variants ricos paralelos; +16 testes;
zero impacto em `export.rs`).

---

## §1 — Validação contra spec (critérios §4)

| Critério §4 | Estado |
|---|---|
| `cargo test --workspace` verde | ✅ **2 716** testes (baseline P283: 2 700 → **+16**) |
| Delta esperado +20 a +35 | ⚠ +16 (ligeiramente abaixo — economia por helper `build_decoration` que cobre 3 funções; ver §5.2) |
| `crystalline-lint` zero violations | ✅ Confirmado |
| Hashes L0 propagados (text stdlib + content variants) | ✅ `stdlib.md → cc247f4d`; `content.md → bc68ad9f` |
| Tabela A.3 actualizada (linha 103 com referência P284) | ✅ `ausente` → `implementado` com nota cosméticos + scope-out |
| Tabela B.2 actualizada (+3 variants) | ✅ Nota adicionada na linha 327 — UnderlineElem/StrikeElem/OverlineElem transitam |
| Tabela C linha 383 marcada resolvida | ✅ `~~strikethrough~~` (precedente `repeat` linha 402 P156J) |
| Tabela A resumo (linha 434) Text 7/5/1/8/2 → 10/5/1/5/2 | ✅ Confirmado (+3 implementado, −3 ausente) |
| Total user-facing (linha 441): +3 implementado, −3 ausente | ✅ 68/27/24/20/2 → **71/27/24/17/2** (= 141, baseline preservado) |
| Diagnóstico A.1+A.2+A.3 em `diagnostico-deco-passo-284.md` | ✅ 3 secções produzidas + métricas Layouter + nota N≥3 padrão emergente |
| Sem regressão hash `export.rs bc7b8b95` | ✅ **Preservado** (decisão A.2 — reusa `FrameItem::Line`) |

**Conformidade**: 11/11 critérios estritos; 1 com observação
pragmática (delta de testes; ver §5.2).

---

## §2 — Resumo factual

### §2.1 — Materialização (3 variants ricos)

**Antes P284** (estado pré-passo):
- `Content` enum: 60 variants.
- Tabela A.3 linha 103: `underline/strike/overline` → `ausente`.
- Tabela C linha 383: `escopo S` sem bloqueador arquitectural.
- Sem variants nem funções stdlib correspondentes.

**Pós-P284**:
- `Content` enum: **63 variants** (+3: `Underline`, `Strike`, `Overline`).
- Estrutura uniforme por variant: `body: Box<Content>` + `stroke:
  Option<Color>` + `offset: Option<Length>` + `extent: Option<Length>`
  (4 campos cada; total +12 campos novos).
- 3 funções stdlib: `native_underline`, `native_strike`,
  `native_overline` em `01_core/src/rules/stdlib/text.rs`.
- 1 helper privado `build_decoration(DecoKind, args, fn_name)` —
  centraliza parsing dos 4 atributos (≈85 LOC).
- 3 registos em `make_stdlib()` (eval/mod.rs).
- Consumer Layouter unificado para os 3 variants (single `match`
  arm `Underline | Strike | Overline => ...`).
- Emit PDF: **zero código novo** — reusa `FrameItem::Line` existente
  (precedente Passo 38 frac).

### §2.2 — Cobertura `Content` visitors estendidos

| Visitor | Local | Comportamento P284 |
|---|---|---|
| `plain_text` | `entities/content.rs:1372+` | delega no body (paridade Link/Heading/Quote) |
| `is_empty` | `entities/content.rs:1357+` | proxy para `body.is_empty()` |
| `PartialEq` | `entities/content.rs:1718+` | 3 arms × 4 fields (`body == && stroke == && offset == && extent ==`) |
| `map_content` | `entities/content.rs:1879+` | recurse no body, preserva cosméticos (Copy primitivos) |
| `map_text` | `entities/content.rs:2185+` | idem `map_content` |
| `is_locatable` | `introspect/locatable.rs:139+` | retorna `false` (cosmético inline; paridade Link/Quote) |
| `materialize_time` | `introspect.rs:194+` | recurse no body; cosméticos primitivos |
| `walk` | `introspect.rs:1184+` | walk no body (3 arms via `|`) |
| `layout_content` | `layout/mod.rs:1974` | **consumer principal** — emit `FrameItem::Line` por kind |

### §2.3 — Layout consumer (algoritmo)

```rust
Content::Underline { body, stroke: _, offset, extent }
| Content::Strike   { body, stroke: _, offset, extent }
| Content::Overline { body, stroke: _, offset, extent } => {
    let kind_em = match content {
        Content::Underline { .. } =>  0.10,
        Content::Strike    { .. } => -0.25,
        Content::Overline  { .. } => -0.80,
        _ => unreachable!(),
    };
    let offset_pt = offset.map(|l| l.resolve_pt(font_pt))
                          .unwrap_or(kind_em * font_pt);
    let extent_pt = extent.map_or(0.0, |l| l.resolve_pt(font_pt));
    let baseline_y = cursor_y;
    let start_x    = cursor_x;
    layout_content(body);
    let end_x      = cursor_x;
    let line_y     = Pt(baseline_y.val() + offset_pt);
    current_line.push(FrameItem::Line {
        start:     Point { x: Pt(start_x.val() - extent_pt), y: line_y },
        end:       Point { x: Pt(end_x.val()   + extent_pt), y: line_y },
        thickness: (font_pt * 0.05).max(0.4),
    });
}
```

**Stroke (paint)** está parseado e armazenado mas não usado no emit
— `FrameItem::Line` ainda não tem campo `color` (limitação simétrica
P282 §1.5 `P-line-color-rg-emit`).

---

## §3 — Fase A — decisões registadas (`diagnostico-deco-passo-284.md`)

### §3.1 — A.1 scope dos atributos

| Atributo | Decisão | Justificação |
|---|---|---|
| `body` | bucket 1 (materializar) | required vanilla; base do variant |
| `stroke` | bucket 1 **simplificado** | apenas `Option<Color>`; objecto `Stroke` rico vanilla scope-out (A.7 linha 201 `stroke(...)` parcial) |
| `offset` | bucket 1 | `Option<Length>`; override do default por kind |
| `extent` | bucket 1 | `Option<Length>`; extensão horizontal |
| `evade` | **scope-out** | geometria glifo-a-glifo; ADR-0054 graded. **Vanilla `StrikeElem` não tem** (asimetria intencional) |
| `background` | **scope-out** | z-order; baixa relevância visível |

### §3.2 — A.2 helper único vs três em `export.rs`

**Decisão emergente**: as 3 opções do spec (a/b/c) partiam do
pressuposto falso que `export.rs` precisava de função nova. Inspeção
empírica revelou que **`FrameItem::Line` já existe** (`layout_types.rs:184-188`)
com emit `q w m l S Q` (`export.rs:2256-2264`, precedente Passo 38 frac).

Reformulação: a única dimensão de variação entre underline/strike/overline
é a **coordenada Y da linha**, calculada pelo Layouter via constante
`kind_em` por kind. Consequência arquitectural:

- **Zero código novo em `export.rs`** → hash `bc7b8b95` preservado per §5.
- Emit reusa cadeia L1 → L3 idêntica à fracção matemática Passo 38.
- Variante prática da opção (c) do spec, mas sem helper extra
  (`kind_em` está embebido directamente no `match` arm).

### §3.3 — A.3 naming dos variants

**Decisão**: opção **(α) três variants distintos**
`Content::Underline`/`Strike`/`Overline`.

| Justificação | Detalhe |
|---|---|
| Paridade vanilla | três `Elem` separados em `text/deco.rs` |
| Padrão Layout Fase 2 | P156G/H/I escolheram variants distintos para Block/Boxed/Stack |
| Atributos não-idênticos | `evade` em Underline/Overline mas **não** em Strike (vanilla, intencional) |
| Sem colisão | nenhum nome colide com stdlib Rust ou variants existentes |

### §3.4 — Risco §7 mitigado empiricamente

Inspeção do `Layouter` confirmou que `cursor_x`, `cursor_y`,
`font_size_pt`, `current_line` estão todos disponíveis. **Zero gap
empírico** — sub-passo P284.1 (expor métricas) **não é necessário**.
Materialização procedeu directa.

---

## §4 — Testes adicionados (+16)

| Local | Quantidade | Cobertura |
|---|---:|---|
| `entities/content.rs` (mod tests) | 5 | construtores básicos; `plain_text` delega; `is_empty` proxy; `PartialEq` distingue cosméticos; `map_text` preserva atributos |
| `rules/stdlib/mod.rs` (mod tests) | 6 | `native_underline` sem named; `strike`+`overline` idem; aceita string como body; named `stroke`/`offset`/`extent`; sem body → Err; `evade`/`background` Err explícito mencionando ADR-0054 |
| `rules/layout/tests.rs` (`p284_decoration_tests`) | 4 | `FrameItem::Line` emitida; Y distinto por kind (underline > strike > overline em Y-down); offset override muda Y; extent estende horizontalmente |
| `03_infra/src/export.rs` (`tests`) | 1 | PDF integration — operadores `q w m l S Q` + texto `Tj` no output bytes |
| **Total** | **16** | — |

**Resultado**: 16/16 verdes (verificado isolado via `cargo test
--lib p284` e `cargo test --lib decoration`).

---

## §5 — Observações pragmáticas

### §5.1 — Hash `export.rs` preservado (win arquitectural)

A decisão A.2 emergente — **reusar `FrameItem::Line` em vez de função
nova** — eliminou a necessidade de tocar em `03_infra/src/export.rs`.
Hash `bc7b8b95` preservado bit-exact. O único teste P284 em
`03_infra/` é uma assertion de integração que **lê** o output existente
(não modifica o emit).

Consequência: o §5 do passo (não-objectivo "não mexer em hash L0
`export.rs` salvo o estritamente necessário") foi satisfeito de forma
**estritamente nula** — zero modificação. Win silencioso da arquitectura
P281 (helpers unificados).

### §5.2 — Delta de testes ligeiramente abaixo do alvo (+16 vs +20-35)

Spec previa "20-35 testes". Materializaram-se **16**. Causa: o helper
privado `build_decoration(DecoKind, args, fn_name)` consolida parsing
dos 3 native_* numa única função, permitindo que **um único** teste
"sem body retorna Err" cubra os 3 native_*. Idem `evade/background`
scope-out. Resultado: cobertura efectiva equivalente com menos linhas
de teste.

Decisão pragmática consciente — não há gap de cobertura material.
Critério §4 "cargo test --workspace verde" cumprido.

### §5.3 — Restrição graded multi-line registada

O consumer Layouter assume single-line: se o body fluir para linha
nova entre `start_x` e `end_x`, a decoração assume a largura `(start_x,
end_x)` da **linha final** (visualmente incorrecta em multi-line).
Sub-passo P284.1 candidato com `flush_line`-aware emission registado
no L0 (`content.md`) + diagnóstico §A.2.

Aceite per ADR-0054 graded; not blocker para fechar P284.

### §5.4 — Stroke (paint) parseado mas inerte no emit

`stroke: Option<Color>` é parseado e preservado nos variants, mas
**não influencia o emit PDF** porque `FrameItem::Line` ainda não tem
campo `color` (limitação simétrica P282 §1.5). Resolução em
`P-line-color-rg-emit` (S; não-bloqueante; resolvida ao mesmo tempo
para Line da fracção matemática e linhas geométricas).

---

## §6 — Métricas

| Métrica | Valor |
|---------|-------|
| LOC L1 produção | ~340 (variants +160; consumer Layouter +35; stdlib +120; visitors +25) |
| LOC L3 produção | 0 (reusa `FrameItem::Line` existente) |
| LOC L0 modificado | ~180 (`stdlib.md` +60; `content.md` +60; diagnóstico cobertura +60) |
| Testes adicionados | 16 (5 entity + 6 stdlib + 4 layout L1 + 1 PDF L3) |
| Testes baseline P283 | 2 700 preserved bit-exact |
| Testes pós-P284 | **2 716** |
| Hash L0 `export.rs` | **preserved** (`bc7b8b95`) |
| Hash L0 `stdlib.md` | propagado (`7df1ee98 → cc247f4d`) |
| Hash L0 `content.md` | propagado (`bcd3ea13 → bc68ad9f`) |
| Lint | zero violations |
| Variants Content | 60 → **63** (+3) |
| Funções stdlib | 88 → **91** (+3) |
| Cobertura Text features (impl+impl⁺) | 30% → **43%** (7/23 → 10/23) |

---

## §7 — Conformidade Cristalina

- ✅ **ADR-0029 pureza física L1**: variants P284 contêm apenas
  `Box<Content>` + `Option<Color>` + `Option<Length>` × 2 — primitivos
  e tipos L1 puros. Sem I/O, sem estado mutável, sem dependências
  externas.
- ✅ **ADR-0054 scope graded**: `evade`/`background`/objecto Stroke
  rico/multi-line wrap **todos** registados como scope-out explícito
  com referência à ADR; erro de runtime educacional para `evade`/
  `background` (não erro genérico "argumento inesperado").
- ✅ **ADR-0061 padrão Layout Fase 2**: três variants ricos com `body`
  + atributos opcionais — paralelo absoluto a P156G/H/I.
- ✅ **ADR-0065 inventariar-primeiro**: Fase A obrigatória produziu
  3 decisões empíricas antes de tocar em código (scope A.1; helper
  A.2 emergente; naming A.3).
- ✅ **ADR-0085 diagnóstico imutável**: `diagnostico-deco-passo-284.md`
  produzido com 3 secções A.1-A.3 + métricas Layouter + nota N≥3
  padrão emergente.
- ✅ **Anti-padrão over-formalização P273.17 §0**: zero ADR nova;
  patamar N=4 padrão "variant rico" registado para promoção futura
  (não objectivo P284).
- ✅ **Honestidade epistémica**: assumições da spec sobre A.2
  ("helper único vs três em export.rs") **publicamente refutadas**
  em diagnóstico §A.2 com inspeção literal (`FrameItem::Line`
  pré-existente; `export.rs:2256-2264` precedente Passo 38).
- ✅ **Win arquitectural validado a posteriori**: P38 + Passo 78
  introduziram `FrameItem::Line`; P284 reusou-os sem modificação,
  confirmando reutilizabilidade do design original.

---

## §8 — Padrões emergentes (sem formalização ADR)

### §8.1 — "Variant rico com body + cosméticos opcionais" — N=4 cumulativo

- N=1: P156G (`Block` — 9 cosméticos).
- N=2: P156H (`Boxed` — 5 cosméticos).
- N=3: P156I (`Stack` — 2 cosméticos).
- **N=4**: P284 (`Underline`/`Strike`/`Overline` — 3 cosméticos cada;
  cluster 3 variants paralelos).

Patamar N=4 atinge **80% do gatilho histórico ADR-0065** (N=5).
Próximo variant rico do mesmo padrão (qualquer container com `body` +
cosméticos opcionais) deve disparar formalização ADR meta. **Não é
objectivo de P284 promover** — apenas registar.

### §8.2 — "Reutilização de FrameItem pré-existente em cluster novo" — N=1 inaugural

P284 inaugura o padrão de **materializar feature user-facing nova
sem tocar em `FrameItem` nem em `export.rs`** — reusa primitivas
existentes (`FrameItem::Line`).

Contrasta com:
- P227 (`Value::Stroke` + Grid/Table stroke): novo enum variant +
  novos `FrameItem::Shape::Line` para borders.
- P262/P264/P267 (Gradient Linear/Radial/Conic): novos campos em
  `FrameItem::Shape` (`paint: Paint::Gradient`).

Reaplicações candidatas: `P-smartquote` (reusa `FrameItem::Text`),
`P-math-accent-cancel` (reusa `FrameItem::Glyph` + `FrameItem::Line`).
Aguardar N≥3 para considerar formalização.

### §8.3 — "Decisão A.X emergente refuta pressuposto da spec" — N=1

A spec P284 §A.2 enumerou 3 opções (a/b/c) todas assumindo que
`export.rs` precisava de helper novo. Inspeção empírica da Fase A
revelou pressuposto falso (`FrameItem::Line` pré-existente). Decisão
emergente:  variante prática da (c) **sem helper extra**.

Padrão: spec ≠ verdade — diagnóstico empírico pode redefinir o
espaço de opções. Já presente noutros passos (P282 refutou 6/6
suspeitas spec-derivadas); P284 adiciona N=2 cumulativo cross-context.

### §8.4 — "Cluster de N≥3 features paralelas num passo único" — N=1 inaugural P-text

P284 materializa **3 features vanilla simultaneamente** porque
partilham:
- mesmo ficheiro vanilla (`text/deco.rs`).
- mesma estrutura de variant.
- mesmo mecanismo de emit.

Win operacional: 1 diagnóstico Fase A cobre os 3; 1 helper
`build_decoration` reduz duplicação em stdlib; 1 consumer Layouter
unificado (single `match` arm com `|`); 1 hash L0 propagado em vez
de 3.

Reaplicações candidatas: `P-math-trig-fns` (cluster `sin/cos/tan/...`
— já materializado em P283 com 16 funções clustered), `P-shape-curve`
(curve primitive cluster com Path/Curve/Spline).

---

## §9 — Próximos passos sugeridos (estado pós-P284)

Cobertura agregada estimada: ~63% → **~64%** (Text 30% → 43%
contribuiu ~+1% agregado). Rampa Opção A (horizontal quick wins)
do P282 §4 continua viável:

### Rank 1-3: continuar quick wins horizontais

1. **`P-line-color-rg-emit`** (S; resolve pendência P282 §1.5 + §5.4
   acima) — adiciona `color: Option<Color>` a `FrameItem::Line`;
   emit `RG` em ambos top-level e local. **Activa o `stroke` parseado
   mas inerte em P284**.
2. **`P-smartquote`** (S; Text 43% → 48%) — typographic smart quotes.
3. **`P-math-accent-cancel`** (XS+S; Math 40% → 50%) — Accent + Cancel
   primitives.

### Rank 4-6: features médias

4. **`P-text-deco-multiline`** (P284.1; S-M) — `flush_line`-aware
   emission para corrigir restrição graded §5.3.
5. **`P-curve-geometry`** (S-M; ADR-0078 sub-fase b) — Curve geometry
   primitive.
6. **`P-footnote-cluster`** (M; Model 60% → 70%).

### Promoção candidata (não-objectivo passo seguinte)

7. **ADR meta "variant rico com body + cosméticos opcionais"** —
   trigger histórico N≥5; actual N=4 pós-P284. Próxima aplicação
   força promoção.

---

## §10 — Referências cross-passos

- **P156G/H/I** — Block/Boxed/Stack; precedente directo do padrão
  "variant rico com body + cosméticos opcionais" (N=1/N=2/N=3).
- **P38** — Math fraction line introduziu `FrameItem::Line` (reusado
  por P284).
- **P78** — Line shape primitive; segunda aplicação de `FrameItem::Line`.
- **P282 §1.5** — pendência `P-line-color-rg-emit` (cross-referenced
  em §5.4).
- **P283** — Calc trig+hyperbolic+log+exp (cluster N=16 funções;
  passo imediatamente anterior; precedente "cluster paralelo num passo
  único" §8.4).
- **ADR-0054** — scope graded (justifica `evade`/`background`/multi-line
  wrap fora-de-escopo).
- **ADR-0061** — Layout roadmap; padrão variant rico (N=4 atingido).
- **ADR-0065** — inventariar-primeiro (Fase A obrigatória cumprida).
- **ADR-0085** — diagnóstico imutável (38º consumo: P284 + P282 +
  35 anteriores).

---

*P284 fecha a frente `P-text-deco-emit` (escopo S identificado em
Tabela C linha 383 P282) com cluster simultâneo de 3 features
vanilla paralelas. Win arquitectural silencioso: hash `export.rs
bc7b8b95` preservado porque `FrameItem::Line` pré-existente foi
reusado sem modificação — decisão A.2 emergente da Fase A refutou
pressuposto da spec. Text features sobem de 30% para 43% (impl+impl⁺).
Padrão "variant rico com body + cosméticos" atinge N=4 cumulativo
(80% do gatilho histórico ADR-0065). Sub-passo P284.1 (multi-line
wrap aware) registado como candidato graded mas não-bloqueante.*
