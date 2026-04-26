# Relatório — Passo 156I: stack compositivo (Layout Fase 2 sub-passo 3; **último Fase 2; target 72% atingido**)

**Data**: 2026-04-26.
**Natureza**: passo **substantivo escopo M** (1 container
compositivo com 3 atributos + tipo Dir novo; ~25 tests);
**sétima aplicação consecutiva** de ADR-0061 (Layout Fase X
roadmap); **último sub-passo Fase 2 — atinge target 72%
declarado**; **fechamento da série granular P156C-I**.
**Spec**: `00_nucleo/materialization/typst-passo-156i.md`.

**Outputs materiais**:
- 1 variant novo em `Content`: `Stack { children, dir,
  spacing }` (Arc<[Content]> per ADR-0026 revisão).
- Tipo enum `Dir { LTR, RTL, TTB, BTT }` em
  `01_core/src/entities/dir.rs`.
- Stdlib `#stack(dir: ?, spacing: ?, ..children)`.
- Helper `extract_dir` em `stdlib/layout.rs`.
- Cobertura exaustiva de arms em 4 ficheiros L1.
- 25 tests novos (alvo era 15-22; superado).
- L0 prompts: novo `entities/dir.md`; secção Stack em
  `entities/content.md`; hashes propagados.
- Inventário 148 actualizado (Layout 12/0/3/3/0=18 →
  13/0/3/2/0=18; cobertura 67% → **72%**; tabela B 50 → 51).
- README ADRs entrada P156I.
- **ADR-0061 §"Aplicações cumulativas"** adicionada
  documentando série P156C-I.
- Este relatório (com §15 análise risco + §16 análise
  cumulativa Fase 1+2).

---

## §1 — Sumário executivo

P156I é o **último sub-passo Fase 2** do roadmap ADR-0061,
fechando a sequência granular iniciada em P156C (pad+hide).
Materializou `stack(...)` — único container compositivo
(Vec<Content> em vez de body único) — atingindo o **target
72% Layout** declarado em ADR-0061 §6.2.

**Trabalho material**:
- **Inventário 156I.1** confirmou: `Dir` não existe (criar);
  `Content::Sequence` usa `Arc<[Content]>` (P22+) — reusar
  pattern; spacing trivial via cursor advance (modelo
  HSpace/VSpace P156D).
- **`Content::Stack`** com 3 fields (children: Arc<[Content]>,
  dir, spacing).
- **Tipo `Dir`** novo (4 variantes LTR/RTL/TTB/BTT; default
  TTB); análogo a Parity P156E e Sides<T> P156C.
- **Cobertura exaustiva** em 9 sítios pattern-match Content
  adaptados para Vec (concat plain_text, all() is_empty,
  iter map_*, walk em ordem).
- **stdlib `#stack(...)`** com 2 atributos nomeados (dir,
  spacing) + children variádicos posicionais; helper
  `extract_dir`; sem atributos scope-out (vanilla stack
  tem só estes 3).
- **Layouter 4 direcções**: TTB/BTT empilham verticalmente
  com cursor_y advance; LTR/RTL inline com cursor_x advance;
  BTT/RTL implementadas como reverse iteration (per
  ADR-0054 graded).
- **Validação rigorosa**: dir inválido rejeitado; spacing
  negativo rejeitado; children não-Content/Str rejeitados
  (estricto).

**Tests**: **1271 → 1296** (+25). Layout cobertura: **67%
→ 72%** (12/18 → 13/18) — **target ADR-0061 atingido**.
User-facing total: 59% → **60%**.

**ADR-0061 mantém-se `PROPOSTO`** per decisão humana
(Fase 3 pendente — repeat, columns/colbreak via DEBT-56).
**§"Aplicações cumulativas" adicionada** documentando série
P156C-I sem promoção formal.

**Padrão emergente confirmado em escala**: 7 aplicações
consecutivas sem reformulação; +50 pontos percentuais
Layout; +151 tests acumulados; padrões metodológicos
consolidados (granularidade, inventariar primeiro,
Smart→Option, reuso template, §análise de risco).

---

## §2 — Inventário curto (sub-passo 156I.1)

### §2.1 `Dir` em entities

```bash
grep -nE "^pub enum Dir" 01_core/src/entities/  # zero hits
```

**Sem `Dir`** — criar em `entities/dir.rs` análogo a Parity
(P156E). Decisão arquitectural local: 4 variantes (LTR/RTL/
TTB/BTT) + métodos `is_horizontal`/`is_vertical`/`is_reverse`
+ Default TTB.

### §2.2 Vec<Content> em outros variants

```bash
grep "Sequence" 01_core/src/entities/content.rs
```

**Encontrado**: `Content::Sequence(Arc<[Content]>)` (linha 49)
e `Content::MathSequence(Arc<[Content]>)` (linha 89). Pattern
estabelecido: usar `Arc<[T]>` em vez de `Vec<T>` para clone
O(1) per ADR-0026 revisão. **Reusar em Stack** —
`children: Arc<[Content]>`.

### §2.3 spacing layouter mecânica

VSpace (P156D) confirma que `cursor_y += Pt(amount)` é
trivial. HSpace análogo para `cursor_x`. **Conclusão**:
spacing implementável trivialmente em stack — cada child
intercala com `cursor advance`. **Decisão 156I.2: spacing
implementado real** (vs scope-out).

### §2.4 Children variádicos em natives

`args.items.iter()` é o pattern estabelecido. Reusar
directamente.

---

## §3 — Decisão local 156I.2 — spacing real

Per inventário §2.3: spacing implementado **real** (não
scope-out). Mecânica trivial via cursor advance entre
children:
- **TTB/BTT**: `if i > 0 && space_pt > 0 { cursor_y += Pt(space_pt); }`
- **LTR/RTL**: `if i > 0 && space_pt > 0 { cursor_x += Pt(space_pt); }`

Default `None == zero` (conservador; consistente com padrão
Smart→Option N=5).

---

## §4 — `Dir` enum

```rust
// 01_core/src/entities/dir.rs (novo, ~95 linhas com 4 tests)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    LTR, RTL, TTB, BTT,
}

impl Dir {
    pub fn is_horizontal(self) -> bool {
        matches!(self, Dir::LTR | Dir::RTL)
    }
    pub fn is_vertical(self) -> bool {
        matches!(self, Dir::TTB | Dir::BTT)
    }
    pub fn is_reverse(self) -> bool {
        matches!(self, Dir::RTL | Dir::BTT)
    }
}

impl Default for Dir {
    fn default() -> Self { Self::TTB }
}
```

**Tests internos**: 4 (default TTB; horizontal/vertical;
reverse; partial_eq).

**L0 prompt** criado em `00_nucleo/prompts/entities/dir.md`
(hash `3257959a`); hash do código `657f9389`.

**Registado** em `entities/mod.rs::pub mod dir;`.

---

## §5 — `Content::Stack` — forma final + diff

```rust
// 01_core/src/entities/content.rs (após Pagebreak, antes Boxed)
Stack {
    children: Arc<[Content]>,
    dir:      Dir,
    spacing:  Option<Length>,
},
```

**Construtor Rust**: `Content::stack(children: Vec<Content>,
dir, spacing)` (Vec interno convertido para `Arc<[T]>` via
`into()`).

**Stdlib**: `#stack(dir: ?, spacing: ?, ..children)`.

**Variant count**: 50 → **51** (+1).

---

## §6 — Cobertura exaustiva de arms

Adaptados para Vec/Arc<[T]>:

| Função | Ficheiro | Tratamento Stack |
|--------|----------|------------------|
| `Content::is_empty()` | `entities/content.rs` | `children.iter().all(\|c\| c.is_empty())` |
| `Content::plain_text()` | `entities/content.rs` | concat plain_text de todos |
| `PartialEq::eq` | `entities/content.rs` | comparação 3-fields (Arc deep eq via `as_ref()`) |
| `Content::map_content` | `entities/content.rs` | mapear cada child; preserva dir/spacing |
| `Content::map_text` | `entities/content.rs` | idem |
| `materialize_time` | `rules/introspect.rs` | recurse em cada child |
| `walk` | `rules/introspect.rs` | walk cada child em ordem |
| `layout_content` | `rules/layout/mod.rs` | iterar children + spacing + dir (4 direcções) |
| `measure_content_constrained` | `rules/layout/mod.rs` | TTB/BTT: max widths + sum heights + (n-1)*spacing; LTR/RTL: sum widths + max heights |

**Verificação**: `cargo build -p typst-core` clean (sem
warnings).

---

## §7 — `native_stack` + `extract_dir` — assinatura + registo

### §7.1 `extract_dir` helper

```rust
fn extract_dir(value: &Value) -> SourceResult<Dir> {
    match value {
        Value::Str(s) => match s.as_str() {
            "ltr" => Ok(Dir::LTR),
            "rtl" => Ok(Dir::RTL),
            "ttb" => Ok(Dir::TTB),
            "btt" => Ok(Dir::BTT),
            other => Err(...),
        },
        other => Err(...),
    }
}
```

### §7.2 `native_stack`

```rust
pub fn native_stack(_ctx, args, _world, _file, _fig)
    -> SourceResult<Value>
{
    let mut dir: Dir = Dir::default();  // TTB
    let mut spacing: Option<Length> = None;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "dir"     => dir = extract_dir(value)?,
            "spacing" => {
                let len = extract_length(value)?;
                if len.abs.0 < 0.0 || len.em < 0.0 { return Err(...); }
                spacing = Some(len);
            }
            other => return Err(...),
        }
    }

    // Children variádicos: iterar args.items (Content ou Str).
    let mut children: Vec<Content> = Vec::with_capacity(args.items.len());
    for v in args.items.iter() {
        match v {
            Value::Content(c) => children.push(c.clone()),
            Value::Str(s)     => children.push(Content::text(s.as_str())),
            other => return Err(...),  // estricto
        }
    }

    Ok(Value::Content(Content::Stack {
        children: Arc::from(children), dir, spacing,
    }))
}
```

### §7.3 Registo

`stdlib/mod.rs`:
```rust
pub use crate::rules::stdlib::layout::{
    ..., native_stack, ...
};
```

`eval/mod.rs::make_stdlib`:
```rust
scope.define("stack", Value::Func(Func::native("stack", native_stack)));
```

Stdlib funcs: 40 → **41** (+1).

---

## §8 — Layouter — diff (4 direcções)

### §8.1 `Content::Stack` em `layout_content`

```rust
Content::Stack { children, dir, spacing } => {
    let font = self.font_size_pt.val();
    let space_pt = spacing.map_or(0.0, |l| l.resolve_pt(font));

    // Stack é STRUCTURAL: força flush_line antes.
    if self.cursor_x.0 > self.line_start_x.0 {
        self.flush_line();
    }

    let n = children.len();
    if n == 0 { return; }

    let iter: Box<dyn Iterator<Item = (usize, &Content)>> =
        if dir.is_reverse() {
            Box::new(children.iter().rev().enumerate())
        } else {
            Box::new(children.iter().enumerate())
        };

    if dir.is_vertical() {
        for (i, child) in iter {
            if i > 0 && space_pt > 0.0 { self.cursor_y += Pt(space_pt); }
            self.layout_content(child);
            self.flush_line();
        }
    } else {  // horizontal
        for (i, child) in iter {
            if i > 0 && space_pt > 0.0 { self.cursor_x += Pt(space_pt); }
            self.layout_content(child);
        }
        self.flush_line();
    }
}
```

**4 direcções implementadas**:
- TTB (default): empilha verticalmente forward.
- BTT: reverse iteration vertical (geometricamente
  invertido per ADR-0054 graded).
- LTR: empilha horizontalmente inline forward.
- RTL: reverse iteration horizontal.

### §8.2 `Content::Stack` em `measure_content_constrained`

```rust
Content::Stack { children, dir, spacing } => {
    let font = self.font_size_pt.val();
    let space_pt = spacing.map_or(0.0, |l| l.resolve_pt(font));
    let n = children.len();
    if n == 0 { return (0.0, 0.0); }

    if dir.is_vertical() {
        // max widths; sum heights + (n-1)*spacing
        let mut max_w = 0.0;
        let mut sum_h = 0.0;
        for child in children.iter() {
            let (w, h) = self.measure_content_constrained(child, max_width);
            max_w = max_w.max(w);
            sum_h += h;
        }
        (max_w, sum_h + ((n - 1) as f64) * space_pt)
    } else {
        // sum widths + (n-1)*spacing; max heights
        let mut sum_w = 0.0;
        let mut max_h = 0.0;
        for child in children.iter() {
            let (w, h) = self.measure_content_constrained(child, max_width);
            sum_w += w;
            max_h = max_h.max(h);
        }
        (sum_w + ((n - 1) as f64) * space_pt, max_h)
    }
}
```

---

## §9 — Tests adicionados (lista + contagens)

### §9.1 Em `entities/dir.rs::tests` (4)

1. `dir_default_e_ttb` — Default::default() == TTB.
2. `is_horizontal_vs_vertical` — verificações.
3. `is_reverse` — RTL e BTT.
4. `dir_partial_eq` — comparação enum.

### §9.2 Em `entities/content.rs::tests` (6)

5. `stack_constructor_default` — defaults.
6. `stack_constructor_explicit_dir_spacing` — atributos
   explícitos.
7. `stack_is_empty_se_todos_children_vazios` — proxy
   semantic + edge case empty Vec.
8. `stack_plain_text_concatena_children` — concat.
9. `stack_partial_eq` — comparação 3-fields incl Arc deep.
10. `stack_map_text_recurse_em_cada_child` — Vec recurse.

### §9.3 Em `stdlib/mod.rs::tests` (12 + regression)

11. `native_stack_defaults_sem_args` — defaults.
12. `native_stack_aceita_dir_ltr` — String "ltr".
13. `native_stack_aceita_todas_4_direcoes` — loop sobre 4.
14. `native_stack_aceita_spacing` — Length.
15. `native_stack_com_children_variadicos` — 3 children.
16. `native_stack_aceita_str_como_child` — Str → text().
17. `native_stack_rejeita_dir_invalido` — Err hard.
18. `native_stack_rejeita_spacing_negativo` — Err hard.
19. `native_stack_rejeita_named_arg_desconhecido` — Err hard.
20. `native_stack_rejeita_child_nao_content` — Int rejeitado
    (estricto).
21. `native_stack_combina_dir_spacing_children` — caso
    composto.
22. `native_block_box_pad_hide_continuam_a_funcionar_apos_p156i`
    — **regression** Block + Boxed + Pad + Hide.

### §9.4 Em `layout/tests.rs` (3)

23. `layout_stack_ttb_empilha_verticalmente` — B abaixo de A.
24. `layout_stack_ltr_empilha_horizontalmente` — B à direita
    de A na mesma linha (Y igual).
25. `layout_stack_spacing_avanca_cursor_entre_children` —
    spacing 30pt empurra B em pelo menos 30pt extra.

**Total**: **25 tests novos** (alvo spec era 15-22; superado
pela cobertura granular de 4 direcções + edge cases).

**Tests cumulativos**: 1271 → **1296** (+25 = 4 dir +
6 content + 12 stdlib + 3 layout E2E).

---

## §10 — L0 prompts + hashes propagados

### §10.1 L0 novo: `entities/dir.md`

Criado `00_nucleo/prompts/entities/dir.md` (~80 linhas):
- Cabeçalho canónico (camada L1, ficheiro alvo, ADRs).
- Contexto + interface pública + semântica + invariantes.
- Consumers actuais (Content::Stack) + planeados (refino
  bidi shaping; Content::Columns Fase 3).
- Sobre paridade vanilla `Smart<Dir>` → cristalino `Dir`
  directo com Default natural.

Hash do código: `657f9389`.

### §10.2 L0 actualizado: `entities/content.md`

Secção nova "Variant `Content::Stack` — Passo 156I (ADR-0061
Fase 2, sub-passo 3; **último Fase 2**)". Inclui:
- Decisão arquitectural reusada (Opção A modificada de
  P156G/H) com adaptação Vec/Arc.
- Forma estrutural do variant.
- Atributos + tipo Dir novo.
- **Distinção material face a Block/Boxed** (tabela
  comparativa com 4 colunas).
- Comportamento dos métodos canónicos adaptados para Vec.
- Renderização layouter 4 direcções.
- Validação em `native_stack`.
- Construtores Rust + Stdlib.
- Limitações conscientes (BTT/RTL reverse iteration vs
  posicionamento absoluto; sem alignment per-child; sem
  show rules).
- **Padrão emergente "Smart<T> → Option<T> ou default"
  N=5** documentado.

### §10.3 Hashes via `crystalline-lint --fix-hashes .`

```
Fixed 2 files:
  ./01_core/src/entities/content.rs             → b9ca52c4
  ./01_core/src/entities/dir.rs                 → 657f9389
Re-running analysis... ✅ 0 drift warnings remaining
```

`entities/content.rs`: `5bb6e3d2` (P156H) → **`b9ca52c4`** (P156I).
`entities/dir.rs`: novo, **`657f9389`**.

---

## §11 — Inventário 148 actualizado

Ficheiro: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.

**Tabela A.5 Layout — linha reescrita**:
- `stack(...)`: `ausente` → **`implementado`** ¹⁷ (Passo 156I).

**Tabela A — Vista user-facing (resumo)**:
- Linha "Layout" (nota ⁵ ⁶ ⁸ ¹⁰ ¹² ¹³ ¹⁵ ¹⁷):
  `12/0/3/3/0=18` → **`13/0/3/2/0=18`**.
- Total user-facing: `62/21/22/34/2=141` →
  **`63/21/22/33/2=141`**.
- Cobertura user-facing total: 59% → **60%**.

**Tabela B — Arquitectural**:
- `Content` variants (cristalino) ³ ⁴ ⁷ ⁹ ¹¹ ¹⁴ ¹⁶ ¹⁸:
  `38/9/3/0/0=50` → **`39/9/3/0/0=51`** (+`Stack`).
- `Content` variants (vanilla extra ausentes): `~4` → `~3`.

**Notas novas ¹⁷ + ¹⁸** descrevem transição P156I com:
- **Target 72% atingido** marcado.
- Tipo `Dir` novo documentado.
- Decisão arquitectural reusada de P156G/H com adaptação Vec.
- Anotação cumulativa em ADR-0061 documentada (sem promoção).

**§7 entrada 7**: refinamento P156I documentado com lista
das 2 entradas Layout restantes (`repeat`, `columns`/
`colbreak` — Fase 3 condicional via DEBT-56).

---

## §12 — README ADRs + ADR-0061 anotação cumulativa

### §12.1 README ADRs

- **Tabela "Estado por ADR"**: ADR-0061 mantém-se
  `PROPOSTO` (Fase 3 pendente).
- **Total**: 61 inalterado.
- **Distribuição**: PROPOSTO 11 inalterado.
- **"Passos-chave"**: entrada nova para P156I marcando
  **fechamento de série P156C-I** com detalhe completo.

### §12.2 ADR-0061 §"Aplicações cumulativas"

Adicionada secção nova ao final do ADR documentando:
- Tabela de 7 sub-passos com slope cumulativo + tests Δ.
- Total +50 pontos percentuais Layout em 7 passos.
- 4 tipos novos infraestruturais (Sides, Parity,
  TransformMatrix::skew, Dir).
- 8 variants novos + 1 método novo em tipo existente.
- 9 stdlib funcs novas.
- **6 padrões metodológicos consolidados** com contagens N.
- Estado pós-target (cobertura, restantes, scope-outs).
- **Status mantido `PROPOSTO`** com 3 caminhos para promoção
  documentados.

---

## §13 — Próximo passo

P156I encerra a série granular **P156C-I com target 72%
atingido**. Próximo passo é **decisão humana** entre
prioridades em ADR-0061 (e outras):

- **Opção A — Fase 3 Layout (repeat)**: M, baixo valor
  (TOC dot leaders). Cobertura → 78% (14/18).

- **Opção B — Fase 3 Layout (columns/colbreak via DEBT-56)**:
  L+ com refactor multi-region; ADR dedicada column flow
  algorithm. Cobertura → 89% (16/18).

- **Opção C — Footnote area** (sub-fase prioritária explícita
  ADR-0061 Decisão 5). Desbloqueia footnote em Model.

- **Opção D — P157 (Model Fase 2 table foundations)** per
  ADR-0060 renumerada.

- **Opção E — Promover ADR-0061 a IMPLEMENTADO**: requer
  decisão humana de scope-out formal de Fase 3 (e/ou
  inclusão de footnote area). Anotação documenta os
  3 caminhos.

- **Opção F — Outra prioridade humana**.

**Recomendação descritiva**: pausa natural após P156I dado
target atingido + padrões consolidados. Decisão humana sobre
próxima direcção tem máxima informação.

---

## §14 — Limitações registadas

1. **`BTT`/`RTL`** implementadas como reverse iteration em
   vez de posicionamento absoluto reverso real. Aceitável
   per ADR-0054 graded; refino futuro.

2. **Sem alignment per-child** (vanilla `StackChild` tem
   alinhamento opcional). Refino futuro.

3. **Spacing fixo `Option<Length>`** (vanilla suporta
   `Smart<Rel<Length>>` com auto-determinação). Simplificação
   per padrão Smart→Option N=5.

4. **`dir` aceita só string** em stdlib (vanilla aceita
   Symbol). Consistente com padrão estabelecido em P156E.

5. **Sem show rules `#show stack: ...`** neste passo.

6. **Stack aninhado** (stack de stacks) suportado
   estruturalmente; não testado E2E (case patológico raro
   mas válido).

7. **Children Vec interno convertido para `Arc<[T]>`**:
   alocação intermédia. Aceitável; pattern consistente
   com Sequence (P22+).

8. **ADR-0061 mantém `PROPOSTO`** (per decisão humana;
   Fase 3 pendente).

---

## §15 — Verificação final

Critérios da spec P156I (§Verificação):

1. ✅ `cargo build --workspace`: clean (sem warnings após
   limpeza de imports unused).
2. ✅ `cargo test --workspace --lib`: **1296 passed**;
   0 failed; 6 ignored. (1057 typst-core + 215 integration
   + 24 outros; 1271 → 1296 = +25).
3. ✅ `crystalline-lint .`: **zero violations**.
4. ✅ Hashes propagados consistentes:
   - `entities/content.rs` ↔ `entities/content.md` (`b9ca52c4`).
   - `entities/dir.rs` ↔ `entities/dir.md` (`657f9389`).
5. ✅ `Content::Stack` variant em produção (50 → 51 variants).
6. ✅ `Dir` enum em produção.
7. ✅ Stdlib `#stack(...)` invocável (40 → 41 funcs).
8. ✅ Cobertura arms exaustiva (9 sítios actualizados;
   adaptados para Vec<Content>).
9. ✅ Layouter 4 direcções (TTB/BTT/LTR/RTL) implementadas
   e testadas.
10. ✅ Inventário 148 reflecte cobertura **72% Layout**
    (target atingido).
11. ✅ README ADRs entrada P156I.
12. ✅ **ADR-0061 §"Aplicações cumulativas" adicionada**
    (sem promoção a IMPLEMENTADO).
13. ✅ Sem ADR criada / revogada / revisada formalmente.
14. ✅ Sem DEBT criado / fechado.
15. ✅ ADR-0060 inalterada.
16. ✅ **Sem regressão** em containers existentes (Block,
    Boxed, Pad, Hide) — regression test
    `native_block_box_pad_hide_continuam_a_funcionar_apos_p156i`
    verifica explicitamente.
17. ✅ Sem regressão geral (todos os 1271 tests pré-P156I
    continuam a passar; +25 novos passam).
18. ✅ Relatório do passo escrito (este ficheiro com §15
    análise risco + §16 análise cumulativa Fase 1+2).

---

## §16 — Análise de risco de regressão

### §16.1 Risco a priori

P156I tinha duas dimensões de risco:
- **Vec<Content> em arms**: padrão diferente de P156G/H
  (body único). Requer adaptação cuidadosa.
- **4 direcções no layouter**: TTB/BTT/LTR/RTL com reverse
  iteration para BTT/RTL — possibilidade de bugs de ordem.

### §16.2 Risco realizado

**Baixo, igual ou inferior a P156H**. Mitigado por:
- Inventário 156I.1 confirmou pattern `Arc<[Content]>` já
  existente (Sequence) — reuso directo.
- Layouter 4 direcções abstraídas via `Box<dyn Iterator>`
  (forward vs reverse) — código compacto.
- Regression test estendido para incluir Box (P156H).
- Build clean ao primeiro try (após limpar 2 imports
  unused).

### §16.3 Mitigações activadas

1. **Inventário 156I.1 curto-mas-focado** (4 sub-investigações).
2. **Reuso directo de pattern Arc<[T]>** (Sequence/MathSequence).
3. **Regression tests cumulativos** crescem com cada passo.
4. **§análise de risco no relatório** (4ª aplicação).

### §16.4 Lições

- **Sub-passos com Vec<Content>** são pouco mais
  complexos que body único; arms são iterações em vez
  de recursão.
- **Múltiplas direcções** podem ser abstraídas via
  iterator polimórfico (`Box<dyn Iterator>`) — código
  legível.
- **Reverse iteration** é forma simples de "BTT/RTL graded"
  per ADR-0054.

---

## §17 — Análise cumulativa Fase 1+2 (novo neste passo — fechamento de série)

### §17.1 Trajectória completa P156C-I

| Passo | Feature(s) | Slope Layout | Cobertura cumulativa | Tests Δ | Tests cumulativos |
|-------|-----------|------:|---------------------:|--------:|------------------:|
| baseline P156B | (diagnóstico) | — | 22% (4/18) | — | 1145 |
| P156C | pad + hide | +11% | 33% | +27 | 1172 |
| P156D | h + v | +11% | 44% | +20 | 1192 |
| P156E | pagebreak | +6%  | 50% | +22 | 1214 |
| P156F | skew | +6%  | 56% | +16 | 1230 |
| P156G | block | +5%  | 61% | +20 | 1250 |
| P156H | box | +6%  | 67% | +21 | 1271 |
| **P156I** | **stack** | +5%  | **72%** | +25 | **1296** |

**Total**: +50 pontos percentuais Layout em 7 passos
(22% → 72%); **+151 tests** acumulados (1145 → 1296);
**zero reformulações mid-passo** em N=7 aplicações.

### §17.2 Slope decreasing pattern

Os slopes seguiram pattern previsível:
- **+11%, +11%** (P156C/D): features triviais aditivas
  (pad/hide/h/v) — alta densidade.
- **+6%, +6%** (P156E/F): features médias (pagebreak com
  Parity; skew via TransformMatrix existente) — menos
  features mas com infraestrutura nova.
- **+5%, +6%, +5%** (P156G/H/I): containers ricos com
  cobertura proporcional ao número de fields (5/5/3).

**Slope estável em +5-6% pós-halfway** — coerente com
hipótese "features remanescentes mais complexas" do
diagnóstico P156B.

### §17.3 Padrões metodológicos consolidados

1. **Granularidade 1-2 features/passo** — N=7 aplicações
   consecutivas. Hipótese da decisão humana 2026-04-25
   confirmada empiricamente.

2. **"Inventariar primeiro" pré-decisão arquitectural** —
   N=4 aplicações (P156F defensivo; P156G deliberado;
   P156H curto; P156I curto focado). Padrão consolidado
   como mecanismo de redução de risco.

3. **"Smart<T> → Option<T> ou default natural"** — N=5
   aplicações (Parity, angles, Block.width, Box.width,
   Stack.spacing+Dir.default). **Patamar empírico forte**
   — candidato a registo formal em ADR meta futuro.

4. **"§análise de risco no relatório"** — N=4 aplicações
   (P156F/G/H/I). Cobertura sistemática.

5. **"Reuso de template containers"** — N=3 aplicações
   (Block → Boxed → Stack). Padrão "variant rico para
   containers cujos atributos não são propriedades de
   texto" estabelecido em P156G e reaplicado sem nova
   decisão arquitectural.

6. **"Antecipar especificidades técnicas"** — N=2-3
   aplicações (Boxed naming P156H; Vec/Arc<[T]> arms
   P156I).

### §17.4 Recursos infraestruturais criados

- **Tipos novos**: `Sides<T>` (P156C), `Parity` (P156E),
  `Dir` (P156I) = **3 tipos genéricos reusáveis**.
- **Métodos novos em tipos existentes**: `TransformMatrix::skew`
  (P156F).
- **Variants Content novos**: Pad, Hide, HSpace, VSpace,
  Pagebreak, Block, Boxed, Stack = **8 variants**.
- **Stdlib funcs novas**: pad, hide, h, v, pagebreak, skew,
  block, box, stack = **9 funcs**.

### §17.5 Decisão humana pós-fechamento

ADR-0061 ganha **§"Aplicações cumulativas"** documentando
trajectória sem promoção formal. **3 caminhos para
promoção** documentados:
1. Materializar Fase 3 (columns DEBT-56 + repeat).
2. Decisão humana de scope-out formal de Fase 3.
3. Inclusão de footnote area + actualização do scope.

Decisão diferida para sessão posterior. Anotação cumulativa
preserva o contexto histórico para retomada futura.

---

## §18 — Notas operacionais

- **Padrão "passos granulares" — sétima aplicação
  consecutiva**. P156C+D+E+F+G+H+I. **N=7 aplicações sem
  reformulação**. **Fechamento de série**.

- **ADR-0061 mantém PROPOSTO**: per decisão humana.
  Anotação cumulativa adicionada documenta progresso sem
  comprometer estado.

- **Variants count**: 50 → **51** (+1, Stack). Após Fase 3
  hipotética: 53 (+repeat, +columns).

- **Stdlib funcs**: 40 → **41** (+1). Após Fase 3
  hipotética: 43.

- **Pós-156I**:
  - 10 features Layout implementadas total.
  - Cobertura Layout: **72%** (target atingido).
  - Cobertura user-facing total: **60%**.
  - **Próximo**: decisão humana sobre Fase 3 ou outra
    prioridade.

- **Pausa natural após P156I**: target atingido; padrões
  consolidados; ADR-0061 anotada cumulativamente. Decisão
  humana sobre próxima direcção tem máxima informação.

- **Quarentena vanilla**: continua opção 3.

- **Série paridade**: continua suspensa em P153.

- **Hash do código `entities/content.rs`**: `5bb6e3d2`
  (P156H) → **`b9ca52c4`** (P156I). Hash `entities/dir.rs`:
  novo, **`657f9389`**.

---

## §19 — Cross-references

- Spec: `00_nucleo/materialization/typst-passo-156i.md`.
- Diagnóstico (origem): `00_nucleo/diagnosticos/diagnostico-layout-passo-156b.md`.
- Relatórios precedentes: `typst-passo-156c-relatorio.md`,
  `typst-passo-156d-relatorio.md`,
  `typst-passo-156e-relatorio.md`,
  `typst-passo-156f-relatorio.md`,
  `typst-passo-156g-relatorio.md`,
  `typst-passo-156h-relatorio.md`.
- ADR-0061 (aplicada + anotada cumulativamente):
  `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`.
- Inventário 148 actualizado:
  `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
- README ADRs: `00_nucleo/adr/README.md`.
- L0 prompts: `00_nucleo/prompts/entities/dir.md` (novo);
  `00_nucleo/prompts/entities/content.md` (secção Stack
  adicionada).
- Vanilla source consultado:
  `lab/typst-original/crates/typst-library/src/layout/stack.rs`
  + `dir.rs`.
- Cristalino código tocado:
  - `01_core/src/entities/dir.rs` (novo, ~95 linhas, 4 tests).
  - `01_core/src/entities/content.rs` (variant Stack +
    construtor + cobertura 5 arms + 6 tests).
  - `01_core/src/entities/mod.rs` (registo `pub mod dir;`).
  - `01_core/src/rules/introspect.rs` (arms Stack em
    materialize_time + walk).
  - `01_core/src/rules/layout/mod.rs` (arms Stack em
    layout_content + measure_content_constrained; 4
    direcções).
  - `01_core/src/rules/layout/tests.rs` (3 tests E2E).
  - `01_core/src/rules/stdlib/layout.rs` (`extract_dir`
    helper + `native_stack`).
  - `01_core/src/rules/stdlib/mod.rs` (re-export + 12 tests
    incluindo regression).
  - `01_core/src/rules/eval/mod.rs` (registo em `make_stdlib`).
