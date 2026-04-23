# Passo 107 — Inventário Sink propagação + DEBT-49 (5ª aplicação ADR-0036)

**Data**: 2026-04-23
**Input**: `eval_*` actualmente com 9 params + `ctx`. Sítios DEBT-49
silenciados em `rules/eval/rules.rs` (ver pt. 1). Canal Sink → L3
activo desde 106.

---

## Parte 1 — Sítios a migrar (âmbito estrito DEBT-49)

Grep de `DEBT-49|silenciad|TODO.*warn|// warning` em
`01_core/src/rules/eval/` e `01_core/src/rules/stdlib/`:

| Ficheiro:linha | Contexto | Envolvente | Silêncio DEBT-49? |
|----------------|----------|------------|------------------|
| `rules/eval/rules.rs:263-265` | `if target != "text" { return Ok(Value::None); }` — targets desconhecidos (`par`, `align`, etc.) | `eval_set_rule` | **SIM** — migrar |
| `rules/eval/rules.rs:293-297` | `_ => { /* DEBT: propriedades #set text não suportadas */ }` — propriedades desconhecidas (`font`, `lang`, `weight string`, …) | `eval_set_rule` | **SIM** — migrar |
| `rules/eval/rules.rs:191` | Comentário histórico `"Outros targets ignorados silenciosamente"` | `eval_set_rule` | Comentário descreve o site acima — atualizar texto |
| `rules/eval/rules.rs:196` | "Outros argumentos de heading ignorados (DEBT-10)" | `eval_set_rule` | **Não** — DEBT-10, fora do escopo |
| `bindings.rs:114` | "Defensivo: argumento não-Int, usar 0 silenciosamente" | `eval_counter_method` | **Não** — defensivo intencional |
| `operators.rs:19,108` | NaN/Inf; erro em conflito | operators | **Não** — não é silêncio DEBT-49 |
| `closures.rs:36` | "Spread ignorado — fronteira deliberada" | eval_args | **Não** — wildcard deliberado |
| `mod.rs:403` | "label handling" — ignorar expr::Label | eval_expr | **Não** — não é DEBT-49 |

Outros sítios em `tests.rs` são comentários de testes.

**Escopo final**: 2 sítios em `eval_set_rule`, ambos na mesma função.
DEBT-49 é geograficamente concentrado.

### Sítios migrados — detalhe

**Site A — `rules.rs:263-265`**:

```rust
if target != "text" {
    return Ok(Value::None);
}
```

→ emitir warning "set: target '<name>' ainda não suportado" antes do
return.

Span: `set.target().to_untyped().span()`.

**Site B — `rules.rs:293-297`**:

```rust
_ => {
    // DEBT: propriedades de #set text não suportadas (font, lang,
    // weight como string, etc.) são silenciosamente ignoradas.
}
```

→ emitir warning "set text: propriedade '<key>' ainda não suportada"
via helper `unsupported_property_warn`.

Span: `named.name().to_untyped().span()`.

---

## Parte 2 — Cadeia transitiva

### Ponto de partida

`eval_set_rule` em `rules.rs:179`.

### Subida na cadeia

Partindo de `eval_set_rule`:

- Caller directo: `eval_expr` (`mod.rs:370`) — **propaga**.
- Caller de `eval_expr`: todos os `eval_*` que dispatcheiam expressões.

### Funções que ganham parâmetro novo

**Leitoras (K = emitem `sink.warn_note(...)` directamente)**:

| Função | Ficheiro:linha |
|--------|----------------|
| `eval_set_rule` | `rules.rs:179` |

**K = 1**

**Propagadoras (P = só passam `sink` adiante)**:

| Função | Ficheiro:linha | Chama |
|--------|----------------|-------|
| `eval_markup` | `mod.rs:230` | `eval_expr`, `intercept_content` |
| `eval_expr` | `mod.rs:296` | tudo (dispatcher) |
| `eval_markup_body` | `mod.rs:449` | `eval_markup` |
| `apply_show_rules` | `rules.rs:37` | `closures::apply_func` |
| `intercept_content` | `rules.rs:155` | `apply_show_rules` |
| `eval_show_rule` | `rules.rs:307` | `eval_expr` |
| `eval_let` | `bindings.rs:28` | `eval_expr` |
| `eval_counter_method` | `bindings.rs:93` | `eval_expr` |
| `eval_field_access` | `bindings.rs:142` | `eval_expr` |
| `eval_conditional` | `control_flow.rs:26` | `eval_expr` |
| `eval_while` | `control_flow.rs:51` | `eval_expr` |
| `eval_for` | `control_flow.rs:81` | `eval_expr` |
| `eval_args` | `closures.rs:37` | `eval_expr` |
| `apply_func` | `closures.rs:66` | `apply_closure` / native ABI |
| `apply_closure` | `closures.rs:95` | `eval_markup`, `eval_expr` |
| `eval_closure_expr` | `closures.rs:156` | (closure-só; revisar) |
| `eval_func_call` | `closures.rs:200` | `apply_func` |
| `eval_strong` | `markup.rs:30` | `eval_markup_body` |
| `eval_emph` | `markup.rs:49` | `eval_markup_body` |
| `eval_heading` | `markup.rs:67` | `eval_markup_body` |
| `eval_list_item` | `markup.rs:106` | `eval_markup_body` |
| `eval_enum_item` | `markup.rs:121` | `eval_markup_body` |
| `eval_module_include` | `modules.rs:38` | `eval_markup` |

**P = 23**

**Totais**: K + P = **24**. Profundidade máxima D ≈ 4
(`eval` → `eval_markup` → `eval_expr` → `eval_set_rule`).

**Gate K+P ≤ 40 e D ≤ 6**: **OK, passa** (24 ≤ 40, 4 ≤ 6).

Comparação com Passo 98 (4ª aplicação ADR-0036, `current_file +
figure_numbering`): ~25 LTs e ~4 níveis. Mesma ordem de grandeza.

**Funções fora de escopo** (não tocam `eval_expr` / `eval_markup`):

- `eval_math_content` + `eval_math_expr` (`math.rs`) — subsistema
  fechado; não dispatch a `Expr::SetRule`.
- `eval_raw`, `eval_link` (`markup.rs`) — leaves sem propagação.
- `eval_module_import` (`modules.rs`) — leaf sem ctx avaliável.
- `eval_binary_op`, `eval_unary_op` (`operators.rs`) — puros.
- Funções em `tests.rs` (`eval_for_test`, `eval_for_test_with_limits`)
  — helpers de teste; passam um sink novo criado localmente.

---

## Parte 3 — Obtenção de `&mut Sink` a partir de `TrackedMut`

### Consulta à API comemo

Ficheiro: `~/.cargo/registry/src/.../comemo-0.4.0/src/track.rs`.

Observações:

1. `TrackedMut<'a, T>` é um wrapper com `value`/`constraint`.
2. Implementa `Deref` e `DerefMut` **não para `T`**, mas para
   `T::SurfaceMut<'a>` — tipo gerado pelo macro `#[comemo::track]`
   que só expõe as funções tracked de `T`.
3. `TrackedMut::reborrow_mut(&mut this)` retorna outro
   `TrackedMut<'_, T>` com lifetime reduzido. **Não** retorna `&mut T`.
4. Não existe `into_inner_mut()` nem equivalente.

### Conclusão — **Gate 107.A.3 dispara**

**`&mut Sink` não é obtível de `TrackedMut<Sink>` sem perder o tracking
comemo.** O único acesso mutável é via métodos declarados dentro de
`#[comemo::track] impl Sink`.

### Decisão revista (revoga decisão 1 do 107)

Em vez de derivar `&mut Sink`, a propagação usa
**`sink: &mut TrackedMut<'_, Sink>`**:

- A assinatura pública de `eval()` continua a receber
  `mut sink: TrackedMut<Sink>` (inalterada).
- Os `eval_*` internos recebem `sink: &mut TrackedMut<'_, Sink>`.
- Para emitir: `sink.warn_note(span, msg, hint)` — DerefMut pelo
  surface tracked.
- Para passar a um filho: passar `sink` (já `&mut`) directamente.

### Consequência — extensão de `warn_note`

A `warn_note` actual só aceita `(span, &str)`. Para preservar hints,
é estendida para `(span, &str, Option<&str>)`:

```rust
#[comemo::track]
impl Sink {
    pub fn warn_note(
        &mut self,
        span: Span,
        message: &str,
        hint: Option<&str>,
    ) { ... }
}
```

Todos os argumentos são `Hash+Eq` (requisito comemo para tracked
methods): `Span` é `Copy+Hash`, `&str` é `Hash`, `Option<&str>` é
`Hash`.

### API dupla continua válida

- `Sink::warn(diag: SourceDiagnostic)` — não-tracked, para consumers
  futuros com acesso a `&mut Sink` directo.
- `Sink::warn_note(span, msg, hint)` — tracked, para consumers com
  `TrackedMut<Sink>`.

---

## Conclusão do inventário

- **K + P = 24** — dentro do gate (≤ 40).
- **D = 4 níveis** — dentro do gate (≤ 6).
- **Gate 107.A.3 disparado**: propagar `&mut TrackedMut<'_, Sink>` em
  vez de `&mut Sink`; usar `warn_note` tracked (estendida com hint);
  gate 107.A.2 passa.
- **Escopo DEBT-49**: 2 sítios em `eval_set_rule`; migração
  concentrada num único ficheiro (`rules.rs`).
- **Pronto para 107.B** com a revisão documentada acima.
