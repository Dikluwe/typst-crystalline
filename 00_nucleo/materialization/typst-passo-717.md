---
# P717 — Métodos mutantes (`push`, `pop`, `insert`, `remove`)

> **Passo:** 717
> **Data:** 2026-07-10
> **Foco:** P716 confirmou que `cetz` usa `arr.push(...)` (e provavelmente `pop`/`insert`/`remove`), o mecanismo irmão de `is_accessor_method`/`call_method_access` (P716) — `is_mutating_method`/`call_method_mut` no vanilla (`typst-eval/methods.rs:9-16`), sobre a mesma fundação `access()` já construída em P716.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S se a sonda confirmar que `access()` (P716) já resolve o alvo sem mudanças; M caso contrário.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P716 (`access()`, `is_accessor_method`, mecanismo de mutação já construído — reaproveitável aqui).

---

## Sonda

### Confirmar a lista completa de mutating methods no vanilla

```bash
grep -n "fn is_mutating_method\|fn call_method_mut" lab/typst-original/crates/typst-eval/src/methods.rs
```

Não assumir que é só `push`/`pop`/`insert`/`remove` — confirmar a lista exacta, e para que tipos cada um se aplica (Array, Dict, ambos?).

### Confirmar o custo de scope-out, se algum método não for usado por `cetz`

```bash
grep -rn "\.push(\|\.pop(\|\.insert(\|\.remove(\|\.dedup(\|\.sort(\|\.shuffle(" ~/.cache/typst/packages/preview/cetz/0.5.2/src/*.typ ~/.cache/typst/packages/preview/cetz/0.5.2/src/**/*.typ 2>/dev/null
```

Se algum método da lista vanilla não tiver uso confirmado em `cetz`, medir (não estimar) o custo de o implementar mesmo assim: quantas linhas de `methods.rs` cobre, se partilha código com os já implementados.

### Confirmar o comportamento exacto de cada método usado

```bash
cat > /tmp/p717-mut.typ <<'EOF'
#let arr = (1, 2, 3)
#{ arr.push(4) }
#arr

#let arr2 = (1, 2, 3)
#{ arr2.pop() }
#arr2

#let arr3 = (1, 2, 3)
#{ arr3.insert(1, 99) }
#arr3

#let arr4 = (1, 2, 3)
#{ arr4.remove(1) }
#arr4
EOF
lab/typst-original/target/release/typst compile /tmp/p717-mut.typ /tmp/p717-vanilla.pdf
pdftotext /tmp/p717-vanilla.pdf -
```

### Confirmar se `access()` de P716 já resolve o alvo sem mudanças

```bash
grep -n "fn access\b" 01_core/src/rules/eval/bindings.rs
```

Confirmar se os métodos mutantes podem reaproveitar `access()` directamente (obtendo `&mut Value`, depois mutando in-place com `.push()`/etc. do Rust), ou se precisam de um caminho diferente.

### Critério de fecho da sonda

- [ ] Lista completa de mutating methods confirmada.
- [ ] Uso real em `cetz` confirmado, com custo medido para qualquer método sem uso confirmado.
- [ ] Comportamento exacto de cada método usado confirmado contra o vanilla.
- [ ] Confirmado se `access()` (P716) já é reaproveitável directamente.
- [ ] **[scope-out]** Métodos sem uso confirmado em `cetz` e com custo alto: decisão registada aqui, não só em prosa.

---

## Implementação

Implementar os métodos confirmados, reaproveitando `access()` (P716) sempre que possível.

### Critério de fecho da implementação

- [ ] Métodos usados por `cetz` implementados e testados.
- [ ] Erros de tipo/aridade seguem a mesma disciplina de P716 (mensagens verbatim do vanilla).
- [ ] `access()`/mecanismo de P716 sem regressão.
- [ ] **[scope-out]** Cada método não implementado, com a razão medida (não estimada) documentada aqui.

---

## Validação

```bash
./target/release/typst /tmp/p717-mut.typ /tmp/p717-depois.pdf
pdftotext /tmp/p717-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

### Campos fixos de progresso (registar sempre, mesma disciplina de P715/P716)

```bash
cat > /tmp/p717-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst /tmp/p717-cetz.typ /tmp/p717-cetz.pdf
echo "Exit code: $?"
```

Registar no relatório: tempo de compilação, exit code, e — se ainda falhar — o próximo bloqueio com `file:line` exacto (não só a mensagem de erro).

Se produzir PDF: usar `mutool draw` para gerar PNG dos dois lados (cristalino e vanilla) e, se possível, um diff de pixels com limiar, em vez de só "comparação visual" por inspecção.

---

## Critério de fecho do passo

- [ ] Sonda completa, lista confirmada, custo de scope-out medido se aplicável.
- [ ] Implementado e testado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — campos fixos de progresso registados (tempo, exit code, próximo bloqueio com `file:line`, ou sucesso com diff de pixels).
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p717.md`, com hash do commit.
- [ ] Grep às ADRs em vigor pelos termos centrais deste passo (`Access`, mutação, `Scopes::get_mut`) antes de fechar o texto, confirmando que nada contradiz o já decidido.
