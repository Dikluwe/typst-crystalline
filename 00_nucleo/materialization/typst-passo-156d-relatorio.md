# Relatório — Passo 156D: h + v spacing (Layout Fase 1 sub-passo 2)

**Data**: 2026-04-25.
**Natureza**: passo **substantivo escopo S agregado**;
**segunda aplicação consecutiva** de ADR-0061 (Layout Fase X
roadmap, status `PROPOSTO`); cadência granular 1-2 features
por passo confirmada (P156C foi primeira).
**Spec**: `00_nucleo/materialization/typst-passo-156d.md`.

**Outputs materiais**:
- 2 variants novos em `Content`: `HSpace`, `VSpace`.
- Stdlib `#h(amount, weak: false)` + `#v(amount, weak: false)`.
- Helper `build_spacing` partilhado por h/v.
- Cobertura exaustiva de arms em 4 ficheiros L1.
- 20 tests novos (alvo era 10-15; superado por cobertura
  granular de weak + edge cases).
- L0 prompt `entities/content.md` ganha secção HSpace+VSpace;
  hash propagado.
- Inventário 148 actualizado (Layout 6/0/3/9/0=18 →
  8/0/3/7/0=18; cobertura 33% → 44%).
- README ADRs entrada P156D.
- Este relatório.

---

## §1 — Sumário executivo

P156D continuou a sequência granular Layout Fase 1 iniciada
em P156C (pad+hide). Adicionou dois variants `Content`
spacing primitives (`HSpace`/`VSpace`) seguindo o modelo
estrutural de P156C (variants leaves, sem body) e padrões
arquitecturais já estabelecidos (assinatura 5-param de
natives; localização canónica em `stdlib/layout.rs`; helpers
partilhados via `extract_length` de P156C + novo
`build_spacing`).

**Trabalho material realizado**:
- **`Content::HSpace { amount: Length, weak: bool }`** com
  layouter que avança `cursor_x` por `amount` resolvido em pt.
- **`Content::VSpace { amount: Length, weak: bool }`** com
  layouter que força `flush_line` antes de avançar `cursor_y`
  (caso contrário texto na linha actual fica meio-render).
- **Cobertura exaustiva** em todos os pattern-match `Content`
  do L1: HSpace/VSpace tratados como leaves no `map_content`,
  `map_text`, `materialize_time`, `walk` (clone directo);
  `is_empty` proxy `amount.is_zero()`; `plain_text` vazio;
  `PartialEq` compara amount + weak.
- **stdlib `#h(amount, weak: false)`** e **`#v(amount,
  weak: false)`** com validação completa (amount obrigatório,
  Length/Float/Int aceites, negativo rejeitado, named args
  desconhecidos rejeitados, weak deve ser Bool).
- **Helper `build_spacing`** partilhado por native_h/native_v
  evita duplicação (~30 linhas de validação reusadas).

**Tests**: **1172 → 1192** (+20). Layout cobertura: **33%
→ 44%** (6/18 → 8/18). User-facing total: 55% → 56%.

**ADR-0061 mantém-se `PROPOSTO`** per decisão humana 2026-04-25
(anotação cumulativa após Fase 1 completa).

**Padrão granular confirmado**: zero reformulações mid-passo;
zero descobertas patológicas; build incremental clean ao
primeiro try. Hipótese da decisão humana validada por
segunda vez consecutiva.

---

## §2 — Inventário pré-materialização (sub-passo 156D.1)

### §2.1 Estado pré-P156D verificado

```bash
grep -E "HSpace|VSpace" 01_core/src/entities/content.rs   # zero hits
grep -E "native_h|native_v" 01_core/src/rules/stdlib/      # zero hits
```

Confirmações:
- `Content` enum com **45 variants** pós-P156C (último: `Hide`).
- **Sem** `Content::HSpace`, `Content::VSpace`.
- **Sem** `native_h`, `native_v` em stdlib.
- `extract_length` helper de P156C disponível em
  `stdlib/layout.rs` (reutilizável).

### §2.2 Localização confirmada

`01_core/src/rules/stdlib/layout.rs` — coesão por domínio
Layout per ADR-0037; mesmo ficheiro que `native_align`,
`native_place`, `native_grid`, `native_page`, `native_pad`,
`native_hide`. Decisão de P156C aplicada directamente.

---

## §3 — Variant `Content::HSpace` — forma final + diff

```rust
// 01_core/src/entities/content.rs (após Hide)
HSpace {
    amount: Length,
    weak:   bool,
},
```

**Construtor Rust**: `Content::h_space(amount, weak)` (naming
`_space` evita conflito com identificadores curtos `h`/`v`
em scope Rust).

**Stdlib**: `#h(amount, weak: false)` em `stdlib/layout.rs::native_h`.

**Cobertura arms adicionados**:
- `is_empty`: `amount.is_zero()`.
- `plain_text`: `String::new()` (não rende texto).
- `PartialEq::eq`: comparação 2 fields (amount + weak).
- `map_content`: terminal (clone directo); HSpace é leaf.
- `map_text`: idem terminal.
- `materialize_time` (introspect): clone directo.
- `walk` (introspect): no-op (sem body, sem effect em counters).
- `layout_content`: `cursor_x += Pt(amount.resolve_pt(font))`.
- `measure_content_constrained`: `(amount_pt, 0.0)`.

---

## §4 — Variant `Content::VSpace` — forma final + diff

```rust
VSpace {
    amount: Length,
    weak:   bool,
},
```

**Construtor Rust**: `Content::v_space(amount, weak)`.

**Stdlib**: `#v(amount, weak: false)` em `native_v`.

**Cobertura arms adicionados**: idênticos a HSpace, com
divergência no `layout_content`:
- `if cursor_x > line_start_x { flush_line(); }` antes do
  avanço (termina linha em curso para evitar texto
  meio-render).
- `cursor_y += Pt(amount.resolve_pt(font))`.
- `measure_content_constrained`: `(0.0, amount_pt)`.

---

## §5 — Cobertura exaustiva de arms

Arms adicionados em todos os pattern-match exaustivos sobre
`Content` no L1:

| Função | Ficheiro | Tratamento HSpace / VSpace |
|--------|----------|---------------------------|
| `Content::is_empty()` | `entities/content.rs` | `amount.is_zero()` |
| `Content::plain_text()` | `entities/content.rs` | `String::new()` (combinado num arm) |
| `PartialEq::eq` | `entities/content.rs` | comparação 2-fields cada |
| `Content::map_content` | `entities/content.rs` | terminal (clone) |
| `Content::map_text` | `entities/content.rs` | terminal (clone) |
| `materialize_time` | `rules/introspect.rs` | clone (leaf) |
| `walk` | `rules/introspect.rs` | no-op (leaf, sem effect counters) |
| `layout_content` | `rules/layout/mod.rs` | HSpace cursor.x++; VSpace flush+cursor.y++ |
| `measure_content_constrained` | `rules/layout/mod.rs` | HSpace `(amount,0)`; VSpace `(0,amount)` |

**Verificação**: `cargo build -p typst-core` clean (sem
warnings de variantes não cobertas).

---

## §6 — `native_h` + `native_v` — assinaturas + registo

### §6.1 Helpers partilhados

```rust
fn extract_weak(args: &Args, fn_name: &str) -> SourceResult<bool>
fn build_spacing(args: &Args, fn_name: &str, valid_named: &[&str])
    -> SourceResult<(Length, bool)>
```

`build_spacing` agrega:
1. Extrai amount (Length, posicional obrigatório).
2. Valida não-negativo (per perfil ADR-0054 graded).
3. Valida que named args estão na whitelist.
4. Resolve weak via `extract_weak` (Bool ou default false).

### §6.2 `native_h` e `native_v`

```rust
pub fn native_h(...) -> SourceResult<Value> {
    let (amount, weak) = build_spacing(args, "h", &["weak"])?;
    Ok(Value::Content(Content::HSpace { amount, weak }))
}

pub fn native_v(...) -> SourceResult<Value> {
    let (amount, weak) = build_spacing(args, "v", &["weak"])?;
    Ok(Value::Content(Content::VSpace { amount, weak }))
}
```

Lógica idêntica modulo o variant produzido — refactor natural
seguindo princípio DRY.

### §6.3 Registo em `make_stdlib`

```rust
scope.define("h", Value::Func(Func::native("h", native_h)));
scope.define("v", Value::Func(Func::native("v", native_v)));
```

Re-export em `stdlib/mod.rs`:

```rust
pub use crate::rules::stdlib::layout::{
    native_align, native_grid, native_h, native_hide, native_pad,
    native_page, native_place, native_v,
};
```

Stdlib funcs: 34 → **36** (+2).

---

## §7 — Layouter — diff

### §7.1 `Content::HSpace` em `layout_content`

```rust
Content::HSpace { amount, weak: _ } => {
    let pt = amount.resolve_pt(self.font_size_pt.val());
    self.cursor_x += Pt(pt);
}
```

### §7.2 `Content::VSpace` em `layout_content`

```rust
Content::VSpace { amount, weak: _ } => {
    let pt = amount.resolve_pt(self.font_size_pt.val());
    if self.cursor_x.0 > self.line_start_x.0 {
        self.flush_line();
    }
    self.cursor_y += Pt(pt);
}
```

### §7.3 `measure_content_constrained` arms

```rust
Content::HSpace { amount, .. } => {
    (amount.resolve_pt(self.font_size_pt.val()), 0.0)
}
Content::VSpace { amount, .. } => {
    (0.0, amount.resolve_pt(self.font_size_pt.val()))
}
```

`weak` ignorado nestes arms (collapse adiado per scope-out
declarado).

---

## §8 — Tests adicionados (lista + contagens)

### §8.1 Em `entities/content.rs::tests` (7)

1. `hspace_constructor` — `Content::h_space(...)`.
2. `vspace_constructor` — `Content::v_space(...)`.
3. `hspace_e_vspace_is_empty_se_amount_zero` — proxy zero.
4. `hspace_e_vspace_plain_text_vazio` — sem texto.
5. `hspace_partial_eq` — comparação 2-fields.
6. `vspace_partial_eq` — comparação 2-fields + cross-variant.
7. `hspace_e_vspace_map_text_preserva` — leaves preservados.

### §8.2 Em `stdlib/mod.rs::tests` (11)

8. `native_h_aceita_length` — Length posicional.
9. `native_h_aceita_int_e_float_como_pt` — coação numérica.
10. `native_h_aceita_weak_true` — weak Bool.
11. `native_h_aceita_amount_zero` — edge case zero.
12. `native_h_rejeita_amount_negativo` — Err hard.
13. `native_h_rejeita_named_arg_desconhecido` — Err hard.
14. `native_h_rejeita_weak_nao_bool` — tipo errado.
15. `native_h_sem_amount_retorna_err` — body obrigatório.
16. `native_v_aceita_length_e_weak` — caso composto VSpace.
17. `native_v_rejeita_amount_negativo` — Err hard.
18. `native_v_sem_amount_retorna_err` — body obrigatório.

### §8.3 Em `layout/tests.rs` (2)

19. `layout_hspace_avanca_cursor_x` — `h(50pt)` afasta texto
    subsequente em pelo menos 50pt no eixo X.
20. `layout_vspace_avanca_cursor_y` — `v(30pt)` empurra texto
    subsequente em pelo menos 30pt no eixo Y após flush_line.

**Total**: **20 tests novos** (alvo spec era 10-15; superado
pela cobertura granular de weak + edge cases). Distribuição
ligeiramente assimétrica entre h e v (h tem 8 tests
individuais; v tem 3 tests por reaproveitar lógica
partilhada via `build_spacing`).

**Tests cumulativos**: 1172 → **1192** (+20 = 7 content + 11
stdlib + 2 layout).

---

## §9 — L0 prompts + hashes propagados

### §9.1 L0 actualizado: `entities/content.md`

Secção nova "Variants `Content::HSpace` + `Content::VSpace`
— Passo 156D (ADR-0061 Fase 1, sub-passo 2)" adicionada
após a secção P156C pad+hide. Inclui:
- Forma estrutural dos 2 variants.
- Atributos `amount` (Length) + `weak` (Bool).
- Comportamento dos métodos canónicos (is_empty proxy zero;
  plain_text vazio; map_* terminais).
- Renderização layouter + scope-outs declarados (Fraction
  amount; weak collapse; amount negativo; h overflow; v
  início-de-página).
- Validação em native_h/native_v.
- Construtores Rust (`h_space`/`v_space` naming).
- Decisão arquitectural confirmada (variant novo per
  ADR-0061 Decisão 4).

### §9.2 Headers `@updated` actualizados

- `entities/content.rs`: `@updated 2026-04-25` (já estava).

### §9.3 Hashes via `crystalline-lint --fix-hashes .`

```
Fixed 1 file:
  ./01_core/src/entities/content.rs             → 4a7e0a99
Re-running analysis... ✅ 0 drift warnings remaining
```

`entities/content.rs`: `daf00164` (P156C) → **`4a7e0a99`** (P156D).
`entities/content.md`: hash propagado para `7ffe1236`.

---

## §10 — Inventário 148 actualizado

Ficheiro: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.

**Tabela A.5 Layout — linha reescrita**:
- `h(amount)` / `v(amount)` (entrada agregada): `ausente`
  → **`implementado`** ⁸ (Passo 156D).

**Tabela A — Vista user-facing (resumo)**:
- Linha "Layout" (nota ⁵ ⁶ ⁸): `6/0/3/9/0=18` →
  **`8/0/3/7/0=18`**.
- Total user-facing: `56/21/22/40/2=141` →
  **`58/21/22/38/2=141`**.
- Cobertura user-facing total: 55% → **56%**.

**Tabela B — Arquitectural**:
- `Content` variants (cristalino) ³ ⁴ ⁷ ⁹: `33/9/3/0/0=45` →
  **`35/9/3/0/0=47`** (+`HSpace`, +`VSpace`).
- `Content` variants (vanilla extra ausentes): `~9` → `~7`
  (h e v saem do conjunto não-capturado).

**Nota nova ⁸**: descrita transição P156D com contagens
explícitas e referência a ADR-0061 mantendo `PROPOSTO`.

**Nota nova ⁹**: descrita actualização de Tabela B.

**§7 entrada 7**: refinamento P156D documentado com lista
das 7 entradas Layout restantes (`pagebreak` manual, `box`,
`block`, `stack`, `repeat`, `columns`/`colbreak`, `skew`)
e mapeamento para sub-passos seguintes da ADR-0061
(P156E pagebreak; Fase 2 block+box+stack; Fase 3 columns
+repeat+skew).

---

## §11 — README ADRs actualizado

Ficheiro: `00_nucleo/adr/README.md`.

- **Tabela "Estado por ADR"**: ADR-0061 mantém-se `PROPOSTO`.
- **Total**: 61 inalterado.
- **Distribuição**: PROPOSTO 11 inalterado.
- **"Passos-chave"**: entrada nova para P156D com detalhe
  de variants adicionados (45 → 47), stdlib `#h`+`#v`,
  cobertura arms, layouter diffs (HSpace cursor.x; VSpace
  flush+cursor.y), tests 1172 → 1192 (+20), Layout
  cobertura 33% → 44%, total user-facing 55% → 56%, hash
  `content.rs` `daf00164` → `4a7e0a99`.

---

## §12 — Próximo passo

P156D encerrou-se com Layout cobertura **33% → 44%** (sem
ADR/DEBT criados, sem regressão, sem reformulações). Próximo
passo é **decisão humana** entre prioridades agora
explicitamente documentadas em ADR-0061:

- **Opção A — P156E (Fase 1 sub-passo 3)**: pagebreak manual
  (S, 1 feature trivial). Continua Fase 1 Layout; cobertura
  → 50% (9/18). Após P156E, Fase 1 atinge as 5 features
  declaradas em ADR-0061 §6.1 menos footnote area
  (adiada).

- **Opção B — Fase 2 Layout (block + box + stack)**: M+
  agregado (3 containers ricos). Cobertura → 67%.

- **Opção C — P157 (Model Fase 2 table foundations)**:
  M+ alternativo (per ADR-0060 renumerada).

- **Opção D — Footnote area + passo dedicado footnote
  Model**: sub-fase prioritária explícita declarada em
  ADR-0061 Decisão 5.

- **Opção E — Outra prioridade humana**.

**Recomendação descritiva** (derivada do historiograma P156A
§4.1 + experiência P156C+P156D): cadência granular 1-2
features por passo está a funcionar consistentemente. P156E
(pagebreak) é a próxima sub-fase mais natural para fechar
Fase 1 Layout simples (sem footnote area, sem refinos).

---

## §13 — Limitações registadas

1. **`weak` collapse semantic não implementado**: atributo
   armazenado mas o layouter ignora. Vanilla colapsa weak
   adjacentes (e.g. `h(weak: true) + h(weak: true)` rende
   como single space). Cristalino mantém ambos
   (over-spacing aceitável per ADR-0054 graded). Refino
   futuro se priorizado.

2. **`amount` Fraction não suportado**: vanilla aceita
   `Fraction` (`h(1fr)` distribui espaço restante na linha).
   Cristalino só `Length` neste passo. Refino futuro per
   ADR-0061 §6.3.

3. **`amount` negativo rejeitado**: divergência intencional
   vs vanilla (que aceita; gera overlap). Refino quando
   layout overflow semantic clara existir.

4. **`h` no fim de linha não força wrap**: cursor.x apenas
   avança; pode exceder largura da página. Aceitável per
   ADR-0054 graded; refino com refactor multi-region
   (DEBT-56 + Fase 3).

5. **`v` no início de página/coluna não colapsa contra
   margem**: vanilla colapsa; cristalino faz avanço simples.
   Aceitável.

6. **Sem show rules `#show h: ...` ou `#show v: ...`**
   neste passo (consistente com adiamento P154B/P155/P156C).

7. **HSpace/VSpace aninhados em containers** (e.g.
   `pad(h(...))` ou `hide(v(...))`) não testados explicitamente,
   mas suportados estruturalmente (cobertura recursiva nos
   containers de P156C aplica-se).

8. **ADR-0061 mantém `PROPOSTO`** (per decisão humana
   2026-04-25). Anotação cumulativa após Fase 1 completa.

---

## §14 — Verificação final

Critérios da spec P156D (§Verificação):

1. ✅ `cargo build --workspace`: clean.
2. ✅ `cargo test --workspace --lib`: **1192 passed**;
   0 failed; 6 ignored. (953 typst-core + 215 integration
   + 24 outros; 1172 → 1192 = +20).
3. ✅ `crystalline-lint .`: **zero violations**.
4. ✅ Hashes propagados consistentes: `entities/content.rs`
   ↔ `entities/content.md` (`4a7e0a99`).
5. ✅ `Content::HSpace` e `Content::VSpace` variants em
   produção (45 → 47 variants).
6. ✅ Stdlib `#h(...)` e `#v(...)` invocáveis (34 → 36 funcs).
7. ✅ Cobertura arms exaustiva.
8. ✅ Inventário 148 reflecte cobertura aumentada (Layout
   33% → 44%).
9. ✅ README ADRs entrada P156D.
10. ✅ Sem ADR criada / revogada / revisada.
11. ✅ Sem DEBT criado / fechado.
12. ✅ ADR-0061 inalterada (mantém-se PROPOSTO).
13. ✅ ADR-0060 inalterada (não tocada por este passo).
14. ✅ Sem regressão (todos os 1172 tests pré-P156D continuam
    a passar; +20 novos passam).
15. ✅ Relatório do passo escrito (este ficheiro).

---

## §15 — Notas operacionais

- **Padrão "passos granulares 1-2 features" — segunda
  aplicação consecutiva confirma hipótese**. P156C foi
  primeira; P156D segunda. Ambas: zero reformulações
  mid-passo, build clean ao primeiro try, tests passam à
  primeira. Hipótese da decisão humana 2026-04-25
  empiricamente reforçada.

- **Helpers partilhados emergem naturalmente**: `extract_length`
  criado em P156C foi reusado em P156D; `build_spacing` novo
  em P156D antecipa reuso futuro (e.g. pagebreak manual em
  P156E pode partilhar validação de named args). Princípio
  DRY aplicado sem sobre-engenharia.

- **Naming `_space` sufixo nos construtores Rust**: `h_space`/
  `v_space` evita conflito com identificadores curtos `h`/`v`
  comuns em código Rust. Stdlib expõe `h`/`v` (paridade
  vanilla); só os helpers Rust internos têm sufixo.

- **ADR-0061 mantém PROPOSTO**: per decisão humana. Anotação
  cumulativa após Fase 1 completa (P156E + ...).

- **Variants count**: 45 → **47** (+2). Após P156E
  (pagebreak): 48. Fase 1 Layout fecha com 5 sub-passos
  (P156C pad+hide, P156D h+v, P156E pagebreak; sem footnote
  area = 5 features = 49 variants total se footnote ficar
  para passo dedicado).

- **Stdlib funcs**: 34 → **36** (+2). Após P156E: 37. Após
  Fase 2: ~40.

- **Pós-156D**:
  - 4 features Layout implementadas total via ADR-0061
    (pad, hide, h, v).
  - Cobertura Layout: 33% → 44%.
  - Cobertura user-facing total: 55% → 56%.
  - **Próximo**: P156E (pagebreak manual) ou alternativa
    humana.

- **Granularidade vs paridade rápida — tendência empírica
  confirmada**: P156C+P156D = 2 passos consecutivos de
  ~20-27 tests cada, cobertura +11%-12% Layout cada,
  zero reformulações. Se P156E mantiver o padrão, hipótese
  fica praticamente provada (N=3 aplicações consecutivas).

- **Quarentena vanilla**: continua opção 3. Sem mudança.

- **Série paridade**: continua suspensa em P153. Sem
  mudança.

- **Hash do código `entities/content.rs`**: `daf00164` (P156C)
  → **`4a7e0a99`** (P156D).

---

## §16 — Cross-references

- Spec: `00_nucleo/materialization/typst-passo-156d.md`.
- Diagnóstico (origem): `00_nucleo/diagnosticos/diagnostico-layout-passo-156b.md`.
- Relatório P156C (precedente): `00_nucleo/materialization/typst-passo-156c-relatorio.md`.
- ADR-0061 (aplicada): `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`.
- Inventário 148 actualizado:
  `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
- README ADRs: `00_nucleo/adr/README.md`.
- L0 prompts: `00_nucleo/prompts/entities/content.md` (secção
  HSpace+VSpace adicionada).
- Vanilla source consultado:
  `lab/typst-original/crates/typst-library/src/layout/spacing.rs`.
- Cristalino código tocado:
  - `01_core/src/entities/content.rs` (variants HSpace+VSpace
    + construtores + cobertura arms + 7 tests).
  - `01_core/src/rules/introspect.rs` (arms HSpace+VSpace em
    materialize_time + walk).
  - `01_core/src/rules/layout/mod.rs` (arms HSpace+VSpace em
    layout_content + measure_content_constrained).
  - `01_core/src/rules/layout/tests.rs` (2 tests E2E).
  - `01_core/src/rules/stdlib/layout.rs` (`extract_weak` +
    `build_spacing` helpers + `native_h` + `native_v`).
  - `01_core/src/rules/stdlib/mod.rs` (re-export + 11 tests).
  - `01_core/src/rules/eval/mod.rs` (registo em
    `make_stdlib`).
