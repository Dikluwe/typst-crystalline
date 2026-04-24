# Passo 128.A — Inventário `text.leading` (DEBT-1 subset)

**Data**: 2026-04-24

---

## Parte 1 — Vanilla `leading`

**Ficheiro**: `lab/typst-original/crates/typst-library/src/model/par.rs:210`.

```rust
pub leading: Length,
```

Propriedade de **`ParElem`** (não `TextElem`). Default típico
`0.65em` (confirmação não crítica — este passo não implementa
default).

Sintaxe canónica vanilla: `#set par(leading: 0.65em)`.

## Parte 2 — `eval_set_par` em L1

**Ficheiro**: `01_core/src/rules/eval/rules.rs:196-268`.

Dispatcher actual em `eval_set_rule`:

```rust
if target == "heading" { /* ... 15 linhas ... */ }
if target == "page"    { /* ... 30 linhas ... */ }
if target == "figure"  { /* ... 20 linhas ... */ }
if target != "text" {
    // unsupported_target_warn
    engine.sink.warn_note(target_span, &msg, &hint);
    return Ok(Value::None);
}
// ... fallthrough: captura para text ...
```

**`par` não existe como target válido**. `#set par(...)` cai no
fallback "unsupported target" e emite warning.

### Testes existentes que dependem deste estado

1. **L1** `rules/eval/tests.rs:1195 fn eval_set_target_desconhecido_ignora`:
   input `"#set par(leading: 1em)"` — assert `result.is_ok()`.
   Warning emitido via Sink mas não consultado — test permanece
   OK mesmo que warning mude.
2. **L3** `integration_tests.rs:2258 fn debt49_set_target_desconhecido_emite_warning`:
   input `"#set par(leading: 10pt)"` — **assert 1 warning com
   "'par'" + "target"**. Se `par` virar target válido, warning
   desaparece e teste quebra.

## Parte 3 — `eval_set_text` arms actuais

Pós-Passo 127:
- `bold` (bool), `italic` (bool), `size` (Length→f64),
  `fill` (Color), `weight` (i64→u16), `tracking` (Length).
- Fallback: warning "propriedade '{X}' ainda não suportada".

## Parte 4 — Teste DEBT-49 L3

Pós-Passo 126: input usa `font, lang, stroke` (3 warnings).
`leading` **não aparece**. Rotação desnecessária.

---

## Decisão 128.A

### Matriz

| Cenário | Custo | Rippling de testes |
|---------|-------|---------------------|
| **(a)** capturar em `#set text` | XS: 2 ficheiros, template 127 | 0 testes afectados |
| **(b)** criar `#set par` como valid target | S: adicionar arm + novo delta-push | Quebra 2 testes (L1 + L3); precisa adaptar ambos |
| **(c)** ambos | M: (b) + duplicar arm | Idem (b) |

**Escolha: (a)**.

### Razão

1. **XS preservado**: 2 ficheiros, template 127 literal.
2. **Zero rippling de testes**: `par` continua unknown target;
   ambos os testes acima continuam válidos.
3. **Divergência vanilla é temporária e inerte**: `leading`
   capturado em `StyleDelta` mas não consumido em layout —
   zero impacto visível. Quando alguém implementar `eval_set_par`
   (passo dedicado), `leading` migra; `StyleDelta.leading`
   fica no sítio — só muda quem preenche.
4. **UX razoável**: utilizador que escreve
   `#set text(leading: 0.65em)` não vê warning. Quem escrever
   `#set par(leading: 0.65em)` (canónico) vê warning de
   target desconhecido — sintoma correcto.

### ADR-0038

**Não anotar.** Terceira aplicação literal do pattern (126/127);
sem nuance nova. Divergência vanilla é nuance *temporal*, não
*arquitectural* — pertence ao relatório do passo + candidato
futuro, não ao ADR.

---

## Gate 128.A

**Passa**. 2 ficheiros L1:
- `entities/style_chain.rs` (+1 campo `Option<Length>`).
- `rules/eval/rules.rs` (+1 arm `"leading"`).

Zero ripple L3/L4. DEBT-49 test intacto (já usa stroke).

---

## Candidato futuro registado

**"Migrar `leading` para `eval_set_par`"**: quando `par` virar
target válido em L1 (passo dedicado com ADR própria que
adicione arm `par` e decide que outras propriedades `par`
captura), mover `"leading"` arm de `text` para `par`. Valor
capturado em `StyleDelta.leading` permanece identical — só o
ponto de captura muda.

Adaptação de testes no migration passo:
- `eval_set_target_desconhecido_ignora` (L1): mudar input.
- `debt49_set_target_desconhecido_emite_warning` (L3): mudar
  input.
