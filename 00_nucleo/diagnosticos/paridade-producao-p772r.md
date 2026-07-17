# P772r — Hint de subtração em `unknown_variable`

> **Passo:** 772r
> **Data:** 2026-07-17
> **Commit-base:** `bde91d6866c3a42002324aff1e2b87e30416b85b` (HEAD).
> **Medido/implementado em:** 2026-07-17T10:xx–10:21Z.
> **Dependência:** P772l §2.4 (achado original).

---

## 1. Sonda — heurística exacta do vanilla

```
grep -n "if you meant to use subtraction\|fn unknown_variable\b" \
  lab/typst-original/crates/typst-library/src/foundations/scope.rs
```

`foundations/scope.rs:424-437`:

```rust
fn unknown_variable(var: &str) -> HintedString {
    let mut res = HintedString::new(eco_format!("unknown variable: {var}"));
    if var.contains('-') {
        res.hint(eco_format!(
            "if you meant to use subtraction, \
             try adding spaces around the minus sign{}: `{}`",
            if var.matches('-').count() > 1 { "s" } else { "" },
            var.replace('-', " - ")
        ));
    }
    res
}
```

### Confirmado por compilação real (não só leitura do código-fonte)

```
#foo-bar
  → error: unknown variable: foo-bar
    hint: if you meant to use subtraction, try adding spaces around the minus sign: `foo - bar`

#foo-bar-baz
  → error: unknown variable: foo-bar-baz
    hint: if you meant to use subtraction, try adding spaces around the minus signs: `foo - bar - baz`
    (plural "signs" — 2 hífens)

#simplyunknown
  → error: unknown variable: simplyunknown
    (sem hint — sem hífen)
```

Condição confirmada: **qualquer** hífen (`contains('-')`), sem verificar
se as partes à volta são identificadores conhecidos ou válidos — o caso
`foo-bar-baz` (três partes, nenhuma definida) ainda ganha o hint,
confirmando que não há heurística adicional além da simples presença do
carácter.

`unknown_variable()` é a **mesma função** usada pelo vanilla tanto em
`Scopes::get` (leitura) como no braço de fallback de `Scopes::get_mut`
(mutação) — por isso a correcção precisa de cobrir os dois pontos
equivalentes do cristalino.

---

## 2. Implementação

### L0 actualizado antes do código

`00_nucleo/prompts/rules/eval.md`: a nota que documentava a ausência de
hint como comportamento aceite (linha ~1191, ligada a P715) foi
substituída por uma referência à nova disciplina (`captured_by` →
`is_constant` → `unknown_variable` com hint, P772q/P772n/P772r). Nova
secção `§P772r` com a heurística, a correcção e os critérios de
verificação.

### Código

`unknown_variable(span, name) -> SourceDiagnostic` (novo, `pub(super)`,
`01_core/src/rules/eval/bindings.rs`, ao lado de `missing_key` que já
usava `SourceDiagnostic::with_hint` — mesmo padrão reaproveitado, sem
infra-estrutura nova). Chamado nos dois pontos onde o cristalino já
construía `"unknown variable: {name}"` directamente:

- `eval_expr`, `Expr::Ident` (`01_core/src/rules/eval/mod.rs`) — leitura.
  Passou a chamar `bindings::unknown_variable(ident.span(), name)`.
- `access()`, `Expr::Ident`, braço final (após `captured_by` de P772q e
  `is_constant` de P772n) (`01_core/src/rules/eval/bindings.rs`) —
  mutação. O braço, que antes construía uma `String` de mensagem para
  os três casos (`captured`/`constant`/`unknown`) e só depois envolvia
  num único `SourceDiagnostic::error(...)`, foi reestruturado para
  construir o `SourceDiagnostic` completo em cada ramo — necessário
  porque só o ramo `unknown_variable` pode ter hint.

### Testes novos

`p772r_hint_subtracao_um_hifen`, `p772r_hint_subtracao_hifens_multiplos_
plural`, `p772r_sem_hifen_sem_hint`, `p772r_hint_tambem_no_caminho_de_
mutacao` (`01_core/src/rules/eval/tests.rs`) — os quatro casos medidos
contra o vanilla, incluindo a confirmação de que o caminho de mutação
(`#{ foo-bar = 1 }`) usa o mesmo helper e produz o mesmo hint.

---

## 3. Validação

### 3.1 Casos do passo (leitura)

```
#foo-bar       → unknown variable: foo-bar       + hint singular   ✓ idêntico ao vanilla
#foo-bar-baz   → unknown variable: foo-bar-baz    + hint plural     ✓ idêntico ao vanilla
#simplyunknown → unknown variable: simplyunknown  sem hint          ✓ idêntico ao vanilla
```

### 3.2 Caminho de mutação (confirma que o helper partilhado cobre os dois pontos)

```
#{ foo-bar = 1 }
  cristalino: unknown variable: foo-bar + hint singular
  vanilla:    unknown variable: foo-bar + hint singular   ✓ idêntico
```

### 3.3 Suite completa

```
cargo test --workspace
  4192 (typst-core, +4 novos) + 645 + 33 + 2 + 29 + 2, 0 falhas
crystalline-lint .
  0 violações (mesmo warning V7 pré-existente, não relacionado)
```

---

## Critério de fecho do passo (`typst-passo-772r.md`)

- [x] Heurística exacta do vanilla confirmada (`contains('-')`, sem
      verificação adicional das partes).
- [x] Texto do hint confirmado palavra por palavra, incluindo singular
      vs plural.
- [x] L0 atualizado antes do código.
- [x] Hint implementado usando `SourceDiagnostic::with_hint` já
      existente (via helper novo `unknown_variable`, reaproveitando o
      padrão de `missing_key`).
- [x] Caso com hífen dá hint (singular e plural); caso sem hífen não dá.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772r.md`.

---

## Próximo passo

Item novo identificado por P772p: lacuna geral de span em `Args`/chamada
de funções nativas (afecta mensagens de erro de argumento de qualquer
função nativa, não só `image()`). Depois, reconfirmar `lacuna-inventario`
(estilo P772e, segunda rodada) para decidir se a varredura sistemática
continua. Com P772l/P772n/P772q/P772r fechados, `foundations::scope`
está classificado por completo excepto §2.6 (avisos de depreciação,
severidade baixa, ainda em aberto).
