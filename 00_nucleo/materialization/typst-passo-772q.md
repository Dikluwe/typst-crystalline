---
# P772q — Mensagem correta para mutação de variável capturada (`Capturer::Function`/`Context`)

> **Passo:** 772q
> **Data:** 2026-07-17
> **Foco:** P772l (§2.2) confirmou que `#let x = 1; #let f() = { x = 2 }; #f()` dá `"unknown variable: x"` no cristalino, mas o vanilla dá `"variables from outside the function are read-only and cannot be modified"` — `x` existe (capturado pela closure), o problema é ser só-leitura, não ausente. Variante `#context { x = 2 }` usa mensagem irmã ("outside the context expression"). P772n resolveu `cannot_mutate_constant` por um mecanismo diferente (estrutural, `base` vs `top`/`scopes`) que não cobre este caso — closures capturam por valor num scope próprio, não vêm de `base`.
> **Tipo:** Implementação directa (causa já mapeada por P772l).
> **Tamanho:** S/M.
> **ADR-0108 EM VIGOR** — confirmar mecanismo exato do vanilla antes de implementar, dado que P772n já mostrou que hipóteses sobre este código costumam estar incompletas.
> **Dependências:** P772l (achado original), P772n (mecanismo irmão já implementado para `cannot_mutate_constant`, referência de padrão mas não reutilizável directamente).

---

## Sonda — mecanismo exato do vanilla

```bash
grep -n "read-only and cannot be modified\|Capturer\|BindingKind" lab/typst-original/crates/typst-library/src/foundations/scope.rs lab/typst-eval/src/*.rs 2>/dev/null
```

Confirmar:
1. Onde exatamente o vanilla marca um binding como "capturado" e por qual `Capturer` (função vs contexto) no momento da captura de closure.
2. A mensagem exata para os dois casos (`Capturer::Function`, `Capturer::Context`), palavra por palavra.
3. Se há um terceiro caso (`Capturer::None` ou similar) que precise de distinção.

```bash
cat > /tmp/p772q-func.typ <<'EOF'
#let x = 1
#let f() = { x = 2 }
#f()
EOF
lab/typst-original/target/release/typst compile /tmp/p772q-func.typ 2>&1

cat > /tmp/p772q-context.typ <<'EOF'
#let x = 1
#context { x = 2 }
EOF
lab/typst-original/target/release/typst compile /tmp/p772q-context.typ 2>&1
```

### Confirmar o estado actual do cristalino (P772l já mapeou, revalidar)

```bash
grep -n "fn get_mut\|captured" 01_core/src/rules/scopes.rs 01_core/src/rules/eval/bindings.rs
```

`Scopes::get_mut` não consulta `captured` (comentário do próprio código, P715, referido por P772l). `access()` trata qualquer `get_mut == None` uniformemente como "unknown variable" — confirmar se isso ainda é verdade depois de P772n ter mexido em `access()`.

---

## Implementação

1. Adicionar a distinção necessária para saber, no momento em que `get_mut` falha, se o nome existe em `captured` (mas não pode ser mutado por estar capturado) — não confundir com o mecanismo de `is_constant` de P772n (`base`), que é outro caminho de falha inteiramente.
2. Se `captured` contém o nome: emitir a mensagem correta (`Capturer::Function` ou `Capturer::Context`, conforme o contexto de avaliação actual — closure normal vs bloco `context`).
3. Se não: cai no comportamento já existente (`unknown_variable` ou `cannot_mutate_constant`, conforme P772n).

Confirmar que a ordem de verificação (captured → constant → unknown) bate com a precedência real do vanilla, não inventada.

---

## Validação

```bash
./target/release/typst compile /tmp/p772q-func.typ 2>&1
./target/release/typst compile /tmp/p772q-context.typ 2>&1
```

Confirmar mensagens idênticas ao vanilla nos dois casos.

```bash
# Não regressão: mutação de variável capturada por valor normal (não em closure) continua a funcionar
cat > /tmp/p772q-normal.typ <<'EOF'
#let x = 1
#{ x = 2 }
#x
EOF
./target/release/typst compile /tmp/p772q-normal.typ 2>&1
```

```bash
# Não regressão: cannot_mutate_constant (P772n) continua a funcionar
cat > /tmp/p772q-const.typ <<'EOF'
#{ calc = 5 }
EOF
./target/release/typst compile /tmp/p772q-const.typ 2>&1
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Mensagens exatas do vanilla confirmadas para `Capturer::Function` e `Capturer::Context`.
- [ ] Mecanismo de detecção de "capturado mas não mutável" implementado, distinto do mecanismo de `is_constant` de P772n.
- [ ] Precedência de verificação (captured/constant/unknown) confirmada contra o vanilla.
- [ ] Os dois casos (`f()`, `context`) dão a mensagem correta.
- [ ] Sem regressão em mutação normal nem em `cannot_mutate_constant`.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772q.md`.

---

## Próximo passo

§2.4 (hint de subtração em `unknown_variable`).
