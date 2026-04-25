# Passo 156D — h + v spacing (Layout Fase 1 sub-passo 2)

**Série**: 156D (passo **substantivo escopo S**;
materialização Fase 1 Layout, segunda sub-fase granular).
**Padrão**: P156A (historiograma) → P156B (diagnóstico
Layout) → P156C (pad+hide) → **P156D (h+v spacing)**.

**Precondição**: Passo 156C encerrado; ADR-0061 PROPOSTO
(Layout roadmap; mantém PROPOSTO até P156I per decisão
humana); 1172 tests; 61 ADRs; 14 DEBTs abertos; cobertura
Layout 33% (6/18 implementado puro pós-P156C); cobertura
user-facing total 55%.

**Numeração**: P156D segue P156C na convenção de letras
consecutivas. **Não conflita** com P157 (Model Fase 2 table
foundations).

**Natureza**: passo **substantivo escopo S** (2 features
triviais; ~10-15 testes adicionados estimados; sem
crates novas; sem ADRs novas; sem DEBTs novos esperados).

**Decisão arquitectural P156D**: **h e v como variants novos**
em `Content` enum, análogos a pad/hide em P156C:

- `Content::HSpace { amount, weak }` — espaço horizontal.
- `Content::VSpace { amount, weak }` — espaço vertical.

Razão: vanilla expõe ambos como `#[elem]` proper
(`HElem`, `VElem` em
`lab/typst-original/crates/typst-library/src/layout/`).
Cristalino segue vanilla — ambos são Content, não Style.
Semantic clara: spacing primitives são structurais.

**ADRs aplicáveis**:
- **ADR-0026 + ADR-0026-R1**: Content enum aceita variants
  novos via decisão deste passo.
- **ADR-0033**: paridade funcional para h e v.
- **ADR-0036**: atomização — cada feature consumer explícito.
- **ADR-0037**: coesão por domínio — Layout permanece em
  `rules/layout/` e `rules/stdlib/layout.rs`.
- **ADR-0054**: perfil observacional graded — h e v
  cumpridos com aproximação (`weak` semantic simplificada).
- **ADR-0061** (PROPOSTO): plano de Layout Fase X. Este
  passo aplica-o pela segunda vez. ADR-0061 mantém-se
  PROPOSTO até P156I.

---

## Contexto

P156B identificou 18 entradas Layout. P156C materializou pad
e hide (4/18 → 6/18 = 33%). P156D continua sequência granular
materializando 2 features triviais (h + v spacing primitives).

**`h(amount, weak)` em vanilla**:
- Atributo posicional `amount` (Length ou Fraction).
- Atributo nomeado `weak: bool` (default false).
- Semantic: insere espaço horizontal entre elementos
  inline.
- `weak: true` colapsa adjacente a outros weak spaces.

**`v(amount, weak)` em vanilla**:
- Mesmos atributos que h.
- Semantic: insere espaço vertical entre blocos.
- `weak: true` colapsa adjacente.

**Hipóteses a confirmar empiricamente** (não compromisso):

- `amount` é `Length` em cristalino (sem `Fraction` por
  agora — fractions são Layout Fase 3 ou refino futuro per
  ADR-0061 §6.3).
- `weak` é `bool` simples; collapse semantic real depende
  de contexto. **Decisão pragmática**: aceitar `weak`
  como atributo armazenado mas comportamento de collapse
  fica para passo refino futuro (DEBT candidato se
  necessário).
- `Content::HSpace { amount: Length, weak: bool }` é forma
  final.
- `Content::VSpace { amount: Length, weak: bool }` é
  forma final.
- Layouter HSpace: avança `cursor.x` por `amount`.
- Layouter VSpace: avança `cursor.y` por `amount`; pode
  forçar `flush_line` antes.
- `measure_content_constrained` HSpace: `(amount, 0)`.
- `measure_content_constrained` VSpace: `(0, amount)`.

---

## Objectivo

Ao fim do passo:

1. **Variant `Content::HSpace { amount, weak }`** adicionado
   em `01_core/src/entities/content.rs`.

2. **Variant `Content::VSpace { amount, weak }`** adicionado
   em `01_core/src/entities/content.rs`.

3. **Cobertura exaustiva de arms** em todos os ficheiros
   que pattern-match sobre `Content`:
   - `entities/content.rs::is_empty()`.
   - `entities/content.rs::plain_text()`.
   - `entities/content.rs::PartialEq::eq`.
   - `entities/content.rs::map_content`.
   - `entities/content.rs::map_text`.
   - `rules/introspect.rs::materialize_time`.
   - `rules/introspect.rs::walk`.
   - `rules/layout/mod.rs::layout_content`.
   - `rules/layout/mod.rs::measure_content_constrained`.

4. **`native_h`** em
   `01_core/src/rules/stdlib/layout.rs` expondo
   `#h(amount, weak: ?)`.

5. **`native_v`** em mesma localização expondo
   `#v(amount, weak: ?)`.

6. **Testes** unit + eval (~10-15 testes adicionados
   estimados):
   - Construtor `Content::HSpace` + `Content::VSpace`.
   - `is_empty()`, `plain_text()`, `partial_eq`,
     `map_content`/`map_text` cobertura.
   - `eval_h` com `amount` Length.
   - `eval_h` com `weak: true`.
   - `eval_h` rejeita `amount` ausente.
   - `eval_h` rejeita named arg desconhecido.
   - `eval_v` análogo.
   - Layouter h: cursor.x avança.
   - Layouter v: cursor.y avança; flush_line se necessário.

7. **L0 prompts** + hashes propagados:
   - `00_nucleo/prompts/entities/content.md` ganha
     secção "Variants `Content::HSpace` + `Content::VSpace`
     — Passo 156D".
   - Hash `entities/content.rs` recomputado (era
     `daf00164` pós-P156C; novo após P156D).
   - Headers `@updated`: data execução.

8. **Inventário 148 actualizado**:
   - Tabela A.5 Layout: linhas `h`/`v` ausente →
     `implementado`.
   - Cobertura Layout: 6/18 → **8/18 = 44%**.
   - Tabela A linha "Layout": `6/0/3/9/0=18` →
     `8/0/3/7/0=18`.
   - Total user-facing: 55% → **~57%**.
   - Tabela B Content variants: 45 → **47**.
   - §7 entrada 7: actualizar progresso Layout Fase 1
     (P156D cumprido).

9. **README dos ADRs actualizado**:
   - Tabela "Estado por ADR": linha ADR-0061 mantém-se
     PROPOSTO.
   - Distribuição inalterada.
   - Total inalterado (61 ADRs).
   - Entrada nova em "Passos-chave da história dos ADRs"
     para P156D.

10. **ADR-0061 NÃO actualizada** neste passo (per decisão
    humana). Mantém-se PROPOSTO. Anotação cumulativa após
    P156I.

11. **Sem DEBTs criados/fechados** (esperado).

12. **Relatório do passo** em
    `00_nucleo/materialization/typst-passo-156d-relatorio.md`.

Este passo **não**:

- Toca outros variants Content excepto adições.
- Toca código fora de `01_core/`.
- Materializa outras features Layout (pagebreak, skew,
  block, box, etc.).
- Implementa `Fraction` para amount (scope-out per
  ADR-0061; refino futuro).
- Implementa collapse semantic real para `weak` (atributo
  armazenado, mas collapse defere).
- Adiciona show rules `#show h: ...` ou `#show v: ...`.
- Toca série paridade (suspensa em P153).

---

## Decisões já tomadas

1. **Variants novos** (não Styled). Coerente com vanilla
   `HElem`/`VElem` e com decisão de P156C.

2. **Naming**: `HSpace` e `VSpace` em Rust (legibilidade);
   exposto como `h` e `v` em stdlib (paridade vanilla).

3. **Granularidade**: 2 features num passo. Consistente
   com decisão humana 2026-04-25 (12 passos granulares).

4. **Localização canónica**: `01_core/src/rules/stdlib/layout.rs`
   per descoberta P156C (coesão por domínio Layout).

5. **Assinatura natives**: forma 5-param canónica
   (`fn(_ctx: &mut EvalContext, args: &Args, _world: &dyn World,
   _current_file: FileId, _figure_numbering: Option<&str>)
   -> SourceResult<Value>`) per descoberta P156C.

6. **`amount` como Length apenas** (sem Fraction). Refino
   futuro se priorizado.

7. **`weak` armazenado mas collapse defere**. Decisão
   pragmática per ADR-0054 graded.

8. **Tests adicionados**: alvo 10-15 (ajustável).

9. **ADR-0061 NÃO anotada** neste passo.

10. **Show rules adiadas**: candidato a passo agregado
    futuro.

## Decisões diferidas (resolvidas neste passo)

11. **`amount` negativo**: rejeitar análogo a P156C
    padding negativo. Validação em `native_h`/`native_v`.
    Se vanilla aceita negativo, **registar como divergência
    consciente** análoga a P155 e P156C.

12. **`amount` zero**: aceitável; produz `Content::HSpace
    { amount: zero, weak: false }`. Layouter trata como
    no-op (cursor não avança). Test edge case.

13. **`v` no início de página/coluna**: vanilla pode
    colapsar contra a margem. Cristalino: aceitar simples
    avanço de cursor.y. Refino futuro se necessário.

14. **`h` no fim de linha**: vanilla pode colapsar contra
    margem direita. Cristalino: aceitar simples avanço
    de cursor.x; se exceder largura, comportamento
    indefinido (não força wrap por agora). Refino futuro.

15. **Conflito de naming**: `h` e `v` são identificadores
    curtos comuns. Verificar durante execução que não
    conflita com variáveis user-defined.

---

## Escopo

**Dentro**:

- Modificação de `01_core/src/entities/content.rs`
  (2 variants novos + arms cobertura).
- Modificação de `01_core/src/rules/introspect.rs`
  (`materialize_time` + `walk`).
- Modificação de `01_core/src/rules/layout/mod.rs`
  (`layout_content` + `measure_content_constrained`).
- Modificação de `01_core/src/rules/stdlib/layout.rs`
  (`native_h` + `native_v`).
- Modificação de `01_core/src/rules/stdlib/mod.rs`
  (re-export).
- Modificação de `01_core/src/rules/eval/mod.rs`
  (registo em `make_stdlib`).
- Tests em `01_core/src/entities/content.rs::tests`,
  `01_core/src/rules/stdlib/mod.rs::tests` (modelo P156C),
  `01_core/src/rules/layout/tests.rs`.
- L0 prompts + hashes.
- Inventário 148 + README ADRs.
- Relatório do passo.

**Fora**:

- Modificação de outros ficheiros L1/L2/L3/L4 não-listados.
- Implementação de outras features Layout.
- Show rules `#show h: ...` ou `#show v: ...`.
- Implementação real de `Fraction` para amount.
- Implementação real de collapse semantic para `weak`.
- Footnote area.
- Crates externas.
- ADRs novas.
- DEBTs novos.
- Modificação de ADR-0061 (anotação cumulativa após
  P156I).
- Modificação de ADR-0060.
- Trabalho em `lab/parity/`.

---

## Sub-passos

### 156D.1 — Verificar pré-condições

```bash
view 01_core/src/entities/content.rs   # confirmar 45 variants pós-P156C
grep -nE "^pub enum Content" 01_core/src/entities/content.rs
view 01_core/src/rules/stdlib/layout.rs  # localização canónica
grep -nE "fn native_pad|fn native_hide" \
  01_core/src/rules/stdlib/layout.rs
```

Confirmar:
- Content tem 45 variants (Pad e Hide em P156C foram
  44º e 45º).
- `stdlib/layout.rs` existe com `native_pad` + `native_hide`.
- Layouter actual usa pattern-match exaustivo sobre Content.

### 156D.2 — Adicionar variants Content::HSpace + Content::VSpace

Edição de `01_core/src/entities/content.rs`:

```rust
pub enum Content {
    // ... 45 variants existentes (Text, Strong, Emph, ...,
    //     Quote, Pad, Hide)
    HSpace {
        amount: Length,
        weak:   bool,
    },
    VSpace {
        amount: Length,
        weak:   bool,
    },
}
```

**Variant count**: 45 → **47** (+2).

### 156D.3 — Cobertura exaustiva de arms

Para cada função em `01_core/` que pattern-match sobre
Content, adicionar arms para `HSpace` e `VSpace`:

| Função | Tratamento HSpace | Tratamento VSpace |
|--------|-------------------|-------------------|
| `is_empty()` | `amount.is_zero()` | `amount.is_zero()` |
| `plain_text()` | `String::new()` (não rende texto) | idem |
| `PartialEq::eq` | comparação 2 fields | comparação 2 fields |
| `map_content` | preserva (Copy fields) | idem |
| `map_text` | preserva | idem |
| `materialize_time` | preserva (sem body) | idem |
| `walk` | no-op (sem body) | idem |
| `layout_content` | avança cursor.x | flush_line; avança cursor.y |
| `measure_content_constrained` | `(amount, 0)` | `(0, amount)` |

### 156D.4 — `native_h` e `native_v`

Em `01_core/src/rules/stdlib/layout.rs` (ao lado de
`native_pad` + `native_hide`):

```rust
pub fn native_h(_ctx: &mut EvalContext, args: &Args,
                _world: &dyn World, _current_file: FileId,
                _figure_numbering: Option<&str>)
    -> SourceResult<Value>
{
    // amount posicional obrigatório
    let amount = match args.items.first() {
        Some(v) => extract_length(v)?,  // helper já existe pós-P156C
        None => return Err(missing_argument("amount")),
    };

    // validação: amount negativo rejeitado per Decisão diferida 11
    if amount.is_negative() {
        return Err(invalid_argument(
            "amount cannot be negative (P156D scope-out)"));
    }

    // named arg: weak: bool
    let mut weak = false;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "weak" => weak = expect_bool(value)?,
            other => return Err(unexpected_named(other)),
        }
    }

    Ok(Value::Content(Content::HSpace { amount, weak }))
}

pub fn native_v(_ctx: &mut EvalContext, args: &Args,
                _world: &dyn World, _current_file: FileId,
                _figure_numbering: Option<&str>)
    -> SourceResult<Value>
{
    // análogo a native_h mas produz VSpace
    let amount = match args.items.first() {
        Some(v) => extract_length(v)?,
        None => return Err(missing_argument("amount")),
    };

    if amount.is_negative() {
        return Err(invalid_argument(
            "amount cannot be negative (P156D scope-out)"));
    }

    let mut weak = false;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "weak" => weak = expect_bool(value)?,
            other => return Err(unexpected_named(other)),
        }
    }

    Ok(Value::Content(Content::VSpace { amount, weak }))
}
```

Registo em `make_stdlib` (em `01_core/src/rules/eval/mod.rs`):

```rust
scope.define("h", Value::Func(Func::native("h", native_h)));
scope.define("v", Value::Func(Func::native("v", native_v)));
```

Re-export em `01_core/src/rules/stdlib/mod.rs`:

```rust
pub use crate::rules::stdlib::layout::{
    native_align, native_grid, native_h, native_hide,
    native_pad, native_page, native_place, native_v,
};
```

Stdlib funcs: 34 → **36** (+2).

### 156D.5 — Layouter h e v

Em `01_core/src/rules/layout/mod.rs::layout_content`:

```rust
match content {
    // ... arms existentes (incluindo Pad e Hide de P156C)
    Content::HSpace { amount, weak: _ } => {
        // weak collapse defere — semantic simples
        let pt = amount.resolve_pt(self.font_size_pt.val());
        self.cursor_x += Pt(pt);
    }
    Content::VSpace { amount, weak: _ } => {
        // weak collapse defere
        let pt = amount.resolve_pt(self.font_size_pt.val());
        if self.cursor_x.0 > self.line_start_x.0 {
            self.flush_line();  // termina linha em curso
        }
        self.cursor_y += Pt(pt);
    }
    // ... fallback
}
```

Em `measure_content_constrained`:

```rust
match content {
    Content::HSpace { amount, weak: _ } => {
        let pt = amount.resolve_pt(self.font_size_pt.val());
        (pt, 0.0)
    }
    Content::VSpace { amount, weak: _ } => {
        let pt = amount.resolve_pt(self.font_size_pt.val());
        (0.0, pt)
    }
    // ... fallback
}
```

### 156D.6 — Tests adicionados (alvo 10-15)

| Ficheiro | Testes |
|----------|--------|
| `01_core/src/entities/content.rs::tests` | (1) hspace_constructor; (2) vspace_constructor; (3) hspace_is_empty_se_amount_zero; (4) vspace_is_empty_se_amount_zero; (5) hspace_partial_eq; (6) vspace_partial_eq; (7) hspace_e_vspace_plain_text_vazio |
| `01_core/src/rules/stdlib/mod.rs::tests` | (8) `native_h` aceita Length; (9) `native_h` aceita weak; (10) `native_h` rejeita amount negativo; (11) `native_h` rejeita named arg desconhecido; (12) `native_h` sem amount → Err; (13) `native_v` análogo (1 test composto que cobre todas as variantes acima); (14) `native_h` aceita amount zero |
| `01_core/src/rules/layout/tests.rs` | (15) layout h avança cursor.x; (16) layout v avança cursor.y após flush |

**Total**: ~15-16 tests. Tests cumulativos: **1172 → ~1187-
1188**.

### 156D.7 — L0 prompts + hashes

Editar `00_nucleo/prompts/entities/content.md`:

Adicionar secção "Variants `Content::HSpace` +
`Content::VSpace` — Passo 156D" após secção P156C
pad+hide:

```markdown
## Variants HSpace e VSpace (Passo 156D)

`Content::HSpace { amount: Length, weak: bool }`:
- spacing primitive horizontal;
- `amount` é Length (sem Fraction neste passo);
- `weak` armazenado mas collapse defere (P156D scope-out).

`Content::VSpace { amount: Length, weak: bool }`:
- spacing primitive vertical;
- semantic análoga a HSpace mas em eixo Y;
- layouter força flush_line antes de avançar cursor.y.

Ambos seguem ADR-0026 perfil "variant novo" porque
vanilla expõe ambos como `#[elem]` proper.
```

Recomputar hashes:

```bash
cd 01_core
cargo run --bin crystalline-lint -- --fix-hashes
```

Verificar:
- `entities/content.rs`: hash novo (era `daf00164` pós-
  P156C; será diferente pós-P156D).
- `entities/content.md`: hash propagado.

Headers `@updated`: data execução.

### 156D.8 — Inventário 148 actualizado

Em
`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

**Tabela A.5 Layout**:
- linha `h`: ausente → **implementado** (referência:
  Passo 156D).
- linha `v`: ausente → **implementado** (referência:
  Passo 156D).

**Tabela A linha "Layout"**:
- antes: `6 | 0 | 3 | 9 | 0 | 18`
- depois: `8 | 0 | 3 | 7 | 0 | 18`
- cobertura Layout: 33% → **44%**.

**Total user-facing**:
- antes: `56 | 21 | 22 | 40 | 2 | 141`.
- depois: `58 | 21 | 22 | 38 | 2 | 141`.
- cobertura user-facing: (58+21)/141 = **56%** (era 55%).

**Tabela B Content**:
- variants cristalino: 45 → **47**.
- variants vanilla extra ausentes: ~9 → ~7 (h+v saem do
  conjunto não-capturado).

**§7 entrada 7**: actualizar progresso Layout Fase 1
(P156D cumprido; restantes 7 entradas Layout: pagebreak,
skew, block, box, stack, repeat, columns/colbreak).

### 156D.9 — README ADRs actualizado

- Tabela "Estado por ADR": linha ADR-0061 mantém-se
  PROPOSTO. Sem mudança.
- Total: 61 inalterado.
- Distribuição: PROPOSTO 11 inalterado.
- "Passos-chave" entrada nova:
  ```
  P156D: aplicação segunda de ADR-0061 — h + v spacing
  variants. Cobertura Layout 33% → 44%.
  ```

### 156D.10 — Relatório do passo

Ficheiro:
`00_nucleo/materialization/typst-passo-156d-relatorio.md`.

Secções (modelo P156C):
1. Sumário executivo.
2. Inventário pré-materialização.
3. Variant `Content::HSpace` — forma final + diff.
4. Variant `Content::VSpace` — forma final + diff.
5. Cobertura exaustiva de arms.
6. `native_h` + `native_v` — assinaturas + registo.
7. Layouter — diff (`layout_content` +
   `measure_content_constrained`).
8. Tests adicionados (lista + contagens).
9. L0 prompts + hashes propagados.
10. Inventário 148 actualizado.
11. README ADRs actualizado.
12. Próximo passo (P156E = pagebreak manual).
13. Limitações registadas.
14. Verificação final.

---

## Verificação

1. ✅ `cargo build --workspace`: clean.
2. ✅ `cargo test --workspace --lib`: ~1187-1188 passed
   (1172 → +15-16); zero falhas.
3. ✅ `crystalline-lint .`: zero violations.
4. ✅ Hashes propagados consistentes: `entities/content.rs`
   ↔ `entities/content.md`.
5. ✅ `Content::HSpace` e `Content::VSpace` variants em
   produção (45 → 47).
6. ✅ Stdlib `#h(...)` e `#v(...)` invocáveis (34 → 36
   funcs).
7. ✅ Cobertura arms exaustiva.
8. ✅ Inventário 148 reflecte cobertura aumentada
   (33% → 44%).
9. ✅ README ADRs entrada P156D.
10. ✅ Sem ADR criada / revogada / revisada.
11. ✅ Sem DEBT criado / fechado.
12. ✅ ADR-0061 inalterada (mantém-se PROPOSTO).
13. ✅ ADR-0060 inalterada.
14. ✅ Sem regressão.
15. ✅ Relatório do passo escrito.

---

## Critério de conclusão

| # | Critério | Estado |
|---|----------|--------|
| 1 | `Content::HSpace` compila + tests passam | ✅ |
| 2 | `Content::VSpace` compila + tests passam | ✅ |
| 3 | Stdlib `#h(amount, weak: ?)` invocável | ✅ |
| 4 | Stdlib `#v(amount, weak: ?)` invocável | ✅ |
| 5 | Layouter h avança cursor.x | ✅ |
| 6 | Layouter v força flush_line + avança cursor.y | ✅ |
| 7 | Inventário 148 reflecte cobertura 44% Layout | ✅ |
| 8 | Próximo passo (156E = pagebreak) tem âncora | ✅ |
| 9 | Sem regressão | ✅ |
| 10 | Relatório do passo escrito | ✅ |

---

## O que pode sair errado

- **`extract_length` helper já existe pós-P156C**: confirmar
  em `01_core/src/rules/stdlib/layout.rs`. Se não existir
  com nome esperado (e.g. `expect_length` em vez de
  `extract_length`), usar nome real.

- **`expect_bool` helper não existe**: provável. Criar
  em `01_core/src/rules/stdlib/layout.rs` ou usar
  pattern match directo:
  ```rust
  let weak_value = match value {
      Value::Bool(b) => *b,
      _ => return Err(invalid_argument("weak must be bool")),
  };
  ```

- **`amount` negativo conflita com vanilla**: vanilla pode
  aceitar h/v com amount negativo. Decisão diferida 11
  rejeita por agora. Se for caso de teste comum, registar
  divergência consciente análoga a P155 e P156C.

- **`weak` collapse semantic real exigida pelo user**:
  per spec, scope-out. Documentar como limitação consciente.
  Se collapse for crítico, abrir DEBT para passo refino.

- **`h` no fim de linha excede largura página**: vanilla
  pode wrap. Cristalino: cursor.x apenas avança. Aceitável;
  registar limitação.

- **`v` no início de página colapsa contra margem em
  vanilla**: cristalino aceita avanço simples. Registar
  limitação.

- **`amount` Fraction não suportado**: vanilla aceita
  `Fraction` (1fr, 2fr) para amount. Cristalino só
  Length neste passo. Per ADR-0061; refino futuro.

- **Tests precisam wrapping em `to_value()` ou `as_content()`**:
  pattern de tests em `stdlib/mod.rs::tests` per modelo
  P156C. Verificar consistência.

- **Naming `h`/`v` conflita com user-defined**: improvável;
  scope user-defined sobrepõe stdlib. Se acontecer,
  documentar.

- **Volume tests excede 16**: aceitável; ajustar
  no relatório.

- **Volume tests inferior a 10**: investigar; weak +
  amount + edge cases (zero, negativo) devem gerar
  ~10-15.

- **Pattern-match em `walk`/`materialize_time` falha**:
  HSpace/VSpace são "leaf" sem body. Adicionar arms
  no-op. Linter V2 alerta se omitido.

---

## Notas operacionais

- **Padrão "passos granulares 1-2 features"**: segunda
  aplicação consecutiva (P156C foi primeira). Hipótese
  da decisão humana 2026-04-25 testada novamente.

- **Localização canónica `stdlib/layout.rs`**: confirmada
  em P156C; este passo aplica directamente.

- **Assinatura natives 5-param**: confirmada em P156C;
  este passo aplica directamente.

- **ADR-0061 mantém PROPOSTO**: per decisão humana.
  Anotação cumulativa após P156I.

- **Variants count**: 45 → **47** (+2). Após P156E
  (pagebreak): 48. Após Fase 1 completa: ~49. Após Fase 2
  (block+box+stack): ~52.

- **Stdlib funcs**: 34 → **36** (+2). Após P156E: 37.
  Após P156I: ~40.

- **Pós-156D**:
  - 4 features Layout implementadas total (pad, hide,
    h, v).
  - Cobertura Layout: 33% → 44%.
  - Cobertura user-facing total: 55% → 56%.
  - **Próximo**: P156E (pagebreak manual — 1 trivial)
    ou alternativa humana.

- **Padrão emergente: 2 features triviais por passo**:
  P156C (pad+hide) + P156D (h+v) confirmam que features
  triviais agrupam-se naturalmente em pares. Tests
  cumulativos pequenos (~14-27 por passo).

- **Quarentena vanilla**: continua opção 3. Sem mudança.

- **Série paridade**: continua suspensa em P153. Sem
  mudança.

- **Granularidade vs paridade rápida**: P156C confirmou
  hipótese (zero reformulações). P156D testa de novo.
  Se algum passo da série P156C-P156I reformular,
  reavaliar granularidade no relatório.
