---
# P740 — Cinco itens pequenos restantes: aviso de return, repr de `Args`, ordem no erro de argumento extra, `rgb(ratio)`, repr de NaN

> **Passo:** 740
> **Data:** 2026-07-10
> **Foco:** Últimos itens pequenos/cosméticos da lista de controlo, todos independentes entre si, agrupados por conveniência. Cada um mantém a sua própria sonda e pode ser adiado individualmente se se revelar maior do que parece.
> **Tipo:** Sonda + Implementação, cinco sub-partes independentes.
> **Tamanho:** M no total.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P738 (aviso de return), P733 (repr de `Args`, ordem no erro), P736 (`linear-rgb`, padrão a espelhar em `rgb`), P739 (repr de NaN).

---

## Parte A — Aviso "this return unconditionally discards the content before it"

### Sonda

```bash
cat > /tmp/p740a-warn.typ <<'EOF'
#{ [conteúdo] return "x" }
EOF
lab/typst-original/target/release/typst compile /tmp/p740a-warn.typ /tmp/p740a-vanilla.pdf 2>&1
```

Confirmar a mensagem exacta do aviso, e se há um "hint" adicional sobre `state`/`counter` (o passo P738 mencionou isto como possivelmente exigindo query de selectores).

### Implementação

Implementar o aviso, seguindo `code.rs:413-430` do vanilla (`warn_for_discarded_content`), no braço de `return` já corrigido em P715/P723.

### Critério de fecho da Parte A

- [ ] Aviso implementado, mensagem igual à do vanilla, testado.

---

## Parte B — Repr de `Value::Args` (sink) é lossy

### Sonda

```bash
cat > /tmp/p740b-args-repr.typ <<'EOF'
#let f(a, ..rest) = rest
#repr(f(1, z: 2, y: 3))
EOF
lab/typst-original/target/release/typst compile /tmp/p740b-args-repr.typ /tmp/p740b-vanilla.pdf
pdftotext /tmp/p740b-vanilla.pdf -
```

### Implementação

Corrigir o `repr` de `Value::Args` para incluir os pares nomeados e posicionais, não só `arguments(...)`.

### Critério de fecho da Parte B

- [ ] Repr correcto, testado com posicionais e nomeados.

---

## Parte C — Ordem entre tipos no erro de argumento extra

### Sonda

```bash
cat > /tmp/p740c-ordem.typ <<'EOF'
#let f(a) = a
#f(1, z: 2, 3)
EOF
lab/typst-original/target/release/typst compile /tmp/p740c-ordem.typ /tmp/p740c-vanilla.pdf 2>&1
```

Confirmar se o vanilla reporta mesmo o nomeado primeiro (ordem original de `Args.items`, lista única), como P733 registou.

### Implementação

Se confirmado como um caso genuinamente alcançável e não só teórico: ajustar `apply_closure` para reportar o primeiro erro na ordem original, não sempre posicional primeiro. Se a mudança exigir reestruturar `items`/`named` para uma lista única (mudança grande): registar como scope-out reforçado, com o custo medido.

### Critério de fecho da Parte C

- [ ] Corrigido se o custo for pequeno; scope-out reforçado com custo medido se for grande.

---

## Parte D — `rgb(ratio, ratio, ratio)`

### Sonda

```bash
cat > /tmp/p740d-rgb-ratio.typ <<'EOF'
#rgb(50%, 0%, 0%)
EOF
lab/typst-original/target/release/typst compile /tmp/p740d-rgb-ratio.typ /tmp/p740d-vanilla.pdf
pdftotext /tmp/p740d-vanilla.pdf -
```

### Implementação

Espelhar exactamente a correcção já feita em `linear-rgb` (P736) para `native_rgb` — aceitar `Ratio` directamente, além de `Int`.

### Critério de fecho da Parte D

- [ ] `rgb(50%, ...)` funciona, testado contra o vanilla.

---

## Parte E — Repr de NaN

### Sonda

```bash
cat > /tmp/p740e-nan-repr.typ <<'EOF'
#repr(calc.inf - calc.inf)
EOF
lab/typst-original/target/release/typst compile /tmp/p740e-nan-repr.typ /tmp/p740e-vanilla.pdf
pdftotext /tmp/p740e-vanilla.pdf -
```

### Implementação

Corrigir `repr_value` para `Float` NaN produzir `float.nan`, não `NaN.0`.

### Critério de fecho da Parte E

- [ ] Repr correcto, testado.

---

## Validação final (todas as partes)

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Cada uma das cinco partes tratada individualmente.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p740.md`, com hash do commit.
- [ ] Cada item actualizado individualmente em `achados-adiados-cetz.md`.
