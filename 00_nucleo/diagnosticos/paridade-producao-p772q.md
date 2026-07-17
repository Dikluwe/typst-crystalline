# P772q — Mensagem correta para mutação de variável capturada (`Capturer::Function`/`Context`)

> **Passo:** 772q
> **Data:** 2026-07-17
> **Commit-base:** `bde91d6866c3a42002324aff1e2b87e30416b85b` (HEAD).
> **Medido/implementado em:** 2026-07-17T00:59–01:17Z.
> **Dependência:** P772l §2.2 (achado original), P772n (mecanismo irmão para `cannot_mutate_constant`, referência de padrão, não reutilizável directamente — confirmado pela sonda).

---

## 1. Sonda — mecanismo exacto do vanilla

```
grep -n "read-only and cannot be modified\|Capturer\|BindingKind" \
  lab/typst-original/crates/typst-library/src/foundations/scope.rs
```

### 1.1 Mensagens exactas (confirmadas palavra por palavra)

```
#let x = 1
#let f() = { x = 2 }
#f()
  → error: variables from outside the function are read-only and cannot be modified

#let x = 1
#context { x = 2 }
  → error: variables from outside the context expression are read-only and cannot be modified
```

### 1.2 Onde o vanilla decide `Capturer::Function` vs `Capturer::Context`

`CapturesVisitor::new(scopes, capturer)` — `Capturer::Function` em
`typst-eval/src/call.rs:576` (fecho de closure normal), `Capturer::Context`
em `typst-eval/src/code.rs:394` (bloco `context`). Decidido **na definição**
da closure/`context`, não na chamada: o visitor percorre o corpo e, para
cada identificador livre encontrado, cria `binding.capture(self.capturer)`
— um único `capturer` para todo o scope capturado dessa closure/context.

### 1.3 Mecanismo de falha — diferente do que P772n usou para `base`

`vm.scopes.get_mut(&self).and_then(|b| b.write().map_err(Into::into))`
(`typst-eval/src/access.rs:38-39`). No vanilla, a variável capturada **é
encontrada** por `get_mut` (o scope capturado fica numa camada alcançável,
não separada) — é `Binding::write()`, chamado a seguir, que falha ao ver
`kind == BindingKind::Captured(capturer)` (`foundations/scope.rs:313-323`).

**Isto é estruturalmente diferente do mecanismo de `base`/`cannot_mutate_
constant` que P772n implementou** (que é exclusão estrutural de âmbito —
`get_mut` nunca alcança `base`, sem precisar de nenhuma flag em `Binding`).
Aqui, a protecção real do vanilla é uma flag *por-binding* (`BindingKind`) —
exactamente a hipótese que P772n tinha e **descartou** para o caso de
`base`. Confirma o que o prompt do passo antecipava: os dois mecanismos são
irmãos na superfície (ambos "mutação falha com mensagem X") mas distintos
por baixo.

### 1.4 Confirmação do estado actual do cristalino (revalidação pós-P772n)

```
$ grep -n "fn get_mut\|captured" 01_core/src/engine/scopes.rs 01_core/src/engine/eval/bindings.rs
```

`Scopes::get_mut` continua a nunca pesquisar `captured` (comportamento de
P715, não alterado por P772n). `access()` (pós-P772n) tratava qualquer
`get_mut == None` como `is_constant` → `unknown_variable` — **não** tinha
nenhuma verificação de `captured` na cadeia. Confirmado empiricamente:
`#let x = 1; #let f() = { x = 2 }; #f()` e `#context { x = 2 }` davam
ambos `"unknown variable: x"` no cristalino antes deste passo.

---

## 2. Implementação

Replicar a estrutura exacta do vanilla (flag por-binding, `Binding::write()`)
exigiria `kind: BindingKind` em `Binding` — schema ainda adiado por
ADR-0017, que P772n já confirmou não ser necessário para o caso irmão.
Mantido assim: `Binding` **não** foi alterado. Em vez disso, replicado só
o **observável** (ADR-0107) por um mecanismo distinto, análogo ao padrão
já usado por P772n para `is_constant`:

1. `Capturer` (novo enum, `entities/scope.rs`): `Function` | `Context`.
   Paridade vanilla `foundations/scope.rs::Capturer`.
2. `ClosureRepr.capturer: Capturer` (novo campo, `entities/func.rs`) —
   decidido na **definição**, não na chamada, tal como no vanilla:
   - `Capturer::Function` em `eval_closure_expr` (`eval/closures.rs`,
     `Expr::Closure`).
   - `Capturer::Context` na construção do `ContextBlockElem`
     (`eval/mod.rs`, `Expr::Contextual`) — este bloco já reusa
     `Func::closure(ClosureRepr {...})` internamente (mesmo mecanismo de
     closures normais, confirmado ao ler o código: `context { }` no
     cristalino **é** uma closure sem parâmetros).
3. `Scopes.captured_by: Option<Capturer>` (novo campo, `rules/scopes.rs`)
   + `with_parent(parent, capturer)` (assinatura alterada — 2 call sites
   em produção: `apply_closure` e a construção do bloco `context`, mais
   3 sites de teste) + `captured_by(name) -> Option<Capturer>` (novo
   método, mesma forma de `is_constant`: verifica `top`/`scopes` primeiro
   — sombra local continua mutável — depois `captured`).
4. `apply_closure` (`eval/closures.rs`) propaga `closure.capturer` para
   `Scopes::with_parent`.
5. `eval/bindings.rs::access`, braço `Expr::Ident` em mutação: ordem
   `captured_by` → `is_constant` → `unknown_variable`. `captured_by`
   verificado **primeiro**, paridade com o vanilla (`get_mut` bem
   sucedido sobre um `Captured` falha em `.write()`, sem nunca chegar a
   considerar `base`).

### Testes novos

`p772q_captured_by_function`, `p772q_captured_by_context`,
`p772q_captured_by_none_para_nome_inexistente`,
`p772q_sombra_local_de_nome_capturado_continua_mutavel`,
`p772q_captured_by_precede_is_constant` (`01_core/src/engine/scopes.rs`).

---

## 3. Validação

### 3.1 Casos do passo

```
#let x = 1; #let f() = { x = 2 }; #f()
  → error: variables from outside the function are read-only and cannot be modified   ✓ idêntico ao vanilla

#let x = 1; #context { x = 2 }
  → error: variables from outside the context expression are read-only and cannot be modified   ✓ idêntico ao vanilla

#let x = 1; #{ x = 2 }; #x
  → sem erro, "2"   ✓ não regride (mutação normal fora de closure)

#{ calc = 5 }
  → error: cannot mutate a constant: calc   ✓ não regride (P772n)
```

### 3.2 Casos extra medidos (precedência e sombra)

```
#let x = 1; #let f(x) = { x = 2; x }; #f(10); #x
  → cristalino: "21"; vanilla: "21"   ✓ idêntico (parâmetro sombreia a captura, mutável)

#let f() = { calc = 5 }; #f()
  → cristalino: "variables from outside the function..."
  → vanilla:    "variables from outside the function..."   ✓ idêntico
    (confirma a precedência captured_by → is_constant: calc É alcançável via
    base, mas está TAMBÉM capturado pela closure — vanilla dá a mensagem de
    "captured", não a de "constant", e o cristalino agora replica isso)
```

### 3.3 Suite completa

```
cargo test --workspace
  4188 (typst-core, +5 novos) + 645 + 33 + 2 + 29 + 2, 0 falhas
crystalline-lint .
  0 violações (mesmo warning V7 pré-existente, não relacionado)
```

---

## Critério de fecho do passo (`typst-passo-772q.md`)

- [x] Mensagens exatas do vanilla confirmadas para `Capturer::Function` e
      `Capturer::Context`.
- [x] Mecanismo de detecção de "capturado mas não mutável" implementado,
      distinto do mecanismo de `is_constant` de P772n (confirmado: vanilla
      usa flag por-binding para este caso, exclusão estrutural para o
      outro — os dois são genuinamente diferentes, não uma coincidência
      de nomenclatura).
- [x] Precedência de verificação (`captured_by` → `is_constant` →
      `unknown`) confirmada contra o vanilla, incluindo o caso de fronteira
      (nome de `base` também capturado por uma closure).
- [x] Os dois casos (`f()`, `context`) dão a mensagem correta.
- [x] Sem regressão em mutação normal nem em `cannot_mutate_constant`.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772q.md`.

---

## Próximo passo

§2.4 — hint de subtracção em `unknown_variable` (`#foo-bar` → hint "if you
meant to use subtraction..."). §2.6 (avisos de depreciação) continua em
aberto, severidade baixa. Com P772l/P772n/P772q fechados, `foundations::
scope` está classificado por completo excepto §2.4/§2.6.
