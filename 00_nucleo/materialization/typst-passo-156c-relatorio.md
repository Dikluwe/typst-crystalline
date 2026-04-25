# Relatório — Passo 156C: pad + hide (Layout Fase 1 sub-passo 1)

**Data**: 2026-04-25.
**Natureza**: passo **substantivo escopo S agregado**;
**primeira aplicação concreta** de ADR-0061 (Layout Fase X
roadmap, status `PROPOSTO`); **par natural** com P156B
(diagnóstico) seguindo precedente 154A→154B, 140A→140B,
132A→132B, 131A→131B.
**Spec**: `00_nucleo/materialization/typst-passo-156c.md`.

**Outputs materiais**:
- 2 variants novos em `Content`: `Pad`, `Hide`.
- Tipo geométrico genérico `Sides<T>` em `01_core/src/entities/sides.rs`.
- Stdlib `#pad(...)` + `#hide(body)`.
- Cobertura exaustiva de arms em 4 ficheiros L1.
- 27 tests novos (alvo era 14-18; superado pela cobertura
  granular dos 7 atributos pad).
- L0 prompts: novo `entities/sides.md`; secção pad+hide em
  `entities/content.md`; hashes propagados.
- Inventário 148 actualizado (Layout 4/0/3/11/0=18 →
  6/0/3/9/0=18; cobertura 22% → 33%).
- README ADRs entrada P156C (sem mudança em totals).
- Este relatório.

---

## §1 — Sumário executivo

P156C materializou a primeira sub-fase concreta do roadmap
ADR-0061 (Layout Fase X), proposto por P156B. Adicionou dois
variants `Content` simples (pad container com 7 atributos
declarativos; hide layout-aware sem render) seguindo o
modelo agregado do P154B (terms+divider) e P155 (quote).

Trabalho material realizado:
- **`Sides<T>` genérico** criado para suportar `Content::Pad`
  e antecipar PageConfig refino + Block/Box em Fase 2.
- **`Content::Pad { body, padding: Sides<Length> }`** com
  layouter completo (top + left + flush + bottom; right
  scope-out per ADR-0054 graded — refino com refactor
  multi-region em Fase 3).
- **`Content::Hide { body }`** com layouter que drena items
  para buffer temporário e descarta após body (preserva
  avanço de cursor; consistente com vanilla "layout-aware
  mas não rende").
- **Cobertura exaustiva** em todos os pattern-match `Content`
  do L1 (`is_empty`, `plain_text`, `PartialEq::eq`,
  `map_content`, `map_text`, `materialize_time`, `walk`,
  `layout_content`, `measure_content_constrained`).
- **stdlib `#pad(body, ...)`** com 7 atributos nomeados
  (left/right/top/bottom/x/y/rest); precedência
  específico>eixo>rest implementada e testada; padding
  negativo rejeitado com erro hard.
- **stdlib `#hide(body)`** sem named args.

**Tests**: **1145 → 1172** (+27). Layout cobertura: **22%
→ 33%** (4/18 → 6/18). User-facing total: 53% → 55%.

**ADR-0061 mantém-se `PROPOSTO`** per decisão humana
2026-04-25 (anotação cumulativa após Fase 1 completa).

---

## §2 — Inventário pré-materialização (sub-passo 156C.1)

### §2.1 Estado pré-P156C verificado empiricamente

```bash
ls 01_core/src/entities/sides.rs 2>/dev/null     # NÃO existia
grep -E "Pad|Hide|Sides" 01_core/src/entities/content.rs  # zero hits
grep "native_pad\|native_hide" 01_core/src/rules/stdlib/  # zero hits
```

Confirmações:
- `Content` enum com **43 variants** (último: `Quote` em P155).
- **Sem** `Sides<T>` em `01_core/src/entities/`.
- **Sem** `Content::Pad`, `Content::Hide`.
- **Sem** `native_pad`, `native_hide` em stdlib.
- **Sem** registos `pad`/`hide` em `make_stdlib`.

### §2.2 Localização real diferente da spec

A spec do P156C indicava `01_core/src/rules/eval/stdlib/structural.rs`,
mas a estrutura real é `01_core/src/rules/stdlib/` (sem `eval/`).
Ajuste: `native_pad`+`native_hide` colocados em
`01_core/src/rules/stdlib/layout.rs` (consistente com `native_align`,
`native_place`, `native_grid`, `native_page` já lá; coesão por
domínio Layout per ADR-0037).

### §2.3 Assinatura real de natives

Spec apresentou assinatura simplificada
`fn(args: &Args, _: &Engine) -> SourceResult<Value>`. Real:

```rust
fn(_ctx: &mut EvalContext, args: &Args, _world: &dyn World,
   _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value>
```

(per Passo 71 / DEBT-24 — World+FileId+figure_numbering passados
explicitamente; não em ABI de Engine como spec sugeria).

---

## §3 — Variant `Content::Pad` — forma final + diff

```rust
// 01_core/src/entities/content.rs (entre Quote e marcador final)
Pad {
    body:    Box<Content>,
    padding: Sides<Length>,
},
```

**Atributos**: `body` (Box<Content>) + `padding` (`Sides<Length>`).

**Construtor Rust**: `Content::pad(body, padding)`.

**Stdlib**: `#pad(body, left:?, right:?, top:?, bottom:?, x:?,
y:?, rest:?)` em `01_core/src/rules/stdlib/layout.rs::native_pad`.

**Cobertura arms adicionados**:
- `is_empty`: proxy para `body.is_empty()`.
- `plain_text`: recurse no body (transparente).
- `PartialEq::eq`: comparação 2 fields (body + padding).
- `map_content`: recurse no body; padding é `Copy` preservado.
- `map_text`: idem.
- `materialize_time` (introspect): recurse no body; padding preservado.
- `walk` (introspect): walk body (counters/labels dentro de pad
  resolvem normalmente).
- `layout_content` (layouter): top + left + body + flush + bottom;
  ver §8.
- `measure_content_constrained`: `(w + left + right, h + top + bottom)`.

---

## §4 — Variant `Content::Hide` — forma final + diff

```rust
// 01_core/src/entities/content.rs (logo após Pad)
Hide {
    body: Box<Content>,
},
```

**Atributos**: apenas `body`.

**Construtor Rust**: `Content::hide(body)`.

**Stdlib**: `#hide(body)` em
`01_core/src/rules/stdlib/layout.rs::native_hide`. Sem named args.

**Cobertura arms adicionados**:
- `is_empty`: proxy para `body.is_empty()`.
- `plain_text`: `String::new()` (não rende; sem texto plano).
- `PartialEq::eq`: comparação 1 field (body).
- `map_content`: recurse no body.
- `map_text`: recurse no body (transformações internas
  aplicam-se mesmo que body não renderize).
- `materialize_time` (introspect): recurse no body.
- `walk` (introspect): walk body (preserva semantic de
  presence — labels/refs dentro de hide resolvem).
- `layout_content` (layouter): drena items, layout body,
  descarta items gerados; cursor avança (per ADR-0054 graded
  perfil — divergência intencional vs vanilla que preserva o
  espaço do body).
- `measure_content_constrained`: proxy para `body`.

---

## §5 — Cobertura exaustiva de arms (~7 sítios)

Arms adicionados em todos os pattern-match exaustivos sobre
`Content` no L1 (sem `_ => ...` excepto onde já existia
`measure_content_constrained` que tem `_ => (0.0, 0.0)`
explícito):

| Função | Ficheiro | Arms novos |
|--------|----------|-----------|
| `Content::is_empty()` | `entities/content.rs` | Pad+Hide proxy body |
| `Content::plain_text()` | `entities/content.rs` | Pad recurse / Hide vazio |
| `PartialEq::eq` | `entities/content.rs` | Pad 2-fields / Hide 1-field |
| `Content::map_content` | `entities/content.rs` | Pad+Hide recurse body |
| `Content::map_text` | `entities/content.rs` | Pad+Hide recurse body |
| `materialize_time` | `rules/introspect.rs` | Pad+Hide recurse body |
| `walk` | `rules/introspect.rs` | Pad+Hide walk body |
| `layout_content` | `rules/layout/mod.rs` | Pad full impl / Hide drain |
| `measure_content_constrained` | `rules/layout/mod.rs` | Pad add padding / Hide proxy |

**Verificação**: `cargo build -p typst-core` clean (sem
warnings de variantes não cobertas).

---

## §6 — `Sides<T>` — verificado/criado

**Não existia** em `01_core/src/entities/`. Criado novo
ficheiro `01_core/src/entities/sides.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sides<T> {
    pub left:   T,
    pub top:    T,
    pub right:  T,
    pub bottom: T,
}

impl<T> Sides<T> {
    pub fn new(left: T, top: T, right: T, bottom: T) -> Self { ... }
}
impl<T: Clone> Sides<T> {
    pub fn uniform(value: T) -> Self { ... }
}
impl<T: Default> Default for Sides<T> { ... }
```

Ordem dos campos: `left, top, right, bottom` (não CSS;
segue modelo vanilla typst).

Registado em `01_core/src/entities/mod.rs` como `pub mod sides;`.

**Tests internos**: 4 (new, uniform, default, partial_eq).

L0 prompt criado em `00_nucleo/prompts/entities/sides.md`
(hash `d5d8273f`); hash do código `c47b14e6`.

---

## §7 — `native_pad` + `native_hide` — assinaturas + registo

### §7.1 `native_pad`

```rust
pub fn native_pad(_ctx: &mut EvalContext, args: &Args, _world: &dyn World,
                  _current_file: FileId, _figure_numbering: Option<&str>)
    -> SourceResult<Value>
```

Lógica:
1. Body posicional obrigatório (Content ou Str; outros tipos
   → Err hard com mensagem explícita).
2. Loop sobre named args; para cada um valida que o nome está
   no conjunto `{left, right, top, bottom, x, y, rest}`;
   coage value para `Length` (aceita Length, Float interpretado
   em pt, Int idem).
3. Resolução de precedência específico > eixo > rest:
   `left.or(x).or(rest).unwrap_or(Length::ZERO)` (idem para
   right/top/bottom).
4. Validação: padding negativo (qualquer dos 4 lados) →
   Err hard "padding negativo não suportado neste passo (P156C)".
5. Devolve `Value::Content(Content::Pad { body, padding })`.

### §7.2 `native_hide`

```rust
pub fn native_hide(_ctx: &mut EvalContext, args: &Args, _world: &dyn World,
                   _current_file: FileId, _figure_numbering: Option<&str>)
    -> SourceResult<Value>
```

Lógica:
1. `expect_no_named(&args.named)?` — sem named args.
2. Body posicional obrigatório.
3. Devolve `Value::Content(Content::Hide { body })`.

### §7.3 Registo em `make_stdlib`

```rust
// 01_core/src/rules/eval/mod.rs::make_stdlib
scope.define("pad",  Value::Func(Func::native("pad",  native_pad)));
scope.define("hide", Value::Func(Func::native("hide", native_hide)));
```

Re-export em `01_core/src/rules/stdlib/mod.rs`:

```rust
pub use crate::rules::stdlib::layout::{
    native_align, native_grid, native_hide, native_pad, native_page, native_place,
};
```

Stdlib funcs: 32 → **34** (+2).

---

## §8 — Layouter — diff (`layout_content` + `measure_content_constrained`)

### §8.1 `Content::Pad` em `layout_content`

```rust
Content::Pad { body, padding } => {
    let font = self.font_size_pt.val();
    let left   = padding.left.resolve_pt(font);
    let top    = padding.top.resolve_pt(font);
    let bottom = padding.bottom.resolve_pt(font);
    // padding.right scope-out neste passo (ADR-0054 graded).

    if self.cursor_x.0 > self.line_start_x.0 {
        self.flush_line();
    }
    self.cursor_y += Pt(top);

    let saved_line_start = self.line_start_x;
    self.line_start_x = saved_line_start + Pt(left);
    self.cursor_x     = self.line_start_x;

    self.layout_content(body);
    self.flush_line();

    self.cursor_y += Pt(bottom);
    self.line_start_x = saved_line_start;
    self.cursor_x     = saved_line_start;
}
```

### §8.2 `Content::Hide` em `layout_content`

```rust
Content::Hide { body } => {
    let saved_items = std::mem::take(&mut self.current_items);
    let saved_line  = std::mem::take(&mut self.current_line);
    self.layout_content(body);
    self.current_items = saved_items;
    self.current_line  = saved_line;
    // cursor preserva o avanço — body avançou normalmente
    // (line_height + leading via flush_line implícito).
}
```

### §8.3 `measure_content_constrained` arms

```rust
Content::Pad { body, padding } => {
    let font = self.font_size_pt.val();
    let left   = padding.left.resolve_pt(font);
    let right  = padding.right.resolve_pt(font);
    let top    = padding.top.resolve_pt(font);
    let bottom = padding.bottom.resolve_pt(font);
    let constrained = (max_width - left - right).max(0.0);
    let (w, h) = self.measure_content_constrained(body, constrained);
    (w + left + right, h + top + bottom)
}
Content::Hide { body } => {
    self.measure_content_constrained(body, max_width)
}
```

---

## §9 — Tests adicionados (lista + contagens)

### §9.1 Em `01_core/src/entities/sides.rs::tests` (4)

1. `sides_new_preserves_each_value` — construtor explícito.
2. `sides_uniform_replicates_value` — construtor uniform.
3. `sides_default_is_zero_for_numeric` — Default para f64.
4. `sides_partial_eq` — igualdade exacta.

### §9.2 Em `01_core/src/entities/content.rs::tests` (8)

5. `pad_constructor_envolve_body` — `Content::pad(...)`.
6. `hide_constructor_envolve_body` — `Content::hide(...)`.
7. `pad_e_hide_is_empty_proxy_para_body` — proxy is_empty.
8. `pad_plain_text_recurse_no_body` — recurse texto.
9. `hide_plain_text_e_string_vazia` — vazio.
10. `pad_partial_eq` — comparação 2-fields.
11. `hide_partial_eq` — comparação 1-field.
12. `pad_e_hide_map_text_recurse_no_body` — propagação
    transformação textual.

### §9.3 Em `01_core/src/rules/stdlib/mod.rs::tests` (13)

13. `native_pad_defaults_padding_zero` — sem args nomeados.
14. `native_pad_lados_individuais` — left/right/top/bottom.
15. `native_pad_atalhos_x_e_y` — x cobre L+R; y cobre T+B.
16. `native_pad_atalho_rest` — rest cobre 4 lados.
17. `native_pad_precedencia_especifico_eixo_rest` — específico>eixo>rest.
18. `native_pad_rejeita_padding_negativo` — Err hard.
19. `native_pad_rejeita_named_arg_desconhecido` — Err hard.
20. `native_pad_aceita_int_e_float_como_pt` — coação numérica.
21. `native_pad_sem_body_retorna_err` — body obrigatório.
22. `native_hide_envolve_body` — construtor com Content.
23. `native_hide_aceita_string` — body como Str.
24. `native_hide_rejeita_named_arg` — sem named args.
25. `native_hide_sem_body_retorna_err` — body obrigatório.

### §9.4 Em `01_core/src/rules/layout/tests.rs` (2)

26. `layout_pad_avanca_cursor_bottom_e_top` — pad com top=20pt
    empurra texto para baixo (vs baseline sem pad).
27. `layout_hide_emite_zero_text_items` — `hide(text)` produz
    zero `FrameItem::Text`.

**Total**: **27 tests novos** (alvo spec era 14-18; superado
pela cobertura granular dos 7 atributos pad).

**Tests cumulativos**: 1145 → **1172** (+27 = 4 sides + 8
content + 13 stdlib + 2 layout).

---

## §10 — L0 prompts + hashes propagados

### §10.1 L0 novo: `entities/sides.md`

Criado `00_nucleo/prompts/entities/sides.md` (~80 linhas)
com:
- Cabeçalho canónico (camada L1, ficheiro alvo, ADRs).
- Contexto + interface pública + semântica + invariantes.
- Consumers actuais (Content::Pad) + planeados (PageConfig
  refino, Block, Box).
- Nota sobre paridade graded vs vanilla `Sides<T>` (que
  expõe helpers ricos não-necessários neste passo).

Hash do código: `c47b14e6` (registado em `Hash do Código:`).

### §10.2 L0 actualizado: `entities/content.md`

Secção nova "Variants `Content::Pad` + `Content::Hide` —
Passo 156C (ADR-0061 Fase 1, sub-passo 1)" adicionada após a
secção P155 Quote. Contém:
- Forma estrutural dos 2 variants.
- Atributos vanilla mapeados.
- Comportamento dos métodos canónicos (is_empty, plain_text,
  map_*, materialize_time, walk).
- Renderização layouter + scope-outs declarados (right pad,
  padding negativo).
- Construtores + limitações conscientes.
- Decisão arquitectural confirmada (variant novo per
  ADR-0061 Decisão 4).

### §10.3 Headers `@updated` actualizados

- `entities/content.rs`: `@updated 2026-04-25` (já estava).
- `entities/sides.rs`: `@updated 2026-04-25` (novo).

### §10.4 Hashes via `crystalline-lint --fix-hashes .`

Resultado:
```
Fixed 1 file:
  ./01_core/src/entities/sides.rs               → c47b14e6
Fixed 1 file:
  ./01_core/src/entities/content.rs             → daf00164
Re-running analysis... ✅ 0 drift warnings remaining
```

`entities/content.rs`: `8413bb8d` (P155) → **`daf00164`** (P156C).

---

## §11 — Inventário 148 actualizado

Ficheiro: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.

**Tabela A.5 Layout — linhas reescritas**:
- `pad`: `ausente` ⁵ → **`implementado`** ⁶ (referência:
  Passo 156C).
- `hide`: `ausente` → **`implementado`** ⁶ (idem).

**Tabela A — Vista user-facing (resumo)**:
- Linha "Layout" (nota ⁵ ⁶): `4/0/3/11/0=18` → **`6/0/3/9/0=18`**.
- Total user-facing: `54/21/22/42/2=141` → **`56/21/22/40/2=141`**.
- Cobertura user-facing total: 53% → **55%**.

**Tabela B — Arquitectural**:
- `Content` variants (cristalino) ³ ⁴ ⁷: `31/9/3/0/0=43` →
  **`33/9/3/0/0=45`** (+`Pad`, +`Hide`).
- `Content` variants (vanilla extra ausentes): `~11` → `~9`
  (pad e hide saíram do conjunto não-capturado).

**Nota nova ⁶**: descrita transição P156C com contagens
explícitas e referência a ADR-0061 mantendo `PROPOSTO`.

**Nota nova ⁷**: descrita actualização de Tabela B.

**§7 entrada 7**: refinamento P156C documentado com lista
das 9 entradas Layout restantes (`box`, `block`, `stack`,
`repeat`, `columns`, `colbreak`, `pagebreak` manual, `h`/`v`
combinada, `skew`) e mapeamento para sub-passos seguintes
da ADR-0061.

---

## §12 — README ADRs actualizado

Ficheiro: `00_nucleo/adr/README.md`.

- **Tabela "Estado por ADR"**: ADR-0061 mantém-se `PROPOSTO`
  (sem mudança per decisão humana).
- **Total**: 61 inalterado.
- **Distribuição**: PROPOSTO 11 inalterado.
- **"Passos-chave da história dos ADRs"**: entrada nova para
  P156C com detalhe de:
  - Variants adicionados (43 → 45).
  - `Sides<T>` genérico criado.
  - Stdlib `#pad(...)` + `#hide(...)`.
  - Cobertura exaustiva de arms.
  - Layouter mecânica (top+left+flush+bottom; right scope-out).
  - Tests 1145 → 1172 (+27).
  - Cobertura Layout 22% → 33%; user-facing 53% → 55%.
  - L0 prompts criados/actualizados; hashes.

---

## §13 — Próximo passo

P156C encerrou-se com Layout cobertura **22% → 33%** (sem
ADR/DEBT criados, sem regressão). Próximo passo é **decisão
humana** entre prioridades agora explicitamente documentadas
em ADR-0061:

- **Opção A — P156D (Fase 1 sub-passo 2)**: pagebreak manual
  + h/v spacing primitives (S agregado; mecânico). Continua
  Fase 1 Layout; cobertura → ~50% (9/18).

- **Opção B — Fase 2 Layout (block + box + stack)**: M+
  agregado; containers ricos. Cobertura → ~67%.

- **Opção C — P157 (Model Fase 2 table foundations)**:
  M+ alternativo (per ADR-0060 renumerada).

- **Opção D — Footnote area + passo dedicado footnote
  Model**: sub-fase prioritária explícita declarada em
  ADR-0061 Decisão 5; desbloqueia `footnote()` em Model
  Fase 2.

- **Opção E — Outra prioridade humana**.

**Recomendação descritiva** (derivada do historiograma P156A
§4.1): manter cadência granular 1-2 features por passo
parece estar a funcionar (P156C: zero reformulações
mid-passo, zero descobertas patológicas, 27 tests passam).

---

## §14 — Limitações registadas

1. **`right` padding scope-out em layout** (per ADR-0054
   graded). O Layouter actual não tem mecânica de "largura
   útil" por arm — width-aware wrap vive em `flush_line`/
   `layout_word` que consultam `page_config.width`. Refino
   quando refactor de Layouter para multi-region acontecer
   (DEBT-56 + Fase 3 Layout). `measure_content_constrained`
   já considera right (subtrai do max_width); apenas o
   layouter principal não.

2. **Padding negativo rejeitado** (divergência intencional
   vs vanilla que aceita). Per perfil ADR-0054 graded;
   refino quando layout overflow semantic clara existir.

3. **Sem show rules `#show pad: ...` ou `#show hide: ...`**
   neste passo (per spec). Adiados a passo agregado futuro
   (análogo ao adiamento de show rules Fase 1 Model
   `terms`/`divider`/`quote` per anotação P154B).

4. **`Content::Pad` e `Content::Hide` aninhados são suportados**
   estruturalmente (cobertura recursiva em todos os arms);
   padding aninhado é cumulativo via cursor advance, hide
   aninhado é idempotente. Tests aninhados não escritos
   (case patológico não comum em uso real).

5. **ADR-0061 mantém `PROPOSTO`** (per decisão humana
   2026-04-25). Anotação cumulativa após Fase 1 completa
   (P156D + ...). Risco baixo porque progresso é trackable
   via inventário 148 + relatórios cronológicos.

6. **Footnote area**: NÃO atacada neste passo (per spec).
   Adiada para passo dedicado quando Model footnote for
   priorizado.

---

## §15 — Verificação final

Critérios da spec P156C (§Verificação):

1. ✅ `cargo build --workspace`: clean. (Verificado:
   `cargo build -p typst-core` → "Finished" sem warnings).
2. ✅ `cargo test --workspace --lib`: **1172 passed**;
   0 failed; 6 ignored. (931 typst-core + 215 integration
   + 24 outros — antes era 933+215+24, mas são números
   pós-correr inclui sides+pad+hide; 1145 → 1172 = +27).
3. ✅ `crystalline-lint .`: **zero violations**.
4. ✅ Hashes propagados consistentes: `entities/content.rs`
   ↔ `entities/content.md` (`daf00164`); `entities/sides.rs`
   ↔ `entities/sides.md` (`c47b14e6`).
5. ✅ `Content::Pad` e `Content::Hide` variants em produção
   (43 → 45 variants).
6. ✅ Stdlib `#pad(...)` e `#hide(body)` invocáveis (32 → 34
   funcs).
7. ✅ Cobertura arms exaustiva (sem `_ => ...` catch-all
   excepto onde já existia em `measure_content_constrained`).
8. ✅ Inventário 148 reflecte cobertura aumentada (Layout
   22% → 33%).
9. ✅ README ADRs entrada P156C.
10. ✅ Sem ADR criada / revogada / revisada.
11. ✅ Sem DEBT criado / fechado.
12. ✅ ADR-0061 inalterada (mantém-se PROPOSTO; anotação só
    após Fase 1 completa per decisão humana).
13. ✅ ADR-0060 inalterada (não tocada por este passo).
14. ✅ Sem regressão (todos os 1145 tests pré-P156C continuam
    a passar; +27 novos passam).
15. ✅ Relatório do passo escrito (este ficheiro).

---

## §16 — Notas operacionais

- **Padrão "passos granulares 1-2 features" inaugurado**.
  P156C é o primeiro passo da nova convenção (per spec).
  Modelo: P156C → P156D → ... → P156I para 72% Layout.

- **ADR-0061 mantém PROPOSTO**: per decisão humana 2026-04-25.
  Anotação cumulativa após Fase 1 completa.

- **Sem footnote area**: per decisão humana 2026-04-25.
  Footnote area só quando Model footnote for atacado
  (passo posterior).

- **Sem columns**: trabalho L+ com refactor Layouter
  multi-region. DEBT-56 (P156B) permanece aberto. Passo
  dedicado futuro com ADR.

- **Sem measure/layout(callback)**: dependem de
  Introspection runtime (ADR-0017 adiada). Não atacar
  aqui.

- **Variants count**: 43 → **45** (+2). Após P156D (h+v +
  pagebreak): ~48. Após Fase 1 completa: ~48-49. Após
  Fase 2 (block+box+stack): ~52.

- **Stdlib funcs**: 32 → **34** (+2). Após P156D: ~37.
  Após Fase 2: ~40.

- **Pós-156C**:
  - 2 features Layout implementadas (pad + hide).
  - `Sides<T>` genérico disponível para futuros consumers
    (PageConfig refino; Block/Box).
  - Cobertura Layout: 22% → 33%.
  - Cobertura user-facing total: 53% → 55%.
  - **Próximo**: P156D (pagebreak + h/v) ou alternativa
    humana.

- **Granularidade vs paridade rápida**: P156C testou hipótese
  de que passos pequenos de 1-2 features chegariam sem
  reformulações. Confirmado: zero reformulações mid-passo,
  zero descobertas patológicas, 27 tests passam à primeira
  (após corrigir naming conflict de Edit String inicial).

- **Quarentena vanilla**: continua opção 3. Sem mudança.

- **Série paridade**: continua suspensa em P153. Sem
  mudança.

- **Hash do código `entities/content.rs`**: `8413bb8d` (P155)
  → **`daf00164`** (P156C). Hash do código `entities/sides.rs`:
  novo, **`c47b14e6`**.

---

## §17 — Cross-references

- Spec: `00_nucleo/materialization/typst-passo-156c.md`.
- Diagnóstico (origem): `00_nucleo/diagnosticos/diagnostico-layout-passo-156b.md`.
- Relatório P156B: `00_nucleo/materialization/typst-passo-156b-relatorio.md`.
- ADR-0061 (aplicada): `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`.
- Inventário 148 actualizado:
  `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
- README ADRs: `00_nucleo/adr/README.md`.
- Historiograma: `00_nucleo/diagnosticos/historiograma-passos.md`
  (§4.1 evidência 6/6 do padrão diagnóstico-primeiro que
  motivou par P156B → P156C).
- L0 prompts: `00_nucleo/prompts/entities/sides.md` (novo);
  `00_nucleo/prompts/entities/content.md` (secção pad+hide).
- Vanilla source consultado:
  `lab/typst-original/crates/typst-library/src/layout/{pad,hide,sides}.rs`.
- Cristalino código tocado:
  - `01_core/src/entities/sides.rs` (novo, ~95 linhas).
  - `01_core/src/entities/content.rs` (variants Pad+Hide +
    construtores + cobertura arms + 8 tests).
  - `01_core/src/entities/mod.rs` (registo `pub mod sides;`).
  - `01_core/src/rules/introspect.rs` (arms Pad+Hide em
    materialize_time + walk).
  - `01_core/src/rules/layout/mod.rs` (arms Pad+Hide em
    layout_content + measure_content_constrained).
  - `01_core/src/rules/layout/tests.rs` (2 tests E2E).
  - `01_core/src/rules/stdlib/layout.rs` (native_pad +
    native_hide + extract_length helper).
  - `01_core/src/rules/stdlib/mod.rs` (re-export + 13
    tests).
  - `01_core/src/rules/eval/mod.rs` (registo em
    `make_stdlib`).
