# Passo 126.A — Inventário `text.weight` (DEBT-1 subset)

**Data**: 2026-04-24
**Método**: grep em `01_core/src/` + leitura de
`style_chain.rs` e `rules/eval/rules.rs`.

---

## Parte 1 — `StyleDelta` actual

**Ficheiro**: `01_core/src/entities/style_chain.rs:25-35`.

```rust
pub struct StyleDelta {
    pub bold:   Option<bool>,
    pub italic: Option<bool>,
    pub size:   Option<f64>,
    pub fill:   Option<Color>,
    pub heading_level: Option<u8>,
}
```

**5 campos**. ADR-0038 (Passo 99) introduziu `fill` e `heading_level`.
ADR-0040 (Passo 102) activou `fill` no eval.

**Propriedades activas** (capturadas em `eval_set_text`):
`bold`, `italic`, `size`, `fill`.

**Propriedades em warning** (não cobertas pelo match do
eval): tudo o resto — `font`, `lang`, `weight`, `leading`,
`stroke`, `tracking`, etc.

`StyleDelta::empty()` e `default_chain()` usam `..empty()`
spread — adicionar campo novo não quebra chamadas.

## Parte 2 — `eval_set_text`

**Ficheiro**: `01_core/src/rules/eval/rules.rs:265-312`.

Estrutura exacta (resumida):

```rust
if target != "text" { /* warning + return */ }

let mut delta = StyleDelta::empty();

for arg in set.args().items() {
    if let Arg::Named(named) = arg {
        let key = named.name().as_str().to_owned();
        let val = eval_expr(...)?;
        match key.as_str() {
            "bold"   => if let Value::Bool(b)   = val { delta.bold   = Some(b); },
            "italic" => if let Value::Bool(b)   = val { delta.italic = Some(b); },
            "size"   => if let Value::Length(l) = val { delta.size   = Some(l.abs.to_pt()); },
            "fill"   => if let Value::Color(c)  = val { delta.fill   = Some(c); },
            _ => {
                let (msg, hint) = unsupported_property_warn("text", &key);
                engine.sink.warn_note(...span..., &msg, &hint);
            }
        }
    }
}
*engine.styles = engine.styles.push(delta);
```

**Helper de warning**: `unsupported_property_warn("text", &key)` ao
nível `rules.rs:31`. Mensagem:
```
text: propriedade '{field}' ainda não suportada
hint: ver ADR-0040 para propriedades cobertas por set text
```

Adicionar `"weight"` ao match basta — não há lista centralizada
separada.

## Parte 3 — Testes / canary DEBT-50

**Grep por `DEBT-50`**:
- `01_core/src/rules/layout/tests.rs:1780` — comentário do
  canary.
- `01_core/src/rules/layout/tests.rs:1787` — `fn debt_50_show_strong_nao_apanha_set_text_bold_porque_bake_in()`.

**Grep por `weight` em testes L1**:
- `font_book.rs`: `FontWeight(u16)` com `from_number`,
  `.weight.distance(...)`. Infraestrutura **já existe** em L1
  para catálogo de fontes, mas desligada de `StyleDelta`.
- Zero testes que esperam warning para `weight` em eval —
  migração desnecessária.

**Canary usa `font`** (não `weight`) — sem colisão.

## Parte 4 — Vanilla

**Ficheiro**: `lab/typst-original/crates/typst-library/src/text/mod.rs`
(consulta opcional). `FontWeight(u16)` com parsing de
`"regular"`/`"bold"`/número + clamp 100-900. Cristalino **já
tem `FontWeight(u16)`** em `entities/font_book.rs:45-47` com
`from_number(weight: u16) -> Self { Self(weight.clamp(100, 900)) }`.

Este passo **não** usa `FontWeight` directamente — `StyleDelta.weight:
Option<u16>` é raw. Forma simbólica + clamping profundo fica para
passo dedicado quando weight for consumido por layout.

---

## Decisões

| Dimensão | Escolha | Razão |
|----------|---------|-------|
| Tipo | `Option<u16>` | Minimalista; FontWeight fica para quando weight for consumido |
| Enum `Style::Weight` | **não adicionar** | bake-in usa `push(delta)`, não `push_styles(Style)`; zero necessidade |
| Resolver `StyleChain::weight()` | **não adicionar** | weight é inerte (não consumido por layout); resolver sem consumer é peso morto |
| Captura no `eval_set_text` | `u16::try_from(i64)` silent | Coerente com outros arms (tipo errado silencioso) |
| Range | sem validação | CSS aceita 0-1000; consumer valida quando existir |
| Teste L1 | 2 testes: captura ok + zero warning | Harness eval existente |

## Gate 126.A

**Passa**. 2 ficheiros tocados:
- `01_core/src/entities/style_chain.rs` (+1 campo + init em
  `empty()`).
- `01_core/src/rules/eval/rules.rs` (+1 match arm).

Não toca `StyleChain` resolvers, pipeline layout, export, ou
consumers indirectos. XS confirmado.
