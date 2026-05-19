# Relatório — Passo 294 (`P-quadratic-curve`)

**Data**: 2026-05-19
**Spec**: `00_nucleo/materialization/typst-passo-294.md`
**Diagnóstico Fase A**: `00_nucleo/diagnosticos/diagnostico-quadratic-curve-passo-294.md`
**Tipo declarado spec**: extensão directa P293, materialização
from-scratch (variant novo + emit novo, 1ª quebra de hash desde P281).
**Tipo após A.0.0 N=2**: **refutação significativa da spec inteira** —
vanilla converte q→c em construct-time, sem variant interno.
**Baseline P293**: 2 802 testes  →  **P294**: 2 808 testes (Δ = +6 net)
**Hash `export.rs`**: `66cb8ac3` preservado bit-exact (**11º passo
consecutivo**: P282→P294)
**ADRs meta novas**: 0

---

## §1 — Sumário executivo

P294 activa `"quadratic"` em `native_curve` (P293) convertendo
quadratic Bézier → cubic em construct-time via fórmula matemática
exacta:

```
C₁ = (P₀ + 2·Q) / 3
C₂ = (P₂ + 2·Q) / 3
```

(onde Q = control quadrático único; P₀ = last_point; P₂ = end).

**Resultado funcional**: `#curve(("quadratic", (cx, cy), (ex, ey)))`
em Typst produz output PDF com curvas Bézier matemáticamente
correctas, emitidas via operator `c` (cubic) existente.

**Resultado metodológico — refutação significativa da spec**: a Fase
A reaplicou A.0.0 (N=2 do template §8.7' inaugurado em P293) e
descobriu via inspecção literal de
`lab/typst-original/.../typst-layout/src/shapes.rs:215-220` que
**vanilla typst NÃO tem variant `QuadraticTo` interno** — converte
q→c em construção. A spec inteira (§1.1 + §1.2 + §A.0 + §A.2 + §A.3
+ §A.4) foi escrita assumindo materialização de variant novo;
**toda essa estrutura ficou invalidada** por uma inspecção de 6
linhas de código vanilla.

Consequência: hash `export.rs 66cb8ac3` **preservado bit-exact**
(11º passo consecutivo) em vez de "quebrar intencionalmente" como a
spec previa. ADR-0098 honrada por mais um passo.

---

## §2 — Fase A (síntese)

| Secção | Veredicto |
|---|---|
| A.0.0 (N=2 reaplica §8.7') | **Refutação significativa de spec inteira** via inspecção vanilla |
| A.0 (ADR-0098 hash) | ✅ preservado bit-exact — spec invalidada |
| A.1 inventário | 4 variants `PathItem` confirmados; 3 sítios match emit identificados |
| A.2 decisão H1' | NÃO adicionar variant; conversão q→c local em `native_curve` |
| A.3 path_bbox | Reusado intacto (P277 sem alteração) |
| A.4 emit | Inalterado — sem `v`/`y` operator; só `c` paridade vanilla |
| A.5 bugs latentes | 5 cenários quadratic fronteira verificados; nenhum bug |
| A.5' anti-reflexão | N=4 cumulativo (P291+P292+P293+P294); 5 elementos novos |

Detalhe completo: `00_nucleo/diagnosticos/diagnostico-quadratic-curve-passo-294.md`.

---

## §3 — Materialização

### §3.1 — `01_core/src/rules/stdlib/shapes.rs` (única alteração)

Mudanças localizadas em `native_curve`:

**(a) Tracking de `last_point`**:

```rust
pub fn native_curve(...) -> SourceResult<Value> {
    let mut path_items: Vec<PathItem> = Vec::new();
    // P294: tracking de last_point para conversão q→c em "quadratic"
    // (paridade vanilla `Curve::last_point`). Arranca em (0,0) — caso
    // fronteira de quadratic sem move anterior é tratado consistentemente.
    let mut last_point: Point = Point::ZERO;
    ...
}
```

**(b) Arms existentes actualizam `last_point`**:

```rust
"move" => {
    let target = Point { x: Pt(x), y: Pt(y) };
    path_items.push(PathItem::MoveTo(target));
    last_point = target;
}
"line" => { /* idem com target */ last_point = target; }
"cubic" => { /* push CubicTo(c1, c2, end) */ last_point = end; }
"close" => {
    path_items.push(PathItem::ClosePath);
    // `close` não move last_point (paridade vanilla).
}
```

**(c) Arm `"quadratic"` — substitui scope-out P293 `Err`**:

```rust
// P294 H1' (descoberta empírica A.0.0 N=2): paridade vanilla
// — converte quadratic→cubic em construct-time, sem variant
// novo. Fórmula `control_q2c(p, c) = (p + 2c) / 3` aplicada a
// start e end (`lab/.../typst-layout/src/shapes.rs:215-220`).
"quadratic" => {
    if arr.len() != 3 {
        return Err(...);
    }
    let (qx, qy) = extract_coordinate(&arr[1])?;
    let (ex, ey) = extract_coordinate(&arr[2])?;
    let p0x = last_point.x.0;
    let p0y = last_point.y.0;
    let c1x = (p0x + 2.0 * qx) / 3.0;
    let c1y = (p0y + 2.0 * qy) / 3.0;
    let c2x = (ex  + 2.0 * qx) / 3.0;
    let c2y = (ey  + 2.0 * qy) / 3.0;
    let end = Point { x: Pt(ex), y: Pt(ey) };
    path_items.push(PathItem::CubicTo(
        Point { x: Pt(c1x), y: Pt(c1y) },
        Point { x: Pt(c2x), y: Pt(c2y) },
        end,
    ));
    last_point = end;
}
```

### §3.2 — Zero alterações em todo o resto

| Ficheiro / componente | Estado pós-P294 |
|---|---|
| `01_core/src/entities/geometry.rs` (`PathItem` enum) | **Inalterado** — 4 variants |
| `01_core/src/entities/geometry.rs` (`path_bbox`, `bezier_cubic_bbox`) | **Inalterado** (P277 robusto sem extensão) |
| `01_core/src/rules/eval/mod.rs` (registo stdlib) | **Inalterado** — `native_curve` já registado P293 |
| `03_infra/src/export.rs` (3 sítios emit cubic) | **Inalterado bit-exact** — hash `66cb8ac3` preservado |
| `02_shell/`, `04_wiring/` | Intactos |

---

## §4 — Testes

### §4.1 — `01_core/src/rules/stdlib/mod.rs`

Removido: 1 teste P293 (`p293_curve_quadratic_scope_out_retorna_err`)
— scope-out já não existe.

Adicionados: 6 testes P294.

| Teste | Verifica |
|---|---|
| `p294_curve_quadratic_activa_via_conversao_q2c` | Substitui o anterior scope-out P293 — quadratic agora retorna `Ok` com `PathItem::CubicTo` |
| **`p294_curve_quadratic_aplica_formula_q2c_exacta`** | **Fórmula exacta em `f64`**: P₀=(0,0), Q=(10,20), P₂=(30,40) → C₁=(20/3, 40/3), C₂=(50/3, 80/3) (ε<1e-9) |
| `p294_curve_quadratic_sem_move_anterior_usa_origem` | Caso fronteira: quadratic antes de move usa `Point::ZERO` como P₀ |
| `p294_curve_quadratic_aridade_errada_retorna_err` | Robustez input |
| **`p294_curve_quadratic_encadeada_actualiza_last_point`** | **Cadeia de quadratics**: segunda usa `end` da primeira como P₀ — verifica tracking correcto de `last_point` |
| `p294_curve_quadratic_e_cubic_misturados` | Sequência `move + quadratic + cubic + close` — interoperabilidade |

### §4.2 — `03_infra/src/export.rs`

1 teste P294 L3 PDF:

**`p294_quadratic_emite_c_operator_e_nao_v_nem_y`** — assertion
tripla:
- `s.contains(" c\n")` — emit cubic operator presente.
- `!s.contains(" v\n")` — operator `v` ausente (vanilla pattern).
- `!s.contains(" y\n")` — operator `y` ausente.

Este teste **codifica a invariante arquitectural P294**: emit PDF
usa apenas `c` operator, paridade vanilla — sem operadores
quadráticos PDF dedicados.

---

## §5 — Validação

### §5.1 — `cargo test --workspace`

```
test result: ok. 2313 passed; 0 failed; 0 ignored
test result: ok.  448 passed; 0 failed; 6 ignored
test result: ok.   24 passed; 0 failed; 0 ignored
test result: ok.    2 passed; 0 failed; 0 ignored
test result: ok.   21 passed; 0 failed; 0 ignored
                  -----
                  2808 passed total
```

Baseline P293 = 2 802; delta = +6 net.

Decomposição:
- +6 testes L1 P294 (em `01_core/src/rules/stdlib/mod.rs`).
- +1 teste L3 P294 (em `03_infra/src/export.rs`).
- −1 teste P293 removido (scope-out já não aplica).
- 2 802 + 6 + 1 − 1 = **2 808** ✓

### §5.2 — `crystalline-lint .`

```
✓ No violations found
```

### §5.3 — Hash `export.rs` preservado

```
//! @prompt-hash 66cb8ac3
```

**11º passo consecutivo** preservando bit-exact: P282, P285, P286,
P287, P288, P289, P290, P291, P292, P293, P294. ADR-0098 §"single
source of truth" mantida.

**Significado**: a spec esperava 1ª quebra desde P281; a descoberta
empírica A.0.0 N=2 revelou que a preservação é **mais correcta
arquitecturalmente** (paridade vanilla typst).

---

## §6 — Estado de hashes L0

| Ficheiro L0 | Antes P294 | Pós P294 |
|---|---|---|
| `entities/geometry.md` | `52271440` | **inalterado** — `PathItem` enum sem alteração |
| `rules/stdlib.md` | `21ade03a` | **inalterado** — L0 é política única (não enumera funções) |
| `infra/export.md` | `31a37c57` | **inalterado** |

`crystalline-lint --fix-hashes .` retornou "Nothing to fix" — confirma
ausência de drift.

---

## §7 — Padrões metodológicos

### §7.1 — §8.3 "refutação pragmática" (**N=6 candidato genuíno, adiado**)

P293 §7.3 refutou §8.3 N=6 anterior em favor de §8.7' inaugural
(template A.0.0). P294 traz **refutação ainda mais significativa**:
não apenas hipótese não-listada (P293 H6), mas **toda a estrutura
arquitectural proposta pela spec invalidada** — variant novo +
emit novo + quebra de hash, tudo refutado.

**Decisão de adiar promoção**:
- P273.17 §0: uma ADR meta por passo no máximo.
- Promover §8.3 hoje impossibilita consolidar §8.7' (mais urgente
  como template inaugural emergente).
- Padrão preservado para promoção em P295+ se a aplicação for nova.

### §7.2 — §8.7' "A.0.0 template" (**N=2 reaplica**)

P293 inaugurou (N=1). P294 reaplica com refutação maior. Limiar
N≥3 não atingido — aguarda P295+ para potencial promoção.

### §7.3 — §8.6 "A.5' anti-reflexão" (**N=4 cumulativo**)

P291 + P292 + P293 + P294. P294 traz 5 elementos novos
identificados em A.5'.3:

1. Refutação significativa de spec inteira (vs P293 hipótese
   não-listada apenas).
2. Hash `export.rs` preservação **inesperada** (spec previa quebra).
3. Conversão matemática em construct-time — paradigma novo
   "transform-on-build".
4. §8.3 N=6 confirmado genuinamente (mais forte que P293).
5. `last_point` tracking dentro de stdlib — primeira vez que
   `native_curve` precisa estado walker.

Limiar N≥3-4 atingido; **não promovido** — padrão é metodológico
interno (anti-padrão over-formalização P273.17 §0 honrado).

### §7.4 — ADR-0098 "single source of truth" (**N=11 cumulativo**)

Hash `export.rs` preservado bit-exact pelo 11º passo consecutivo.
Invariante robusta sobre 11 features distintas — Style cumulativos
(P288-P292), curve cubic (P293), e agora curve quadratic (P294).

### §7.5 — ADR-0099 "activação posterior" (**N=10 reaplicação**)

P294 caso paralelo: scope-out `"quadratic"` em P293 (`Err` retornado)
→ activação P294 (conversão q→c). **Mas não é materialização nova
de variant/emit** — é activação posterior pura de path inerte
existente. Esta interpretação é honesta em A.4.2 do diagnóstico.

---

## §8 — Cobertura vanilla vs cristalino

`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

- **Linha 194** actualizada para incluir P294: documenta H1'
  (descoberta A.0.0 N=2), fórmula `c₁=(p+2c)/3` paridade vanilla,
  preservação hash 11º passo consecutivo.

Cobertura `curve` agora **completa** para os 5 segment kinds
vanilla: `move` / `line` / `cubic` / `quadratic` / `close`. Único
remanescente é a sintaxe scope-methods (`curve.move(...)` em vez
de `("move", ...)`), bloqueada por scope methods stdlib —
frente independente.

---

## §9 — Frentes pendentes pós-P294

| Frente | Estado |
|---|---|
| `native_quadratic` standalone | ✅ **fechado P294** — preferimos integração em `native_curve` |
| `curve.move`/`curve.cubic`/`curve.quadratic` scope methods (sintaxe vanilla fiel) | Bloqueado por scope methods em stdlib — frente independente |
| `native_path` (parser SVG path string) | Frente vanilla separada; não bloqueia curve |
| Outras operações Bézier (length, intersection, subdivide) | Passos próprios futuros |
| `PathItem::QuadraticTo` variant nativo | **Não necessário** — A.0.0 N=2 confirma vanilla pattern β |

---

## §10 — Decisão sobre P295

Frentes ortogonais disponíveis (P292 §9.2 ranking + novos):

1. **math-accent-cancel** — frente original P292 ranking #2.
2. **footnote-cluster** — frente original P292 ranking #3.
3. **`curve.move`/scope-methods** — extensão sintáctica curve.
4. **`native_path` SVG-string parser** — frente vanilla separada.
5. **`Length` em `Stroke`** — variant cobertura tabela cobertura
   linha 201 (`parcial`).

Decisão fica para o operador humano. **P294 não dita P295**.

Se §8.7' (A.0.0 template) reaplicar em P295 com refutação genuína,
N≥3 atingido → candidato a promoção ADR-meta.

---

## §11 — Honestidade epistémica registada

P294 documenta um caso paradigmático de **divergência consciente
entre spec e realidade**:

1. **Spec foi escrita** assumindo divergência arquitectural de
   vanilla (variant interno + emit novo + quebra de hash).
2. **Fase A obrigatória** (ADR-0065 inventariar-primeiro) revelou
   que a assumption é **factualmente incorrecta** — vanilla
   resolve elegante via conversão.
3. **A.0.0 N=2** (template inaugurado P293) capturou e
   documentou a refutação antes de qualquer escrita de código.
4. **Resultado**: implementação 5× menor que a spec previa, hash
   preservado, e paridade vanilla **superior** ao plano original.

Este caso fortalece §8.7' (A.0.0 template) como mecanismo robusto
contra escrita de código baseada em assumptions não-verificadas.

---

## §12 — Fecho

P294 fechado com:

- **+6 testes net** (6 P294 L1 + 1 P294 L3 − 1 P293 scope-out) — todos
  verdes.
- **0 violations** no `crystalline-lint`.
- **Hash `export.rs` preservado** (11º passo consecutivo).
- **0 ADRs meta novas** — §8.3 N=6 candidato genuíno adiado per
  P273.17 §0; anti-padrão over-formalização honrado.
- **7.º paradigma consumer arquitecturalmente distinto** registado
  (P288-P294 série consecutiva).
- **Spec invalidada arquitecturalmente** pela A.0.0 N=2 — desvio
  documentado e justificado.

**MARCO**:
- **2.º passo ortogonal pós-série cumulativa** P288-P292.
- **A.0.0 reaplicada N=2** com refutação **mais significativa** que
  a inaugural — template §8.7' robustecido.
- **Conversão matemática em construct-time** estabelecida como
  pattern arquitectural — paralelo conceptual ao ADR-0099
  "activação posterior" mas com paradigma novo "transform-on-build".
- Cobertura `curve` completa para os 5 segment kinds vanilla.
